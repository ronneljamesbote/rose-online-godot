//! Items: one file per item type (`weapon.toml`, `head.toml`, ...) with one `[[item]]`
//! table per item, and `grades.toml` with one `[[grade]]` per refine grade. The keys are the
//! property names of the Godot resources (`RoseItemWeapon`, ...) they are exported from. A
//! key left out has its default: 0, empty text, `false`, or no stat.

use std::{num::NonZeroUsize, sync::Arc, time::Duration};

use anyhow::{bail, Context};
use arrayvec::ArrayVec;
use rose_data::{
    AbilityType, BackItemData, BaseItemData, BodyItemData, ConsumableItemData, EffectFileId,
    EffectId, FaceItemData, FeetItemData, GemItemData, HandsItemData, HeadItemData, ItemClass,
    ItemDatabase, ItemGradeData, ItemReference, ItemType, JewelleryItemData, JobClassId,
    MaterialItemData, QuestItemData, SkillId, SoundId, StatusEffectId, StringDatabase,
    SubWeaponItemData, VehicleItemData, VehiclePartIndex, VehicleType, WeaponItemData,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// An id number where 0 means none.
fn id<T>(number: u16, new: fn(u16) -> Option<T>) -> Option<T> {
    new(number)
}

fn number<T: Copy>(id: Option<T>, get: fn(&T) -> u16) -> u16 {
    id.as_ref().map_or(0, get)
}

/// A stat and its value, from the export's `<name>` and `<name>_value` pair.
fn stat<V: Copy>(stat: Option<AbilityType>, value: V) -> Option<(AbilityType, V)> {
    stat.map(|stat| (stat, value))
}

/// Gives `text` a `'static` lifetime by keeping it in `keep`, which the database owns.
fn keep(keep: &mut Vec<String>, text: &str) -> &'static str {
    if text.is_empty() {
        return "";
    }
    let owned = text.to_string();
    // SAFETY: the String's buffer never moves or changes while it sits in `keep`, and
    // `keep` lives in the ItemDatabase (`with_text`) as long as the items pointing into it.
    let text: &'static str = unsafe { std::mem::transmute::<&str, &'static str>(owned.as_str()) };
    keep.push(owned);
    text
}

/// Whether an item type looks different on male and female characters (its own model for
/// each), rather than one model for everyone.
pub fn gendered_models(item_type: ItemType) -> bool {
    matches!(item_type, ItemType::Head | ItemType::Body | ItemType::Hands | ItemType::Feet)
}

/// One export struct per item type: the fields every item has, then the type's own.
macro_rules! item_export {
    ($(#[$meta:meta])* $name:ident, $item_type:ident { $($(#[$field_meta:meta])* $field:ident: $ty:ty,)* }) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
        #[serde(default, deny_unknown_fields)]
        pub struct $name {
            /// The item number (the row in iROSE's table). Saved characters store it.
            pub id: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub name: String,
            #[serde(skip_serializing_if = "is_default")]
            pub description: String,
            #[serde(skip_serializing_if = "is_default")]
            pub item_class: ItemClass,
            /// Bit 0x02: can't be dropped, traded or sold to players.
            #[serde(skip_serializing_if = "is_default")]
            pub trade_restriction: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub base_price: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub price_rate: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub weight: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub quality: u32,
            /// The icon: a texture's Godot path (`res://assets/rose/3ddata/control/res/item1/919.tres`
            /// for the imported ones).
            #[serde(skip_serializing_if = "is_default")]
            pub icon: String,
            /// The scene shown when it lies on the ground.
            #[serde(skip_serializing_if = "is_default")]
            pub field_model: String,
            /// The scene worn or held, the same on everyone (face and back items, weapons,
            /// shields, cart parts).
            #[serde(skip_serializing_if = "is_default")]
            pub model: String,
            /// The scenes worn by male and female characters (hats, armour, gloves, shoes).
            #[serde(skip_serializing_if = "is_default")]
            pub model_male: String,
            #[serde(skip_serializing_if = "is_default")]
            pub model_female: String,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_sound: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub craft_skill_type: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub craft_skill_level: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub craft_material: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub craft_difficulty: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_job_class: u16,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_union_1: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_union_2: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_stat_1: Option<AbilityType>,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_stat_1_value: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_stat_2: Option<AbilityType>,
            #[serde(skip_serializing_if = "is_default")]
            pub equip_stat_2_value: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_union_1: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_union_2: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_1: Option<AbilityType>,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_1_value: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_2: Option<AbilityType>,
            #[serde(skip_serializing_if = "is_default")]
            pub add_stat_2_value: i32,
            #[serde(skip_serializing_if = "is_default")]
            pub durability: u8,
            #[serde(skip_serializing_if = "is_default")]
            pub rare_type: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub defence: u32,
            #[serde(skip_serializing_if = "is_default")]
            pub resistance: u32,
            $(
                $(#[$field_meta])*
                #[serde(skip_serializing_if = "is_default")]
                pub $field: $ty,
            )*
        }

        impl $name {
            pub const ITEM_TYPE: ItemType = ItemType::$item_type;

            fn set_base(&mut self, item: &BaseItemData) {
                self.id = item.id.item_number as u32;
                self.name = item.name.to_string();
                self.description = item.description.to_string();
                self.item_class = item.class;
                self.trade_restriction = item.trade_restriction;
                self.base_price = item.base_price;
                self.price_rate = item.price_rate;
                self.weight = item.weight;
                self.quality = item.quality;
                self.icon = item.icon.clone();
                self.field_model = item.field_model.clone();
                if gendered_models(Self::ITEM_TYPE) {
                    (self.model_male, self.model_female) = (item.model_male.clone(), item.model_female.clone());
                } else {
                    self.model = item.model_male.clone();
                }
                self.equip_sound = number(item.equip_sound_id, SoundId::get);
                self.craft_skill_type = item.craft_skill_type;
                self.craft_skill_level = item.craft_skill_level;
                self.craft_material = item.craft_material;
                self.craft_difficulty = item.craft_difficulty;
                self.equip_job_class = number(item.equip_job_class_requirement, JobClassId::get);
                let union = |i: usize| item.equip_union_requirement.get(i).map_or(0, |u| u.get() as u32);
                (self.equip_union_1, self.equip_union_2) = (union(0), union(1));
                let needs = |i: usize| item.equip_ability_requirement.get(i).copied();
                (self.equip_stat_1, self.equip_stat_1_value) = needs(0).map_or((None, 0), |(s, v)| (Some(s), v));
                (self.equip_stat_2, self.equip_stat_2_value) = needs(1).map_or((None, 0), |(s, v)| (Some(s), v));
                let add_union = |i: usize| item.add_ability_union_requirement.get(i).copied().unwrap_or(0);
                (self.add_stat_union_1, self.add_stat_union_2) = (add_union(0), add_union(1));
                let adds = |i: usize| item.add_ability.get(i).copied();
                (self.add_stat_1, self.add_stat_1_value) = adds(0).map_or((None, 0), |(s, v)| (Some(s), v));
                (self.add_stat_2, self.add_stat_2_value) = adds(1).map_or((None, 0), |(s, v)| (Some(s), v));
                self.durability = item.durability;
                self.rare_type = item.rare_type;
                self.defence = item.defence;
                self.resistance = item.resistance;
            }

            fn base(&self, text: &mut Vec<String>) -> Result<BaseItemData, anyhow::Error> {
                let mut equip_union_requirement = ArrayVec::new();
                for union in [self.equip_union_1, self.equip_union_2] {
                    if let Some(union) = NonZeroUsize::new(union as usize) {
                        equip_union_requirement.push(union);
                    }
                }
                let mut add_ability_union_requirement = ArrayVec::new();
                for union in [self.add_stat_union_1, self.add_stat_union_2] {
                    if union != 0 {
                        add_ability_union_requirement.push(union);
                    }
                }
                let equip_ability_requirement = [
                    stat(self.equip_stat_1, self.equip_stat_1_value),
                    stat(self.equip_stat_2, self.equip_stat_2_value),
                ]
                .into_iter()
                .flatten()
                .collect();
                let add_ability =
                    [stat(self.add_stat_1, self.add_stat_1_value), stat(self.add_stat_2, self.add_stat_2_value)]
                        .into_iter()
                        .flatten()
                        .collect();
                Ok(BaseItemData {
                    id: ItemReference::new(Self::ITEM_TYPE, self.id as usize),
                    trade_restriction: self.trade_restriction,
                    name: keep(text, &self.name),
                    description: keep(text, &self.description),
                    class: self.item_class,
                    base_price: self.base_price,
                    price_rate: self.price_rate,
                    weight: self.weight,
                    quality: self.quality,
                    icon: self.icon.clone(),
                    field_model: self.field_model.clone(),
                    model_male: if self.model.is_empty() { self.model_male.clone() } else { self.model.clone() },
                    model_female: if self.model.is_empty() { self.model_female.clone() } else { self.model.clone() },
                    equip_sound_id: id(self.equip_sound, SoundId::new),
                    craft_skill_type: self.craft_skill_type,
                    craft_skill_level: self.craft_skill_level,
                    craft_material: self.craft_material,
                    craft_difficulty: self.craft_difficulty,
                    equip_job_class_requirement: id(self.equip_job_class, JobClassId::new),
                    equip_union_requirement,
                    equip_ability_requirement,
                    add_ability_union_requirement,
                    add_ability,
                    durability: self.durability,
                    rare_type: self.rare_type,
                    defence: self.defence,
                    resistance: self.resistance,
                })
            }
        }
    };
}

item_export!(
    /// A mask, glasses or other face item (`face.toml`).
    FaceExport, Face {}
);
item_export!(
    /// A helmet or hat (`head.toml`).
    HeadExport, Head {
        hair_type: u32,
    }
);
item_export!(
    /// Armour or clothes (`body.toml`).
    BodyExport, Body {}
);
item_export!(
    /// Gloves and gauntlets (`hands.toml`).
    HandsExport, Hands {}
);
item_export!(
    /// Boots and shoes (`feet.toml`). Item 0 holds the move speed without shoes.
    FeetExport, Feet {
        move_speed: u32,
    }
);
item_export!(
    /// Back armour, bags, wings and ammunition boxes (`back.toml`).
    BackExport, Back {
        move_speed: u32,
    }
);
item_export!(
    /// Rings, necklaces and earrings (`jewellery.toml`).
    JewelleryExport, Jewellery {}
);
item_export!(
    /// A weapon (`weapon.toml`). Item 0 is fighting unarmed.
    WeaponExport, Weapon {
        attack_range: i32,
        attack_power: i32,
        attack_speed: i32,
        motion_type: u32,
        is_magic_damage: bool,
        bullet_effect: u16,
        attack_effect: u16,
        attack_start_sound: u16,
        attack_fire_sound: u16,
        hit_sounds: u32,
        gem_position: u32,
    }
);
item_export!(
    /// A shield or support tool (`subweapon.toml`).
    SubWeaponExport, SubWeapon {
        gem_position: u32,
    }
);
item_export!(
    /// A potion, food, scroll, skill book, repair tool ... (`consumable.toml`).
    ConsumableExport, Consumable {
        store_skin: i32,
        confile_index: usize,
        add_fuel: i32,
        use_need_stat: Option<AbilityType>,
        use_need_stat_value: i32,
        use_add_stat: Option<AbilityType>,
        use_add_stat_value: i32,
        add_data_value: i32,
        learn_skill_id: u16,
        use_skill_id: u16,
        status_effect_id: u16,
        status_effect_value: i32,
        cooldown_type_id: usize,
        cooldown_seconds: u32,
        effect: u16,
        effect_sound: u16,
    }
);
item_export!(
    /// A gem or jewel for a socket (`gem.toml`).
    GemExport, Gem {
        gem_stat_1: Option<AbilityType>,
        gem_stat_1_value: i32,
        gem_stat_2: Option<AbilityType>,
        gem_stat_2_value: i32,
        gem_effect: u16,
        gem_sprite_id: u32,
    }
);
item_export!(
    /// A crafting material or ammunition (`material.toml`).
    MaterialExport, Material {
        bullet_effect: u16,
    }
);
item_export!(
    /// A quest item (`quest.toml`).
    QuestExport, Quest {}
);
item_export!(
    /// A cart or castle gear part (`vehicle.toml`).
    VehicleExport, Vehicle {
        vehicle_type: VehicleType,
        version: u32,
        vehicle_part: VehiclePartIndex,
        move_speed: u32,
        max_fuel: u32,
        fuel_use_rate: u32,
        attack_range: i32,
        attack_power: i32,
        attack_speed: i32,
        base_motion_index: u32,
        base_avatar_motion_index: u32,
        skill_requirement_id: u16,
        skill_requirement_level: i32,
        ride_effect: u16,
        ride_sound: u16,
        dismount_effect: u16,
        dismount_sound: u16,
        dead_effect: u16,
        dead_sound: u16,
        stop_sound: u16,
        move_effect: u16,
        move_sound: u16,
        hit_effect: u16,
        hit_sound: u16,
        bullet_effect: u16,
        bullet_fire_point: u32,
        /// Up to 8 effect files, 0 for none; trailing zeros left out.
        dummy_effects: Vec<u16>,
        has_seat: bool,
    }
);

/// A refine grade (`grades.toml`): what each grade adds and the colour its glow has.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GradeExport {
    pub grade: u32,
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
    /// Red, green and blue, 0 to 255.
    #[serde(skip_serializing_if = "is_default")]
    pub glow_colour: [u8; 3],
}

/// Item types whose export has no fields of its own.
macro_rules! plain_item {
    ($export:ident, $data:ident) => {
        impl $export {
            pub fn from_data(item: &$data) -> Self {
                let mut out = Self::default();
                out.set_base(&item.item_data);
                out
            }

            pub fn to_data(&self, text: &mut Vec<String>) -> Result<$data, anyhow::Error> {
                Ok($data {
                    item_data: self.base(text)?,
                })
            }
        }
    };
}

plain_item!(FaceExport, FaceItemData);
plain_item!(BodyExport, BodyItemData);
plain_item!(HandsExport, HandsItemData);
plain_item!(JewelleryExport, JewelleryItemData);
plain_item!(QuestExport, QuestItemData);

impl HeadExport {
    pub fn from_data(item: &HeadItemData) -> Self {
        let mut out = Self {
            hair_type: item.hair_type,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<HeadItemData, anyhow::Error> {
        Ok(HeadItemData {
            item_data: self.base(text)?,
            hair_type: self.hair_type,
        })
    }
}

impl FeetExport {
    pub fn from_data(item: &FeetItemData) -> Self {
        let mut out = Self {
            move_speed: item.move_speed,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<FeetItemData, anyhow::Error> {
        Ok(FeetItemData {
            item_data: self.base(text)?,
            move_speed: self.move_speed,
        })
    }
}

impl BackExport {
    pub fn from_data(item: &BackItemData) -> Self {
        let mut out = Self {
            move_speed: item.move_speed,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<BackItemData, anyhow::Error> {
        Ok(BackItemData {
            item_data: self.base(text)?,
            move_speed: self.move_speed,
        })
    }
}

impl WeaponExport {
    pub fn from_data(item: &WeaponItemData) -> Self {
        let mut out = Self {
            attack_range: item.attack_range,
            attack_power: item.attack_power,
            attack_speed: item.attack_speed,
            motion_type: item.motion_type,
            is_magic_damage: item.is_magic_damage,
            bullet_effect: number(item.bullet_effect_id, EffectId::get),
            attack_effect: number(item.effect_id, EffectId::get),
            attack_start_sound: number(item.attack_start_sound_id, SoundId::get),
            attack_fire_sound: number(item.attack_fire_sound_id, SoundId::get),
            hit_sounds: item.attack_hit_sound_index,
            gem_position: item.gem_position,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<WeaponItemData, anyhow::Error> {
        Ok(WeaponItemData {
            item_data: self.base(text)?,
            attack_range: self.attack_range,
            attack_power: self.attack_power,
            attack_speed: self.attack_speed,
            motion_type: self.motion_type,
            is_magic_damage: self.is_magic_damage,
            bullet_effect_id: id(self.bullet_effect, EffectId::new),
            effect_id: id(self.attack_effect, EffectId::new),
            attack_start_sound_id: id(self.attack_start_sound, SoundId::new),
            attack_fire_sound_id: id(self.attack_fire_sound, SoundId::new),
            attack_hit_sound_index: self.hit_sounds,
            gem_position: self.gem_position,
        })
    }
}

impl SubWeaponExport {
    pub fn from_data(item: &SubWeaponItemData) -> Self {
        let mut out = Self {
            gem_position: item.gem_position,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<SubWeaponItemData, anyhow::Error> {
        Ok(SubWeaponItemData {
            item_data: self.base(text)?,
            gem_position: self.gem_position,
        })
    }
}

impl ConsumableExport {
    pub fn from_data(item: &ConsumableItemData) -> Self {
        let pair = |p: Option<(AbilityType, i32)>| p.map_or((None, 0), |(s, v)| (Some(s), v));
        let (use_need_stat, use_need_stat_value) = pair(item.ability_requirement);
        let (use_add_stat, use_add_stat_value) = pair(item.add_ability);
        let (status_effect_id, status_effect_value) = item
            .apply_status_effect
            .map_or((0, 0), |(id, value)| (id.get(), value));
        let mut out = Self {
            store_skin: item.store_skin,
            confile_index: item.confile_index,
            add_fuel: item.add_fuel,
            use_need_stat,
            use_need_stat_value,
            use_add_stat,
            use_add_stat_value,
            add_data_value: item.add_data_value,
            learn_skill_id: number(item.learn_skill_id, SkillId::get),
            use_skill_id: number(item.use_skill_id, SkillId::get),
            status_effect_id,
            status_effect_value,
            cooldown_type_id: item.cooldown_type_id,
            cooldown_seconds: item.cooldown_duration.as_secs() as u32,
            effect: number(item.effect_file_id, EffectFileId::get),
            effect_sound: number(item.effect_sound_id, SoundId::get),
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<ConsumableItemData, anyhow::Error> {
        Ok(ConsumableItemData {
            item_data: self.base(text)?,
            store_skin: self.store_skin,
            confile_index: self.confile_index,
            add_fuel: self.add_fuel,
            ability_requirement: stat(self.use_need_stat, self.use_need_stat_value),
            add_ability: stat(self.use_add_stat, self.use_add_stat_value),
            add_data_value: self.add_data_value,
            learn_skill_id: id(self.learn_skill_id, SkillId::new),
            use_skill_id: id(self.use_skill_id, SkillId::new),
            apply_status_effect: id(self.status_effect_id, StatusEffectId::new)
                .map(|id| (id, self.status_effect_value)),
            cooldown_type_id: self.cooldown_type_id,
            cooldown_duration: Duration::from_secs(self.cooldown_seconds as u64),
            effect_file_id: id(self.effect, EffectFileId::new),
            effect_sound_id: id(self.effect_sound, SoundId::new),
        })
    }
}

impl GemExport {
    pub fn from_data(item: &GemItemData) -> Self {
        let adds = |i: usize| {
            item.gem_add_ability
                .get(i)
                .copied()
                .map_or((None, 0), |(s, v)| (Some(s), v))
        };
        let ((gem_stat_1, gem_stat_1_value), (gem_stat_2, gem_stat_2_value)) = (adds(0), adds(1));
        let mut out = Self {
            gem_stat_1,
            gem_stat_1_value,
            gem_stat_2,
            gem_stat_2_value,
            gem_effect: number(item.gem_effect_id, EffectId::get),
            gem_sprite_id: item.gem_sprite_id,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<GemItemData, anyhow::Error> {
        Ok(GemItemData {
            item_data: self.base(text)?,
            gem_add_ability: [
                stat(self.gem_stat_1, self.gem_stat_1_value),
                stat(self.gem_stat_2, self.gem_stat_2_value),
            ]
            .into_iter()
            .flatten()
            .collect(),
            gem_effect_id: id(self.gem_effect, EffectId::new),
            gem_sprite_id: self.gem_sprite_id,
        })
    }
}

impl MaterialExport {
    pub fn from_data(item: &MaterialItemData) -> Self {
        let mut out = Self {
            bullet_effect: number(item.bullet_effect_id, EffectId::get),
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<MaterialItemData, anyhow::Error> {
        Ok(MaterialItemData {
            item_data: self.base(text)?,
            bullet_effect_id: id(self.bullet_effect, EffectId::new),
        })
    }
}

impl VehicleExport {
    pub fn from_data(item: &VehicleItemData) -> Self {
        let (skill_requirement_id, skill_requirement_level) = item
            .equip_skill_requirement
            .map_or((0, 0), |(id, level)| (id.get(), level));
        let mut dummy_effect_file_ids: Vec<u16> = item
            .dummy_effect_file_ids
            .iter()
            .map(|id| number(*id, EffectFileId::get))
            .collect();
        while dummy_effect_file_ids.last() == Some(&0) {
            dummy_effect_file_ids.pop();
        }
        let mut out = Self {
            vehicle_type: item.vehicle_type,
            version: item.version,
            vehicle_part: item.vehicle_part,
            move_speed: item.move_speed,
            max_fuel: item.max_fuel,
            fuel_use_rate: item.fuel_use_rate,
            attack_range: item.attack_range,
            attack_power: item.attack_power,
            attack_speed: item.attack_speed,
            base_motion_index: item.base_motion_index,
            base_avatar_motion_index: item.base_avatar_motion_index,
            skill_requirement_id,
            skill_requirement_level,
            ride_effect: number(item.ride_effect_file_id, EffectFileId::get),
            ride_sound: number(item.ride_sound_id, SoundId::get),
            dismount_effect: number(item.dismount_effect_file_id, EffectFileId::get),
            dismount_sound: number(item.dismount_sound_id, SoundId::get),
            dead_effect: number(item.dead_effect_file_id, EffectFileId::get),
            dead_sound: number(item.dead_sound_id, SoundId::get),
            stop_sound: number(item.stop_sound_id, SoundId::get),
            move_effect: number(item.move_effect_file_id, EffectFileId::get),
            move_sound: number(item.move_sound_id, SoundId::get),
            hit_effect: number(item.hit_effect_id, EffectId::get),
            hit_sound: number(item.hit_sound_id, SoundId::get),
            bullet_effect: number(item.bullet_effect_id, EffectId::get),
            bullet_fire_point: item.bullet_fire_point,
            dummy_effects: dummy_effect_file_ids,
            has_seat: item.has_seat,
            ..Default::default()
        };
        out.set_base(&item.item_data);
        out
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<VehicleItemData, anyhow::Error> {
        if self.dummy_effects.len() > 8 {
            bail!(
                "dummy_effects has {} entries, at most 8",
                self.dummy_effects.len()
            );
        }
        let mut dummy_effect_file_ids = [None; 8];
        for (slot, number) in dummy_effect_file_ids
            .iter_mut()
            .zip(&self.dummy_effects)
        {
            *slot = id(*number, EffectFileId::new);
        }
        Ok(VehicleItemData {
            item_data: self.base(text)?,
            vehicle_type: self.vehicle_type,
            version: self.version,
            vehicle_part: self.vehicle_part,
            move_speed: self.move_speed,
            max_fuel: self.max_fuel,
            fuel_use_rate: self.fuel_use_rate,
            attack_range: self.attack_range,
            attack_power: self.attack_power,
            attack_speed: self.attack_speed,
            base_motion_index: self.base_motion_index,
            base_avatar_motion_index: self.base_avatar_motion_index,
            equip_skill_requirement: id(self.skill_requirement_id, SkillId::new)
                .map(|id| (id, self.skill_requirement_level)),
            ride_effect_file_id: id(self.ride_effect, EffectFileId::new),
            ride_sound_id: id(self.ride_sound, SoundId::new),
            dismount_effect_file_id: id(self.dismount_effect, EffectFileId::new),
            dismount_sound_id: id(self.dismount_sound, SoundId::new),
            dead_effect_file_id: id(self.dead_effect, EffectFileId::new),
            dead_sound_id: id(self.dead_sound, SoundId::new),
            stop_sound_id: id(self.stop_sound, SoundId::new),
            move_effect_file_id: id(self.move_effect, EffectFileId::new),
            move_sound_id: id(self.move_sound, SoundId::new),
            hit_effect_id: id(self.hit_effect, EffectId::new),
            hit_sound_id: id(self.hit_sound, SoundId::new),
            bullet_effect_id: id(self.bullet_effect, EffectId::new),
            bullet_fire_point: self.bullet_fire_point,
            dummy_effect_file_ids,
            has_seat: self.has_seat,
        })
    }
}

impl GradeExport {
    pub fn from_data(grade: u32, data: &ItemGradeData) -> Self {
        let channel = |c: f32| (c * 255.0).round() as u8;
        let (r, g, b) = data.glow_colour;
        Self {
            grade,
            attack: data.attack,
            hit: data.hit,
            defence: data.defence,
            resistance: data.resistance,
            avoid: data.avoid,
            glow_colour: [channel(r), channel(g), channel(b)],
        }
    }

    pub fn to_data(&self) -> ItemGradeData {
        // Divided as the iROSE loader does, so the colours are the same to the last bit.
        let [r, g, b] = self.glow_colour;
        ItemGradeData {
            attack: self.attack,
            hit: self.hit,
            defence: self.defence,
            resistance: self.resistance,
            avoid: self.avoid,
            glow_colour: (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0),
        }
    }
}

/// One `<type>.toml` file: its items, in id order.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemFile<T> {
    #[serde(default)]
    pub item: Vec<T>,
}

/// `grades.toml`.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GradeFile {
    #[serde(default)]
    pub grade: Vec<GradeExport>,
}

/// Every item and grade, as exported.
#[derive(Debug, Default)]
pub struct ItemsExport {
    pub face: Vec<FaceExport>,
    pub head: Vec<HeadExport>,
    pub body: Vec<BodyExport>,
    pub hands: Vec<HandsExport>,
    pub feet: Vec<FeetExport>,
    pub back: Vec<BackExport>,
    pub jewellery: Vec<JewelleryExport>,
    pub weapon: Vec<WeaponExport>,
    pub subweapon: Vec<SubWeaponExport>,
    pub consumable: Vec<ConsumableExport>,
    pub gem: Vec<GemExport>,
    pub material: Vec<MaterialExport>,
    pub quest: Vec<QuestExport>,
    pub vehicle: Vec<VehicleExport>,
    pub grades: Vec<GradeExport>,
}

/// The file name (and Godot folder, `godot/data/items/<name>/`) of each item type.
pub const ITEM_TYPE_FILES: [(ItemType, &str); 14] = [
    (ItemType::Face, "face"),
    (ItemType::Head, "head"),
    (ItemType::Body, "body"),
    (ItemType::Hands, "hands"),
    (ItemType::Feet, "feet"),
    (ItemType::Back, "back"),
    (ItemType::Jewellery, "jewellery"),
    (ItemType::Weapon, "weapon"),
    (ItemType::SubWeapon, "subweapon"),
    (ItemType::Consumable, "consumable"),
    (ItemType::Gem, "gem"),
    (ItemType::Material, "material"),
    (ItemType::Quest, "quest"),
    (ItemType::Vehicle, "vehicle"),
];

/// The file with the refine grades (and the Godot folder `godot/data/items/grades/`).
pub const GRADES_FILE: &str = "grades";

pub fn item_type_name(item_type: ItemType) -> &'static str {
    ITEM_TYPE_FILES
        .iter()
        .find(|(t, _)| *t == item_type)
        .map(|(_, n)| *n)
        .unwrap()
}

fn sorted<T>(mut items: Vec<T>, id: fn(&T) -> u32) -> Vec<T> {
    items.sort_by_key(id);
    items
}

/// How the files are written: TOML in the repository (people read and diff it), JSON in
/// the server's upload (the module reads it about 20 times faster).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Toml,
    Json,
}

impl Format {
    pub fn extension(self) -> &'static str {
        match self {
            Format::Toml => "toml",
            Format::Json => "json",
        }
    }

    pub(crate) fn parse<T: DeserializeOwned>(self, name: &str, text: &str) -> Result<T, anyhow::Error> {
        match self {
            Format::Toml => toml::from_str(text).with_context(|| format!("{name}.toml")),
            Format::Json => serde_json::from_str(text).with_context(|| format!("{name}.json")),
        }
    }

    pub(crate) fn write<T: Serialize>(self, name: &str, value: &T) -> Result<String, anyhow::Error> {
        match self {
            Format::Toml => toml::to_string(value).with_context(|| format!("writing {name}.toml")),
            Format::Json => {
                serde_json::to_string(value).with_context(|| format!("writing {name}.json"))
            }
        }
    }
}

/// Builds the item table of one type: index = item number, refusing a number used twice.
fn table<E, D>(
    name: &str,
    exports: &[E],
    id: fn(&E) -> u32,
    mut to_data: impl FnMut(&E) -> Result<D, anyhow::Error>,
) -> Result<Vec<Option<D>>, anyhow::Error> {
    let mut table: Vec<Option<D>> = Vec::new();
    for export in exports {
        let number = id(export) as usize;
        if table.len() <= number {
            table.resize_with(number + 1, || None);
        }
        if table[number].is_some() {
            bail!("{name}.toml: item {number} is there twice");
        }
        table[number] =
            Some(to_data(export).with_context(|| format!("{name}.toml: item {number}"))?);
    }
    Ok(table)
}

macro_rules! each_type {
    ($m:ident) => {
        $m!(face, FaceExport, get_face_item);
        $m!(head, HeadExport, get_head_item);
        $m!(body, BodyExport, get_body_item);
        $m!(hands, HandsExport, get_hands_item);
        $m!(feet, FeetExport, get_feet_item);
        $m!(back, BackExport, get_back_item);
        $m!(jewellery, JewelleryExport, get_jewellery_item);
        $m!(weapon, WeaponExport, get_weapon_item);
        $m!(subweapon, SubWeaponExport, get_sub_weapon_item);
        $m!(consumable, ConsumableExport, get_consumable_item);
        $m!(gem, GemExport, get_gem_item);
        $m!(material, MaterialExport, get_material_item);
        $m!(quest, QuestExport, get_quest_item);
        $m!(vehicle, VehicleExport, get_vehicle_item);
    };
}

impl ItemsExport {
    /// Every item of a database (the converter's starting point and the same-data check).
    pub fn from_database(items: &ItemDatabase) -> Self {
        let mut out = Self::default();
        macro_rules! from {
            ($field:ident, $export:ident, $get:ident) => {
                out.$field = items
                    .iter_items($export::ITEM_TYPE)
                    .filter_map(|r| items.$get(r.item_number))
                    .map($export::from_data)
                    .collect();
            };
        }
        each_type!(from);
        out.grades = items
            .item_grades()
            .iter()
            .enumerate()
            .map(|(i, g)| GradeExport::from_data(i as u32, g))
            .collect();
        out
    }

    /// The item database, with names and descriptions from the export (`strings` is only
    /// kept for the other databases that share it).
    pub fn to_database(&self, strings: Arc<StringDatabase>) -> Result<ItemDatabase, anyhow::Error> {
        let mut text = Vec::new();
        let face = table("face", &self.face, |e| e.id, |e| e.to_data(&mut text))?;
        let head = table("head", &self.head, |e| e.id, |e| e.to_data(&mut text))?;
        let body = table("body", &self.body, |e| e.id, |e| e.to_data(&mut text))?;
        let hands = table("hands", &self.hands, |e| e.id, |e| e.to_data(&mut text))?;
        let feet = table("feet", &self.feet, |e| e.id, |e| e.to_data(&mut text))?;
        let back = table("back", &self.back, |e| e.id, |e| e.to_data(&mut text))?;
        let jewellery = table(
            "jewellery",
            &self.jewellery,
            |e| e.id,
            |e| e.to_data(&mut text),
        )?;
        let weapon = table("weapon", &self.weapon, |e| e.id, |e| e.to_data(&mut text))?;
        let subweapon = table(
            "subweapon",
            &self.subweapon,
            |e| e.id,
            |e| e.to_data(&mut text),
        )?;
        let consumable = table(
            "consumable",
            &self.consumable,
            |e| e.id,
            |e| e.to_data(&mut text),
        )?;
        let gem = table("gem", &self.gem, |e| e.id, |e| e.to_data(&mut text))?;
        let material = table(
            "material",
            &self.material,
            |e| e.id,
            |e| e.to_data(&mut text),
        )?;
        let quest = table("quest", &self.quest, |e| e.id, |e| e.to_data(&mut text))?;
        let vehicle = table("vehicle", &self.vehicle, |e| e.id, |e| e.to_data(&mut text))?;
        let mut grades: Vec<Option<ItemGradeData>> = Vec::new();
        for grade in &self.grades {
            let at = grade.grade as usize;
            if grades.len() <= at {
                grades.resize_with(at + 1, || None);
            }
            if grades[at].is_some() {
                bail!("grades.toml: grade {at} is there twice");
            }
            grades[at] = Some(grade.to_data());
        }
        let item_grades = grades
            .into_iter()
            .enumerate()
            .map(|(i, g)| g.with_context(|| format!("grades.toml: grade {i} is missing")))
            .collect::<Result<_, _>>()?;
        Ok(ItemDatabase::new(
            strings,
            face,
            head,
            body,
            hands,
            feet,
            back,
            jewellery,
            weapon,
            subweapon,
            consumable,
            gem,
            material,
            quest,
            vehicle,
            item_grades,
        )
        .with_text(text))
    }

    /// The export's files: (file name without `.toml`, contents).
    pub fn to_files(&self) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        self.to_files_as(Format::Toml)
    }

    /// The export's files in `format`.
    pub fn to_files_as(
        &self,
        format: Format,
    ) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut files = Vec::new();
        macro_rules! file {
            ($field:ident, $export:ident, $get:ident) => {
                let name = item_type_name($export::ITEM_TYPE);
                let items = sorted(self.$field.clone(), |e| e.id);
                files.push((name, format.write(name, &ItemFile { item: items })?));
            };
        }
        each_type!(file);
        let grade = sorted(self.grades.clone(), |g| g.grade);
        files.push((
            GRADES_FILE,
            format.write(GRADES_FILE, &GradeFile { grade })?,
        ));
        Ok(files)
    }

    /// Reads the export's files through `read` (file name without `.toml` -> its text, or
    /// None when it isn't there). None when there is no export at all; an export with some
    /// files missing is an error.
    pub fn from_files(
        read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        Self::from_files_as(Format::Toml, read)
    }

    /// Like `from_files`, for files in `format`.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let mut out = Self::default();
        let (mut found, mut missing) = (0, Vec::new());
        macro_rules! read {
            ($field:ident, $export:ident, $get:ident) => {
                let name = item_type_name($export::ITEM_TYPE);
                match read(name)? {
                    Some(text) => {
                        found += 1;
                        out.$field = format.parse::<ItemFile<$export>>(name, &text)?.item;
                    }
                    None => missing.push(name),
                }
            };
        }
        each_type!(read);
        match read(GRADES_FILE)? {
            Some(text) => {
                found += 1;
                out.grades = format.parse::<GradeFile>(GRADES_FILE, &text)?.grade;
            }
            None => missing.push(GRADES_FILE),
        }
        match (found, missing.is_empty()) {
            (0, _) => Ok(None),
            (_, true) => Ok(Some(out)),
            _ => bail!(
                "the item export has no {}",
                missing
                    .iter()
                    .map(|m| format!("{m}.{}", format.extension()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 0;
        macro_rules! count {
            ($field:ident, $export:ident, $get:ident) => {
                count += self.$field.len();
            };
        }
        each_type!(count);
        count
    }
}

/// Every difference between two item databases, one line per item field that differs
/// (compared by their debug output, so every field counts), plus missing or extra items.
pub fn compare_item_databases(a: &ItemDatabase, b: &ItemDatabase) -> Vec<String> {
    let mut out = Vec::new();
    for (item_type, name) in ITEM_TYPE_FILES {
        let mut ids: Vec<usize> = a
            .iter_items(item_type)
            .chain(b.iter_items(item_type))
            .map(|r| r.item_number)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        for number in ids {
            let item = ItemReference::new(item_type, number);
            let (x, y) = (a.get_item(item), b.get_item(item));
            match (&x, &y) {
                (Some(_), None) => out.push(format!("{name} {number}: missing")),
                (None, Some(_)) => out.push(format!("{name} {number}: extra")),
                _ => {
                    let (x, y) = (crate::same_art_debug(format!("{x:#?}")), crate::same_art_debug(format!("{y:#?}")));
                    if x != y {
                        for (l, r) in x.lines().zip(y.lines()).filter(|(l, r)| l != r) {
                            out.push(format!("{name} {number}: {} -> {}", l.trim(), r.trim()));
                        }
                        if x.lines().count() != y.lines().count() {
                            out.push(format!("{name} {number}: the fields differ"));
                        }
                    }
                }
            }
        }
    }
    let (x, y) = (
        format!("{:?}", a.item_grades()),
        format!("{:?}", b.item_grades()),
    );
    if x != y {
        out.push(format!("grades: {x} -> {y}"));
    }
    out
}
