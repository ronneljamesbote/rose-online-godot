//! Monsters, NPCs, their drop tables and the shops' tabs: `monsters.toml` (one
//! `[[monster]]` per monster), `npcs.toml` (`[[npc]]`, the town NPCs), `drop-tables.toml`
//! (`[[drop_table]]`) and `shop-tabs.toml` (`[[shop_tab]]`). The keys are the property names
//! of the Godot resources (`RoseMonster`, `RoseNpc`, `RoseDropTable`, `RoseShopTab`) they are
//! exported from; a key left out has its default (0, empty text, `false`).
//!
//! Monsters and NPCs share one numbering (iROSE's LIST_NPC.STB rows): a monster and an NPC
//! can't have the same id. iROSE uses some columns for one thing on a monster and another on
//! an NPC (the drop table is the NPC's minimap icon, the drop item rate its shop union ...);
//! the game's NPC data has both, so the export fills both from the one field the resource has.
//!
//! Items are named as `<type>/<number>` (`weapon/3`), like the item export's file names.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    num::{NonZeroU16, NonZeroUsize},
    sync::Arc,
};

use anyhow::{bail, Context};
use enum_map::EnumMap;
use rose_data::{
    EffectFileId, EffectId, ItemReference, MotionFileData, MotionId, NpcConversationData, NpcData,
    NpcDatabase, NpcId, NpcMotionAction, NpcStoreTabData, NpcStoreTabId, SoundId, StringDatabase,
};
use serde::{Deserialize, Serialize};

use crate::items::{item_type_name, ITEM_TYPE_FILES};
use crate::Format;

pub const MONSTERS_FILE: &str = "monsters";
pub const NPCS_FILE: &str = "npcs";
pub const DROP_TABLES_FILE: &str = "drop-tables";
pub const SHOP_TABS_FILE: &str = "shop-tabs";

/// Slots in a drop table (iROSE's ITEM_DROP.STB columns 0 to 29) and items in a group
/// (5 columns each, from column 31).
pub const DROP_SLOTS: usize = 30;
pub const DROP_GROUPS: usize = 4;
pub const DROP_GROUP_ITEMS: usize = 5;
/// The columns of a drop table row as the game reads it: the slots, an unused column, then
/// the groups.
pub const DROP_ROW_COLUMNS: usize = DROP_SLOTS + 1 + DROP_GROUPS * DROP_GROUP_ITEMS;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// An item as the export names it: `weapon/3`.
pub fn item_name(item: ItemReference) -> String {
    format!("{}/{}", item_type_name(item.item_type), item.item_number)
}

/// The item an export name (`weapon/3`) means.
pub fn parse_item_name(name: &str) -> Result<ItemReference, anyhow::Error> {
    let (kind, number) = name
        .split_once('/')
        .with_context(|| format!("{name:?} is not an item (type/number)"))?;
    let (item_type, _) = ITEM_TYPE_FILES
        .iter()
        .find(|(_, n)| *n == kind)
        .with_context(|| format!("{name:?}: no item type {kind:?}"))?;
    let number: usize = number
        .parse()
        .with_context(|| format!("{name:?}: {number:?} is not a number"))?;
    if number == 0 {
        bail!("{name:?}: item numbers start at 1");
    }
    Ok(ItemReference::new(*item_type, number))
}

/// An optional item (`""` = none).
fn parse_slot_item(name: &str) -> Result<Option<ItemReference>, anyhow::Error> {
    if name.is_empty() {
        Ok(None)
    } else {
        parse_item_name(name).map(Some)
    }
}

/// Gives `text` a `'static` lifetime by keeping it in `keep`, which the database owns.
fn keep(keep: &mut Vec<String>, text: &str) -> &'static str {
    if text.is_empty() {
        return "";
    }
    let owned = text.to_string();
    // SAFETY: the String's buffer never moves or changes while it sits in `keep`, and
    // `keep` lives in the NpcDatabase (`with_text`) as long as the NPCs pointing into it.
    let text: &'static str = unsafe { std::mem::transmute::<&str, &'static str>(owned.as_str()) };
    keep.push(owned);
    text
}

fn number<T: Copy>(id: Option<T>, get: fn(&T) -> u16) -> u16 {
    id.as_ref().map_or(0, get)
}

fn colour_channel(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// The parts of the game's NPC data a monster or NPC resource doesn't hold yet: its motions
/// (LIST_NPC.CHR, until the models step of the plan), the conversation files and the motion
/// of each action. The iROSE loader gives them (`rose_data_irose::get_npc_motions` ...).
pub struct NpcModels {
    pub motions: HashMap<u16, Vec<(MotionId, MotionFileData)>>,
    pub conversations: HashMap<String, NpcConversationData>,
    pub action_map: EnumMap<NpcMotionAction, MotionId>,
}

/// The fields a monster and an NPC both have, then the kind's own.
macro_rules! character_export {
    ($(#[$meta:meta])* $name:ident { $($(#[$field_meta:meta])* $field:ident: $ty:ty,)* }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(default, deny_unknown_fields)]
        pub struct $name {
            /// The number (iROSE's LIST_NPC.STB row). Spawn points, quests and saved data
            /// name it by this.
            pub id: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub name: String,
            #[serde(skip_serializing_if = "is_default")]
            pub level: i32,
            /// Health per level (the real maximum is level × this).
            #[serde(skip_serializing_if = "is_default")]
            pub health_points: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub attack: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub hit: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub defence: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub resistance: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub avoid: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub attack_speed: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub is_magic_damage: bool,
            /// Centimetres.
            #[serde(skip_serializing_if = "is_default")]
            pub attack_range: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub walk_speed: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub run_speed: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub reward_xp: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub is_untargetable: bool,
            #[serde(skip_serializing_if = "is_default")]
            pub ai_file_index: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub death_quest_trigger_name: String,
            #[serde(skip_serializing_if = "is_default")]
            pub npc_quest_type: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub npc_type_index: u16,
            /// Size in percent of the model's.
            #[serde(skip_serializing_if = "is_default")]
            pub scale: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub height: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub right_hand_part_index: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub left_hand_part_index: u32,
            /// Red, green, blue from 0 to 255.
            #[serde(skip_serializing_if = "is_default")]
            pub glow_colour: [u8; 3],
            #[serde(skip_serializing_if = "is_default")]
            pub hit_sound_material_type: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub normal_effect_sound: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub attack_sound: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub hitted_sound: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub hand_hit_effect_id: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub die_effect_file_id: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub die_sound: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub create_effect_index: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub create_sound: u16,
            $(
                $(#[$field_meta])*
                #[serde(skip_serializing_if = "is_default")]
                pub $field: $ty,
            )*
        }

        impl $name {
            fn set_common(&mut self, npc: &NpcData) {
                self.id = npc.id.get();
                self.name = npc.name.to_string();
                self.level = npc.level;
                self.health_points = npc.health_points;
                self.attack = npc.attack;
                self.hit = npc.hit;
                self.defence = npc.defence;
                self.resistance = npc.resistance;
                self.avoid = npc.avoid;
                self.attack_speed = npc.attack_speed;
                self.is_magic_damage = npc.is_attack_magic_damage;
                self.attack_range = npc.attack_range;
                self.walk_speed = npc.walk_speed;
                self.run_speed = npc.run_speed;
                self.reward_xp = npc.reward_xp;
                self.is_untargetable = npc.is_untargetable;
                self.ai_file_index = npc.ai_file_index;
                self.death_quest_trigger_name = npc.death_quest_trigger_name.clone();
                self.npc_quest_type = npc.npc_quest_type;
                self.npc_type_index = npc.npc_type_index.map_or(0, NonZeroU16::get);
                self.scale = (npc.scale * 100.0).round() as u32;
                self.height = npc.npc_height;
                self.right_hand_part_index = npc.right_hand_part_index;
                self.left_hand_part_index = npc.left_hand_part_index;
                let (r, g, b) = npc.glow_colour;
                self.glow_colour = [colour_channel(r), colour_channel(g), colour_channel(b)];
                self.hit_sound_material_type = npc.hit_sound_material_type;
                self.normal_effect_sound = number(npc.normal_effect_sound_id, SoundId::get);
                self.attack_sound = number(npc.attack_sound_id, SoundId::get);
                self.hitted_sound = number(npc.hitted_sound_id, SoundId::get);
                self.hand_hit_effect_id = number(npc.hand_hit_effect_id, EffectId::get);
                self.die_effect_file_id = number(npc.die_effect_file_id, EffectFileId::get);
                self.die_sound = number(npc.die_sound_id, SoundId::get);
                self.create_effect_index = npc.create_effect_index;
                self.create_sound = number(npc.create_sound_id, SoundId::get);
            }

            /// The game's NPC data with the fields the kind doesn't have left at none; the
            /// kind fills them in.
            fn common(
                &self,
                text: &mut Vec<String>,
                motions: &HashMap<u16, Vec<(MotionId, MotionFileData)>>,
            ) -> Result<NpcData, anyhow::Error> {
                let id = NpcId::new(self.id).context("ids start at 1")?;
                let channel = |i: usize| self.glow_colour[i] as f32 / 255.0;
                Ok(NpcData {
                    id,
                    name: keep(text, &self.name),
                    walk_speed: self.walk_speed,
                    run_speed: self.run_speed,
                    scale: self.scale as f32 / 100.0,
                    right_hand_part_index: self.right_hand_part_index,
                    left_hand_part_index: self.left_hand_part_index,
                    level: self.level,
                    health_points: self.health_points,
                    attack: self.attack,
                    hit: self.hit,
                    defence: self.defence,
                    resistance: self.resistance,
                    avoid: self.avoid,
                    attack_speed: self.attack_speed,
                    is_attack_magic_damage: self.is_magic_damage,
                    ai_file_index: self.ai_file_index,
                    reward_xp: self.reward_xp,
                    drop_table_index: 0,
                    drop_money_rate: 0,
                    drop_item_rate: 0,
                    npc_minimap_icon_index: 0,
                    summon_point_requirement: 0,
                    store_tabs: [None; 4],
                    store_union_number: None,
                    is_untargetable: self.is_untargetable,
                    attack_range: self.attack_range,
                    npc_type_index: NonZeroU16::new(self.npc_type_index),
                    hit_sound_material_type: self.hit_sound_material_type,
                    face_icon_index: 0,
                    summon_monster_type: 0,
                    normal_effect_sound_id: SoundId::new(self.normal_effect_sound),
                    attack_sound_id: SoundId::new(self.attack_sound),
                    hitted_sound_id: SoundId::new(self.hitted_sound),
                    hand_hit_effect_id: EffectId::new(self.hand_hit_effect_id),
                    die_effect_file_id: EffectFileId::new(self.die_effect_file_id),
                    die_sound_id: SoundId::new(self.die_sound),
                    npc_quest_type: self.npc_quest_type,
                    glow_colour: (channel(0), channel(1), channel(2)),
                    create_effect_index: self.create_effect_index,
                    create_sound_id: SoundId::new(self.create_sound),
                    death_quest_trigger_name: self.death_quest_trigger_name.clone(),
                    npc_height: self.height,
                    motion_data: motions.get(&self.id).cloned().unwrap_or_default(),
                })
            }
        }
    };
}

character_export!(
    /// A monster (`monsters.toml`): anything that isn't a town NPC, also the ones only
    /// skills or quests summon.
    MonsterExport {
        /// The drop table (its id, 0 = none). On an NPC this column is its minimap icon.
        drop_table: u32,
        /// Chance in percent that a drop is money.
        drop_money_rate: i32,
        /// Chance in percent that an item drop comes from the monster's own table rather
        /// than the zone's. On an NPC this column is its shop union.
        drop_item_rate: i32,
        /// Summon points a summoned monster takes from its summoner. On an NPC this column is
        /// its first shop tab.
        summon_point_requirement: u32,
        /// On an NPC this column is its face icon.
        summon_monster_type: u32,
    }
);

character_export!(
    /// A town NPC (`npcs.toml`): one a zone places as an NPC (IFO or zone scene).
    NpcExport {
        /// The icon on the minimap. On a monster this column is its drop table.
        minimap_icon_index: u32,
        /// The union whose members get the union price (0 = none). On a monster this column
        /// is its drop item rate.
        store_union: u32,
        /// The shop tabs (their ids, 0 = none).
        shop_tab_1: u16,
        shop_tab_2: u16,
        shop_tab_3: u16,
        shop_tab_4: u16,
        /// The portrait in the dialog window. On a monster this column is its summon type.
        face_icon_index: u32,
    }
);

impl MonsterExport {
    pub fn from_data(npc: &NpcData) -> Self {
        let mut out = Self::default();
        out.set_common(npc);
        out.drop_table = npc.drop_table_index;
        out.drop_money_rate = npc.drop_money_rate;
        out.drop_item_rate = npc.drop_item_rate;
        out.summon_point_requirement = npc.summon_point_requirement;
        out.summon_monster_type = npc.summon_monster_type;
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>, models: &NpcModels) -> Result<NpcData, anyhow::Error> {
        let mut npc = self.common(text, &models.motions)?;
        npc.drop_table_index = self.drop_table;
        npc.npc_minimap_icon_index = self.drop_table;
        npc.drop_money_rate = self.drop_money_rate;
        npc.drop_item_rate = self.drop_item_rate;
        npc.store_union_number = usize::try_from(self.drop_item_rate).ok().and_then(NonZeroUsize::new);
        npc.summon_point_requirement = self.summon_point_requirement;
        npc.store_tabs[0] = u16::try_from(self.summon_point_requirement)
            .ok()
            .and_then(NpcStoreTabId::new);
        npc.summon_monster_type = self.summon_monster_type;
        npc.face_icon_index = self.summon_monster_type;
        Ok(npc)
    }
}

impl NpcExport {
    pub fn shop_tabs(&self) -> [u16; 4] {
        [self.shop_tab_1, self.shop_tab_2, self.shop_tab_3, self.shop_tab_4]
    }

    pub fn from_data(npc: &NpcData) -> Self {
        let mut out = Self::default();
        out.set_common(npc);
        out.minimap_icon_index = npc.npc_minimap_icon_index;
        out.store_union = npc.drop_item_rate.max(0) as u32;
        let tab = |i: usize| number(npc.store_tabs[i], NpcStoreTabId::get);
        (out.shop_tab_1, out.shop_tab_2, out.shop_tab_3, out.shop_tab_4) = (tab(0), tab(1), tab(2), tab(3));
        out.face_icon_index = npc.face_icon_index;
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>, models: &NpcModels) -> Result<NpcData, anyhow::Error> {
        let mut npc = self.common(text, &models.motions)?;
        npc.npc_minimap_icon_index = self.minimap_icon_index;
        npc.drop_table_index = self.minimap_icon_index;
        npc.drop_item_rate = self.store_union as i32;
        npc.store_union_number = NonZeroUsize::new(self.store_union as usize);
        npc.summon_point_requirement = self.shop_tab_1 as u32;
        npc.store_tabs = self.shop_tabs().map(NpcStoreTabId::new);
        npc.face_icon_index = self.face_icon_index;
        npc.summon_monster_type = self.face_icon_index;
        Ok(npc)
    }
}

/// One slot of a drop table: an item (`""` = nothing), or a group number from 1 to 4 (one
/// of that group's 5 items, at random).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropSlot {
    Group(u8),
    Item(String),
}

impl Default for DropSlot {
    fn default() -> Self {
        DropSlot::Item(String::new())
    }
}

/// A drop table (`drop-tables.toml`): what a monster (or a zone, by the zone's number) drops.
/// How a drop picks a slot is on the wiki page "Drops".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DropTableExport {
    /// The number (iROSE's ITEM_DROP.STB row). A zone's drops are the table with its number.
    pub id: u32,
    /// For finding it only (the monsters or zone using it).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// Up to 30 slots, in order (the order matters: low slots drop more often).
    #[serde(skip_serializing_if = "is_default")]
    pub slots: Vec<DropSlot>,
    #[serde(skip_serializing_if = "is_default")]
    pub group_1: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub group_2: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub group_3: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub group_4: Vec<String>,
}

fn trim_end<T: PartialEq + Default>(mut list: Vec<T>) -> Vec<T> {
    while list.last().is_some_and(|v| *v == T::default()) {
        list.pop();
    }
    list
}

impl DropTableExport {
    /// From a drop table row as the game reads it (`DROP_ROW_COLUMNS` numbers); `decode`
    /// turns a cell into an item (iROSE's type × 1000 + number). A cell that is neither an
    /// item nor a group is an error.
    pub fn from_row(
        id: u32,
        name: &str,
        row: &[i32],
        decode: impl Fn(i32) -> Option<ItemReference>,
    ) -> Result<Self, anyhow::Error> {
        let item = |column: usize| -> Result<String, anyhow::Error> {
            let value = row.get(column).copied().unwrap_or(0);
            if value == 0 {
                return Ok(String::new());
            }
            decode(value)
                .map(item_name)
                .with_context(|| format!("drop table {id} column {column}: {value} is not an item"))
        };
        let mut slots = Vec::new();
        for column in 0..DROP_SLOTS {
            let value = row.get(column).copied().unwrap_or(0);
            slots.push(if (1..=DROP_GROUPS as i32).contains(&value) {
                DropSlot::Group(value as u8)
            } else {
                DropSlot::Item(item(column)?)
            });
        }
        let group = |g: usize| -> Result<Vec<String>, anyhow::Error> {
            let start = DROP_SLOTS + 1 + g * DROP_GROUP_ITEMS;
            Ok(trim_end((start..start + DROP_GROUP_ITEMS).map(item).collect::<Result<_, _>>()?))
        };
        Ok(Self {
            id,
            name: name.to_string(),
            slots: trim_end(slots),
            group_1: group(0)?,
            group_2: group(1)?,
            group_3: group(2)?,
            group_4: group(3)?,
        })
    }

    pub fn groups(&self) -> [&Vec<String>; DROP_GROUPS] {
        [&self.group_1, &self.group_2, &self.group_3, &self.group_4]
    }

    /// The row as the game reads it; `encode` turns an item into iROSE's number.
    pub fn to_row(&self, encode: impl Fn(ItemReference) -> Option<i32>) -> Result<Vec<i32>, anyhow::Error> {
        let mut row = vec![0; DROP_ROW_COLUMNS];
        if self.slots.len() > DROP_SLOTS {
            bail!("drop table {}: {} slots, at most {DROP_SLOTS}", self.id, self.slots.len());
        }
        let code = |name: &str| -> Result<i32, anyhow::Error> {
            match parse_slot_item(name)? {
                None => Ok(0),
                Some(item) => encode(item).with_context(|| format!("{name} has no iROSE number")),
            }
        };
        for (i, slot) in self.slots.iter().enumerate() {
            row[i] = match slot {
                DropSlot::Group(g) if (1..=DROP_GROUPS as u8).contains(g) => *g as i32,
                DropSlot::Group(g) => bail!("drop table {} slot {}: no group {g} (1 to {DROP_GROUPS})", self.id, i + 1),
                DropSlot::Item(name) => code(name).with_context(|| format!("drop table {} slot {}", self.id, i + 1))?,
            };
        }
        for (g, items) in self.groups().into_iter().enumerate() {
            if items.len() > DROP_GROUP_ITEMS {
                bail!("drop table {} group {}: {} items, at most {DROP_GROUP_ITEMS}", self.id, g + 1, items.len());
            }
            for (i, name) in items.iter().enumerate() {
                row[DROP_SLOTS + 1 + g * DROP_GROUP_ITEMS + i] =
                    code(name).with_context(|| format!("drop table {} group {}", self.id, g + 1))?;
            }
        }
        Ok(row)
    }

    /// Every item it names.
    pub fn items(&self) -> impl Iterator<Item = &str> {
        self.slots
            .iter()
            .filter_map(|s| match s {
                DropSlot::Item(name) => Some(name.as_str()),
                DropSlot::Group(_) => None,
            })
            .chain(self.groups().into_iter().flatten().map(String::as_str))
            .filter(|n| !n.is_empty())
    }
}

/// A shop tab (`shop-tabs.toml`): the items an NPC's shop shows on one tab, in their places.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ShopTabExport {
    /// The number (iROSE's LIST_SELL.STB row). NPCs name their tabs by it.
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The items in shop order (`""` = an empty place), up to 48.
    #[serde(skip_serializing_if = "is_default")]
    pub items: Vec<String>,
}

/// Places on a shop tab (iROSE's LIST_SELL.STB columns 2 to 49).
pub const SHOP_TAB_ITEMS: usize = 48;

impl ShopTabExport {
    pub fn from_data(id: NpcStoreTabId, tab: &NpcStoreTabData) -> Self {
        let mut items = vec![String::new(); tab.items.keys().map(|i| *i as usize + 1).max().unwrap_or(0)];
        for (place, item) in &tab.items {
            items[*place as usize] = item_name(*item);
        }
        Self {
            id: id.get(),
            name: tab.name.to_string(),
            items,
        }
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<NpcStoreTabData, anyhow::Error> {
        if self.items.len() > SHOP_TAB_ITEMS {
            bail!("shop tab {}: {} items, at most {SHOP_TAB_ITEMS}", self.id, self.items.len());
        }
        let mut items = HashMap::new();
        for (place, name) in self.items.iter().enumerate() {
            if let Some(item) = parse_slot_item(name).with_context(|| format!("shop tab {}", self.id))? {
                items.insert(place as u16, item);
            }
        }
        Ok(NpcStoreTabData {
            name: keep(text, &self.name),
            items,
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MonsterFile {
    #[serde(default)]
    monster: Vec<MonsterExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NpcFile {
    #[serde(default)]
    npc: Vec<NpcExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DropTableFile {
    #[serde(default)]
    drop_table: Vec<DropTableExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ShopTabFile {
    #[serde(default)]
    shop_tab: Vec<ShopTabExport>,
}

/// Every monster, NPC, drop table and shop tab, as exported.
#[derive(Debug, Default)]
pub struct NpcsExport {
    pub monsters: Vec<MonsterExport>,
    pub npcs: Vec<NpcExport>,
    pub drop_tables: Vec<DropTableExport>,
    pub shop_tabs: Vec<ShopTabExport>,
}

/// What the converter left out because it can't be a link: a shop place naming an item
/// that doesn't exist (the shop never showed it).
pub type Skipped = Vec<String>;

impl NpcsExport {
    /// The converter's starting point and the same-data check: the NPC database with
    /// `town_npcs` (the ids any zone places as an NPC) as NPCs and the rest as monsters, the
    /// drop table rows (by row number, `DROP_ROW_COLUMNS` cells each) and `item_exists` to
    /// leave out shop places naming missing items.
    pub fn from_database(
        npcs: &NpcDatabase,
        town_npcs: &BTreeSet<u16>,
        drop_rows: &BTreeMap<u32, (String, Vec<i32>)>,
        decode: impl Fn(i32) -> Option<ItemReference>,
        item_exists: impl Fn(ItemReference) -> bool,
    ) -> Result<(Self, Skipped), anyhow::Error> {
        let mut out = Self::default();
        let mut skipped = Vec::new();
        for npc in npcs.iter() {
            if town_npcs.contains(&npc.id.get()) {
                out.npcs.push(NpcExport::from_data(npc));
            } else {
                out.monsters.push(MonsterExport::from_data(npc));
            }
        }
        let mut tab_ids: BTreeSet<u16> = out.npcs.iter().flat_map(|n| n.shop_tabs()).filter(|t| *t != 0).collect();
        tab_ids.extend(npcs.store_tabs().map(|(id, _)| id.get()));
        for id in tab_ids {
            let tab = NpcStoreTabId::new(id).and_then(|t| npcs.get_store_tab(t));
            let mut export = match tab {
                Some(tab) => ShopTabExport::from_data(NpcStoreTabId::new(id).unwrap(), tab),
                None => ShopTabExport { id, ..Default::default() },
            };
            for (place, name) in export.items.iter_mut().enumerate() {
                if let Ok(Some(item)) = parse_slot_item(name) {
                    if !item_exists(item) {
                        skipped.push(format!("shop tab {id} place {}: {name} doesn't exist", place + 1));
                        name.clear();
                    }
                }
            }
            export.items = trim_end(export.items);
            out.shop_tabs.push(export);
        }
        for (id, (name, row)) in drop_rows {
            out.drop_tables.push(DropTableExport::from_row(*id, name, row, &decode)?);
        }
        Ok((out, skipped))
    }

    /// The NPC database: the export's monsters and NPCs with `models` for what they don't
    /// hold yet. `strings` is only kept for the other databases that share it.
    pub fn to_database(&self, strings: Arc<StringDatabase>, models: NpcModels) -> Result<NpcDatabase, anyhow::Error> {
        self.check()?;
        let mut text = Vec::new();
        let mut table: Vec<Option<NpcData>> = Vec::new();
        let mut put = |npc: NpcData| {
            let at = npc.id.get() as usize;
            if table.len() <= at {
                table.resize_with(at + 1, || None);
            }
            table[at] = Some(npc);
        };
        for monster in &self.monsters {
            put(monster.to_data(&mut text, &models).with_context(|| format!("monster {}", monster.id))?);
        }
        for npc in &self.npcs {
            put(npc.to_data(&mut text, &models).with_context(|| format!("npc {}", npc.id))?);
        }
        let mut tabs = HashMap::new();
        for tab in &self.shop_tabs {
            let data = tab.to_data(&mut text)?;
            // Like iROSE: a tab with no items is no tab.
            if !data.items.is_empty() {
                tabs.insert(NpcStoreTabId::new(tab.id).context("shop tab ids start at 1")?, data);
            }
        }
        Ok(NpcDatabase::new(strings, table, models.conversations, tabs, models.action_map).with_text(text))
    }

    /// The drop table rows as the game reads them, by row number.
    pub fn drop_rows(&self, encode: impl Fn(ItemReference) -> Option<i32>) -> Result<BTreeMap<u32, Vec<i32>>, anyhow::Error> {
        let mut rows = BTreeMap::new();
        for table in &self.drop_tables {
            rows.insert(table.id, table.to_row(&encode)?);
        }
        Ok(rows)
    }

    /// Ids used twice, and links to drop tables or shop tabs that aren't there.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut ids = BTreeMap::new();
        for (id, what) in self
            .monsters
            .iter()
            .map(|m| (m.id, "monster"))
            .chain(self.npcs.iter().map(|n| (n.id, "npc")))
        {
            if id == 0 {
                bail!("{what} 0: ids start at 1");
            }
            if let Some(other) = ids.insert(id, what) {
                bail!("{other} {id} and {what} {id}: monsters and NPCs share their numbers, one can't take another's");
            }
        }
        let tables: BTreeSet<u32> = self.drop_tables.iter().map(|t| t.id).collect();
        if tables.len() != self.drop_tables.len() {
            bail!("{}: a drop table id is there twice", DROP_TABLES_FILE);
        }
        let tabs: BTreeSet<u16> = self.shop_tabs.iter().map(|t| t.id).collect();
        if tabs.len() != self.shop_tabs.len() {
            bail!("{}: a shop tab id is there twice", SHOP_TABS_FILE);
        }
        for monster in &self.monsters {
            if monster.drop_table != 0 && !tables.contains(&monster.drop_table) {
                bail!("monster {} names drop table {}, which isn't there", monster.id, monster.drop_table);
            }
        }
        for npc in &self.npcs {
            for tab in npc.shop_tabs() {
                if tab != 0 && !tabs.contains(&tab) {
                    bail!("npc {} names shop tab {tab}, which isn't there", npc.id);
                }
            }
        }
        for table in &self.drop_tables {
            for name in table.items() {
                parse_item_name(name).with_context(|| format!("drop table {}", table.id))?;
            }
        }
        Ok(())
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut monster = self.monsters.clone();
        monster.sort_by_key(|m| m.id);
        let mut npc = self.npcs.clone();
        npc.sort_by_key(|n| n.id);
        let mut drop_table = self.drop_tables.clone();
        drop_table.sort_by_key(|t| t.id);
        let mut shop_tab = self.shop_tabs.clone();
        shop_tab.sort_by_key(|t| t.id);
        Ok(vec![
            (MONSTERS_FILE, format.write(MONSTERS_FILE, &MonsterFile { monster })?),
            (NPCS_FILE, format.write(NPCS_FILE, &NpcFile { npc })?),
            (DROP_TABLES_FILE, format.write(DROP_TABLES_FILE, &DropTableFile { drop_table })?),
            (SHOP_TABS_FILE, format.write(SHOP_TABS_FILE, &ShopTabFile { shop_tab })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; some missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let files = [MONSTERS_FILE, NPCS_FILE, DROP_TABLES_FILE, SHOP_TABS_FILE];
        let mut texts = Vec::new();
        for name in files {
            texts.push(read(name)?);
        }
        match texts.iter().filter(|t| t.is_some()).count() {
            0 => return Ok(None),
            n if n < files.len() => bail!(
                "the monster and NPC export has no {}",
                files
                    .iter()
                    .zip(&texts)
                    .filter(|(_, t)| t.is_none())
                    .map(|(n, _)| format!("{n}.{}", format.extension()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => {}
        }
        let text = |i: usize| texts[i].as_deref().unwrap();
        Ok(Some(Self {
            monsters: format.parse::<MonsterFile>(MONSTERS_FILE, text(0))?.monster,
            npcs: format.parse::<NpcFile>(NPCS_FILE, text(1))?.npc,
            drop_tables: format.parse::<DropTableFile>(DROP_TABLES_FILE, text(2))?.drop_table,
            shop_tabs: format.parse::<ShopTabFile>(SHOP_TABS_FILE, text(3))?.shop_tab,
        }))
    }
}

/// Every difference between two NPC databases: one line per NPC field that differs
/// (compared by their debug output, so every field counts), missing or extra NPCs, and shop
/// tabs that differ.
pub fn compare_npc_databases(a: &NpcDatabase, b: &NpcDatabase) -> Vec<String> {
    let mut out = Vec::new();
    let ids: BTreeSet<u16> = a.iter().chain(b.iter()).map(|n| n.id.get()).collect();
    for id in ids {
        let npc = NpcId::new(id).unwrap();
        match (a.get_npc(npc), b.get_npc(npc)) {
            (Some(_), None) => out.push(format!("npc {id}: missing")),
            (None, Some(_)) => out.push(format!("npc {id}: extra")),
            (x, y) => {
                let (x, y) = (format!("{x:#?}"), format!("{y:#?}"));
                if x != y {
                    for (l, r) in x.lines().zip(y.lines()).filter(|(l, r)| l != r) {
                        out.push(format!("npc {id}: {} -> {}", l.trim(), r.trim()));
                    }
                    if x.lines().count() != y.lines().count() {
                        out.push(format!("npc {id}: the fields differ"));
                    }
                }
            }
        }
    }
    let tab = |db: &NpcDatabase, id: NpcStoreTabId| {
        db.get_store_tab(id).map(|t| {
            let items: BTreeMap<u16, String> = t.items.iter().map(|(p, i)| (*p, item_name(*i))).collect();
            format!("{:?} {items:?}", t.name)
        })
    };
    let tabs: BTreeSet<u16> = a.store_tabs().chain(b.store_tabs()).map(|(id, _)| id.get()).collect();
    for id in tabs {
        let id = NpcStoreTabId::new(id).unwrap();
        let (x, y) = (tab(a, id), tab(b, id));
        if x != y {
            out.push(format!("shop tab {}: {x:?} -> {y:?}", id.get()));
        }
    }
    out
}

/// Every difference between two sets of drop table rows, in the cells the game reads (a
/// missing row reads as all zeros).
pub fn compare_drop_rows(a: &BTreeMap<u32, Vec<i32>>, b: &BTreeMap<u32, Vec<i32>>) -> Vec<String> {
    let mut out = Vec::new();
    let rows: BTreeSet<u32> = a.keys().chain(b.keys()).copied().collect();
    for row in rows {
        for column in (0..DROP_SLOTS).chain(DROP_SLOTS + 1..DROP_ROW_COLUMNS) {
            let cell = |m: &BTreeMap<u32, Vec<i32>>| m.get(&row).and_then(|r| r.get(column).copied()).unwrap_or(0);
            let (x, y) = (cell(a), cell(b));
            if x != y {
                out.push(format!("drop table {row} column {column}: {x} -> {y}"));
            }
        }
    }
    out
}
