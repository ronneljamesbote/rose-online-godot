//! The game data as Godot resources: one `.tres` per thing under `res://data/`, edited in the
//! Inspector. So far items (`res://data/items/<type>/<id>-<name>.tres`, one class per item
//! type, `RoseItemWeapon` and so on, plus `RoseItemGrade` for the refine grades), monsters
//! (`RoseMonster`), town NPCs (`RoseNpc`), drop tables (`RoseDropTable`) and shop tabs
//! (`RoseShopTab`).
//!
//! The game itself never reads these files. `--export-data` writes them to
//! `overrides/data/` as TOML (the layout is in `crates/rose-data-export`), and the server and
//! the client load that. `--convert-data` writes the resources once from the iROSE tables.
//!
//! A resource's properties are the export's keys, one to one, so both directions go through
//! JSON: the export struct's serde form on one side, the resource's own properties (as
//! `ClassDb` lists them) on the other. A property the struct doesn't know, or a key the
//! class doesn't have, is an error, so the two can't drift apart unnoticed. A link to another
//! resource is, in the export, the item it names (`weapon/3`) or the other thing's id.

use std::path::Path;

use enum_map::Enum;
use godot::{
    classes::{
        ClassDb, DirAccess, IResource, ProjectSettings, Resource, ResourceLoader, ResourceSaver,
        ResourceUid,
    },
    global::Error,
    prelude::*,
    register::info::{PropertyHint, PropertyInfo, PropertyUsageFlags},
};
use rose_data::{AbilityType, ItemClass};
use rose_data_export::items::{
    BackExport, BodyExport, ConsumableExport, FaceExport, FeetExport, GemExport, GradeExport,
    HandsExport, HeadExport, JewelleryExport, MaterialExport, QuestExport, SubWeaponExport,
    VehicleExport, WeaponExport, GRADES_FILE,
};
use rose_data_export::npcs::{DropTableExport, MonsterExport, NpcExport, ShopTabExport};
use rose_data_export::{ItemsExport, NpcsExport};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

/// Where the item resources live.
pub const ITEMS_DIR: &str = "res://data/items/";
/// Where the monster, NPC, drop table and shop tab resources live.
pub const MONSTERS_DIR: &str = "res://data/monsters/";
pub const NPCS_DIR: &str = "res://data/npcs/";
pub const DROP_TABLES_DIR: &str = "res://data/drop_tables/";
pub const SHOP_TABS_DIR: &str = "res://data/shop_tabs/";

/// Where to find a linked resource by what the export holds: `items/weapon/3`,
/// `drop_table/147`, `shop_tab/12` -> its `res://` path.
type Links = std::collections::HashMap<String, String>;

/// What an empty stat field holds (no stat).
const NO_STAT: &str = "None";

/// The Inspector's dropdowns for the text fields that name a game enum.
fn enum_hint(property: &mut PropertyInfo) {
    if property.variant_type != VariantType::STRING {
        return;
    }
    let name = property.property_name.to_string();
    let options: Vec<String> = if name == "item_class" {
        (0..ItemClass::LENGTH)
            .map(|i| format!("{:?}", ItemClass::from_usize(i)))
            .collect()
    } else if name.contains("stat") {
        std::iter::once(NO_STAT.to_string())
            .chain((0..AbilityType::LENGTH).map(|i| format!("{:?}", AbilityType::from_usize(i))))
            .collect()
    } else if name == "vehicle_type" {
        vec!["Cart".into(), "CastleGear".into()]
    } else if name == "vehicle_part" {
        vec!["Body".into(), "Engine".into(), "Leg".into(), "Arms".into()]
    } else {
        return;
    };
    property.hint_info.hint = PropertyHint::ENUM;
    property.hint_info.hint_string = GString::from(options.join(",").as_str());
}

/// One class per item type: the fields every item has, then the type's own (in a group
/// named after the type).
macro_rules! item_resource {
    ($(#[$($meta:tt)*])* $name:ident { $($(#[$($field_meta:tt)*])* $field:ident: $ty:ty,)* }) => {
        $(#[$($meta)*])*
        #[derive(GodotClass)]
        #[class(tool, base=Resource, init)]
        pub struct $name {
            /// The item number, unique among items of this type. Saved characters, shops,
            /// drop lists and quests name the item by it, so don't change it once players
            /// have the item. A new item takes any free number.
            #[export_group(name = "Item")]
            #[export]
            id: i64,
            /// The name players see.
            #[export]
            name: GString,
            /// The tooltip text under the name.
            #[export(multiline)]
            description: GString,
            /// What kind of item this is (sword, potion, metal ...). It decides the weapon
            /// motions, which slot it fits, and what crafting and the shops treat it as.
            #[export]
            #[init(val = GString::from("Unknown"))]
            item_class: GString,

            /// The price in Zuly before the shop's rate and the town's prices are applied.
            #[export_group(name = "Price and weight")]
            #[export]
            base_price: i64,
            /// How much the town's price swings move this item's price (iROSE's price rate).
            #[export]
            price_rate: i64,
            /// Weight in the bag.
            #[export]
            weight: i64,
            /// Quality: the level the item counts as for prices, drops and refining.
            #[export]
            quality: i64,
            /// Flags: 2 = can't be dropped, traded or sold to players.
            #[export]
            trade_restriction: i64,

            /// The icon: its number in the item icon sheets (0 = none).
            #[export_group(name = "Look and sound")]
            #[export]
            icon_index: i64,
            /// The model shown when the item lies on the ground.
            #[export]
            field_model_index: i64,
            /// The sound played when it is equipped (0 = none).
            #[export]
            equip_sound_id: i64,

            /// The crafting skill that makes it (its skill type number, 0 = can't be crafted).
            #[export_group(name = "Crafting")]
            #[export]
            craft_skill_type: i64,
            /// The level of that skill needed.
            #[export]
            craft_skill_level: i64,
            /// The recipe: its row in the crafting recipe list.
            #[export]
            craft_material: i64,
            /// How hard crafting it is.
            #[export]
            craft_difficulty: i64,

            /// The job class that may equip it (its number, 0 = any).
            #[export_group(name = "Requirements")]
            #[export]
            equip_job_class: i64,
            /// A union whose members may equip it (0 = none).
            #[export]
            equip_union_1: i64,
            /// A second union whose members may equip it (0 = none).
            #[export]
            equip_union_2: i64,
            /// A stat needed to equip it.
            #[export]
            #[init(val = GString::from(NO_STAT))]
            equip_stat_1: GString,
            /// How much of that stat is needed.
            #[export]
            equip_stat_1_value: i64,
            /// A second stat needed to equip it.
            #[export]
            #[init(val = GString::from(NO_STAT))]
            equip_stat_2: GString,
            /// How much of the second stat is needed.
            #[export]
            equip_stat_2_value: i64,

            /// The union whose members get the first added stat (0 = everyone).
            #[export_group(name = "Added stats")]
            #[export]
            add_stat_union_1: i64,
            /// The union whose members get the second added stat (0 = everyone).
            #[export]
            add_stat_union_2: i64,
            /// A stat the item adds while worn.
            #[export]
            #[init(val = GString::from(NO_STAT))]
            add_stat_1: GString,
            /// How much it adds (can be negative).
            #[export]
            add_stat_1_value: i64,
            /// A second stat the item adds while worn.
            #[export]
            #[init(val = GString::from(NO_STAT))]
            add_stat_2: GString,
            /// How much the second one adds.
            #[export]
            add_stat_2_value: i64,

            /// The durability a new item starts with (0 to 255).
            #[export_group(name = "Defence and durability")]
            #[export]
            durability: i64,
            /// iROSE's rare type: whether the item can drop with a socket.
            #[export]
            rare_type: i64,
            /// Defence while worn.
            #[export]
            defence: i64,
            /// Magic resistance while worn.
            #[export]
            resistance: i64,

            $(
                $(#[$($field_meta)*])*
                #[export]
                $field: $ty,
            )*
            base: Base<Resource>,
        }

        #[godot_api]
        impl IResource for $name {
            fn on_validate_property(&self, property: &mut PropertyInfo) {
                enum_hint(property);
            }
        }
    };
}

item_resource!(
    /// A face item: a mask, glasses or the like. Edit it here, then run `--export-data` so
    /// the server and the game get the change (see `godot/data/README.md`).
    RoseItemFace {}
);
item_resource!(
    /// A helmet or hat. Edit it here, then run `--export-data` (see `godot/data/README.md`).
    RoseItemHead {
        /// How the hair is drawn under it (0 = all of it, higher numbers hide more).
        #[export_group(name = "Head")]
        hair_type: i64,
    }
);
item_resource!(
    /// Armour or clothes. Edit it here, then run `--export-data` (see `godot/data/README.md`).
    RoseItemBody {}
);
item_resource!(
    /// Gloves or gauntlets. Edit it here, then run `--export-data` (see `godot/data/README.md`).
    RoseItemHands {}
);
item_resource!(
    /// Boots or shoes. Item 0 holds the walking speed without shoes. Edit it here, then run
    /// `--export-data` (see `godot/data/README.md`).
    RoseItemFeet {
        /// The run speed they give (iROSE's move speed; item 0's is the speed barefoot).
        #[export_group(name = "Feet")]
        move_speed: i64,
    }
);
item_resource!(
    /// Back armour, a bag, wings or an ammunition box. Edit it here, then run `--export-data`
    /// (see `godot/data/README.md`).
    RoseItemBack {
        /// The move speed it adds.
        #[export_group(name = "Back")]
        move_speed: i64,
    }
);
item_resource!(
    /// A ring, necklace or earring (the class says which). Edit it here, then run
    /// `--export-data` (see `godot/data/README.md`).
    RoseItemJewellery {}
);
item_resource!(
    /// A weapon. Item 0 is fighting unarmed (its range, sounds and effects). Edit it here,
    /// then run `--export-data` (see `godot/data/README.md`).
    RoseItemWeapon {
        /// How far it reaches, in centimetres (bows and guns: how far they shoot).
        #[export_group(name = "Weapon")]
        attack_range: i64,
        /// Attack power.
        attack_power: i64,
        /// Attack speed (higher is faster).
        attack_speed: i64,
        /// Which set of attack motions the character uses.
        motion_type: i64,
        /// Hits with magic damage (staffs and wands) instead of physical.
        is_magic_damage: bool,
        /// The projectile it shoots (an effect number, 0 = none).
        bullet_effect_id: i64,
        /// The effect drawn on the weapon (0 = none).
        effect_id: i64,
        /// The sound when the swing starts (0 = none).
        attack_start_sound_id: i64,
        /// The sound when it fires (0 = none).
        attack_fire_sound_id: i64,
        /// Which hit sound plays on the target.
        attack_hit_sound_index: i64,
        /// Where on the model a socketed gem shows.
        gem_position: i64,
    }
);
item_resource!(
    /// A shield or support tool. Edit it here, then run `--export-data` (see
    /// `godot/data/README.md`).
    RoseItemSubWeapon {
        /// Where on the model a socketed gem shows.
        #[export_group(name = "Sub weapon")]
        gem_position: i64,
    }
);
item_resource!(
    /// A potion, food, scroll, skill book, repair tool ... Several of the fields below came
    /// from one iROSE column (its "data value"); the game reads the one the item's class uses.
    /// Edit it here, then run `--export-data` (see `godot/data/README.md`).
    RoseItemConsumable {
        /// The look of the personal shop it opens (shop skins).
        #[export_group(name = "Use")]
        store_skin: i64,
        /// The script or effect file row it runs when used.
        confile_index: i64,
        /// Fuel it adds to a cart (engine fuel).
        add_fuel: i64,
        /// A stat needed to use it.
        #[init(val = GString::from(NO_STAT))]
        use_need_stat: GString,
        /// How much of that stat is needed.
        use_need_stat_value: i64,
        /// The stat it raises when used (health for a potion ...).
        #[init(val = GString::from(NO_STAT))]
        use_add_stat: GString,
        /// How much it raises it.
        use_add_stat_value: i64,
        /// iROSE's data value on its own: a repair hammer with a value other than 0 never
        /// lowers durability.
        add_data_value: i64,
        /// The skill a skill book teaches (0 = none).
        learn_skill_id: i64,
        /// The skill a scroll casts (0 = none).
        use_skill_id: i64,
        /// The status effect it puts on (0 = none).
        status_effect_id: i64,
        /// The strength of that status effect.
        status_effect_value: i64,
        /// Items with the same cooldown type share one cooldown.
        cooldown_type_id: i64,
        /// Seconds before an item of the same cooldown type can be used again.
        cooldown_seconds: i64,
        /// The effect played when it is used (0 = none).
        effect_file_id: i64,
        /// The sound played when it is used (0 = none).
        effect_sound_id: i64,
    }
);
item_resource!(
    /// A gem or jewel for a socket. Edit it here, then run `--export-data` (see
    /// `godot/data/README.md`).
    RoseItemGem {
        /// A stat the gem adds in a socket.
        #[export_group(name = "Gem")]
        #[init(val = GString::from(NO_STAT))]
        gem_stat_1: GString,
        /// How much it adds.
        gem_stat_1_value: i64,
        /// A second stat the gem adds.
        #[init(val = GString::from(NO_STAT))]
        gem_stat_2: GString,
        /// How much the second one adds.
        gem_stat_2_value: i64,
        /// The glow effect on the socketed item (0 = none).
        gem_effect_id: i64,
        /// The gem's picture in the socket.
        gem_sprite_id: i64,
    }
);
item_resource!(
    /// A crafting material or ammunition. Edit it here, then run `--export-data` (see
    /// `godot/data/README.md`).
    RoseItemMaterial {
        /// Arrows, bullets and shells: the projectile they fly as (0 = none).
        #[export_group(name = "Material")]
        bullet_effect_id: i64,
    }
);
item_resource!(
    /// A quest item. Edit it here, then run `--export-data` (see `godot/data/README.md`).
    RoseItemQuest {}
);
item_resource!(
    /// A cart or castle gear part. Edit it here, then run `--export-data` (see
    /// `godot/data/README.md`).
    RoseItemVehicle {
        /// A cart or a castle gear.
        #[export_group(name = "Vehicle")]
        #[init(val = GString::from("Cart"))]
        vehicle_type: GString,
        /// Which part of the vehicle it is.
        #[init(val = GString::from("Body"))]
        vehicle_part: GString,
        /// Which body the part fits (parts only fit a body of the same version).
        version: i64,
        /// Move speed (the body's).
        move_speed: i64,
        /// Fuel the engine holds.
        max_fuel: i64,
        /// How fast it uses fuel.
        fuel_use_rate: i64,
        /// Weapon arms: reach in centimetres.
        attack_range: i64,
        /// Weapon arms: attack power.
        attack_power: i64,
        /// Weapon arms: attack speed.
        attack_speed: i64,
        /// The vehicle's own motions (a row in the motion list).
        base_motion_index: i64,
        /// The driver's motions while riding.
        base_avatar_motion_index: i64,
        /// A skill needed to use the part (0 = none).
        skill_requirement_id: i64,
        /// The level of that skill needed.
        skill_requirement_level: i64,
        /// Effect when getting on (0 = none).
        ride_effect_file_id: i64,
        /// Sound when getting on (0 = none).
        ride_sound_id: i64,
        /// Effect when getting off.
        dismount_effect_file_id: i64,
        /// Sound when getting off.
        dismount_sound_id: i64,
        /// Effect when it is destroyed.
        dead_effect_file_id: i64,
        /// Sound when it is destroyed.
        dead_sound_id: i64,
        /// Sound when it stops.
        stop_sound_id: i64,
        /// Effect while moving.
        move_effect_file_id: i64,
        /// Sound while moving.
        move_sound_id: i64,
        /// Effect on the target when its weapon hits.
        hit_effect_id: i64,
        /// Sound on the target when its weapon hits.
        hit_sound_id: i64,
        /// The projectile its weapon fires.
        bullet_effect_id: i64,
        /// The model point the projectile leaves from.
        bullet_fire_point: i64,
        /// Up to 8 effects fixed to the model's points (0 = none).
        dummy_effect_file_ids: PackedInt32Array,
        /// It has a second seat: the driver can take a passenger.
        has_seat: bool,
    }
);

/// A refine grade: what an item refined to this grade gets added, and its glow. Edit it
/// here, then run `--export-data` (see `godot/data/README.md`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseItemGrade {
    /// The grade (0 = not refined, up to 9).
    #[export]
    grade: i64,
    /// Attack a weapon of this grade gets added (iROSE: percent of its attack power).
    #[export]
    attack: i64,
    /// Hit rate added.
    #[export]
    hit: i64,
    /// Defence added (armour; percent of its defence).
    #[export]
    defence: i64,
    /// Magic resistance added.
    #[export]
    resistance: i64,
    /// Avoid added.
    #[export]
    avoid: i64,
    /// The colour of the glow on the refined item (only red, green and blue count).
    #[export]
    glow_colour: Color,
    base: Base<Resource>,
}

/// A monster or a town NPC: the fields both have, then the kind's own (in a group named after
/// it). Monsters and NPCs share one numbering.
macro_rules! character_resource {
    ($(#[$($meta:tt)*])* $name:ident { $($(#[$($field_meta:tt)*])* $field:ident: $ty:ty,)* }) => {
        $(#[$($meta)*])*
        #[derive(GodotClass)]
        #[class(tool, base=Resource, init)]
        pub struct $name {
            /// The number, shared by monsters and NPCs (iROSE's LIST_NPC.STB row): no monster
            /// and NPC may have the same one. Spawn points, zone NPC markers, quests and skills
            /// name it by this, and it picks the model and motions (LIST_NPC.CHR) until models
            /// are made in Godot. Don't change it once it is in use.
            #[export_group(name = "Character")]
            #[export]
            id: i64,
            /// The name players see.
            #[export]
            name: GString,
            /// The level: what XP, drops and the level difference to players go by.
            #[export]
            level: i64,
            /// Health per level: the real maximum is level × this (the wiki shows both).
            #[export]
            health_points: i64,
            /// Experience for the kill, before the level difference and party share.
            #[export]
            reward_xp: i64,
            /// Size in percent of the model's (100 = as modelled).
            #[export(range = (1.0, 1000.0, 1.0, or_greater, suffix = "%"))]
            #[init(val = 100)]
            scale: i64,
            /// How tall it is, in centimetres (where the name and effects sit).
            #[export]
            height: i64,

            /// Attack power.
            #[export_group(name = "Combat")]
            #[export]
            attack: i64,
            /// Hit rate.
            #[export]
            hit: i64,
            /// Defence.
            #[export]
            defence: i64,
            /// Magic resistance.
            #[export]
            resistance: i64,
            /// Avoid (dodge) rate.
            #[export]
            avoid: i64,
            /// Attack speed (higher is faster; 100 = one attack motion per second).
            #[export]
            attack_speed: i64,
            /// How far it reaches, in centimetres.
            #[export]
            attack_range: i64,
            /// Hits with magic damage instead of physical.
            #[export]
            is_magic_damage: bool,
            /// Can't be clicked or attacked.
            #[export]
            is_untargetable: bool,
            /// Walking speed, centimetres per second.
            #[export]
            walk_speed: i64,
            /// Running speed (chasing, fleeing), centimetres per second.
            #[export]
            run_speed: i64,

            /// Its AI script (a row of FILE_AI.STB, 0 = none): how it wanders, picks fights
            /// and uses skills. Becomes a link when AI is made in Godot.
            #[export_group(name = "Behaviour")]
            #[export]
            ai_file_index: i64,
            /// The quest trigger run when it dies (a QSD trigger name, empty = none).
            #[export]
            death_quest_trigger_name: GString,
            /// iROSE's quest type number for it.
            #[export]
            npc_quest_type: i64,
            /// iROSE's NPC type number (groups the same kind of monster, 0 = none).
            #[export]
            npc_type_index: i64,

            /// The weapon model part in its right hand (0 = none).
            #[export_group(name = "Look and sound")]
            #[export]
            right_hand_part_index: i64,
            /// The model part in its left hand (0 = none).
            #[export]
            left_hand_part_index: i64,
            /// A glow drawn on the model (black = none).
            #[export]
            glow_colour: Color,
            /// What its body sounds like when hit (the hit sound set).
            #[export]
            hit_sound_material_type: i64,
            /// The sound it makes now and then (0 = none).
            #[export]
            normal_effect_sound_id: i64,
            /// The sound of its attack (0 = none).
            #[export]
            attack_sound_id: i64,
            /// The sound when it is hit (0 = none).
            #[export]
            hitted_sound_id: i64,
            /// The effect on its target when its hand hits (0 = none).
            #[export]
            hand_hit_effect_id: i64,
            /// The effect when it dies (0 = none).
            #[export]
            die_effect_file_id: i64,
            /// The sound when it dies (0 = none).
            #[export]
            die_sound_id: i64,
            /// The effect when it appears (0 = none).
            #[export]
            create_effect_index: i64,
            /// The sound when it appears (0 = none).
            #[export]
            create_sound_id: i64,

            $(
                $(#[$($field_meta)*])*
                #[export]
                $field: $ty,
            )*
            base: Base<Resource>,
        }
    };
}

character_resource!(
    /// A monster (also the ones only skills or quests summon). Edit it here, then run
    /// `--export-data` (see `godot/data/README.md`). Spawn points take it by drag and drop.
    RoseMonster {
        /// What it drops (empty = only the zone's drops). How a drop picks from it is on the
        /// wiki page "Drops".
        #[export_group(name = "Drops")]
        drop_table: Option<Gd<RoseDropTable>>,
        /// Chance in percent that a drop is money instead of an item.
        drop_money_rate: i64,
        /// Chance in percent that an item drop comes from its own table instead of the zone's;
        /// also raises the chance of better items.
        drop_item_rate: i64,
        /// Summoned by a skill: the summon points it takes from its summoner.
        #[export_group(name = "Summon")]
        summon_point_requirement: i64,
        /// Summoned by a skill: iROSE's summon type number.
        summon_monster_type: i64,
    }
);

character_resource!(
    /// A town NPC: one a zone places with an NPC marker. Edit it here, then run
    /// `--export-data` (see `godot/data/README.md`). NPC markers take it by drag and drop.
    RoseNpc {
        /// Its shop's tabs, left to right (empty = no shop, or fewer tabs).
        #[export_group(name = "Shop")]
        shop_tab_1: Option<Gd<RoseShopTab>>,
        /// The second tab.
        shop_tab_2: Option<Gd<RoseShopTab>>,
        /// The third tab.
        shop_tab_3: Option<Gd<RoseShopTab>>,
        /// The fourth tab.
        shop_tab_4: Option<Gd<RoseShopTab>>,
        /// Members of this union buy at the union price (0 = no union).
        store_union: i64,
        /// The icon marking it on the minimap.
        #[export_group(name = "Icons")]
        minimap_icon_index: i64,
        /// The portrait in the dialog window.
        face_icon_index: i64,
    }
);

/// What a monster (or a zone) drops. A drop picks one of the slots at random (low slots
/// more often when the monster is far above the player); a slot is an item, a group number
/// from 1 to 4 (then one of that group's items, at random), or empty (nothing). The full
/// rules are on the wiki page "Drops". Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseDropTable {
    /// The number (iROSE's ITEM_DROP.STB row). A zone's own drops are the table with the
    /// zone's number; monsters link theirs.
    #[export]
    id: i64,
    /// For finding it only (the monsters or zone using it).
    #[export]
    name: GString,
    /// Up to 30 slots, in order: an item resource (drag one in), a group number 1 to 4 (as
    /// a whole number), or empty. Low slots drop more often.
    #[export]
    slots: VarArray,
    /// Group 1: up to 5 items, one picked at random when a slot says 1.
    #[export_group(name = "Groups")]
    #[export]
    group_1: Array<Option<Gd<Resource>>>,
    /// Group 2 (slots saying 2).
    #[export]
    group_2: Array<Option<Gd<Resource>>>,
    /// Group 3 (slots saying 3).
    #[export]
    group_3: Array<Option<Gd<Resource>>>,
    /// Group 4 (slots saying 4).
    #[export]
    group_4: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

/// One tab of an NPC's shop: the items it sells, in their places. Several NPCs can share a
/// tab. Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseShopTab {
    /// The number (iROSE's LIST_SELL.STB row).
    #[export]
    id: i64,
    /// The tab's name players see.
    #[export]
    name: GString,
    /// The items (drag item resources in), in shop order; an empty place stays empty in the
    /// shop. Up to 48. Prices come from each item's price.
    #[export]
    items: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

/// `find`'s index of one folder: each number's file (and name), and when it was made.
type FoundIndex = (std::time::Instant, std::collections::HashMap<i64, (String, String)>);

thread_local! {
    static FOUND: std::cell::RefCell<std::collections::HashMap<String, FoundIndex>> = Default::default();
}

/// The resource under `dir` with number `id`, and its name (the editor's spawn and NPC
/// markers, which store numbers). Scans the folder once, and again when a number isn't
/// found or its file moved (at most every few seconds: the markers ask every frame).
pub fn find(dir: &str, id: i64) -> Option<(Gd<Resource>, String)> {
    if id <= 0 {
        return None;
    }
    let mut loader = ResourceLoader::singleton();
    let (cached, fresh) = FOUND.with_borrow(|found| match found.get(dir) {
        Some((made, ids)) => (ids.get(&id).cloned(), made.elapsed().as_secs() < 3),
        None => (None, false),
    });
    let named = |resource: Gd<Resource>| {
        let name = resource.get("name").try_to::<GString>().map(|n| n.to_string()).unwrap_or_default();
        (resource, name)
    };
    if let Some((path, _)) = cached {
        if let Some(resource) = loader.load(&path).filter(|r| r.get("id").try_to::<i64>().ok() == Some(id)) {
            return Some(named(resource));
        }
    }
    if fresh {
        return None;
    }
    let mut files = Vec::new();
    tres_files(dir, &mut files);
    let mut ids = std::collections::HashMap::new();
    for path in files {
        if let Some(resource) = loader.load(&path) {
            let number = resource.get("id").try_to::<i64>().unwrap_or(0);
            let name = resource.get("name").try_to::<GString>().map(|n| n.to_string()).unwrap_or_default();
            ids.insert(number, (path, name));
        }
    }
    let found = ids.get(&id).cloned();
    FOUND.with_borrow_mut(|f| f.insert(dir.to_string(), (std::time::Instant::now(), ids)));
    let (path, _) = found?;
    Some(named(loader.load(&path)?))
}

/// The class of each export kind, with its file name.
const ITEM_CLASSES: [(&str, &str); 15] = [
    ("RoseItemFace", "face"),
    ("RoseItemHead", "head"),
    ("RoseItemBody", "body"),
    ("RoseItemHands", "hands"),
    ("RoseItemFeet", "feet"),
    ("RoseItemBack", "back"),
    ("RoseItemJewellery", "jewellery"),
    ("RoseItemWeapon", "weapon"),
    ("RoseItemSubWeapon", "subweapon"),
    ("RoseItemConsumable", "consumable"),
    ("RoseItemGem", "gem"),
    ("RoseItemMaterial", "material"),
    ("RoseItemQuest", "quest"),
    ("RoseItemVehicle", "vehicle"),
    ("RoseItemGrade", GRADES_FILE),
];

/// The saved properties a class declares itself: name and type.
fn own_properties(class: &str) -> Vec<(String, VariantType)> {
    let list = ClassDb::singleton()
        .class_get_property_list_ex(class)
        .no_inheritance(true)
        .done();
    list.iter_shared()
        .filter_map(|p| {
            let usage = p.get("usage")?.try_to::<i64>().ok()?;
            if usage & PropertyUsageFlags::STORAGE.ord() as i64 == 0 {
                return None;
            }
            let name = p.get("name")?.try_to::<GString>().ok()?.to_string();
            let ty = p.get("type")?.try_to::<i64>().ok()?;
            Some((name, VariantType::from_ord(ty as i32)))
        })
        .collect()
}

/// The kind of thing an id in a link property names (`drop_table`, `shop_tab_2` ...), as
/// `Links` keys it.
fn link_kind(property: &str) -> &str {
    if property.starts_with("shop_tab") {
        "shop_tab"
    } else {
        property
    }
}

/// The item kind of an item class (`RoseItemWeapon` -> `weapon`).
fn item_kind(class: &str) -> Option<&'static str> {
    ITEM_CLASSES
        .iter()
        .find(|(c, kind)| *c == class && *kind != GRADES_FILE)
        .map(|(_, kind)| *kind)
}

/// What a link holds in the export: an item as `weapon/3`, anything else its id.
fn link_value(resource: &Gd<Resource>) -> Value {
    let id = resource.get("id").try_to::<i64>().unwrap_or(0);
    match item_kind(&resource.get_class().to_string()) {
        Some(kind) => Value::from(format!("{kind}/{id}")),
        None => Value::from(id),
    }
}

/// An array element in the export: an empty place is `""`, a link its value, a number itself.
fn element_value(value: &Variant) -> Value {
    match value.get_type() {
        VariantType::NIL => Value::from(""),
        VariantType::INT => Value::from(value.to::<i64>()),
        VariantType::OBJECT => match value.try_to::<Option<Gd<Resource>>>() {
            Ok(Some(resource)) => link_value(&resource),
            _ => Value::from(""),
        },
        _ => Value::from(value.to_string()),
    }
}

/// A resource's properties as the export's JSON object. Empty text and `None` are left out
/// (they mean the default); a colour becomes [red, green, blue] from 0 to 255; a link the
/// item it names (`weapon/3`) or the linked thing's id; an array a list of those (an empty
/// place `""`).
fn to_json(resource: &Gd<Resource>) -> Map<String, Value> {
    let mut out = Map::new();
    for (name, ty) in own_properties(&resource.get_class().to_string()) {
        let value = resource.get(&name);
        let json = match ty {
            VariantType::INT => Value::from(value.to::<i64>()),
            VariantType::BOOL => Value::from(value.to::<bool>()),
            VariantType::STRING => {
                let text = value.to::<GString>().to_string();
                if text.is_empty() || text == NO_STAT {
                    continue;
                }
                Value::from(text)
            }
            VariantType::COLOR => {
                let c = value.to::<Color>();
                let channel = |v: f32| Value::from((v.clamp(0.0, 1.0) * 255.0).round() as u8);
                Value::from(vec![channel(c.r), channel(c.g), channel(c.b)])
            }
            VariantType::PACKED_INT32_ARRAY => Value::from(
                value
                    .to::<PackedInt32Array>()
                    .as_slice()
                    .iter()
                    .map(|v| Value::from(*v))
                    .collect::<Vec<_>>(),
            ),
            VariantType::OBJECT => match value.try_to::<Option<Gd<Resource>>>() {
                Ok(Some(linked)) => link_value(&linked),
                _ => continue,
            },
            VariantType::ARRAY => {
                let Ok(array) = value.try_to::<AnyArray>() else { continue };
                let mut list: Vec<Value> = array.iter_shared().map(|v| element_value(&v)).collect();
                while list.last() == Some(&Value::from("")) {
                    list.pop();
                }
                Value::from(list)
            }
            _ => continue,
        };
        out.insert(name, json);
    }
    out
}

/// The resource a link value names (`weapon/3` -> the item, 147 in `drop_table` -> that
/// drop table), `None` for an empty one.
fn resolve(key: &str, value: &Value, links: &Links) -> Result<Option<Gd<Resource>>, String> {
    let link = match value {
        Value::String(s) if s.is_empty() => return Ok(None),
        Value::String(s) => format!("items/{s}"),
        Value::Number(n) if n.as_i64() == Some(0) => return Ok(None),
        Value::Number(n) => format!("{}/{n}", link_kind(key)),
        _ => return Err(format!("{key}: {value} is not a link")),
    };
    let path = links.get(&link).ok_or(format!("{key}: no resource for {link}"))?;
    ResourceLoader::singleton()
        .load(path)
        .map(Some)
        .ok_or(format!("{key}: could not load {path}"))
}

/// Sets a resource's properties from the export's JSON object, finding linked resources in
/// `links`.
fn set_from_json(resource: &mut Gd<Resource>, object: &Map<String, Value>, links: &Links) -> Result<(), String> {
    let class = resource.get_class().to_string();
    let properties = own_properties(&class);
    for (key, value) in object {
        let Some((_, ty)) = properties.iter().find(|(name, _)| name == key) else {
            return Err(format!("{class} has no property {key}"));
        };
        let variant = match (ty, value) {
            (&VariantType::INT, Value::Number(n)) => {
                n.as_i64().ok_or(format!("{key}: {n}"))?.to_variant()
            }
            (&VariantType::BOOL, Value::Bool(b)) => b.to_variant(),
            (&VariantType::STRING, Value::String(s)) => GString::from(s.as_str()).to_variant(),
            (&VariantType::COLOR, Value::Array(rgb)) if rgb.len() == 3 => {
                let c = |i: usize| rgb[i].as_u64().unwrap_or(0) as f32 / 255.0;
                Color::from_rgb(c(0), c(1), c(2)).to_variant()
            }
            (&VariantType::PACKED_INT32_ARRAY, Value::Array(values)) => values
                .iter()
                .map(|v| v.as_i64().unwrap_or(0) as i32)
                .collect::<PackedInt32Array>()
                .to_variant(),
            (&VariantType::OBJECT, _) => match resolve(key, value, links)? {
                Some(linked) => linked.to_variant(),
                None => Variant::nil(),
            },
            (&VariantType::ARRAY, Value::Array(values)) => {
                let mut list = VarArray::new();
                for element in values {
                    let item = match element {
                        Value::Number(n) => n.as_i64().ok_or(format!("{key}: {n}"))?.to_variant(),
                        _ => match resolve(key, element, links)? {
                            Some(linked) => linked.to_variant(),
                            None => Variant::nil(),
                        },
                    };
                    list.push(&item);
                }
                // Into the property's own array, which may be typed (Array[Resource]).
                let array = resource.get(key);
                array.call("assign", &[list.to_variant()]);
                array
            }
            _ => return Err(format!("{class}.{key}: can't hold {value}")),
        };
        resource.set(key, &variant);
    }
    Ok(())
}

fn to_resource<T: Serialize>(class: &str, export: &T, links: &Links) -> Result<Gd<Resource>, String> {
    let Value::Object(object) = serde_json::to_value(export).map_err(|e| e.to_string())? else {
        return Err(format!("{class}: not an object"));
    };
    let mut resource = ClassDb::singleton()
        .instantiate(class)
        .try_to::<Gd<Resource>>()
        .map_err(|_| format!("{class} is not a resource class"))?;
    set_from_json(&mut resource, &object, links)?;
    Ok(resource)
}

fn from_resource<T: DeserializeOwned>(resource: &Gd<Resource>) -> Result<T, String> {
    serde_json::from_value(Value::Object(to_json(resource))).map_err(|e| e.to_string())
}

/// A file name for a thing: its number, then its name in lower case with dashes
/// (`2-short-sword`), like the wiki's pages.
fn file_stem(id: u32, name: &str) -> String {
    let mut slug = String::new();
    for c in name.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
        } else if !slug.ends_with('-') && !slug.is_empty() {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-');
    let slug: String = slug.chars().take(48).collect();
    let slug = slug.trim_end_matches('-');
    if slug.is_empty() {
        id.to_string()
    } else {
        format!("{id}-{slug}")
    }
}

/// Every `.tres` under `dir` (a `res://` folder), sorted.
fn tres_files(dir: &str, out: &mut Vec<String>) {
    let Some(mut access) = DirAccess::open(dir) else {
        return;
    };
    for sub in access.get_directories().as_slice() {
        tres_files(&format!("{}{}/", dir, sub), out);
    }
    for file in access.get_files().as_slice() {
        let file = file.to_string();
        if file.ends_with(".tres") {
            out.push(format!("{dir}{file}"));
        }
    }
    let _ = access.list_dir_end();
    out.sort();
}

/// `--convert-data`: writes one resource per item and grade from the iROSE tables
/// (with `overrides/game-data.toml`) into `res://data/items/`. Refuses when item resources
/// are already there, unless `force` (which overwrites files of the same name).
pub fn convert_items(force: bool) -> Result<(usize, String), String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    tres_files(ITEMS_DIR, &mut existing);
    if !existing.is_empty() && !force {
        return Err(format!(
            "{ITEMS_DIR} already has {} resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    let strings =
        rose_data_irose::get_string_database(&data.vfs, 1).map_err(|e| format!("{e:#}"))?;
    let items =
        rose_data_irose::get_item_database(&data.vfs, strings).map_err(|e| format!("{e:#}"))?;
    let export = ItemsExport::from_database(&items);
    let mut count = 0;
    macro_rules! write_all {
        ($field:ident, $class:literal, $folder:literal) => {
            for item in &export.$field {
                write_resource(
                    $class,
                    &format!(
                        "{ITEMS_DIR}{}/{}.tres",
                        $folder,
                        file_stem(item.id, &item.name)
                    ),
                    item,
                    &Links::new(),
                )?;
                count += 1;
            }
        };
    }
    write_all!(face, "RoseItemFace", "face");
    write_all!(head, "RoseItemHead", "head");
    write_all!(body, "RoseItemBody", "body");
    write_all!(hands, "RoseItemHands", "hands");
    write_all!(feet, "RoseItemFeet", "feet");
    write_all!(back, "RoseItemBack", "back");
    write_all!(jewellery, "RoseItemJewellery", "jewellery");
    write_all!(weapon, "RoseItemWeapon", "weapon");
    write_all!(subweapon, "RoseItemSubWeapon", "subweapon");
    write_all!(consumable, "RoseItemConsumable", "consumable");
    write_all!(gem, "RoseItemGem", "gem");
    write_all!(material, "RoseItemMaterial", "material");
    write_all!(quest, "RoseItemQuest", "quest");
    write_all!(vehicle, "RoseItemVehicle", "vehicle");
    for grade in &export.grades {
        write_resource(
            "RoseItemGrade",
            &format!("{ITEMS_DIR}grades/{}.tres", grade.grade),
            grade,
            &Links::new(),
        )?;
        count += 1;
    }
    let dir = ProjectSettings::singleton()
        .globalize_path(ITEMS_DIR)
        .to_string();
    Ok((count, dir))
}

fn write_resource<T: Serialize>(class: &str, path: &str, export: &T, links: &Links) -> Result<(), String> {
    let resource = to_resource(class, export, links).map_err(|e| format!("{path}: {e}"))?;
    let folder = path.rsplit_once('/').map_or(path, |(dir, _)| dir);
    if DirAccess::make_dir_recursive_absolute(folder) != Error::OK {
        return Err(format!("could not make {folder}"));
    }
    // A file written again keeps its uid, so what links to it stays linked.
    let uid = file_uid(path).unwrap_or(-1);
    let mut saver = ResourceSaver::singleton();
    match saver.save_ex(&resource).path(path).done() {
        Error::OK => {}
        error => return Err(format!("could not write {path}: {error:?}")),
    }
    // Outside the editor the saver writes no uid; give each file one now, as the editor
    // would, so links to it (drop lists, shops ...) survive moving or renaming it.
    let uid = if uid >= 0 {
        uid
    } else {
        ResourceUid::singleton().create_id()
    };
    match saver.set_uid(path, uid) {
        Error::OK => {}
        error => return Err(format!("could not give {path} a uid: {error:?}")),
    }
    add_link_uids(path)
}

/// Outside the editor the saver writes links without their uid; adds each linked file's
/// uid (from its first line) to its `ext_resource` line, as the editor writes them, so the
/// links survive moving or renaming the linked files.
fn add_link_uids(path: &str) -> Result<(), String> {
    let file = ProjectSettings::singleton().globalize_path(path).to_string();
    let text = std::fs::read_to_string(&file).map_err(|e| format!("could not read {path}: {e}"))?;
    if !text.contains("[ext_resource ") {
        return Ok(());
    }
    let mut out = String::with_capacity(text.len() + 1024);
    for line in text.split_inclusive('\n') {
        if line.starts_with("[ext_resource ") && !line.contains(" uid=\"") {
            let linked = line.split(" path=\"").nth(1).and_then(|rest| rest.split('"').next());
            if let Some(uid) = linked.and_then(file_uid) {
                let uid = ResourceUid::singleton().id_to_text(uid);
                out.push_str(&line.replacen(" path=\"", &format!(" uid=\"{uid}\" path=\""), 1));
                continue;
            }
        }
        out.push_str(line);
    }
    std::fs::write(&file, out).map_err(|e| format!("could not write {path}: {e}"))
}

/// The uid in a text resource's first line (`uid="uid://..."`), read from the file itself:
/// outside the editor the uid cache doesn't know files the editor hasn't scanned.
fn file_uid(path: &str) -> Option<i64> {
    let file = ProjectSettings::singleton()
        .globalize_path(path)
        .to_string();
    let text = std::fs::read_to_string(file).ok()?;
    let header = text.lines().next()?;
    let start = header.find("uid=\"")? + 5;
    let end = start + header[start..].find('"')?;
    let id = ResourceUid::singleton().text_to_id(&header[start..end]);
    (id >= 0).then_some(id)
}

/// Reads every item resource into the export. Two resources with the same number (of one
/// type) are an error naming both files.
pub fn read_items() -> Result<(ItemsExport, usize), String> {
    let files = register_uids(ITEMS_DIR);
    let mut export = ItemsExport::default();
    let mut seen: std::collections::HashMap<(&str, u32), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    for path in &files {
        let resource = loader.load(path).ok_or(format!("could not load {path}"))?;
        let class = resource.get_class().to_string();
        let Some(&(_, kind)) = ITEM_CLASSES.iter().find(|(c, _)| *c == class) else {
            return Err(format!("{path} is a {class}, not an item resource"));
        };
        macro_rules! push {
            ($field:ident, $ty:ty) => {{
                let item: $ty = from_resource(&resource).map_err(|e| format!("{path}: {e}"))?;
                if let Some(other) = seen.insert((kind, item.id), path.clone()) {
                    return Err(format!(
                        "{path} and {other} are both {kind} item {}",
                        item.id
                    ));
                }
                export.$field.push(item);
            }};
        }
        match kind {
            "face" => push!(face, FaceExport),
            "head" => push!(head, HeadExport),
            "body" => push!(body, BodyExport),
            "hands" => push!(hands, HandsExport),
            "feet" => push!(feet, FeetExport),
            "back" => push!(back, BackExport),
            "jewellery" => push!(jewellery, JewelleryExport),
            "weapon" => push!(weapon, WeaponExport),
            "subweapon" => push!(subweapon, SubWeaponExport),
            "consumable" => push!(consumable, ConsumableExport),
            "gem" => push!(gem, GemExport),
            "material" => push!(material, MaterialExport),
            "quest" => push!(quest, QuestExport),
            "vehicle" => push!(vehicle, VehicleExport),
            _ => {
                let grade: GradeExport =
                    from_resource(&resource).map_err(|e| format!("{path}: {e}"))?;
                if let Some(other) = seen.insert((kind, grade.grade), path.clone()) {
                    return Err(format!("{path} and {other} are both grade {}", grade.grade));
                }
                export.grades.push(grade);
            }
        }
    }
    Ok((export, files.len()))
}

/// Every `.tres` under `dir`, after telling the uid cache their uids. Outside the editor the
/// cache is empty, and loading a link whose uid it doesn't know is slow (a warning each);
/// knowing them also makes the links the converter writes carry the uid.
fn register_uids(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    tres_files(dir, &mut files);
    let mut uids = ResourceUid::singleton();
    for path in &files {
        if let Some(uid) = file_uid(path) {
            if uids.has_id(uid) {
                uids.set_id(uid, path);
            } else {
                uids.add_id(uid, path);
            }
        }
    }
    files
}

/// The links to every resource under `dir`, keyed `<prefix>/<id>` (for items
/// `items/<kind>/<id>`), loading each.
fn index_links(dir: &str, links: &mut Links) -> Result<(), String> {
    let files = register_uids(dir);
    let mut loader = ResourceLoader::singleton();
    for path in files {
        let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
        let class = resource.get_class().to_string();
        let id = resource.get("id").try_to::<i64>().unwrap_or(0);
        let key = match class.as_str() {
            "RoseDropTable" => format!("drop_table/{id}"),
            "RoseShopTab" => format!("shop_tab/{id}"),
            _ => match item_kind(&class) {
                Some(kind) => format!("items/{kind}/{id}"),
                None => continue,
            },
        };
        links.insert(key, path);
    }
    Ok(())
}

/// The monster, NPC, drop table and shop tab folders, with their classes.
const NPC_DIRS: [(&str, &str); 4] = [
    (MONSTERS_DIR, "RoseMonster"),
    (NPCS_DIR, "RoseNpc"),
    (DROP_TABLES_DIR, "RoseDropTable"),
    (SHOP_TABS_DIR, "RoseShopTab"),
];

/// `--convert-data` for monsters, NPCs, drop tables and shop tabs: one resource each from
/// the iROSE tables (LIST_NPC.STB, ITEM_DROP.STB, LIST_SELL.STB), linking the item resources
/// (which must be there). A town NPC is one a zone places as an NPC (its IFO files or its
/// Godot scene's export); every other row is a monster. Returns the count and what couldn't
/// be kept.
pub fn convert_npcs(force: bool) -> Result<(usize, Vec<String>), String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    for (dir, _) in NPC_DIRS {
        tres_files(dir, &mut existing);
    }
    if !existing.is_empty() && !force {
        return Err(format!(
            "res://data/ already has {} monster, NPC, drop table or shop tab resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    let mut links = Links::new();
    index_links(ITEMS_DIR, &mut links)?;
    if links.is_empty() {
        return Err(format!("no item resources in {ITEMS_DIR}; the drop tables and shops link them"));
    }
    let strings =
        rose_data_irose::get_string_database(&data.vfs, 1).map_err(|e| format!("{e:#}"))?;
    let zones = rose_data_irose::get_zone_database(&data.vfs, strings.clone()).map_err(|e| format!("{e:#}"))?;
    // The zone scenes' NPCs, as exported (overrides/zones/).
    let project = ProjectSettings::singleton().globalize_path("res://").to_string();
    let layouts = rose_game_overrides::ZoneLayout::load_dir(
        &Path::new(&project).join("..").join(rose_game_overrides::ZONES_DIR),
    )
    .map_err(|e| format!("{e:#}"))?;
    let scene_npcs: Vec<u16> = layouts
        .iter()
        .flat_map(|(_, layout)| layout.npc.iter().map(|n| n.npc as u16))
        .collect();
    let item_exists = |item: rose_data::ItemReference| {
        links.contains_key(&format!("items/{}", rose_data_export::npcs::item_name(item)))
    };
    let (export, skipped) =
        rose_game_data::npcs_export_from_irose(&data.vfs, strings, item_exists, &zones, scene_npcs)
            .map_err(|e| format!("{e:#}"))?;
    let mut count = 0;
    for tab in &export.shop_tabs {
        let path = format!("{SHOP_TABS_DIR}{}.tres", file_stem(tab.id as u32, &tab.name));
        write_resource("RoseShopTab", &path, tab, &links)?;
        count += 1;
    }
    for table in &export.drop_tables {
        let path = format!("{DROP_TABLES_DIR}{}.tres", file_stem(table.id, &table.name));
        write_resource("RoseDropTable", &path, table, &links)?;
        count += 1;
    }
    index_links(SHOP_TABS_DIR, &mut links)?;
    index_links(DROP_TABLES_DIR, &mut links)?;
    for monster in &export.monsters {
        let name = if monster.name.trim().is_empty() { "monster" } else { &monster.name };
        let path = format!("{MONSTERS_DIR}{}.tres", file_stem(monster.id as u32, name));
        write_resource("RoseMonster", &path, monster, &links)?;
        count += 1;
    }
    for npc in &export.npcs {
        let name = if npc.name.trim().is_empty() { "npc" } else { &npc.name };
        let path = format!("{NPCS_DIR}{}.tres", file_stem(npc.id as u32, name));
        write_resource("RoseNpc", &path, npc, &links)?;
        count += 1;
    }
    Ok((count, skipped))
}

/// Reads every monster, NPC, drop table and shop tab resource into the export. Two of one
/// kind with the same number (monsters and NPCs count as one kind) are an error naming both
/// files.
pub fn read_npcs() -> Result<(NpcsExport, usize), String> {
    let mut export = NpcsExport::default();
    let mut seen: std::collections::HashMap<(&str, u32), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    let mut count = 0;
    for (dir, _) in NPC_DIRS {
        register_uids(dir);
    }
    for (dir, _) in NPC_DIRS {
        let mut files = Vec::new();
        tres_files(dir, &mut files);
        for path in files {
            count += 1;
            let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
            let class = resource.get_class().to_string();
            let mut check = |kind: &'static str, id: u32| -> Result<(), String> {
                match seen.insert((kind, id), path.clone()) {
                    Some(other) => Err(format!("{path} and {other} both have number {id}")),
                    None => Ok(()),
                }
            };
            let read = |e: String| format!("{path}: {e}");
            match class.as_str() {
                "RoseMonster" => {
                    let monster: MonsterExport = from_resource(&resource).map_err(read)?;
                    check("character", monster.id as u32)?;
                    export.monsters.push(monster);
                }
                "RoseNpc" => {
                    let npc: NpcExport = from_resource(&resource).map_err(read)?;
                    check("character", npc.id as u32)?;
                    export.npcs.push(npc);
                }
                "RoseDropTable" => {
                    let table: DropTableExport = from_resource(&resource).map_err(read)?;
                    check("drop table", table.id)?;
                    export.drop_tables.push(table);
                }
                "RoseShopTab" => {
                    let tab: ShopTabExport = from_resource(&resource).map_err(read)?;
                    check("shop tab", tab.id as u32)?;
                    export.shop_tabs.push(tab);
                }
                _ => return Err(format!("{path} is a {class}, not a monster, NPC, drop table or shop tab")),
            }
        }
    }
    export.check().map_err(|e| format!("{e:#}"))?;
    Ok((export, count))
}

/// `--export-data`: writes `overrides/data/` (in the repository) from the resources, or
/// with `check` only compares. Returns the files written or compared and those that differ.
pub fn export(check: bool) -> Result<ExportResult, String> {
    let started = std::time::Instant::now();
    let (items, item_resources) = read_items()?;
    if item_resources == 0 {
        return Err(format!(
            "no item resources in {ITEMS_DIR}; run --convert-data first"
        ));
    }
    let (npcs, npc_resources) = read_npcs()?;
    if npc_resources == 0 {
        return Err(format!(
            "no monster resources in {MONSTERS_DIR}; run --convert-data first"
        ));
    }
    let read_ms = started.elapsed().as_millis();
    let project = ProjectSettings::singleton()
        .globalize_path("res://")
        .to_string();
    let dir = Path::new(&project)
        .join("..")
        .join(rose_data_export::DATA_DIR);
    let toml = rose_data_export::Format::Toml;
    let mut files: Vec<(String, String)> = items
        .to_files()
        .map_err(|e| format!("{e:#}"))?
        .into_iter()
        .map(|(name, text)| (rose_data_export::items_file(name), text))
        .collect();
    files.extend(
        npcs.to_files_as(toml)
            .map_err(|e| format!("{e:#}"))?
            .into_iter()
            .map(|(name, text)| (rose_data_export::data_file_as(name, toml), text)),
    );
    // The written files must read back (the server and the game load them the same way).
    let find = |path: String| Ok(files.iter().find(|(n, _)| *n == path).map(|(_, t)| t.clone()));
    ItemsExport::from_files(|name| find(rose_data_export::items_file(name))).map_err(|e| format!("{e:#}"))?;
    NpcsExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the monster export doesn't read back")?
        .drop_rows(rose_game_data::encode_item_base1000)
        .map_err(|e| format!("{e:#}"))?;
    let mut result = ExportResult {
        resources: item_resources + npc_resources,
        items: items.count(),
        monsters: npcs.monsters.len(),
        npcs: npcs.npcs.len(),
        read_ms,
        ..Default::default()
    };
    for (name, text) in files {
        let path = dir.join(name);
        if std::fs::read_to_string(&path)
            .ok()
            .as_deref()
            .map(|t| t.replace("\r\n", "\n"))
            != Some(text.clone())
        {
            result.changed.push(path.to_string_lossy().to_string());
            if !check {
                std::fs::create_dir_all(path.parent().unwrap())
                    .and_then(|_| std::fs::write(&path, &text))
                    .map_err(|e| format!("could not write {}: {e}", path.display()))?;
            }
        }
        result.files.push(
            path.canonicalize()
                .unwrap_or(path)
                .to_string_lossy()
                .to_string(),
        );
    }
    Ok(result)
}

#[derive(Default)]
pub struct ExportResult {
    pub files: Vec<String>,
    pub changed: Vec<String>,
    pub resources: usize,
    pub items: usize,
    pub monsters: usize,
    pub npcs: usize,
    pub read_ms: u128,
}

/// Entry point for GDScript: the `--convert-data` and `--export-data` commands.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct RoseDataExport {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseDataExport {
    /// Writes the resources of every kind that has none yet (all of them with `force`) from
    /// the iROSE tables: items first, then monsters, NPCs, drop tables and shop tabs, which
    /// link the items. Needs RoseData.open. Returns {resources, folder, skipped (kinds already
    /// there), lost (values that couldn't be kept)}, or {error}.
    #[func]
    fn convert(force: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        let mut count = 0;
        let mut kept = VarArray::new();
        let mut lost = VarArray::new();
        let mut items = Vec::new();
        tres_files(ITEMS_DIR, &mut items);
        if force || items.is_empty() {
            match convert_items(force) {
                Ok((n, _)) => count += n,
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"items".to_variant());
        }
        let mut others = Vec::new();
        for (dir, _) in NPC_DIRS {
            tres_files(dir, &mut others);
        }
        if force || others.is_empty() {
            match convert_npcs(force) {
                Ok((n, skipped)) => {
                    count += n;
                    for line in skipped {
                        lost.push(&line.to_variant());
                    }
                }
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"monsters and NPCs".to_variant());
        }
        let folder = ProjectSettings::singleton().globalize_path("res://data/").to_string();
        result.set("resources", count as i64);
        result.set("folder", folder.as_str());
        result.set("skipped", &kept);
        result.set("lost", &lost);
        result
    }

    /// Writes (or with `check` compares) `overrides/data/` from the resources. Returns
    /// {files, changed, resources, items, monsters, npcs, read_ms}, or {error}.
    #[func]
    fn export(check: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        match export(check) {
            Ok(out) => {
                let list = |v: &[String]| v.iter().map(|p| p.to_variant()).collect::<VarArray>();
                result.set("files", &list(&out.files));
                result.set("changed", &list(&out.changed));
                result.set("resources", out.resources as i64);
                result.set("items", out.items as i64);
                result.set("monsters", out.monsters as i64);
                result.set("npcs", out.npcs as i64);
                result.set("read_ms", out.read_ms as i64);
            }
            Err(error) => result.set("error", error.as_str()),
        }
        result
    }
}
