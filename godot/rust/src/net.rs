//! Connection to the SpacetimeDB server (module `rose`).
//!
//! GDScript polls this node: `get_entities()` returns every entity with its position on
//! the server's motion path at the current time, and `poll_damage_events()` drains the
//! hits since the last call. Messages are processed in `_process` with `frame_tick`, so
//! table callbacks run on the main thread.

use std::sync::{Arc, Mutex};

use godot::{classes::INode, prelude::*};
use spacetimedb_sdk::{
    __codegen::{WithInsert, WithUpdate},
    DbContext, Table,
};

use rose_data::{AmmoIndex, EquipmentIndex};
use rose_game_common::components::{AbilityValues, DroppedItem, Equipment, Hotbar, HotbarSlot, Inventory, ItemSlot, SkillList};

use crate::items::item_dict;

use crate::module_bindings::*;

fn local_now_us() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_micros() as i64)
}

/// Position on a straight-line move at server time `t_us`, in ROSE cm. Same formula as the module.
fn motion_position(m: &Motion, t_us: i64) -> (f32, f32) {
    let (dx, dy) = (m.to_x - m.from_x, m.to_y - m.from_y);
    let d = (dx * dx + dy * dy).sqrt();
    if d < 0.01 || m.speed <= 0.0 {
        return (m.to_x, m.to_y);
    }
    let f = (((t_us - m.started_at_us).max(0) as f32 / 1e6) * m.speed / d).min(1.0);
    (m.from_x + dx * f, m.from_y + dy * f)
}

/// Splits `ws://host:3000/name` into the server address and the database name.
fn split_database(uri: &str) -> (String, String) {
    let uri = uri.trim().trim_end_matches('/');
    let after_scheme = uri.find("://").map_or(0, |i| i + 3);
    match uri[after_scheme..].find('/') {
        Some(slash) if after_scheme + slash + 1 < uri.len() => {
            let (address, name) = uri.split_at(after_scheme + slash);
            (address.to_string(), name[1..].to_string())
        }
        _ => (uri.to_string(), "rose".to_string()),
    }
}

/// ROSE (x, y) in cm to Godot (x, z) in metres.
fn to_godot(x: f32, y: f32) -> (f32, f32) {
    (x / 100.0, -y / 100.0)
}

/// Item numbers the character model is built from: male, face, hair, head, body, hands,
/// feet, weapon, sub weapon (RoseCharacter.build's arguments).
fn player_look(p: &Player) -> VarArray {
    let equipment: Equipment = serde_json::from_str(&p.equipment).unwrap_or_default();
    let item = |index: EquipmentIndex| {
        equipment.get_equipment_item(index).map_or(0, |item| item.item.item_number as i64)
    };
    let mut look = VarArray::new();
    look.push(&(p.gender == 0).to_variant());
    for value in [
        p.face as i64,
        p.hair as i64,
        item(EquipmentIndex::Head),
        item(EquipmentIndex::Body),
        item(EquipmentIndex::Hands),
        item(EquipmentIndex::Feet),
        item(EquipmentIndex::Weapon),
        item(EquipmentIndex::SubWeapon),
    ] {
        look.push(&value.to_variant());
    }
    look
}

#[derive(Default)]
struct Shared {
    damage: Vec<DamageEvent>,
    xp: Vec<XpEvent>,
    /// Messages for the player: server notices and refused actions.
    notices: Vec<String>,
    /// Smallest (local receive time - server start time) seen on a fresh motion change.
    /// Covers clock skew between this PC and the server plus the fastest one-way delay.
    clock_offset_us: Option<i64>,
}

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct RoseNet {
    base: Base<Node>,
    conn: Option<DbConnection>,
    shared: Arc<Mutex<Shared>>,
    error: String,
}

impl RoseNet {
    fn server_now_us(&self) -> i64 {
        local_now_us() - self.shared.lock().unwrap().clock_offset_us.unwrap_or(0)
    }

    fn my_id(&self) -> Option<u64> {
        let c = self.conn.as_ref()?;
        c.try_identity().and_then(|i| c.db.player().identity().find(&i)).and_then(|p| p.entity_id)
    }

    fn fail(&mut self, error: String) {
        godot_warn!("rose: server connection: {error}");
        self.error = error;
        self.conn = None;
    }
}

#[godot_api]
impl INode for RoseNet {
    fn process(&mut self, _delta: f64) {
        let Some(c) = self.conn.as_ref() else { return };
        if let Err(error) = c.frame_tick() {
            self.fail(format!("{error}"));
        }
    }
}

#[godot_api]
impl RoseNet {
    /// Connects and subscribes to every public table. The identity token is kept in
    /// `token_path`, so the same file gives the same character on the next run.
    /// `uri` may end in a database name (`wss://host/rose-friends`); the default is `rose`.
    #[func]
    fn connect_to(&mut self, uri: GString, token_path: GString) -> bool {
        let (uri, database) = split_database(&uri.to_string());
        let token_path = token_path.to_string();
        let shared = self.shared.clone();
        let saved = token_path.clone();
        let result = DbConnection::builder()
            .with_uri(uri)
            .with_database_name(database)
            .with_token(std::fs::read_to_string(&token_path).ok())
            .on_connect(move |_, _, token| {
                std::fs::write(&saved, token).ok();
            })
            .build();
        let conn = match result {
            Ok(conn) => conn,
            Err(error) => {
                self.fail(format!("{error}"));
                return false;
            }
        };

        let s = shared.clone();
        conn.db.damage_event().on_insert(move |_, ev| s.lock().unwrap().damage.push(ev.clone()));
        let s = shared.clone();
        conn.db.xp_event().on_insert(move |_, ev| s.lock().unwrap().xp.push(ev.clone()));
        let s = shared.clone();
        conn.db.notice().on_insert(move |ctx, ev| {
            if ctx.try_identity() == Some(ev.identity) {
                s.lock().unwrap().notices.push(ev.text.clone());
            }
        });
        let s = shared.clone();
        conn.db.motion().on_update(move |_, _, new| {
            let sample = local_now_us() - new.started_at_us;
            let mut s = s.lock().unwrap();
            s.clock_offset_us = Some(s.clock_offset_us.map_or(sample, |o| o.min(sample)));
        });
        conn.subscription_builder().subscribe_to_all_tables();
        self.error.clear();
        self.conn = Some(conn);
        true
    }

    #[func]
    fn disconnect_from(&mut self) {
        if let Some(c) = self.conn.take() {
            c.disconnect().ok();
        }
    }

    #[func]
    fn is_online(&self) -> bool {
        self.conn.as_ref().is_some_and(|c| c.is_active())
    }

    /// True once our own entity and its motion and combat rows have arrived.
    #[func]
    fn is_ready(&self) -> bool {
        let (Some(c), Some(id)) = (self.conn.as_ref(), self.my_id()) else { return false };
        c.db.motion().entity_id().find(&id).is_some() && c.db.combat().entity_id().find(&id).is_some()
    }

    #[func]
    fn get_error(&self) -> GString {
        GString::from(self.error.as_str())
    }

    #[func]
    fn my_entity_id(&self) -> i64 {
        self.my_id().map_or(-1, |id| id as i64)
    }

    /// Every entity in our zone: id, kind ("player", "monster" or "npc"), name, npc_id, position
    /// x/z and destination to_x/to_z (Godot metres), moving, speed (m/s), chasing, level,
    /// range (m), hp, max_hp, mp, max_mp, target (-1 for none), swinging, hit_in (seconds
    /// until the swing's hit frame), dead, for players look (see player_look), for NPCs
    /// direction (degrees) and store, and while casting a skill cast_skill, cast_started (a
    /// server time, to tell casts apart), cast_motion and action_motion (motion ids, -1 for
    /// none), cast_effect_in and cast_ends_in (seconds) and cast_target.
    #[func]
    fn get_entities(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let t = self.server_now_us();
        let zone = self.my_id().and_then(|id| c.db.entity().entity_id().find(&id)).map(|e| e.zone_id);
        let looks: std::collections::HashMap<u64, VarArray> =
            c.db.player().iter().filter_map(|p| p.entity_id.map(|id| (id, player_look(&p)))).collect();

        for e in c.db.entity().iter() {
            if zone.is_some_and(|z| z != e.zone_id) {
                continue;
            }
            let Some(m) = c.db.motion().entity_id().find(&e.entity_id) else { continue };
            let (x, y) = motion_position(&m, t);
            let (gx, gz) = to_godot(x, y);
            let (tx, tz) = to_godot(m.to_x, m.to_y);
            let moving = (x - m.to_x).abs() > 0.5 || (y - m.to_y).abs() > 0.5;

            let mut d = VarDictionary::new();
            d.set("id", e.entity_id as i64);
            d.set(
                "kind",
                match e.kind {
                    EntityKind::Player => "player",
                    EntityKind::Monster => "monster",
                    EntityKind::Npc => "npc",
                },
            );
            if let Some(cast) = c.db.skill_cast().entity_id().find(&e.entity_id) {
                if let Some(started) = cast.started_at_us {
                    let skill = crate::data::get()
                        .and_then(|g| rose_data::SkillId::new(cast.skill_id).and_then(|id| g.skills.get_skill(id)));
                    d.set("cast_skill", cast.skill_id as i64);
                    d.set("cast_started", started);
                    d.set("cast_motion", skill.and_then(|s| s.casting_motion_id).map_or(-1, |m| m.get() as i64));
                    d.set("action_motion", skill.and_then(|s| s.action_motion_id).map_or(-1, |m| m.get() as i64));
                    d.set("cast_effect_in", (cast.effect_at_us - t) as f64 / 1e6);
                    d.set("cast_ends_in", (cast.ends_at_us - t) as f64 / 1e6);
                    if let Some(target) = cast.target {
                        d.set("cast_target", target as i64);
                    }
                }
            }
            if let Some(npc) = c.db.npc().entity_id().find(&e.entity_id) {
                d.set("direction", npc.direction);
                d.set("store", npc.has_store);
            }
            d.set("name", e.name.as_str());
            d.set("npc_id", e.npc_id as i64);
            d.set("x", gx);
            d.set("z", gz);
            d.set("to_x", tx);
            d.set("to_z", tz);
            d.set("moving", moving);
            d.set("speed", m.speed / 100.0);
            d.set("chasing", m.chase_target.is_some());
            if let Some(look) = looks.get(&e.entity_id) {
                d.set("look", look);
            }
            if let Some(st) = c.db.stats().entity_id().find(&e.entity_id) {
                d.set("level", st.level);
                d.set("range", st.attack_range / 100.0);
            }
            if let Some(cb) = c.db.combat().entity_id().find(&e.entity_id) {
                d.set("hp", cb.hp);
                d.set("max_hp", cb.max_hp);
                d.set("mp", cb.mp);
                d.set("max_mp", cb.max_mp);
                d.set("target", cb.attack_target.map_or(-1, |t| t as i64));
                d.set("swinging", cb.swing_hit_at_us.is_some());
                d.set("hit_in", cb.swing_hit_at_us.map_or(0.0, |at| (at - t) as f64 / 1e6));
                d.set("dead", cb.dead_until_us.is_some_and(|until| until > t) || cb.hp <= 0);
            }
            out.push(&d.to_variant());
        }
        out
    }

    /// Why there is nothing to play yet, or "" when the server has its game data.
    #[func]
    fn get_server_notice(&self) -> GString {
        let Some(c) = self.conn.as_ref() else { return GString::new() };
        match c.db.game_data_status().id().find(&0) {
            Some(s) if s.ready => GString::new(),
            Some(s) if !s.error.is_empty() => GString::from(format!("The server's game data failed to load: {}", s.error).as_str()),
            _ => GString::from("The server has no game data yet. Its host needs to run upload-game-data.sh."),
        }
    }

    /// Our character: name, zone, level, xp, xp_needed, stat_points, skill_points, money, the six
    /// basic stats (str, dex, int, con, cha, sen) with their raise costs (str_cost, ...; -1 at
    /// the maximum), and the ability values attack, defence, hit, avoid, critical, resistance,
    /// attack_speed, move_speed, range. Empty until the character is in the world.
    #[func]
    fn get_character(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = self.conn.as_ref() else { return d };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return d };
        d.set("name", p.name.as_str());
        d.set("zone", p.zone_id as i64);
        d.set("level", p.level as i64);
        d.set("xp", p.xp as i64);
        d.set("xp_needed", rose_game_irose::data::levelup_require_xp(p.level) as i64);
        d.set("stat_points", p.stat_points as i64);
        d.set("skill_points", p.skill_points as i64);
        let inventory: Inventory = serde_json::from_str(&p.inventory).unwrap_or_default();
        d.set("money", inventory.money.0);
        for (key, value) in [
            ("str", p.strength),
            ("dex", p.dexterity),
            ("int", p.intelligence),
            ("con", p.concentration),
            ("cha", p.charm),
            ("sen", p.sense),
        ] {
            d.set(key, value);
            let cost = rose_game_irose::data::basic_stat_increase_cost(value).map_or(-1, |c| c as i64);
            d.set(format!("{key}_cost").as_str(), cost);
        }
        if let Some(st) = p.entity_id.and_then(|id| c.db.stats().entity_id().find(&id)) {
            d.set("attack", st.attack_power);
            d.set("defence", st.defence);
            d.set("hit", st.hit);
            d.set("avoid", st.avoid);
            d.set("critical", st.critical);
            d.set("resistance", st.resistance);
            d.set("attack_speed", st.attack_speed);
            d.set("move_speed", st.move_speed);
            d.set("range", st.attack_range / 100.0);
        }
        d
    }

    /// Experience we gained since the last call: xp, level (0 for an admin grant).
    #[func]
    fn poll_xp_events(&self) -> VarArray {
        let mut out = VarArray::new();
        let me = self.conn.as_ref().and_then(|c| c.try_identity());
        for ev in std::mem::take(&mut self.shared.lock().unwrap().xp) {
            if Some(ev.identity) != me {
                continue;
            }
            let mut d = VarDictionary::new();
            d.set("xp", ev.xp as i64);
            d.set("level", ev.level as i64);
            out.push(&d.to_variant());
        }
        out
    }

    /// Spend stat points: 0 STR, 1 DEX, 2 INT, 3 CON, 4 CHA, 5 SEN.
    #[func]
    fn add_basic_stat(&self, stat: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.add_basic_stat_then(stat as u8, move |_, r| report(&s, r)).ok();
        }
    }

    /// Hits since the last call: attacker, defender, amount, critical, killed.
    #[func]
    fn poll_damage_events(&self) -> VarArray {
        let mut out = VarArray::new();
        for ev in std::mem::take(&mut self.shared.lock().unwrap().damage) {
            let mut d = VarDictionary::new();
            d.set("attacker", ev.attacker as i64);
            d.set("defender", ev.defender as i64);
            d.set("amount", ev.amount);
            d.set("critical", ev.is_critical);
            d.set("killed", ev.killed);
            out.push(&d.to_variant());
        }
        out
    }

    /// Estimated clock difference to the server in milliseconds (0 until the first move).
    #[func]
    fn clock_offset_ms(&self) -> f64 {
        self.shared.lock().unwrap().clock_offset_us.unwrap_or(0) as f64 / 1000.0
    }

    /// Click-to-move to a Godot position (metres).
    #[func]
    fn move_to(&self, x: f32, z: f32) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.move_to(x * 100.0, -z * 100.0).ok();
        }
    }

    /// Our character ran into a wall at a Godot position (metres): the server stops the move there.
    #[func]
    fn move_collision(&self, x: f32, z: f32) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.move_collision(x * 100.0, -z * 100.0).ok();
        }
    }

    #[func]
    fn attack(&self, target: i64) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.attack(target as u64).ok();
        }
    }

    #[func]
    fn stop(&self) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.stop().ok();
        }
    }

    #[func]
    fn set_player_name(&self, name: GString) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.set_name(name.to_string()).ok();
        }
    }

    /// Messages since the last call (picked up items, refused actions).
    #[func]
    fn poll_notices(&self) -> PackedStringArray {
        let notices = std::mem::take(&mut self.shared.lock().unwrap().notices);
        notices.iter().map(|n| GString::from(n.as_str())).collect()
    }

    /// Items on the ground in our zone: id, x, z (Godot metres), item (see item_dict; money has
    /// name "N Zuly" and model 0), mine (we may pick it up now).
    #[func]
    fn get_ground_items(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return out };
        let t = self.server_now_us();
        for g in c.db.ground_item().iter().filter(|g| g.zone_id == p.zone_id) {
            let Ok(dropped) = serde_json::from_str::<DroppedItem>(&g.item) else { continue };
            let item = match &dropped {
                DroppedItem::Item(item) => item_dict(item),
                DroppedItem::Money(money) => {
                    let mut d = VarDictionary::new();
                    d.set("name", format!("{} Zuly", money.0).as_str());
                    d.set("model", 0i64);
                    d.set("type", "Money");
                    d.set("quantity", money.0);
                    d
                }
            };
            let (x, z) = to_godot(g.x, g.y);
            let mut d = VarDictionary::new();
            d.set("id", g.drop_id as i64);
            d.set("x", x);
            d.set("z", z);
            d.set("item", &item);
            d.set("mine", g.owner.map_or(true, |o| o == p.identity) || t >= g.owner_until_us);
            out.push(&d.to_variant());
        }
        out
    }

    /// Our items: money, pages (equipment, consumables, materials, vehicles; 30 slots each,
    /// null when empty), equipped (11 slots in EquipmentIndex order: face, head, body, back,
    /// hands, feet, weapon, off-hand, necklace, ring, earring) and ammo (arrows, bullets, shells).
    #[func]
    fn get_inventory(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = self.conn.as_ref() else { return d };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return d };
        let inventory: Inventory = serde_json::from_str(&p.inventory).unwrap_or_default();
        let equipment: Equipment = serde_json::from_str(&p.equipment).unwrap_or_default();
        d.set("money", inventory.money.0);
        let mut pages = VarArray::new();
        for page in [&inventory.equipment, &inventory.consumables, &inventory.materials, &inventory.vehicles] {
            let mut slots = VarArray::new();
            for slot in page.slots.iter() {
                slots.push(&slot.as_ref().map_or(Variant::nil(), |item| item_dict(item).to_variant()));
            }
            pages.push(&slots.to_variant());
        }
        d.set("pages", &pages);
        use EquipmentIndex::*;
        let mut equipped = VarArray::new();
        for index in [Face, Head, Body, Back, Hands, Feet, Weapon, SubWeapon, Necklace, Ring, Earring] {
            let item = equipment.get_equipment_item(index).map(|e| rose_data::Item::Equipment(e.clone()));
            equipped.push(&item.map_or(Variant::nil(), |item| item_dict(&item).to_variant()));
        }
        d.set("equipped", &equipped);
        let mut ammo = VarArray::new();
        for index in [AmmoIndex::Arrow, AmmoIndex::Bullet, AmmoIndex::Throw] {
            let item = equipment.get_ammo_item(index).map(|a| rose_data::Item::Stackable(a.clone()));
            ammo.push(&item.map_or(Variant::nil(), |item| item_dict(&item).to_variant()));
        }
        d.set("ammo", &ammo);
        d
    }

    /// Our skills by page (0 basic, 1 active, 2 passive, 3 clan): each page 30 slots, null
    /// or a skill as skills::skill_dict, plus cooldown (seconds left).
    #[func]
    fn get_skills(&self) -> VarArray {
        let mut pages = VarArray::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return pages };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return pages };
        let skill_list: SkillList = serde_json::from_str(&p.skill_list).unwrap_or_default();
        let t = self.server_now_us();
        let cooldowns: Vec<SkillCooldownRow> = p
            .entity_id
            .map(|id| c.db.skill_cooldown().iter().filter(|r| r.entity_id == id).collect())
            .unwrap_or_default();
        for page_type in 0..4usize {
            let mut slots = VarArray::new();
            if let Some(page) = skill_list.get_page(page_type) {
                for slot in page.skills.iter() {
                    let skill = slot.and_then(|id| game.skills.get_skill(id));
                    let Some(skill) = skill else {
                        slots.push(&Variant::nil());
                        continue;
                    };
                    let mut d = crate::skills::skill_dict(skill);
                    let key = match skill.cooldown {
                        rose_data::SkillCooldown::Skill { .. } => skill.id.get() as u32,
                        rose_data::SkillCooldown::Group { group, .. } => 100_000 + group.get() as u32,
                    };
                    let until = cooldowns.iter().filter(|r| r.key == key).map(|r| r.until_us).max().unwrap_or(0);
                    d.set("cooldown", ((until - t).max(0)) as f64 / 1e6);
                    slots.push(&d.to_variant());
                }
            }
            pages.push(&slots.to_variant());
        }
        pages
    }

    #[func]
    fn level_up_skill(&self, page: i64, index: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.level_up_skill_then(page as u8, index as u16, move |_, r| report(&s, r)).ok();
        }
    }

    /// Use the skill in a skill slot on an entity (-1 for none) or a ground point (Godot x, z).
    #[func]
    fn cast_skill(&self, page: i64, index: i64, target: i64, x: f32, z: f32) {
        let s = self.shared.clone();
        let target = (target >= 0).then_some(target as u64);
        if let Some(c) = self.conn.as_ref() {
            c.reducers.cast_skill_then(page as u8, index as u16, target, x * 100.0, -z * 100.0, move |_, r| report(&s, r)).ok();
        }
    }

    /// The first hotbar page: 8 entries, null or {kind ("item" or "skill"), page, index}
    /// plus the item (as in get_inventory) or skill (as in get_skills).
    #[func]
    fn get_hotbar(&self) -> VarArray {
        let mut out = VarArray::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return out };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return out };
        let hotbar: Hotbar = serde_json::from_str(&p.hotbar).unwrap_or_default();
        let inventory: Inventory = serde_json::from_str(&p.inventory).unwrap_or_default();
        let skill_list: SkillList = serde_json::from_str(&p.skill_list).unwrap_or_default();
        for slot in hotbar.pages[0].iter() {
            let entry = match slot {
                Some(HotbarSlot::Inventory(item_slot @ ItemSlot::Inventory(page, index))) => inventory.get_item(*item_slot).map(|item| {
                    let mut d = item_dict(item);
                    d.set("kind", "item");
                    d.set("page", *page as i64);
                    d.set("index", *index as i64);
                    d
                }),
                Some(HotbarSlot::Skill(skill_slot)) => skill_list
                    .get_skill(*skill_slot)
                    .and_then(|id| game.skills.get_skill(id))
                    .map(|skill| {
                        let mut d = crate::skills::skill_dict(skill);
                        d.set("kind", "skill");
                        d.set("page", skill_slot.0 as i64);
                        d.set("index", skill_slot.1 as i64);
                        d
                    }),
                _ => None,
            };
            out.push(&entry.map_or(Variant::nil(), |d| d.to_variant()));
        }
        out
    }

    /// Put an item ("item", page, index) or a skill ("skill", page, index) on a hotbar slot,
    /// or clear it ("").
    #[func]
    fn set_hotbar(&self, slot: i64, kind: GString, page: i64, index: i64) {
        let kind = match kind.to_string().as_str() {
            "item" => 1,
            "skill" => 2,
            _ => 0,
        };
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.set_hotbar_slot_then(slot as u8, kind, page as u8, index as u16, move |_, r| report(&s, r)).ok();
        }
    }

    /// Status effects on an entity: name, icon, seconds (left).
    #[func]
    fn get_status_effects(&self, entity_id: i64) -> VarArray {
        let mut out = VarArray::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return out };
        let t = self.server_now_us();
        for row in c.db.status_effect().iter().filter(|r| r.entity_id == entity_id as u64) {
            let Some(data) = rose_data::StatusEffectId::new(row.status_effect_id).and_then(|id| game.status_effects.get_status_effect(id)) else {
                continue;
            };
            let mut d = VarDictionary::new();
            d.set("name", data.name);
            if let Some(texture) = crate::items::icon(crate::items::IconSheet::State, data.icon_id) {
                d.set("icon", &texture);
            }
            d.set("seconds", ((row.expires_at_us - t).max(0)) as f64 / 1e6);
            out.push(&d.to_variant());
        }
        out
    }

    /// Our store buy and sell rates and the world's price rates.
    fn price_rates(&self) -> (i32, i32, WorldRates) {
        let default_rates = WorldRates {
            id: 0,
            xp_rate: 0,
            drop_rate: 0,
            drop_money_rate: 0,
            reward_rate: 0,
            world_price_rate: 100,
            item_price_rate: 50,
            town_price_rate: 100,
        };
        let Some(c) = self.conn.as_ref() else { return (0, 0, default_rates) };
        let rates = c.db.world_rates().id().find(&0).unwrap_or(default_rates);
        let av: Option<AbilityValues> = self
            .my_id()
            .and_then(|id| c.db.stats().entity_id().find(&id))
            .and_then(|st| serde_json::from_str(&st.ability_values).ok());
        let (buy, sell) = av.map_or((0, 0), |av| (av.get_npc_store_buy_rate(), av.get_npc_store_sell_rate()));
        (buy, sell, rates)
    }

    /// An NPC's store: name and tabs, each with a name and items (index, item as in
    /// get_inventory, price). Empty when the NPC has no store.
    #[func]
    fn get_store(&self, entity_id: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return d };
        let Some(npc) = c.db.npc().entity_id().find(&(entity_id as u64)) else { return d };
        let Some(data) = rose_data::NpcId::new(npc.npc_id).and_then(|id| game.npcs.get_npc(id)) else { return d };
        let (buy_rate, _, rates) = self.price_rates();
        d.set("name", data.name);
        let mut tabs = VarArray::new();
        for (tab_index, tab_id) in data.store_tabs.iter().enumerate() {
            let Some(tab) = tab_id.and_then(|id| game.npcs.get_store_tab(id)) else { continue };
            let mut entries: Vec<(u16, rose_data::ItemReference)> = tab.items.iter().map(|(i, r)| (*i, *r)).collect();
            entries.sort_by_key(|(i, _)| *i);
            let mut items = VarArray::new();
            for (index, reference) in entries {
                let Some(base) = game.items.get_base_item(reference) else { continue };
                let Some(item) = rose_data::Item::from_item_data(base, 1) else { continue };
                let price = rose_game_irose::data::npc_store_buy_price(
                    &game.items,
                    reference,
                    buy_rate,
                    rates.item_price_rate,
                    rates.town_price_rate,
                );
                let mut entry = VarDictionary::new();
                entry.set("index", index as i64);
                entry.set("item", &item_dict(&item));
                entry.set("price", price.unwrap_or(0) as i64);
                entry.set("stackable", reference.item_type.is_stackable_item());
                items.push(&entry.to_variant());
            }
            let mut t = VarDictionary::new();
            t.set("tab", tab_index as i64);
            t.set("name", tab.name);
            t.set("items", &items);
            tabs.push(&t.to_variant());
        }
        d.set("tabs", &tabs);
        d
    }

    /// What a store pays for one of the item in an inventory slot, or -1.
    #[func]
    fn sell_price(&self, page: i64, index: i64) -> i64 {
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return -1 };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return -1 };
        let inventory: Inventory = serde_json::from_str(&p.inventory).unwrap_or_default();
        let page = match page {
            0 => &inventory.equipment,
            1 => &inventory.consumables,
            2 => &inventory.materials,
            3 => &inventory.vehicles,
            _ => return -1,
        };
        let Some(Some(item)) = page.slots.get(index.max(0) as usize) else { return -1 };
        let (_, sell_rate, rates) = self.price_rates();
        rose_game_irose::data::npc_store_sell_price(
            &game.items,
            item,
            sell_rate,
            rates.world_price_rate,
            rates.item_price_rate,
            rates.town_price_rate,
        )
        .map_or(-1, |p| p as i64)
    }

    /// Buy from and sell to an NPC's store in one go. buy: [[tab, index, quantity], ...],
    /// sell: [[page, index, quantity], ...].
    #[func]
    fn store_transaction(&self, npc_entity_id: i64, buy: VarArray, sell: VarArray) {
        let triple = |v: Variant| -> Option<(i64, i64, i64)> {
            let a = v.try_to::<VarArray>().ok()?;
            Some((a.get(0)?.try_to().ok()?, a.get(1)?.try_to().ok()?, a.get(2)?.try_to().ok()?))
        };
        let buy: Vec<StoreBuy> = buy
            .iter_shared()
            .filter_map(triple)
            .map(|(tab, index, quantity)| StoreBuy { tab: tab as u8, index: index as u16, quantity: quantity.max(1) as u32 })
            .collect();
        let sell: Vec<StoreSell> = sell
            .iter_shared()
            .filter_map(triple)
            .map(|(page, index, quantity)| StoreSell { page: page as u8, index: index as u16, quantity: quantity.max(1) as u32 })
            .collect();
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.npc_store_transaction_then(npc_entity_id as u64, buy, sell, move |_, r| report(&s, r)).ok();
        }
    }

    /// We walked into a warp gate (its WARP.STB id).
    #[func]
    fn use_warp_gate(&self, warp_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.use_warp_gate_then(warp_id as u16, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn pickup_item(&self, drop_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.pickup_item_then(drop_id as u64, move |_, r| report(&s, r)).ok();
        }
    }

    /// Equip (gear, ammo) from inventory page 0-3, slot 0-29.
    #[func]
    fn equip_item(&self, page: i64, index: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.equip_item_then(page as u8, index as u16, move |_, r| report(&s, r)).ok();
        }
    }

    /// Take off equipped slot 0-10 (see get_inventory's equipped).
    #[func]
    fn unequip_item(&self, slot: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.unequip_item_then(slot as u8, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn unequip_ammo(&self, slot: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.unequip_ammo_then(slot as u8, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn use_item(&self, page: i64, index: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.use_item_then(page as u8, index as u16, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn drop_item(&self, page: i64, index: i64, quantity: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.drop_item_then(page as u8, index as u16, quantity.max(1) as u32, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn move_item(&self, page: i64, from: i64, to: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.move_item_then(page as u8, from as u16, to as u16, move |_, r| report(&s, r)).ok();
        }
    }
}

/// A refused reducer call becomes a notice for the player.
fn report(shared: &Arc<Mutex<Shared>>, result: Result<Result<(), String>, spacetimedb_sdk::__codegen::InternalError>) {
    let message = match result {
        Ok(Ok(())) => return,
        Ok(Err(message)) => message,
        Err(error) => format!("{error}"),
    };
    let mut first = message.chars();
    let message = first.next().map_or(String::new(), |c| c.to_uppercase().collect::<String>() + first.as_str());
    shared.lock().unwrap().notices.push(message);
}
