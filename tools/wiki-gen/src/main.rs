//! Writes the first draft of the wiki's table pages from the iROSE client data: skills,
//! monsters, NPCs (with their full dialog), quests, items and zones, with zone maps and icon
//! sheets. After the first run the pages are the source of truth and are edited by hand;
//! see tools/wiki-gen/README.md.
//!
//! Usage: wiki-gen <path to data.idx> <wiki folder> <folder for DDS images to convert>

mod images;
mod items;
mod lua4;
mod npcs;
mod page;
mod quests;
mod script;
mod skills;
mod zones;

use std::{
    collections::{BTreeSet, HashMap},
    path::PathBuf,
};

use anyhow::Context;
use rose_data::{AbilityType, ItemReference, ItemType, NpcConversationId, QuestTrigger};
use rose_data_irose::{decode_ability_type, decode_item_base1000, encode_item_type};
use rose_file_readers::{
    ConFile, HostFilesystemDevice, LtbFile, QsdCondition, QsdReward, StbFile, VfsIndex, VirtualFilesystem,
};
use rose_game_data::GameData;

use page::{slug, Pages};
use script::{Effect, Script};

pub const ITEM_TYPES: [(ItemType, &str, &str); 14] = [
    (ItemType::Weapon, "weapon", "Weapons"),
    (ItemType::SubWeapon, "subweapon", "Shields and off-hand"),
    (ItemType::Head, "head", "Headgear"),
    (ItemType::Body, "body", "Armour"),
    (ItemType::Hands, "hands", "Gloves"),
    (ItemType::Feet, "feet", "Shoes"),
    (ItemType::Back, "back", "Back items"),
    (ItemType::Face, "face", "Face items"),
    (ItemType::Jewellery, "jewellery", "Jewellery"),
    (ItemType::Consumable, "consumable", "Consumables"),
    (ItemType::Gem, "gem", "Gems"),
    (ItemType::Material, "material", "Materials"),
    (ItemType::Quest, "quest", "Quest items"),
    (ItemType::Vehicle, "vehicle", "Cart and castle gear parts"),
];

pub type ItemKey = (usize, usize);

pub fn ikey(item: ItemReference) -> ItemKey {
    (encode_item_type(item.item_type).unwrap_or(0), item.item_number)
}

pub fn type_folder(t: ItemType) -> &'static str {
    ITEM_TYPES.iter().find(|(it, _, _)| *it == t).map_or("other", |(_, f, _)| f)
}

/// Where a monster spawns.
pub struct Spawn {
    pub zone: u16,
    pub x: f32,
    pub y: f32,
    pub count: usize,
    pub tactic: bool,
    pub interval: u32,
    pub limit: u32,
}

/// Where an NPC stands.
pub struct Placement {
    pub zone: u16,
    pub x: f32,
    pub y: f32,
    pub conversation: String,
}

/// A loaded NPC dialog file.
pub struct Dialog {
    pub con: ConFile,
    pub script: Option<Script>,
}

impl Dialog {
    pub fn effects(&self, function: &str) -> Vec<Effect> {
        if function.is_empty() {
            return Vec::new();
        }
        self.script.as_ref().map(|s| s.effects(function)).unwrap_or_default()
    }

    /// Every effect of every function the dialog calls.
    pub fn all_effects(&self) -> Vec<Effect> {
        let mut out = Vec::new();
        let messages = self.con.initial_messages.iter().chain(self.con.menus.iter().flat_map(|m| m.messages.iter()));
        for m in messages {
            for f in [&m.condition_function, &m.action_function] {
                for e in self.effects(f) {
                    if !out.contains(&e) {
                        out.push(e);
                    }
                }
            }
        }
        out
    }
}

/// Everything the page writers share: the data, each thing's page, and who refers to what.
pub struct Ctx<'a> {
    pub vfs: &'a VirtualFilesystem,
    pub game: &'a GameData,
    pub ltb: Option<LtbFile>,
    pub drop_stb: Option<StbFile>,
    pub zone_stb: Option<StbFile>,

    pub item_pages: HashMap<ItemKey, (String, String)>,
    pub skill_pages: HashMap<u16, (String, String)>,
    /// Every skill id (each level) to its base skill id.
    pub skill_base: HashMap<u16, u16>,
    pub monster_pages: HashMap<u16, (String, String)>,
    pub npc_pages: HashMap<u16, (String, String)>,
    pub quest_pages: HashMap<usize, (String, String)>,
    pub zone_pages: HashMap<u16, (String, String)>,

    pub spawns: HashMap<u16, Vec<Spawn>>,
    pub placements: HashMap<u16, Vec<Placement>>,
    pub dialogs: HashMap<String, Dialog>,
    /// Quest ids each trigger belongs to.
    pub trigger_quests: HashMap<String, BTreeSet<usize>>,
    /// NPCs whose dialog checks or runs each trigger.
    pub trigger_npcs: HashMap<String, BTreeSet<u16>>,
    /// Monsters whose death runs each trigger.
    pub trigger_monsters: HashMap<String, BTreeSet<u16>>,
}

impl Ctx<'_> {
    pub fn item_link(&self, item: ItemReference) -> String {
        match self.item_pages.get(&ikey(item)) {
            Some((p, n)) => page::link(p, n),
            None => format!("{} {}", type_folder(item.item_type), item.item_number),
        }
    }

    pub fn item_table_link(&self, item: ItemReference) -> String {
        match self.item_pages.get(&ikey(item)) {
            Some((p, n)) => page::table_link(p, n),
            None => format!("{} {}", type_folder(item.item_type), item.item_number),
        }
    }

    pub fn item_link_sn(&self, sn: usize) -> String {
        decode_item_base1000(sn).map_or_else(|| format!("item {sn}"), |i| self.item_link(i))
    }

    pub fn skill_link(&self, id: u16) -> String {
        let base = self.skill_base.get(&id).copied().unwrap_or(id);
        let level = self.game.skills.get_skill(rose_data::SkillId::new(id).unwrap_or(rose_data::SkillId::new(1).unwrap()));
        match self.skill_pages.get(&base) {
            Some((p, n)) => match level {
                Some(s) if s.level > 1 => format!("{} level {}", page::link(p, n), s.level),
                _ => page::link(p, n),
            },
            None => format!("skill {id}"),
        }
    }

    /// A monster or NPC.
    pub fn npc_link(&self, id: u16) -> String {
        if let Some((p, n)) = self.monster_pages.get(&id).or_else(|| self.npc_pages.get(&id)) {
            return page::link(p, n);
        }
        let name = rose_data::NpcId::new(id).and_then(|n| self.game.npcs.get_npc(n)).map_or("", |n| n.name);
        if name.is_empty() {
            format!("NPC {id}")
        } else {
            format!("{name} (NPC {id})")
        }
    }

    pub fn quest_link(&self, id: usize) -> String {
        match self.quest_pages.get(&id) {
            Some((p, n)) => page::link(p, n),
            None => format!("quest {id}"),
        }
    }

    pub fn zone_link(&self, id: u16) -> String {
        match self.zone_pages.get(&id) {
            Some((p, n)) => page::link(p, n),
            None => format!("zone {id}"),
        }
    }

    pub fn ability_name(&self, ability: AbilityType) -> String {
        let name = self.game.string_database.get_ability_type(ability);
        if name.is_empty() {
            format!("{ability:?}")
        } else {
            name.to_string()
        }
    }

    pub fn ability_id_name(&self, id: usize) -> String {
        decode_ability_type(id).map_or_else(|| format!("ability {id}"), |a| self.ability_name(a))
    }

    pub fn dialog_text(&self, string_id: u32) -> String {
        self.ltb.as_ref().and_then(|l| l.get_string(string_id as usize, 2)).unwrap_or_default()
    }

    pub fn status_effect_name(&self, id: rose_data::StatusEffectId) -> String {
        self.game.status_effects.get_status_effect(id).map_or_else(|| format!("status {}", id.get()), |s| s.name.to_string())
    }

    pub fn job_class_name(&self, id: rose_data::JobClassId) -> String {
        self.game.job_class.get(id).map_or_else(|| format!("job group {}", id.get()), |j| j.name.to_string())
    }

    pub fn drop_value(&self, row: usize, column: usize) -> i32 {
        self.drop_stb.as_ref().and_then(|s| s.try_get_int(row, column)).unwrap_or(0)
    }

    /// The items a drop table row can give: (item, slots out of 30 that can give it).
    pub fn drop_row(&self, row: usize) -> Vec<(ItemReference, f32)> {
        let mut out: Vec<(ItemReference, f32)> = Vec::new();
        let mut add = |item: ItemReference, share: f32| match out.iter_mut().find(|(i, _)| *i == item) {
            Some((_, s)) => *s += share,
            None => out.push((item, share)),
        };
        for column in 0..30 {
            let v = self.drop_value(row, column);
            if (1..=4).contains(&v) {
                for g in 0..5 {
                    if let Some(item) = decode_item_base1000(self.drop_value(row, (26 + v * 5 + g) as usize) as usize) {
                        add(item, 0.2);
                    }
                }
            } else if let Some(item) = decode_item_base1000(v.max(0) as usize) {
                add(item, 1.0);
            }
        }
        out
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        anyhow::bail!("usage: wiki-gen <data.idx> <wiki folder> <folder for DDS images>");
    }
    let data_idx = PathBuf::from(&args[1]);
    let out = PathBuf::from(&args[2]);
    let dds_dir = PathBuf::from(&args[3]);
    let root = data_idx.parent().context("data.idx has no folder")?.to_path_buf();
    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(&data_idx)?),
        Box::new(HostFilesystemDevice::new(root)),
    ]);
    let game = rose_game_data::load_game_data(&vfs)?;
    let _ = std::fs::remove_file(dds_dir.join("convert.json"));
    let mut cx = Ctx {
        vfs: &vfs,
        game: &game,
        ltb: vfs.read_file::<LtbFile, _>("3DDATA/EVENT/ULNGTB_CON.LTB").ok(),
        drop_stb: vfs.read_file::<StbFile, _>("3DDATA/STB/ITEM_DROP.STB").ok(),
        zone_stb: vfs.read_file::<StbFile, _>("3DDATA/STB/LIST_ZONE.STB").ok(),
        item_pages: HashMap::new(),
        skill_pages: HashMap::new(),
        skill_base: HashMap::new(),
        monster_pages: HashMap::new(),
        npc_pages: HashMap::new(),
        quest_pages: HashMap::new(),
        zone_pages: HashMap::new(),
        spawns: HashMap::new(),
        placements: HashMap::new(),
        dialogs: HashMap::new(),
        trigger_quests: HashMap::new(),
        trigger_npcs: HashMap::new(),
        trigger_monsters: HashMap::new(),
    };
    index(&mut cx);
    eprintln!(
        "{} items, {} skills, {} monsters, {} NPCs, {} quests, {} zones",
        cx.item_pages.len(),
        cx.skill_pages.len(),
        cx.monster_pages.len(),
        cx.npc_pages.len(),
        cx.quest_pages.len(),
        cx.zone_pages.len()
    );

    let mut pages = Pages::default();
    items::write(&cx, &mut pages);
    skills::write(&cx, &mut pages);
    npcs::write_monsters(&cx, &mut pages);
    npcs::write_npcs(&cx, &mut pages);
    quests::write(&cx, &mut pages);
    zones::write(&cx, &mut pages, &dds_dir)?;
    images::write_icons(&cx, &out, &dds_dir)?;

    for p in pages.0.values() {
        p.write(&out)?;
    }
    eprintln!("wrote {} pages", pages.0.len());
    Ok(())
}

/// Every thing's page path, and who refers to what.
fn index(cx: &mut Ctx) {
    let game = cx.game;
    for (item_type, folder, _) in ITEM_TYPES {
        for item in game.items.iter_items(item_type) {
            let Some(base) = game.items.get_base_item(item) else { continue };
            if base.name.trim().is_empty() {
                continue;
            }
            let path = format!("items/{folder}/{}-{}", item.item_number, slug(base.name));
            cx.item_pages.insert(ikey(item), (path, base.name.trim().to_string()));
        }
    }

    for skill in game.skills.iter() {
        let base = skill.base_skill_id.unwrap_or(skill.id).get();
        cx.skill_base.insert(skill.id.get(), base);
    }
    for skill in game.skills.iter() {
        let base = skill.base_skill_id.unwrap_or(skill.id).get();
        if skill.name.trim().is_empty() || cx.skill_pages.contains_key(&base) {
            continue;
        }
        cx.skill_pages.insert(base, (format!("skills/{base}-{}", slug(skill.name)), skill.name.trim().to_string()));
    }

    for zone in game.zones.iter() {
        let name = if zone.name.trim().is_empty() { format!("Zone {}", zone.id.get()) } else { zone.name.trim().to_string() };
        cx.zone_pages.insert(zone.id.get(), (format!("zones/{}-{}", zone.id.get(), slug(&name)), name));
        for sp in &zone.monster_spawns {
            for (list, tactic) in [(&sp.basic_spawns, false), (&sp.tactic_spawns, true)] {
                for (npc, count) in list {
                    cx.spawns.entry(npc.get()).or_default().push(Spawn {
                        zone: zone.id.get(),
                        x: sp.position.x,
                        y: sp.position.y,
                        count: *count,
                        tactic,
                        interval: sp.interval,
                        limit: sp.limit_count,
                    });
                }
            }
        }
        for npc in &zone.npcs {
            cx.placements.entry(npc.npc_id.get()).or_default().push(Placement {
                zone: zone.id.get(),
                x: npc.position.x,
                y: npc.position.y,
                conversation: npc.conversation.get().to_string(),
            });
        }
    }

    // Monsters: anything that spawns, is summoned by a skill or spawned by a quest.
    let mut monsters: BTreeSet<u16> = cx.spawns.keys().copied().collect();
    for skill in game.skills.iter() {
        if let Some(n) = skill.summon_npc_id {
            monsters.insert(n.get());
        }
    }
    for trigger in game.quests.triggers.values() {
        for r in &trigger.rewards {
            if let QsdReward::SpawnMonster { npc, .. } = r {
                monsters.insert(*npc as u16);
            }
        }
    }
    for id in monsters {
        let Some(npc) = rose_data::NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        let name = if npc.name.trim().is_empty() { format!("Monster {id}") } else { npc.name.trim().to_string() };
        cx.monster_pages.insert(id, (format!("monsters/{id}-{}", slug(&name)), name));
    }
    for (&id, _) in cx.placements.iter() {
        let Some(npc) = rose_data::NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        let name = if npc.name.trim().is_empty() { format!("NPC {id}") } else { npc.name.trim().to_string() };
        cx.npc_pages.insert(id, (format!("npcs/{id}-{}", slug(&name)), name));
    }

    for (id, quest) in game.quests.quests.iter().enumerate() {
        let Some(quest) = quest else { continue };
        if quest.name.trim().is_empty() {
            continue;
        }
        cx.quest_pages.insert(id, (format!("quests/{id}-{}", slug(quest.name)), quest.name.trim().to_string()));
    }

    // Dialogs.
    let conversations: BTreeSet<String> =
        cx.placements.values().flatten().map(|p| p.conversation.clone()).collect();
    for c in conversations {
        let Some(data) = game.npcs.get_conversation(&NpcConversationId::new(c.clone())) else { continue };
        let Ok(con) = cx.vfs.read_file::<ConFile, _>(data.filename.as_str()) else { continue };
        let script = Script::load(&con);
        // WIKI_GEN_DUMP=<CON path as LIST_EVENT names it> prints that dialog's script and menus.
        if std::env::var("WIKI_GEN_DUMP").ok().as_deref() == Some(data.filename.as_str()) {
            if let Some(s) = &script {
                s.dump();
            }
            for (i, m) in con.menus.iter().enumerate() {
                for msg in &m.messages {
                    eprintln!("menu {i}: {:?} value {} cond {} act {}", msg.message_type, msg.message_value, msg.condition_function, msg.action_function);
                }
            }
        }
        cx.dialogs.insert(c, Dialog { con, script });
    }
    for (&npc, list) in &cx.placements {
        for p in list {
            let Some(d) = cx.dialogs.get(&p.conversation) else { continue };
            for e in d.all_effects() {
                if let Effect::CheckTrigger(t) | Effect::RunTrigger(t) = e {
                    cx.trigger_npcs.entry(t).or_default().insert(npc);
                }
            }
        }
    }
    for npc in game.npcs.iter() {
        if !npc.death_quest_trigger_name.is_empty() {
            cx.trigger_monsters.entry(npc.death_quest_trigger_name.clone()).or_default().insert(npc.id.get());
        }
    }

    // Which quest each trigger works on.
    for (name, trigger) in &game.quests.triggers {
        let ids = trigger_own_quests(trigger);
        if !ids.is_empty() {
            cx.trigger_quests.entry(name.clone()).or_default().extend(ids);
        }
    }
    // Triggers chained after one that names a quest (alternatives checked in turn) and
    // triggers run by a quest's trigger work on the same quest.
    for _ in 0..4 {
        let mut add: Vec<(String, BTreeSet<usize>)> = Vec::new();
        for (name, trigger) in &game.quests.triggers {
            let Some(ids) = cx.trigger_quests.get(name) else { continue };
            for next in trigger.next_trigger_name.iter().chain(trigger.rewards.iter().filter_map(|r| match r {
                QsdReward::Trigger { name } => Some(name),
                QsdReward::TriggerAfterDelay { trigger, .. } => Some(trigger),
                _ => None,
            })) {
                if !cx.trigger_quests.contains_key(next) {
                    add.push((next.clone(), ids.clone()));
                }
            }
        }
        if add.is_empty() {
            break;
        }
        for (n, ids) in add {
            cx.trigger_quests.entry(n).or_default().extend(ids);
        }
    }
}

/// Quests a trigger names itself: the quest it selects, adds or changes to.
pub fn trigger_own_quests(trigger: &QuestTrigger) -> BTreeSet<usize> {
    let mut ids = BTreeSet::new();
    for c in &trigger.conditions {
        if let QsdCondition::SelectQuest { id } = c {
            ids.insert(*id);
        }
    }
    for r in &trigger.rewards {
        match r {
            QsdReward::AddQuest { id } | QsdReward::SelectQuest { id } | QsdReward::ChangeSelectedQuest { id, .. } => {
                ids.insert(*id);
            }
            _ => {}
        }
    }
    ids
}
