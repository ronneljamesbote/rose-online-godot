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
    DbContext, Identity, Table,
};

use rose_data::{AmmoIndex, EquipmentIndex, VehiclePartIndex};
use rose_game_common::components::{AbilityValues, DroppedItem, Equipment, Hotbar, HotbarSlot, Inventory, ItemSlot, SkillList};

use crate::{
    conversation::{self, Action, ClientWorld, Conversation, ScriptContext, WorldNpc},
    items::item_dict,
};
use rose_quest::{QuestCharacter, QuestNpc};

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

/// Item numbers of the body, engine, legs and arms on a player's vehicle (0 for none).
fn vehicle_parts(p: &Player) -> VarArray {
    let equipment: Equipment = serde_json::from_str(&p.equipment).unwrap_or_default();
    let mut parts = VarArray::new();
    for index in [VehiclePartIndex::Body, VehiclePartIndex::Engine, VehiclePartIndex::Leg, VehiclePartIndex::Arms] {
        parts.push(&equipment.get_vehicle_item(index).map_or(0, |e| e.item.item_number as i64).to_variant());
    }
    parts
}

#[derive(Default)]
struct Shared {
    damage: Vec<DamageEvent>,
    xp: Vec<XpEvent>,
    /// Messages for the player: server notices and refused actions.
    notices: Vec<String>,
    /// Chat messages not yet shown, and the newest message id seen (the server keeps them
    /// for a minute, so a resubscribe sends some again).
    chat: Vec<ChatMessage>,
    chat_seen: u64,
    /// The first subscription has arrived, so a missing player row means no character yet.
    subscribed: bool,
    /// Answer to create_character: "ok" or why it was refused.
    create_result: Option<String>,
    /// Why the connection failed or closed.
    connection_error: Option<String>,
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
    conversation: Option<Conversation>,
    /// Stores and banks a conversation opened, for GDScript to show.
    windows: Vec<(String, u64)>,
}

impl RoseNet {
    fn server_now_us(&self) -> i64 {
        local_now_us() - self.shared.lock().unwrap().clock_offset_us.unwrap_or(0)
    }

    fn my_id(&self) -> Option<u64> {
        let c = self.conn.as_ref()?;
        c.try_identity().and_then(|i| c.db.player().identity().find(&i)).and_then(|p| p.entity_id)
    }

    /// Our character as the quest rules see it.
    fn quest_character(&self) -> Option<QuestCharacter> {
        let c = self.conn.as_ref()?;
        let p = c.try_identity().and_then(|i| c.db.player().identity().find(&i))?;
        let id = p.entity_id?;
        let combat = c.db.combat().entity_id().find(&id);
        let (x, y) = c
            .db
            .motion()
            .entity_id()
            .find(&id)
            .map_or((p.last_x, p.last_y), |m| motion_position(&m, self.server_now_us()));
        let ability_values = c.db.stats().entity_id().find(&id).and_then(|st| serde_json::from_str(&st.ability_values).ok())?;
        let zone_id = c.db.entity().entity_id().find(&id).map_or(p.zone_id, |e| e.zone_id);
        Some(QuestCharacter {
            gender: p.gender,
            face: p.face,
            hair: p.hair,
            job: p.job,
            level: p.level,
            xp: p.xp,
            stat_points: p.stat_points,
            skill_points: p.skill_points,
            basic_stats: rose_game_common::components::BasicStats {
                strength: p.strength,
                dexterity: p.dexterity,
                intelligence: p.intelligence,
                concentration: p.concentration,
                charm: p.charm,
                sense: p.sense,
            },
            ability_values,
            hp: combat.as_ref().map_or(p.last_hp, |c| c.hp),
            mp: combat.as_ref().map_or(p.last_mp, |c| c.mp),
            zone_id,
            x,
            y,
            team: 2,
            equipment: serde_json::from_str(&p.equipment).unwrap_or_default(),
            inventory: serde_json::from_str(&p.inventory).unwrap_or_default(),
            skill_list: serde_json::from_str(&p.skill_list).unwrap_or_default(),
            quest_state: serde_json::from_str(&p.quest_state).unwrap_or_default(),
            union_membership: serde_json::from_str(&p.union_membership).unwrap_or_default(),
        })
    }

    fn client_world(&self, zone_id: u16) -> ClientWorld {
        let t = self.server_now_us();
        let mut npcs = Vec::new();
        if let Some(c) = self.conn.as_ref() {
            for n in c.db.npc().iter() {
                let (x, y) = c.db.motion().entity_id().find(&n.entity_id).map_or((0.0, 0.0), |m| motion_position(&m, t));
                npcs.push(WorldNpc {
                    npc_id: n.npc_id,
                    npc: QuestNpc { entity_id: n.entity_id, zone_id: n.zone_id, x, y },
                    variables: n.variables.clone(),
                });
            }
        }
        let party = self.conn.as_ref().and_then(|c| {
            let me = c.try_identity()?;
            let party = c.db.party().party_id().find(&c.db.party_member().identity().find(&me)?.party_id)?;
            let member_count = c.db.party_member().iter().filter(|m| m.party_id == party.party_id).count();
            Some(rose_quest::QuestParty { is_leader: party.owner == me, level: 1, member_count })
        });
        let save_zone_name = self.conn.as_ref().and_then(|c| {
            let save = c.db.save_point().identity().find(&c.try_identity()?)?;
            Some(c.db.zone_info().zone_id().find(&save.zone_id)?.name)
        });
        ClientWorld { t, zone_id, npcs, party, save_zone_name }
    }

    /// Run `f` on the open conversation with a fresh script context, then carry out what
    /// the scripts asked for.
    fn with_conversation<R>(&mut self, f: impl FnOnce(&mut Option<Conversation>, &mut ScriptContext) -> R) -> Option<R> {
        let game = crate::data::get()?;
        let ch = self.quest_character()?;
        let name = self
            .conn
            .as_ref()
            .and_then(|c| c.try_identity().and_then(|i| c.db.player().identity().find(&i)))
            .map(|p| p.name)
            .unwrap_or_default();
        let mut world = self.client_world(ch.zone_id);
        let mut actions = Vec::new();
        let mut conversation = self.conversation.take();
        let before = conversation.as_ref().and_then(|c| c.npc_entity);
        let result = {
            let mut cx = ScriptContext { game, ch: &ch, world: &mut world, name: &name, actions: &mut actions };
            f(&mut conversation, &mut cx)
        };
        // The NPC we are talking to, for windows the scripts open on it.
        let talking = conversation.as_ref().and_then(|c| c.npc_entity).or(before);
        self.conversation = conversation;
        for action in actions {
            match action {
                Action::QuestTrigger(trigger) => {
                    let s = self.shared.clone();
                    if let Some(c) = self.conn.as_ref() {
                        c.reducers.quest_trigger_then(trigger, move |_, r| report(&s, r)).ok();
                    }
                }
                Action::OpenStore(entity) => self.windows.push(("store".into(), entity)),
                Action::OpenWindow(kind) => {
                    if let Some(npc) = talking {
                        self.windows.push((kind.into(), npc));
                    }
                }
                Action::Notice(text) => self.shared.lock().unwrap().notices.push(text),
                Action::SetSavePoint => {
                    let s = self.shared.clone();
                    if let Some(c) = self.conn.as_ref() {
                        c.reducers.set_save_point_then(move |_, r| report(&s, r)).ok();
                    }
                }
            }
        }
        Some(result)
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
        // The SDK can panic on a connection the server closed; treat that as closed too.
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| c.frame_tick())) {
            Ok(Ok(())) => {}
            Ok(Err(error)) => self.fail(format!("{error}")),
            Err(_) => {
                let reason = self.shared.lock().map(|s| s.connection_error.clone()).ok().flatten();
                self.fail(reason.unwrap_or_else(|| "the server closed the connection".into()));
            }
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
        let token_path = token_path.to_string();
        let token = std::fs::read_to_string(&token_path).ok();
        self.connect(uri.to_string(), token, Some(token_path))
    }

    /// Connects with a token from the account website (its /api/game/login). The token
    /// is short lived and is not saved.
    #[func]
    fn connect_with_token(&mut self, uri: GString, token: GString) -> bool {
        self.connect(uri.to_string(), Some(token.to_string()), None)
    }
}

impl RoseNet {
    fn connect(&mut self, uri: String, token: Option<String>, save_token_to: Option<String>) -> bool {
        let (uri, database) = split_database(&uri);
        let shared = self.shared.clone();
        *shared.lock().unwrap() = Shared::default();
        let (s0, s1, s2) = (shared.clone(), shared.clone(), shared.clone());
        let result = DbConnection::builder()
            .with_uri(uri)
            .with_database_name(database)
            .with_token(token)
            .on_connect(move |conn, _, token| {
                if let Some(path) = save_token_to {
                    std::fs::write(&path, token).ok();
                }
                // Subscribe once the server has let us in; a refused connection has no
                // sender left to subscribe with.
                conn.subscription_builder()
                    .on_applied(move |_| s0.lock().unwrap().subscribed = true)
                    .subscribe_to_all_tables();
            })
            .on_connect_error(move |_, error| {
                let text = format!("{error}");
                // A connection the module turns away closes before the first message.
                let text = if text.contains("before receiving the initial connection message") {
                    "the server turned the connection down. Sign in with your account (or check the server address)".to_string()
                } else {
                    text
                };
                s1.lock().unwrap().connection_error = Some(text);
            })
            .on_disconnect(move |_, error| {
                s2.lock().unwrap().connection_error = Some(error.map_or("the server closed the connection".into(), |e| format!("{e}")));
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
        Table::on_insert(&conn.db.my_chat(), move |_, m| {
            let mut s = s.lock().unwrap();
            if m.id > s.chat_seen {
                s.chat_seen = m.id;
                s.chat.push(m.clone());
            }
        });
        let s = shared.clone();
        conn.db.motion().on_update(move |_, _, new| {
            let sample = local_now_us() - new.started_at_us;
            let mut s = s.lock().unwrap();
            s.clock_offset_us = Some(s.clock_offset_us.map_or(sample, |o| o.min(sample)));
        });
        self.error.clear();
        self.conn = Some(conn);
        true
    }
}

#[godot_api(secondary)]
impl RoseNet {
    /// Why the connection failed or was closed, or "".
    #[func]
    fn get_connection_error(&self) -> GString {
        GString::from(self.shared.lock().unwrap().connection_error.as_deref().unwrap_or(""))
    }

    /// Signed in, but this account has no character yet (show the creation screen).
    #[func]
    fn needs_character(&self) -> bool {
        let Some(c) = self.conn.as_ref() else { return false };
        if !self.shared.lock().unwrap().subscribed {
            return false;
        }
        c.try_identity().is_some_and(|i| c.db.player().identity().find(&i).is_none())
    }

    /// Make our character (see the module's create_character). The answer comes from
    /// poll_create_result.
    #[func]
    fn create_character(&self, name: GString, female: bool, face: i64, hair: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers
                .create_character_then(name.to_string(), female as u8, face as u8, hair as u8, move |_, r| {
                    let answer = match r {
                        Ok(Ok(())) => "ok".to_string(),
                        Ok(Err(message)) => message,
                        Err(error) => format!("{error}"),
                    };
                    s.lock().unwrap().create_result = Some(answer);
                })
                .ok();
        }
    }

    /// "" while waiting, then "ok" or the reason the name or look was refused.
    #[func]
    fn poll_create_result(&self) -> GString {
        GString::from(self.shared.lock().unwrap().create_result.take().unwrap_or_default().as_str())
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
    /// none), cast_effect_in and cast_ends_in (seconds) and cast_target; summon_owner for
    /// summons, hidden under Stealth, conditions ("Asleep", "Stunned" and so on).
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
                    // Monsters cast with their own motions, which the server names.
                    if let Some(motions) = c.db.npc_cast_motion().entity_id().find(&e.entity_id) {
                        d.set("cast_motion", motions.cast_motion as i64);
                        d.set("action_motion", motions.action_motion as i64);
                    } else {
                        d.set("cast_motion", skill.and_then(|s| s.casting_motion_id).map_or(-1, |m| m.get() as i64));
                        d.set("action_motion", skill.and_then(|s| s.action_motion_id).map_or(-1, |m| m.get() as i64));
                    }
                    d.set("cast_effect_in", (cast.effect_at_us - t) as f64 / 1e6);
                    d.set("cast_ends_in", (cast.ends_at_us - t) as f64 / 1e6);
                    if let Some(target) = cast.target {
                        d.set("cast_target", target as i64);
                    }
                }
            }
            d.set("sitting", c.db.sitting().entity_id().find(&e.entity_id).is_some());
            if c.db.driving().entity_id().find(&e.entity_id).is_some() {
                if let Some(p) = c.db.player().iter().find(|p| p.entity_id == Some(e.entity_id)) {
                    d.set("vehicle", &vehicle_parts(&p));
                }
            }
            // A passenger: the driver whose back seat it sits on.
            if let Some(ride) = c.db.passenger().guest().find(&e.entity_id) {
                d.set("riding", ride.driver as i64);
            }
            if let Some(summon) = c.db.summon().entity_id().find(&e.entity_id) {
                d.set("summon_owner", summon.owner as i64);
            }
            // Stealth or a disguise: drawn faded. Stun, sleep, silence and taunt: named
            // under the name tag ("conditions").
            use rose_data::StatusEffectType as S;
            let index = |t: S| enum_map::Enum::into_usize(t) as u8;
            let mut conditions: Vec<&str> = Vec::new();
            for r in c.db.status_effect().iter().filter(|r| r.entity_id == e.entity_id) {
                if r.effect_type == index(S::Disguise) || r.effect_type == index(S::Transparent) {
                    d.set("hidden", true);
                }
                let shown = [(S::Fainting, "Stunned"), (S::Sleep, "Asleep"), (S::Dumb, "Silenced"), (S::Taunt, "Taunted")];
                if let Some((_, name)) = shown.iter().find(|(t, _)| index(*t) == r.effect_type) {
                    conditions.push(name);
                }
            }
            if !conditions.is_empty() {
                d.set("conditions", conditions.join(", ").as_str());
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
        d.set("job", p.job as i64);
        d.set("xp", p.xp as i64);
        d.set("stamina", c.db.stamina().identity().find(&p.identity).map_or(0, |s| s.value) as i64);
        // Experience owed from deaths adds to what the level needs.
        let debt = c.db.xp_debt().identity().find(&p.identity).map_or(0, |d| d.xp);
        d.set("xp_needed", (rose_game_irose::data::levelup_require_xp(p.level) + debt) as i64);
        d.set("xp_debt", debt as i64);
        // While fallen: seconds until we get up by ourselves, and where the save point is.
        if let Some(f) = p.entity_id.and_then(|id| c.db.fallen().entity_id().find(&id)) {
            d.set("fallen", true);
            d.set("auto_revive_in", ((f.auto_revive_at_us - self.server_now_us()) / 1_000_000).max(0));
            d.set("penalty_xp", f.penalty_xp as i64);
        }
        if let Some(save) = c.db.save_point().identity().find(&p.identity) {
            let zone = c.db.zone_info().zone_id().find(&save.zone_id).map_or_else(String::new, |z| z.name);
            d.set("save_zone", zone.as_str());
        }
        // The engine's life is the vehicle's fuel (0-1000).
        let equipment: Equipment = serde_json::from_str(&p.equipment).unwrap_or_default();
        if let Some(engine) = equipment.get_vehicle_item(VehiclePartIndex::Engine) {
            d.set("fuel", engine.life as i64);
        }
        d.set("driving", p.entity_id.is_some_and(|id| c.db.driving().entity_id().find(&id).is_some()));
        d.set("passenger", p.entity_id.is_some_and(|id| c.db.passenger().guest().find(&id).is_some()));
        let has_seat = equipment
            .get_vehicle_item(VehiclePartIndex::Arms)
            .and_then(|a| crate::data::get()?.items.get_vehicle_item(a.item.item_number))
            .is_some_and(|v| v.has_seat);
        d.set("has_seat", has_seat);
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

    /// Send a chat line with the iROSE prefixes: "!text" shouts to the zone, "#text" talks
    /// to the party, "@name text" whispers, anything else is heard by players nearby.
    #[func]
    fn send_chat(&self, line: GString) {
        let line = line.to_string();
        let line = line.trim();
        let (channel, to, text) = if let Some(text) = line.strip_prefix('!') {
            (ChatChannel::Shout, "", text)
        } else if let Some(text) = line.strip_prefix('#') {
            (ChatChannel::Party, "", text)
        } else if let Some(rest) = line.strip_prefix('@') {
            let (to, text) = rest.split_once(' ').unwrap_or((rest, ""));
            (ChatChannel::Whisper, to, text)
        } else {
            (ChatChannel::Nearby, "", line)
        };
        if text.trim().is_empty() {
            return;
        }
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.send_chat_then(channel, to.to_string(), text.to_string(), move |_, r| report(&s, r)).ok();
        }
    }

    /// Chat messages since the last call: channel (nearby, shout, party, whisper), from,
    /// to (whispers), text, entity (the speaker) and mine (we sent it).
    #[func]
    fn poll_chat(&self) -> VarArray {
        let messages = std::mem::take(&mut self.shared.lock().unwrap().chat);
        let my_id = self.my_id();
        let mut out = VarArray::new();
        for m in messages {
            let mut d = VarDictionary::new();
            d.set(
                "channel",
                match m.channel {
                    ChatChannel::Nearby => "nearby",
                    ChatChannel::Shout => "shout",
                    ChatChannel::Party => "party",
                    ChatChannel::Whisper => "whisper",
                },
            );
            d.set("from", m.from_name.as_str());
            d.set("to", m.to_name.as_str());
            d.set("text", m.text.as_str());
            d.set("entity", m.from_entity.map_or(-1, |e| e as i64));
            d.set("mine", m.from_entity.is_some() && m.from_entity == my_id);
            out.push(&d.to_variant());
        }
        out
    }

    /// Sit down, or stand up.
    #[func]
    fn sit(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.sit_then(move |_, r| report(&s, r)).ok();
        }
    }

    /// Get up after dying: at the save point, or at this zone's revive point.
    #[func]
    fn revive(&self, at_save_point: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.revive_player_then(at_save_point, move |_, r| report(&s, r)).ok();
        }
    }

    /// Save this zone as where we get up after dying (what GF_setRevivePosition asks for).
    #[func]
    fn set_save_point(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.set_save_point_then(move |_, r| report(&s, r)).ok();
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
        let mut vehicle = VarArray::new();
        for index in [VehiclePartIndex::Body, VehiclePartIndex::Engine, VehiclePartIndex::Leg, VehiclePartIndex::Arms] {
            let item = equipment.get_vehicle_item(index).map(|e| rose_data::Item::Equipment(e.clone()));
            vehicle.push(&item.map_or(Variant::nil(), |item| item_dict(&item).to_variant()));
        }
        d.set("vehicle", &vehicle);
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
            d.set("description", data.description);
            d.set("bad", data.status_effect_type.is_bad());
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

    /// Talk to a town NPC: opens its conversation. False when it has nothing to say.
    #[func]
    fn open_conversation(&mut self, npc_entity_id: i64) -> bool {
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return false };
        let Some(npc) = c.db.npc().entity_id().find(&(npc_entity_id as u64)) else { return false };
        let Some(path) = conversation::conversation_path(game, &npc.conversation) else { return false };
        let title = c.db.entity().entity_id().find(&npc.entity_id).map(|e| e.name).unwrap_or_default();
        let entity = npc.entity_id;
        self.with_conversation(|conv, cx| {
            *conv = Conversation::open(&path, Some(entity), title, cx);
            conv.is_some()
        })
        .unwrap_or(false)
    }

    /// The open conversation: open, npc (entity id), title, message and responses (BBCode).
    #[func]
    fn get_conversation(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        d.set("open", self.conversation.is_some());
        if let Some(conv) = self.conversation.as_ref() {
            d.set("npc", conv.npc_entity.map_or(-1, |id| id as i64));
            d.set("title", conv.title.as_str());
            d.set("message", conv.message.as_str());
            let mut responses = VarArray::new();
            for r in conv.responses.iter() {
                responses.push(&GString::from(r.text.as_str()).to_variant());
            }
            d.set("responses", &responses);
        }
        d
    }

    /// Pick response `index` (0-based) of the open conversation.
    #[func]
    fn choose_response(&mut self, index: i64) {
        self.with_conversation(|conv, cx| {
            if let Some(c) = conv.as_mut() {
                if !c.choose(cx, index.max(0) as usize) {
                    *conv = None;
                }
            }
        });
    }

    #[func]
    fn close_conversation(&mut self) {
        self.conversation = None;
    }

    /// Windows a conversation opened since the last call: [kind ("store" or "bank"), npc entity id].
    #[func]
    fn poll_windows(&mut self) -> VarArray {
        let mut out = VarArray::new();
        for (kind, entity) in self.windows.drain(..) {
            let mut a = VarArray::new();
            a.push(&GString::from(kind.as_str()).to_variant());
            a.push(&(entity as i64).to_variant());
            out.push(&a.to_variant());
        }
        out
    }

    /// Our active quests: slot, id, name, description, items (as in get_inventory, with
    /// quantity) and time_left (seconds, -1 for none).
    #[func]
    fn get_quests(&self) -> VarArray {
        let mut out = VarArray::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return out };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return out };
        let state: rose_game_common::components::QuestState = serde_json::from_str(&p.quest_state).unwrap_or_default();
        let now = rose_quest::world_ticks(self.server_now_us());
        for (slot, quest) in state.active_quests.iter().enumerate() {
            let Some(quest) = quest else { continue };
            let data = game.quests.get_quest_data(quest.quest_id);
            let mut d = VarDictionary::new();
            d.set("slot", slot as i64);
            d.set("id", quest.quest_id as i64);
            d.set("name", data.map_or_else(|| format!("Quest {}", quest.quest_id), |q| q.name.to_string()).as_str());
            let description = data.map_or("", |q| q.description);
            d.set("description", conversation::format_text(description, &p.name, p.level).as_str());
            let mut items = VarArray::new();
            for item in quest.items.iter().flatten() {
                items.push(&item_dict(item).to_variant());
            }
            d.set("items", &items);
            let left = quest.expire_time.map_or(-1, |e| (e.0.saturating_sub(now.0) * 10) as i64);
            d.set("time_left", left);
            out.push(&d.to_variant());
        }
        out
    }

    #[func]
    fn abandon_quest(&self, slot: i64, quest_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.abandon_quest_then(slot as u8, quest_id as u32, move |_, r| report(&s, r)).ok();
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

    /// Take off a vehicle part: 0 body, 1 engine, 2 legs, 3 arms.
    #[func]
    fn unequip_vehicle_part(&self, part: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.unequip_vehicle_part_then(part as u8, move |_, r| report(&s, r)).ok();
        }
    }

    /// Get on or off our cart or castle gear.
    #[func]
    fn drive_toggle(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.drive_toggle_then(move |_, r| report(&s, r)).ok();
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

    /// Our party: leader (are we), xp_sharing (0 equal, 1 by level), item_sharing (0 equal,
    /// 1 in turn) and members in joining order (identity, name, level, online, leader,
    /// entity or -1, hp, max_hp). Empty when we aren't in one.
    #[func]
    fn get_party(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = self.conn.as_ref() else { return d };
        let Some(me) = c.try_identity() else { return d };
        let Some(party) = c.db.party_member().identity().find(&me).and_then(|m| c.db.party().party_id().find(&m.party_id)) else {
            return d;
        };
        let mut members: Vec<_> = c.db.party_member().iter().filter(|m| m.party_id == party.party_id).collect();
        members.sort_by_key(|m| m.joined_us);
        let mut list = VarArray::new();
        for m in members {
            let Some(p) = c.db.player().identity().find(&m.identity) else { continue };
            let mut e = VarDictionary::new();
            e.set("identity", m.identity.to_hex().to_string());
            e.set("name", p.name.clone());
            e.set("level", p.level as i64);
            e.set("online", p.online);
            e.set("leader", m.identity == party.owner);
            e.set("me", m.identity == me);
            let combat = p.entity_id.and_then(|id| c.db.combat().entity_id().find(&id));
            e.set("entity", p.entity_id.map_or(-1, |id| id as i64));
            e.set("hp", combat.as_ref().map_or(0, |c| c.hp as i64));
            e.set("max_hp", combat.as_ref().map_or(1, |c| c.max_hp.max(1) as i64));
            list.push(&e.to_variant());
        }
        d.set("leader", party.owner == me);
        d.set("xp_sharing", party.xp_sharing as i64);
        d.set("item_sharing", party.item_sharing as i64);
        d.set("members", &list);
        d
    }

    /// Party invitations to us: [invite_id, from name].
    #[func]
    fn get_party_invites(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let Some(me) = c.try_identity() else { return out };
        for i in c.db.party_invitation().iter().filter(|i| i.to == me) {
            let name = c.db.player().identity().find(&i.from).map_or_else(String::new, |p| p.name);
            let mut a = VarArray::new();
            a.push(&(i.invite_id as i64).to_variant());
            a.push(&name.to_variant());
            out.push(&a.to_variant());
        }
        out
    }

    /// The personal shop on this entity: title, owner, mine (bool), items (each an item
    /// dictionary plus id and price). Empty if it has none.
    #[func]
    fn get_personal_store(&self, entity_id: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = self.conn.as_ref() else { return d };
        let Some(store) = c.db.personal_store().entity_id().find(&(entity_id as u64)) else { return d };
        d.set("title", store.title.as_str());
        d.set("owner", c.db.player().identity().find(&store.owner).map_or_else(String::new, |p| p.name).as_str());
        d.set("mine", c.try_identity() == Some(store.owner));
        let mut items = VarArray::new();
        let mut rows: Vec<_> = c.db.store_item().iter().filter(|r| r.store_entity == store.entity_id).collect();
        rows.sort_by_key(|r| r.id);
        for r in rows {
            let Ok(item) = serde_json::from_str::<rose_data::Item>(&r.item) else { continue };
            let mut it = crate::items::item_dict(&item);
            it.set("id", r.id as i64);
            it.set("price", r.price);
            items.push(&it.to_variant());
        }
        d.set("items", &items);
        // The buy list, with where our matching bag item is (page and index, -1 if none).
        let bag = self.my_inventory();
        let mut wants = VarArray::new();
        let mut rows: Vec<_> = c.db.store_want().iter().filter(|r| r.store_entity == store.entity_id).collect();
        rows.sort_by_key(|r| r.id);
        for r in rows {
            let Ok(reference) = serde_json::from_str::<rose_data::ItemReference>(&r.item) else { continue };
            let Some(base) = crate::data::get().and_then(|g| g.items.get_base_item(reference)) else { continue };
            let Some(item) = rose_data::Item::from_item_data(base, r.quantity.max(1)) else { continue };
            let mut it = crate::items::item_dict(&item);
            it.set("id", r.id as i64);
            it.set("price", r.price);
            let (mut page, mut index, mut have) = (-1i64, -1i64, 0i64);
            if let Some(bag) = bag.as_ref() {
                let pages = [&bag.equipment, &bag.consumables, &bag.materials, &bag.vehicles];
                for (pi, pg) in pages.iter().enumerate() {
                    for (si, slot) in pg.slots.iter().enumerate() {
                        let Some(mine) = slot else { continue };
                        let usable = match mine {
                            rose_data::Item::Equipment(e) => e.life > 0,
                            _ => true,
                        };
                        if mine.get_item_reference() == reference && usable {
                            if page < 0 {
                                (page, index) = (pi as i64, si as i64);
                            }
                            have += mine.get_quantity() as i64;
                        }
                    }
                }
            }
            it.set("page", page);
            it.set("index", index);
            it.set("have", have);
            wants.push(&it.to_variant());
        }
        d.set("wants", &wants);
        d
    }

    /// Items whose name contains `text` (any case), for a shop's buy list: up to 20 item
    /// dictionaries that players may trade.
    #[func]
    fn find_items(&self, text: GString) -> VarArray {
        let mut out = VarArray::new();
        let wanted = text.to_string().to_lowercase();
        let Some(game) = crate::data::get() else { return out };
        if wanted.trim().len() < 2 {
            return out;
        }
        use rose_data::ItemType as T;
        for item_type in [
            T::Material, T::Gem, T::Consumable, T::Weapon, T::SubWeapon, T::Head, T::Body, T::Hands, T::Feet, T::Back,
            T::Face, T::Jewellery, T::Vehicle,
        ] {
            for reference in game.items.iter_items(item_type) {
                let Some(base) = game.items.get_base_item(reference) else { continue };
                if base.name.is_empty() || base.trade_restriction & 0x02 != 0 || !base.name.to_lowercase().contains(wanted.trim()) {
                    continue;
                }
                let Some(item) = rose_data::Item::from_item_data(base, 1) else { continue };
                out.push(&crate::items::item_dict(&item).to_variant());
                if out.len() >= 20 {
                    return out;
                }
            }
        }
        out
    }

    /// Repair at an NPC: kind 0 = equipped slot, 1 = vehicle part, 2 = bag (slot is the page).
    #[func]
    fn repair_at_npc(&self, npc: i64, kind: i64, slot: i64, index: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers
                .repair_at_npc_then(npc as u64, kind.max(0) as u8, slot.max(0) as u8, index.max(0) as u16, move |_, r| report(&s, r))
                .ok();
        }
    }

    /// Repair with the hammer at (tool_page, tool_index); the target as in repair_at_npc.
    #[func]
    fn repair_with_item(&self, tool_page: i64, tool_index: i64, kind: i64, slot: i64, index: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers
                .repair_with_item_then(
                    tool_page.max(0) as u8,
                    tool_index.max(0) as u16,
                    kind.max(0) as u8,
                    slot.max(0) as u8,
                    index.max(0) as u16,
                    move |_, r| report(&s, r),
                )
                .ok();
        }
    }

    /// Sell `quantity` of our bag item at page/index to a shop's buy list entry.
    #[func]
    fn store_sell(&self, store_entity: i64, want_id: i64, page: i64, index: i64, quantity: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers
                .store_sell_then(store_entity as u64, want_id as u64, page.max(0) as u8, index.max(0) as u16, quantity.max(0) as u32, move |_, r| {
                    report(&s, r)
                })
                .ok();
        }
    }

    /// Ride offers made to us: [offer id, driver name].
    #[func]
    fn get_ride_offers(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let Some(me) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)).and_then(|p| p.entity_id) else { return out };
        for o in c.db.ride_offer().iter().filter(|o| o.guest == me) {
            let name = c.db.entity().entity_id().find(&o.driver).map_or_else(String::new, |e| e.name);
            let mut a = VarArray::new();
            a.push(&(o.id as i64).to_variant());
            a.push(&name.to_variant());
            out.push(&a.to_variant());
        }
        out
    }

    #[func]
    fn ride_offer_to(&self, entity_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.ride_offer_to_then(entity_id as u64, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn ride_answer(&self, offer_id: i64, accept: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.ride_answer_then(offer_id as u64, accept, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn ride_leave(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.ride_leave_then(move |_, r| report(&s, r)).ok();
        }
    }

    /// Titles of the open shops by entity id, for the signs over their owners.
    #[func]
    fn get_store_titles(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        if let Some(c) = self.conn.as_ref() {
            for s in c.db.personal_store().iter() {
                d.set(s.entity_id as i64, s.title.as_str());
            }
        }
        d
    }

    /// Open our shop: `listings` holds [page, index, quantity, price] for each item to sell,
    /// `wanted` holds [type name, item number, quantity, price] for each item to buy.
    #[func]
    fn store_open(&self, title: GString, listings: VarArray, wanted: VarArray) {
        let s = self.shared.clone();
        let Some(c) = self.conn.as_ref() else { return };
        let listings: Vec<StoreListing> = listings
            .iter_shared()
            .filter_map(|v| {
                let a = v.try_to::<VarArray>().ok()?;
                let n = |i: usize| a.get(i).and_then(|x| x.try_to::<i64>().ok()).unwrap_or(0);
                Some(StoreListing { page: n(0) as u8, index: n(1) as u16, quantity: n(2).max(0) as u32, price: n(3) })
            })
            .collect();
        let wanted: Vec<StoreWanted> = wanted
            .iter_shared()
            .filter_map(|v| {
                let a = v.try_to::<VarArray>().ok()?;
                let n = |i: usize| a.get(i).and_then(|x| x.try_to::<i64>().ok()).unwrap_or(0);
                let item_type = a.get(0)?.try_to::<GString>().ok()?.to_string();
                Some(StoreWanted { item_type, item_number: n(1).max(0) as u32, quantity: n(2).max(0) as u32, price: n(3) })
            })
            .collect();
        c.reducers.store_open_then(title.to_string(), listings, wanted, move |_, r| report(&s, r)).ok();
    }

    #[func]
    fn store_close(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.store_close_then(move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn store_buy(&self, store_entity: i64, store_item_id: i64, quantity: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.store_buy_then(store_entity as u64, store_item_id as u64, quantity.max(0) as u32, move |_, r| report(&s, r)).ok();
        }
    }

    /// Our friends: name, online, level, job, zone (empty while offline), online ones first.
    #[func]
    fn get_friends(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let mut friends: Vec<_> = c.db.my_friends().iter().collect();
        friends.sort_by(|a, b| b.online.cmp(&a.online).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
        for f in friends {
            let mut d = VarDictionary::new();
            d.set("name", f.name.as_str());
            d.set("online", f.online);
            d.set("level", f.level as i64);
            d.set("job", f.job as i64);
            d.set("zone", f.zone.as_str());
            out.push(&d.to_variant());
        }
        out
    }

    /// Friend requests sent to us: [request id, from name].
    #[func]
    fn get_friend_requests(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        for r in c.db.my_friend_requests().iter() {
            let mut a = VarArray::new();
            a.push(&(r.request_id as i64).to_variant());
            a.push(&r.from_name.to_variant());
            out.push(&a.to_variant());
        }
        out
    }

    /// Ask a player (by name) to be friends.
    #[func]
    fn friend_ask(&self, name: GString) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.friend_ask_then(name.to_string(), move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn friend_answer(&self, request_id: i64, accept: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.friend_answer_then(request_id as u64, accept, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn friend_remove(&self, name: GString) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.friend_remove_then(name.to_string(), move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn party_invite(&self, entity_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.party_invite_then(entity_id as u64, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn party_answer(&self, invite_id: i64, accept: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            if accept {
                c.reducers.party_accept_then(invite_id as u64, move |_, r| report(&s, r)).ok();
            } else {
                c.reducers.party_decline_then(invite_id as u64, move |_, r| report(&s, r)).ok();
            }
        }
    }

    #[func]
    fn party_leave(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.party_leave_then(move |_, r| report(&s, r)).ok();
        }
    }

    /// The leader removes a member, or hands them the lead.
    #[func]
    fn party_member_action(&self, identity: GString, action: GString) {
        let s = self.shared.clone();
        let (Some(c), Ok(member)) = (self.conn.as_ref(), Identity::from_hex(identity.to_string())) else { return };
        match action.to_string().as_str() {
            "kick" => c.reducers.party_kick_then(member, move |_, r| report(&s, r)).ok(),
            "lead" => c.reducers.party_set_leader_then(member, move |_, r| report(&s, r)).ok(),
            _ => None,
        };
    }

    #[func]
    fn party_set_rules(&self, xp_sharing: i64, item_sharing: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.party_set_rules_then(xp_sharing as u8, item_sharing as u8, move |_, r| report(&s, r)).ok();
        }
    }

    /// The craft skill in this slot, if it is one.
    fn craft_skill(&self, page: i64, index: i64) -> Option<&'static rose_data::SkillData> {
        let (c, game) = (self.conn.as_ref()?, crate::data::get()?);
        let p = c.try_identity().and_then(|i| c.db.player().identity().find(&i))?;
        let skill_list: SkillList = serde_json::from_str(&p.skill_list).ok()?;
        let id = skill_list.get_skill(rose_game_common::components::SkillSlot(page as usize, index as usize))?;
        let skill = game.skills.get_skill(id)?;
        (matches!(skill.skill_type, rose_data::SkillType::CreateWindow) && skill.item_make_number > 0).then_some(skill)
    }

    /// What the craft skill in this slot makes: [{type (iROSE type number), number, item
    /// (as items::item_dict), level, materials: [{name, quantity, icon?}]}], easiest first.
    /// Empty for skills that don't craft.
    #[func]
    fn get_craft_items(&self, page: i64, index: i64) -> VarArray {
        let mut out = VarArray::new();
        let (Some(skill), Some(game)) = (self.craft_skill(page, index), crate::data::get()) else { return out };
        let mut found = Vec::new();
        use rose_data::ItemType::*;
        for item_type in [Face, Head, Body, Hands, Feet, Back, Jewellery, Weapon, SubWeapon, Consumable, Gem, Material, Vehicle] {
            for number in 1..2000usize {
                let reference = rose_data::ItemReference::new(item_type, number);
                let Some(data) = game.items.get_base_item(reference) else { continue };
                if data.craft_skill_type != skill.item_make_number || data.craft_skill_level > skill.level || data.name.is_empty() {
                    continue;
                }
                let Some(recipe) = game.craft_recipes.get(data.craft_material as usize).and_then(|r| r.as_ref()) else { continue };
                found.push((data.craft_skill_level, data.name, reference, recipe));
            }
        }
        found.sort_by_key(|f| (f.0, f.1));
        for (level, _, reference, recipe) in found {
            let Some(item) = game.items.get_base_item(reference).and_then(|d| rose_data::Item::from_item_data(d, 1)) else { continue };
            let mut d = VarDictionary::new();
            d.set("type", rose_data_irose::encode_item_type(reference.item_type).unwrap_or(0) as i64);
            d.set("number", reference.item_number as i64);
            d.set("item", &crate::items::item_dict(&item));
            d.set("level", level as i64);
            d.set("materials", &recipe_materials(game, recipe));
            out.push(&d.to_variant());
        }
        out
    }

    /// Bag slots holding enough of each material of this item's recipe: [page, index] per
    /// step, [-1, -1] where we have none.
    #[func]
    fn find_craft_materials(&self, item_type: i64, item_number: i64) -> VarArray {
        let out = VarArray::new();
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return out };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return out };
        let Some(reference) = rose_data_irose::decode_item_type(item_type as usize)
            .map(|t| rose_data::ItemReference::new(t, item_number as usize))
        else {
            return out;
        };
        let Some(recipe) = game
            .items
            .get_base_item(reference)
            .and_then(|d| game.craft_recipes.get(d.craft_material as usize))
            .and_then(|r| r.as_ref())
        else {
            return out;
        };
        let inventory: Inventory = serde_json::from_str(&p.inventory).unwrap_or_default();
        find_recipe_slots(game, &inventory, recipe, None)
    }

    /// Craft with the skill in this slot; `slots` are [page, index] per recipe step.
    #[func]
    fn craft_item(&self, page: i64, index: i64, item_type: i64, item_number: i64, slots: VarArray) {
        let s = self.shared.clone();
        let Some(c) = self.conn.as_ref() else { return };
        let materials = craft_slots(&slots);
        c.reducers
            .craft_item_then(page as u8, index as u16, item_type as u8, item_number as u16, materials, move |_, r| report(&s, r))
            .ok();
    }

    /// What the craft skill in this slot does: "craft", "refine", "disassemble", or "" when
    /// it isn't a craft skill.
    #[func]
    fn craft_skill_kind(&self, page: i64, index: i64) -> GString {
        match self.craft_skill(page, index).map(|s| s.item_make_number) {
            Some(MAKE_DISASSEMBLE) => "disassemble",
            Some(MAKE_REFINE) => "refine",
            Some(_) => "craft",
            None => "",
        }
        .into()
    }

    fn my_inventory(&self) -> Option<Inventory> {
        let c = self.conn.as_ref()?;
        let p = c.try_identity().and_then(|i| c.db.player().identity().find(&i))?;
        serde_json::from_str(&p.inventory).ok()
    }

    fn bag_item(inventory: &Inventory, page: i64, index: i64) -> Option<&rose_data::Item> {
        let pages = [&inventory.equipment, &inventory.consumables, &inventory.materials, &inventory.vehicles];
        pages.get(page as usize)?.slots.get(index as usize)?.as_ref()
    }

    /// Refining this bag item: {item, grade, materials (as get_craft_items), slots (as
    /// find_craft_materials), mp, zuly, ready}, or {error} when it can't be refined.
    #[func]
    fn get_refine_info(&self, page: i64, index: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let (Some(inventory), Some(game)) = (self.my_inventory(), crate::data::get()) else { return d };
        let Some(item) = Self::bag_item(&inventory, page, index) else { return d };
        d.set("item", &item_dict(item));
        let rose_data::Item::Equipment(e) = item else {
            d.set("error", "Only equipment can be refined");
            return d;
        };
        if e.grade >= 9 {
            d.set("error", "This is the highest grade");
            return d;
        }
        let base = if e.item.item_type == rose_data::ItemType::Weapon { 1 } else { 11 };
        let Some(recipe) = game.craft_recipes.get(base + e.grade as usize).and_then(|r| r.as_ref()) else {
            d.set("error", "This can't be refined");
            return d;
        };
        let quality = game.items.get_base_item(e.item).map_or(0, |d| d.quality) as i64;
        let grade = e.grade as i64;
        let slots = find_recipe_slots(game, &inventory, recipe, Some((page as usize, index as usize)));
        let ready = slots.iter_shared().all(|v| v.try_to::<VarArray>().ok().and_then(|a| a.get(0)).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(-1) >= 0);
        d.set("grade", grade);
        d.set("materials", &recipe_materials(game, recipe));
        d.set("slots", &slots);
        d.set("mp", ((grade + 4) as f32 * (quality + 20) as f32 * 0.9) as i64);
        d.set("zuly", ((grade * (grade + 1) * quality * (quality + 20)) as f32 * 0.2) as i64);
        d.set("ready", ready);
        d
    }

    /// Disassembling this bag item: {item, gem (the set gem's name, when it gives that back),
    /// outputs (material names), mp, zuly}, or {error}.
    #[func]
    fn get_disassemble_info(&self, page: i64, index: i64) -> VarDictionary {
        let mut d = VarDictionary::new();
        let (Some(inventory), Some(game)) = (self.my_inventory(), crate::data::get()) else { return d };
        let Some(item) = Self::bag_item(&inventory, page, index) else { return d };
        d.set("item", &item_dict(item));
        let Some(data) = game.items.get_base_item(item.get_item_reference()) else { return d };
        let quality = data.quality as i64;
        if let rose_data::Item::Equipment(e) = item {
            if e.has_socket && e.gem > 300 {
                let gem = game.items.get_base_item(rose_data::ItemReference::gem(e.gem as usize));
                let gem_quality = gem.map_or(0, |g| g.quality) as i64;
                d.set("gem", gem.map_or("the gem", |g| g.name));
                d.set("mp", quality / 2 + gem_quality);
                d.set("zuly", quality * 5 + 50);
                return d;
            }
        }
        let Some(recipe) = (data.craft_material != 0).then(|| game.craft_recipes.get(data.craft_material as usize)).flatten().and_then(|r| r.as_ref()) else {
            d.set("error", "This can't be taken apart");
            return d;
        };
        let mut outputs = VarArray::new();
        for (step, m) in recipe.materials.iter().enumerate() {
            let Some(m) = m else { break };
            let output = match m.item {
                Some(item) => Some(item),
                None if step == 0 => {
                    let grade = ((quality - 20) / 12).clamp(1, 10) as usize;
                    Some(rose_data::ItemReference::new(rose_data::ItemType::Material, (recipe.raw_material_class as usize).saturating_sub(421) * 10 + grade))
                }
                None => None,
            };
            if let Some(name) = output.and_then(|r| game.items.get_base_item(r)).map(|d| d.name) {
                outputs.push(&GString::from(name).to_variant());
            }
        }
        d.set("outputs", &outputs);
        d.set("mp", quality + 30);
        d.set("zuly", quality * 10 + 20);
        d
    }

    fn craft_tool(npc: i64, skill_page: i64, skill_index: i64) -> CraftTool {
        if npc >= 0 {
            CraftTool::Npc(npc as u64)
        } else {
            CraftTool::Skill(CraftSlot { page: skill_page as u8, index: skill_index as u16 })
        }
    }

    /// Refine a bag item at an NPC (`npc` its entity id) or with the skill in a skill slot
    /// (`npc` -1); `slots` as find_craft_materials.
    #[func]
    fn refine_item(&self, npc: i64, skill_page: i64, skill_index: i64, page: i64, index: i64, slots: VarArray) {
        let s = self.shared.clone();
        let Some(c) = self.conn.as_ref() else { return };
        let tool = Self::craft_tool(npc, skill_page, skill_index);
        c.reducers.refine_item_then(tool, page as u8, index as u16, craft_slots(&slots), move |_, r| report(&s, r)).ok();
    }

    /// Disassemble a bag item, as refine_item.
    #[func]
    fn disassemble_item(&self, npc: i64, skill_page: i64, skill_index: i64, page: i64, index: i64) {
        let s = self.shared.clone();
        let Some(c) = self.conn.as_ref() else { return };
        let tool = Self::craft_tool(npc, skill_page, skill_index);
        c.reducers.disassemble_item_then(tool, page as u8, index as u16, move |_, r| report(&s, r)).ok();
    }

    /// Set this bag gem into the first equipped item with an empty socket. False (with a
    /// notice) when no worn item has one.
    #[func]
    fn insert_gem(&self, page: i64, index: i64) -> bool {
        let Some(c) = self.conn.as_ref() else { return false };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return false };
        let equipment: Equipment = serde_json::from_str(&p.equipment).unwrap_or_default();
        use EquipmentIndex::*;
        let order = [Face, Head, Body, Back, Hands, Feet, Weapon, SubWeapon, Necklace, Ring, Earring];
        let preferred = [Weapon, SubWeapon, Body, Head, Hands, Feet, Back, Necklace, Ring, Earring, Face];
        let Some(slot) = preferred
            .iter()
            .find(|&&i| equipment.get_equipment_item(i).is_some_and(|e| e.has_socket && e.gem <= 300))
            .and_then(|i| order.iter().position(|o| o == i))
        else {
            self.shared.lock().unwrap().notices.push("None of your worn items has an empty gem socket".into());
            return false;
        };
        let s = self.shared.clone();
        c.reducers.insert_gem_then(slot as u8, page as u8, index as u16, move |_, r| report(&s, r)).ok();
        true
    }

    /// The PvP state of the zone we are in, from LIST_ZONE.STB: 0 when players can't fight,
    /// 1 all except clan, 2 all except party, 3 all.
    #[func]
    fn zone_pvp(&self) -> i64 {
        let (Some(c), Some(game)) = (self.conn.as_ref(), crate::data::get()) else { return 0 };
        let Some(p) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)) else { return 0 };
        rose_data::ZoneId::new(p.zone_id).and_then(|z| game.zone_list.get_zone(z)).map_or(0, |z| z.pvp_state as i64)
    }

    /// Whether this entity is another player we may fight here (the server's pvp.rs rules).
    #[func]
    fn is_enemy_player(&self, entity_id: i64) -> bool {
        let Some(c) = self.conn.as_ref() else { return false };
        let Some(me) = c.try_identity() else { return false };
        let Some(other) = c.db.player().iter().find(|p| p.entity_id == Some(entity_id as u64)) else { return false };
        if other.identity == me {
            return false;
        }
        match self.zone_pvp() {
            2 => {
                let party = |i: &Identity| c.db.party_member().identity().find(i).map(|m| m.party_id);
                party(&me).is_none() || party(&me) != party(&other.identity)
            }
            1 | 3 => true,
            _ => false,
        }
    }

    /// Trade requests sent to us: [request id, from name] each.
    #[func]
    fn get_trade_requests(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let Some(me) = c.try_identity() else { return out };
        for r in c.db.trade_request().iter().filter(|r| r.to == me) {
            let name = c.db.player().identity().find(&r.from).map_or_else(String::new, |p| p.name);
            let mut a = VarArray::new();
            a.push(&(r.request_id as i64).to_variant());
            a.push(&name.to_variant());
            out.push(&a.to_variant());
        }
        out
    }

    /// Our trade, or {} when not trading: {with (name), with_entity, mine, theirs}, each side
    /// {items: [item_dict plus page and index], money, locked, accepted}.
    #[func]
    fn get_trade(&self) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(c) = self.conn.as_ref() else { return d };
        let Some(me) = c.try_identity() else { return d };
        let Some(t) = c.db.trade().iter().find(|t| t.a == me || t.b == me) else { return d };
        let side = |items: &str, money: i64, locked: bool, accepted: bool| {
            let mut sd = VarDictionary::new();
            let mut list = VarArray::new();
            for o in serde_json::from_str::<Vec<serde_json::Value>>(items).unwrap_or_default() {
                let Ok(item) = serde_json::from_value::<rose_data::Item>(o["item"].clone()) else { continue };
                let mut id = item_dict(&item);
                id.set("page", o["page"].as_i64().unwrap_or(0));
                id.set("index", o["index"].as_i64().unwrap_or(0));
                list.push(&id.to_variant());
            }
            sd.set("items", &list);
            sd.set("money", money);
            sd.set("locked", locked);
            sd.set("accepted", accepted);
            sd
        };
        let a = side(&t.a_items, t.a_money, t.a_locked, t.a_accepted);
        let b = side(&t.b_items, t.b_money, t.b_locked, t.b_accepted);
        let (mine, theirs, other) = if t.a == me { (a, b, t.b) } else { (b, a, t.a) };
        let other = c.db.player().identity().find(&other);
        d.set("with", other.as_ref().map_or_else(String::new, |p| p.name.clone()));
        d.set("with_entity", other.and_then(|p| p.entity_id).map_or(-1, |e| e as i64));
        d.set("mine", &mine);
        d.set("theirs", &theirs);
        d
    }

    #[func]
    fn trade_ask(&self, entity_id: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.trade_ask_then(entity_id as u64, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn trade_answer(&self, request_id: i64, accept: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.trade_answer_then(request_id as u64, accept, move |_, r| report(&s, r)).ok();
        }
    }

    /// Put these bag items ([page, index, quantity] each) and Zuly on the table.
    #[func]
    fn trade_offer(&self, slots: VarArray, money: i64) {
        let s = self.shared.clone();
        let Some(c) = self.conn.as_ref() else { return };
        let slots: Vec<TradeSlot> = slots
            .iter_shared()
            .filter_map(|v| v.try_to::<VarArray>().ok())
            .map(|a| {
                let n = |i| a.get(i).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0);
                TradeSlot { page: n(0) as u8, index: n(1) as u16, quantity: n(2) as u32 }
            })
            .collect();
        c.reducers.trade_offer_then(slots, money, move |_, r| report(&s, r)).ok();
    }

    #[func]
    fn trade_lock(&self, locked: bool) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.trade_lock_then(locked, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn trade_accept(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.trade_accept_then(move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn trade_cancel(&self) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.trade_cancel_then(move |_, r| report(&s, r)).ok();
        }
    }

    /// Our bank: 120 slots (four pages of 30), null when empty.
    #[func]
    fn get_bank(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let Some(id) = c.try_identity() else { return out };
        let slots: Vec<Option<rose_data::Item>> =
            c.db.bank().identity().find(&id).and_then(|b| serde_json::from_str(&b.slots).ok()).unwrap_or_default();
        for i in 0..120 {
            let item = slots.get(i).and_then(|s| s.as_ref());
            out.push(&item.map_or(Variant::nil(), |item| item_dict(item).to_variant()));
        }
        out
    }

    #[func]
    fn bank_deposit(&self, npc_entity_id: i64, page: i64, index: i64, quantity: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers
                .bank_deposit_then(npc_entity_id as u64, page as u8, index as u16, quantity.max(1) as u32, move |_, r| report(&s, r))
                .ok();
        }
    }

    #[func]
    fn bank_withdraw(&self, npc_entity_id: i64, slot: i64, quantity: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.bank_withdraw_then(npc_entity_id as u64, slot as u16, quantity.max(1) as u32, move |_, r| report(&s, r)).ok();
        }
    }

    #[func]
    fn bank_move(&self, from: i64, to: i64) {
        let s = self.shared.clone();
        if let Some(c) = self.conn.as_ref() {
            c.reducers.bank_move_then(from as u16, to as u16, move |_, r| report(&s, r)).ok();
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

/// Skill item_make_number of Item Disassembly and Item Refining.
const MAKE_DISASSEMBLE: u32 = 41;
const MAKE_REFINE: u32 = 42;

/// A recipe's materials for the UI: [{name, quantity, icon?}].
fn recipe_materials(game: &crate::data::GameData, recipe: &rose_game_data::CraftRecipe) -> VarArray {
    let mut materials = VarArray::new();
    for m in recipe.materials.iter().flatten() {
        let mut md = VarDictionary::new();
        match m.item.and_then(|r| game.items.get_base_item(r)).and_then(|d| rose_data::Item::from_item_data(d, 1)) {
            Some(item) => {
                let item_d = item_dict(&item);
                md.set("name", &item_d.get("name").unwrap_or_default());
                md.set("icon", &item_d.get("icon").unwrap_or_default());
            }
            None => {
                let class = game
                    .decoder
                    .decode_item_class(recipe.raw_material_class as usize)
                    .map_or_else(|| "material".to_string(), |c| crate::skills::split_camel_case(&format!("{c:?}")));
                md.set("name", format!("any {class}"));
            }
        }
        md.set("quantity", m.quantity as i64);
        materials.push(&md.to_variant());
    }
    materials
}

/// Bag slots holding enough of each recipe step's material: [page, index] per step, [-1, -1]
/// where we have none. `skip` is a slot not to use (the item being worked on).
fn find_recipe_slots(
    game: &crate::data::GameData,
    inventory: &Inventory,
    recipe: &rose_game_data::CraftRecipe,
    skip: Option<(usize, usize)>,
) -> VarArray {
    let mut out = VarArray::new();
    let pages = [&inventory.equipment, &inventory.consumables, &inventory.materials, &inventory.vehicles];
    let mut used: Vec<(usize, usize)> = skip.into_iter().collect();
    for (step, material) in recipe.materials.iter().enumerate() {
        let Some(material) = material else { continue };
        let mut found = (-1i64, -1i64);
        'pages: for (page_index, page) in pages.iter().enumerate() {
            for (slot_index, slot) in page.slots.iter().enumerate() {
                let Some(item) = slot else { continue };
                let r = item.get_item_reference();
                let enough = matches!(item, rose_data::Item::Equipment(_)) || item.get_quantity() >= material.quantity;
                let class = game.items.get_base_item(r).map(|d| d.class);
                if enough && !used.contains(&(page_index, slot_index)) && class.is_some_and(|class| recipe.accepts(step, r, class)) {
                    used.push((page_index, slot_index));
                    found = (page_index as i64, slot_index as i64);
                    break 'pages;
                }
            }
        }
        let mut a = VarArray::new();
        a.push(&found.0.to_variant());
        a.push(&found.1.to_variant());
        out.push(&a.to_variant());
    }
    out
}

/// [page, index] arrays from GDScript as reducer slots.
fn craft_slots(slots: &VarArray) -> Vec<CraftSlot> {
    slots
        .iter_shared()
        .filter_map(|v| v.try_to::<VarArray>().ok())
        .map(|a| CraftSlot {
            page: a.get(0).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as u8,
            index: a.get(1).and_then(|v| v.try_to::<i64>().ok()).unwrap_or(0) as u16,
        })
        .collect()
}
