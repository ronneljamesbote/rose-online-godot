//! The game data as Godot resources: one `.tres` per thing under `res://data/`, edited in the
//! Inspector. So far items (`res://data/items/<type>/<id>-<name>.tres`, one class per item
//! type, `RoseItemWeapon` and so on, plus `RoseItemGrade` for the refine grades), monsters
//! (`RoseMonster`), town NPCs (`RoseNpc`), drop tables (`RoseDropTable`), shop tabs
//! (`RoseShopTab`), skills (`RoseSkill`, one per skill level, in a folder per skill),
//! status effects (`RoseStatusEffect`), job classes (`RoseJobClass`), zone settings
//! (`RoseZoneSettings`), warp gates (`RoseWarpGate`), crafting recipes (`RoseCraftRecipe`),
//! new characters (`RoseNewCharacter`) and sounds (`RoseSound`, `RoseHitSounds`,
//! `RoseFootsteps`, linking the imported audio files).
//!
//! The game itself never reads these files. `--export-data` writes them to
//! `export/data/` as TOML (the layout is in `crates/rose-data-export`), and the server and
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
        AudioStream, ClassDb, DirAccess, IResource, ProjectSettings, Resource, ResourceLoader, ResourceSaver,
        ResourceUid,
    },
    global::Error,
    prelude::*,
    register::info::{PropertyHint, PropertyInfo, PropertyUsageFlags},
};
use rose_data::{
    AbilityType, ItemClass, SkillActionMode, SkillBasicCommand, SkillDamageType, SkillTargetFilter,
    SkillType, StatusEffectClearedByType, StatusEffectType,
};
use rose_data_export::items::{
    BackExport, BodyExport, ConsumableExport, FaceExport, FeetExport, GemExport, GradeExport,
    HandsExport, HeadExport, JewelleryExport, MaterialExport, QuestExport, SubWeaponExport,
    VehicleExport, WeaponExport, GRADES_FILE,
};
use rose_data_export::npcs::{DropTableExport, MonsterExport, NpcExport, ShopTabExport};
use rose_data_export::skills::{JobClassExport, SkillExport, StatusEffectExport};
use rose_data_export::sounds::{FootstepsExport, HitSoundsExport, SoundExport};
use rose_data_export::effects::{AttackEffectExport, EffectFileExport};
use rose_data_export::world::{CraftRecipeExport, NewCharacterExport, WarpGateExport, ZoneExport};
use rose_data_export::{EffectsExport, ItemsExport, NpcsExport, SkillsExport, SoundsExport, WorldExport};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

/// Where the item resources live.
pub const ITEMS_DIR: &str = "res://data/items/";
/// Where the monster, NPC, drop table and shop tab resources live.
pub const MONSTERS_DIR: &str = "res://data/monsters/";
pub const NPCS_DIR: &str = "res://data/npcs/";
pub const DROP_TABLES_DIR: &str = "res://data/drop_tables/";
pub const SHOP_TABS_DIR: &str = "res://data/shop_tabs/";
/// Where the skill, status effect and job class resources live.
pub const SKILLS_DIR: &str = "res://data/skills/";
pub const STATUS_EFFECTS_DIR: &str = "res://data/status_effects/";
pub const JOB_CLASSES_DIR: &str = "res://data/job_classes/";
/// Where the zone settings, warp gate, crafting recipe and new character resources live.
pub const ZONES_DIR: &str = "res://data/zones/";
pub const WARP_GATES_DIR: &str = "res://data/warp_gates/";
pub const CRAFT_RECIPES_DIR: &str = "res://data/craft_recipes/";
pub const NEW_CHARACTERS_DIR: &str = "res://data/new_characters/";
/// Where the sound, hit sound and footstep resources live.
pub const SOUNDS_DIR: &str = "res://data/sounds/";
pub const HIT_SOUNDS_DIR: &str = "res://data/hit_sounds/";
pub const FOOTSTEPS_DIR: &str = "res://data/footsteps/";
/// Where the effect file and attack effect resources live.
pub const EFFECT_FILES_DIR: &str = "res://data/effect_files/";
pub const ATTACK_EFFECTS_DIR: &str = "res://data/attack_effects/";

/// Where to find a linked resource by what the export holds: `items/weapon/3`,
/// `drop_table/147`, `shop_tab/12` -> its `res://` path.
type Links = std::collections::HashMap<String, String>;

/// What an empty stat field holds (no stat).
const NO_STAT: &str = "None";

/// The names of an enum's values, in order (with `NO_STAT` first for an optional one).
fn variants<E: Enum + std::fmt::Debug>(optional: bool) -> Vec<String> {
    optional
        .then(|| NO_STAT.to_string())
        .into_iter()
        .chain((0..E::LENGTH).map(|i| format!("{:?}", E::from_usize(i))))
        .collect()
}

/// The Inspector's dropdowns for the text fields that name a game enum.
fn enum_hint(property: &mut PropertyInfo) {
    if property.variant_type != VariantType::STRING {
        return;
    }
    let name = property.property_name.to_string();
    let options: Vec<String> = if name == "item_class" {
        variants::<ItemClass>(false)
    } else if name.starts_with("required_item_class") {
        variants::<ItemClass>(true)
    } else if name == "skill_type" {
        variants::<SkillType>(false)
    } else if name == "target_filter" {
        variants::<SkillTargetFilter>(false)
    } else if name == "action_mode" {
        variants::<SkillActionMode>(false)
    } else if name == "basic_command" {
        variants::<SkillBasicCommand>(true)
    } else if name == "raw_material_class" {
        variants::<ItemClass>(true)
    } else if name == "gender" {
        vec!["Male".into(), "Female".into()]
    } else if name == "damage_type" {
        variants::<SkillDamageType>(false)
    } else if name == "status_effect_type" {
        variants::<StatusEffectType>(false)
    } else if name == "cleared_by" {
        variants::<StatusEffectClearedByType>(false)
    } else if name.contains("stat") {
        std::iter::once(NO_STAT.to_string())
            .chain((0..AbilityType::LENGTH).map(|i| format!("{:?}", AbilityType::from_usize(i))))
            .collect()
    } else if name == "vehicle_type" {
        vec!["Cart".into(), "CastleGear".into()]
    } else if name == "bullet_move" {
        vec![NO_STAT.into(), "Linear".into(), "Parabola".into(), "Immediate".into()]
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
            /// The sound played when it is equipped (empty = none): drag a `RoseSound` in
            /// from `res://data/sounds/`.
            #[export]
            equip_sound: Option<Gd<RoseSound>>,

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
        /// The projectile it shoots (empty = none): drag a `RoseAttackEffect` in from
        /// `res://data/attack_effects/`. Bows and guns shoot their ammunition's instead.
        bullet_effect: Option<Gd<RoseAttackEffect>>,
        /// How its attacks look (empty = none): the effects on the weapon, its trail and the
        /// spark on what it hits, a `RoseAttackEffect`.
        attack_effect: Option<Gd<RoseAttackEffect>>,
        /// The sound when the swing starts (empty = none).
        attack_start_sound: Option<Gd<RoseSound>>,
        /// The sound when it fires (empty = its projectile's sound).
        attack_fire_sound: Option<Gd<RoseSound>>,
        /// What it sounds like when it hits (a `RoseHitSounds` from `res://data/hit_sounds/`:
        /// one sound for armour, cloth and stone; empty = silent).
        hit_sounds: Option<Gd<RoseHitSounds>>,
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
        /// The effect played when it is used (empty = none): drag a `RoseEffectFile` in from
        /// `res://data/effect_files/`.
        effect: Option<Gd<RoseEffectFile>>,
        /// The sound played when it is used (empty = none).
        effect_sound: Option<Gd<RoseSound>>,
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
        /// The glow on the socketed item (empty = none): a `RoseAttackEffect` whose point
        /// effects are drawn on the weapon.
        gem_effect: Option<Gd<RoseAttackEffect>>,
        /// The gem's picture in the socket.
        gem_sprite_id: i64,
    }
);
item_resource!(
    /// A crafting material or ammunition. Edit it here, then run `--export-data` (see
    /// `godot/data/README.md`).
    RoseItemMaterial {
        /// Arrows, bullets and shells: the projectile they fly as (empty = none), a
        /// `RoseAttackEffect` from `res://data/attack_effects/`.
        #[export_group(name = "Material")]
        bullet_effect: Option<Gd<RoseAttackEffect>>,
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
        /// Effect when getting on (empty = none): a `RoseEffectFile`.
        ride_effect: Option<Gd<RoseEffectFile>>,
        /// Sound when getting on (empty = none).
        ride_sound: Option<Gd<RoseSound>>,
        /// Effect when getting off.
        dismount_effect: Option<Gd<RoseEffectFile>>,
        /// Sound when getting off.
        dismount_sound: Option<Gd<RoseSound>>,
        /// Effect when it is destroyed.
        dead_effect: Option<Gd<RoseEffectFile>>,
        /// Sound when it is destroyed.
        dead_sound: Option<Gd<RoseSound>>,
        /// Sound when it stops.
        stop_sound: Option<Gd<RoseSound>>,
        /// Effect while moving.
        move_effect: Option<Gd<RoseEffectFile>>,
        /// Sound while moving.
        move_sound: Option<Gd<RoseSound>>,
        /// How its weapon's hits look (a `RoseAttackEffect`).
        hit_effect: Option<Gd<RoseAttackEffect>>,
        /// Sound on the target when its weapon hits.
        hit_sound: Option<Gd<RoseSound>>,
        /// The projectile its weapon fires (a `RoseAttackEffect`).
        bullet_effect: Option<Gd<RoseAttackEffect>>,
        /// The model point the projectile leaves from.
        bullet_fire_point: i64,
        /// Up to 8 effects fixed to the model's points, in order (`RoseEffectFile`s; an
        /// empty place = none on that point).
        dummy_effects: Array<Option<Gd<RoseEffectFile>>>,
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
            /// What its body is made of, which picks the sound of a weapon hitting it (the
            /// attacker's `hit_sounds`). Players count as cloth.
            #[export(enum = (Armour = 0, Cloth = 1, Stone = 2))]
            hit_sound_material_type: i64,
            /// The sound it makes now and then (empty = none): drag a `RoseSound` in from
            /// `res://data/sounds/`.
            #[export]
            normal_effect_sound: Option<Gd<RoseSound>>,
            /// The sound of its attack (empty = none).
            #[export]
            attack_sound: Option<Gd<RoseSound>>,
            /// The sound when it is hit (empty = none).
            #[export]
            hitted_sound: Option<Gd<RoseSound>>,
            /// How its bare-hand hits look on its target (empty = none): drag a
            /// `RoseAttackEffect` in from `res://data/attack_effects/`.
            #[export]
            hand_hit_effect: Option<Gd<RoseAttackEffect>>,
            /// The effect when it dies (empty = none): a `RoseEffectFile` from
            /// `res://data/effect_files/`.
            #[export]
            die_effect: Option<Gd<RoseEffectFile>>,
            /// The sound when it dies (empty = none).
            #[export]
            die_sound: Option<Gd<RoseSound>>,
            /// The effect when it appears (0 = none).
            #[export]
            create_effect_index: i64,
            /// The sound when it appears (empty = none).
            #[export]
            create_sound: Option<Gd<RoseSound>>,

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

/// One level of a skill. Each level is its own resource with its own id (characters store
/// the id of the level they learnt); the levels of one skill share `base_skill_id` and sit
/// in one folder. Edit it here, then run `--export-data` (see `godot/data/README.md`). How
/// skills work is on the wiki page "Skills".
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseSkill {
    /// The number (iROSE's LIST_SKILL.STB row). Characters, hotbars, skill books and quests
    /// name the skill level by it, so don't change it once it is in use. A new skill level
    /// takes any free number.
    #[export_group(name = "Skill")]
    #[export]
    id: i64,
    /// The name players see.
    #[export]
    name: GString,
    /// The tooltip text.
    #[export(multiline)]
    description: GString,
    /// The id of the skill's first level (0 = this is the first level). The levels of a
    /// skill share it: learning the next level replaces this one, and other skills that
    /// need this skill name the first level. A plain number because the first level would
    /// have to link to itself.
    #[export]
    base_skill_id: i64,
    /// Which level of the skill this is (1, 2, 3 ...).
    #[export]
    #[init(val = 1)]
    level: i64,
    /// What kind of skill: an attack, a buff on self or a target, an area spell, a passive,
    /// a basic action (sit, pick up ...), an emote, a summon ...
    #[export]
    #[init(val = GString::from("BasicAction"))]
    skill_type: GString,
    /// The tab of the skill window it shows on (0 basic, 1 active, 2 passive, 3 clan ...).
    #[export]
    page: i64,
    /// The icon: its number in the skill icon sheet.
    #[export]
    icon_number: i64,

    /// Skill points it costs to learn this level.
    #[export_group(name = "Learning")]
    #[export]
    learn_point_cost: i64,
    /// Zuly it costs to learn this level.
    #[export]
    learn_money_cost: i64,
    /// The job class that may learn it (empty = any job). Drag a job class resource in.
    #[export]
    required_job_class: Option<Gd<RoseJobClass>>,
    /// iROSE's planet requirement. iROSE keeps it in the cast range's column, so most skills
    /// show their cast range here; the game doesn't read it.
    #[export]
    required_planet: i64,
    /// A union whose members may learn it (0 = none).
    #[export]
    required_union_1: i64,
    /// A second union (0 = none).
    #[export]
    required_union_2: i64,
    /// A third union (0 = none).
    #[export]
    required_union_3: i64,
    /// A skill needed first (its first level), at `required_skill_1_level`.
    #[export]
    required_skill_1: Option<Gd<RoseSkill>>,
    /// The level of that skill needed.
    #[export]
    required_skill_1_level: i64,
    /// A second skill needed first.
    #[export]
    required_skill_2: Option<Gd<RoseSkill>>,
    /// The level of the second skill needed.
    #[export]
    required_skill_2_level: i64,
    /// A third skill needed first.
    #[export]
    required_skill_3: Option<Gd<RoseSkill>>,
    /// The level of the third skill needed.
    #[export]
    required_skill_3_level: i64,
    /// A stat needed to learn it (Level for the character level).
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_stat_1: GString,
    /// How much of that stat is needed.
    #[export]
    required_stat_1_value: i64,
    /// A second stat needed to learn it.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_stat_2: GString,
    /// How much of the second stat is needed.
    #[export]
    required_stat_2_value: i64,
    /// A weapon or item class needed in hand to use it (None = any). Any one of the five
    /// will do.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_item_class_1: GString,
    /// Another class that will do.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_item_class_2: GString,
    /// Another class that will do.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_item_class_3: GString,
    /// Another class that will do.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_item_class_4: GString,
    /// Another class that will do.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    required_item_class_5: GString,

    /// What one use costs (Mana, Health, Stamina ...).
    #[export_group(name = "Use")]
    #[export]
    #[init(val = GString::from(NO_STAT))]
    use_stat_1: GString,
    /// How much it costs (Mana before Save Mana ...).
    #[export]
    use_stat_1_value: i64,
    /// A second cost.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    use_stat_2: GString,
    /// How much the second cost is.
    #[export]
    use_stat_2_value: i64,
    /// Milliseconds before it can be used again (iROSE stores steps of 200 ms).
    #[export(range = (0.0, 600000.0, 200.0, or_greater, suffix = "ms"))]
    cooldown_ms: i64,
    /// Skills of the same cooldown group share one cooldown (0 = its own).
    #[export]
    cooldown_group: i64,
    /// Who it can be used on (self, party, allies, monsters, enemies, the dead ...).
    #[export]
    #[init(val = GString::from("OnlySelf"))]
    target_filter: GString,
    /// How far away the target may be, in centimetres (0 = the attack range).
    #[export(range = (0.0, 10000.0, 1.0, or_greater, suffix = "cm"))]
    cast_range: i64,
    /// The radius of the area it reaches, in centimetres (0 = one target).
    #[export(range = (0.0, 10000.0, 1.0, or_greater, suffix = "cm"))]
    scope: i64,
    /// What the caster does afterwards (Stop, Attack, Restore; see the wiki's "Changed from
    /// iROSE" on the Skills page).
    #[export]
    #[init(val = GString::from("Stop"))]
    action_mode: GString,
    /// For a basic action: what it does (sit, pick up, attack, trade, party invite ...).
    #[export]
    #[init(val = GString::from(NO_STAT))]
    basic_command: GString,

    /// How its damage is worked out (weapon, magic, natural magic ...).
    #[export_group(name = "Effect")]
    #[export]
    #[init(val = GString::from("ContinuousAttack"))]
    damage_type: GString,
    /// The skill's power: damage power, the XP given back by a resurrection, or the
    /// percentage of an additional damage or damage shield buff.
    #[export]
    power: i64,
    /// iROSE's harm value (not read by the game).
    #[export]
    harm: i64,
    /// For a crafting skill: which crafting it opens (items with this craft skill type).
    #[export]
    item_make_number: i64,
    /// The base chance in percent of its status effects landing (0 = they always land).
    #[export]
    success_ratio: i64,
    /// A status effect it gives. Drag a status effect resource in.
    #[export]
    status_effect_1: Option<Gd<RoseStatusEffect>>,
    /// A second status effect it gives.
    #[export]
    status_effect_2: Option<Gd<RoseStatusEffect>>,
    /// How long its status effects last, in seconds.
    #[export(range = (0.0, 3600.0, 1.0, or_greater, suffix = "s"))]
    status_effect_seconds: i64,
    /// The stat the first status effect changes (or the HP/MP it restores).
    #[export]
    #[init(val = GString::from(NO_STAT))]
    add_stat_1: GString,
    /// The flat amount (grows with the caster's INT).
    #[export]
    add_stat_1_value: i64,
    /// The percentage of the target's current value.
    #[export]
    add_stat_1_rate: i64,
    /// The stat the second status effect changes.
    #[export]
    #[init(val = GString::from(NO_STAT))]
    add_stat_2: GString,
    /// The flat amount of the second change.
    #[export]
    add_stat_2_value: i64,
    /// The percentage of the second change.
    #[export]
    add_stat_2_rate: i64,
    /// The monster it summons. Drag a monster resource in.
    #[export]
    summon_monster: Option<Gd<RoseMonster>>,
    /// The zone it warps to (0 = none).
    #[export]
    warp_zone: i64,
    /// Where in that zone, in centimetres (east).
    #[export]
    warp_x: i64,
    /// Where in that zone, in centimetres (north).
    #[export]
    warp_y: i64,

    /// The casting motion (a motion number, 0 = none).
    #[export_group(name = "Motions and effects")]
    #[export]
    casting_motion_id: i64,
    /// Its speed in percent (100 = as made).
    #[export(range = (1.0, 1000.0, 1.0, or_greater, suffix = "%"))]
    #[init(val = 100)]
    casting_motion_speed: i64,
    /// A motion repeated while casting (0 = none).
    #[export]
    casting_repeat_motion_id: i64,
    /// How many times it repeats.
    #[export]
    #[init(val = 1)]
    casting_repeat_motion_count: i64,
    /// An effect while casting (empty = none): drag a `RoseEffectFile` in from
    /// `res://data/effect_files/`.
    #[export]
    casting_effect_1: Option<Gd<RoseEffectFile>>,
    /// The model point it sits on (-1 = the character's position).
    #[export]
    #[init(val = -1)]
    casting_effect_1_point: i64,
    /// A second casting effect.
    #[export]
    casting_effect_2: Option<Gd<RoseEffectFile>>,
    /// Its model point (-1 = none).
    #[export]
    #[init(val = -1)]
    casting_effect_2_point: i64,
    /// A third casting effect.
    #[export]
    casting_effect_3: Option<Gd<RoseEffectFile>>,
    /// Its model point (-1 = none).
    #[export]
    #[init(val = -1)]
    casting_effect_3_point: i64,
    /// A fourth casting effect.
    #[export]
    casting_effect_4: Option<Gd<RoseEffectFile>>,
    /// Its model point (-1 = none).
    #[export]
    #[init(val = -1)]
    casting_effect_4_point: i64,
    /// The motion when it goes off (0 = none). Its hit markers time the hits.
    #[export]
    action_motion_id: i64,
    /// Its speed in percent (100 = as made).
    #[export(range = (1.0, 1000.0, 1.0, or_greater, suffix = "%"))]
    #[init(val = 100)]
    action_motion_speed: i64,
    /// The projectile it fires (empty = none): a `RoseAttackEffect` from
    /// `res://data/attack_effects/` (its bullet, speed, flight and hit). A self skill shows
    /// the projectile's bullet effect on its caster instead.
    #[export]
    bullet_effect: Option<Gd<RoseAttackEffect>>,
    /// The model point the projectile leaves from.
    #[export]
    bullet_point: i64,
    /// The sound when it fires (empty = none): drag a `RoseSound` in from
    /// `res://data/sounds/`.
    #[export]
    bullet_fire_sound: Option<Gd<RoseSound>>,
    /// The effect on the target when it hits (empty = none): a `RoseEffectFile`.
    #[export]
    hit_effect: Option<Gd<RoseEffectFile>>,
    /// The target's model point the hit effect sits on (-1 = none).
    #[export]
    #[init(val = -1)]
    hit_effect_point: i64,
    /// The sound when it hits (empty = none).
    #[export]
    hit_sound: Option<Gd<RoseSound>>,
    /// A second effect on the target when it hits (empty = none).
    #[export]
    hit_dummy_effect_1: Option<Gd<RoseEffectFile>>,
    /// Its sound (empty = none).
    #[export]
    hit_dummy_sound_1: Option<Gd<RoseSound>>,
    /// A third effect on the target when it hits (empty = none).
    #[export]
    hit_dummy_effect_2: Option<Gd<RoseEffectFile>>,
    /// Its sound (empty = none).
    #[export]
    hit_dummy_sound_2: Option<Gd<RoseSound>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        enum_hint(property);
    }
}

/// A status effect: a buff, a debuff, healing or poison over time, a stun ... Skills and
/// items give it. Edit it here, then run `--export-data`. How they work is on the wiki page
/// "Status effects".
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseStatusEffect {
    /// The number (iROSE's LIST_STATUS.STB row). Skills, items and saved characters name it
    /// by this.
    #[export]
    id: i64,
    /// The name players see.
    #[export]
    name: GString,
    /// The tooltip text.
    #[export(multiline)]
    description: GString,
    /// The chat line when it starts.
    #[export]
    start_message: GString,
    /// The chat line when it ends.
    #[export]
    end_message: GString,
    /// What it does (raise HP, poison, raise attack, sleep, stun, clear bad effects ...).
    #[export]
    #[init(val = GString::from("IncreaseHp"))]
    status_effect_type: GString,
    /// A new one may replace one still running.
    #[export]
    can_be_reapplied: bool,
    /// Which clearing removes it (ClearGood: removed by "clear good effects", ClearBad: by
    /// "clear bad effects", ClearNone: can't be cleared).
    #[export]
    #[init(val = GString::from("ClearGood"))]
    cleared_by: GString,
    /// The status effect it runs each second (its number; often this effect's own). Recovery
    /// potions name an HP or MP recovery effect here.
    #[export]
    apply_status_effect_1: i64,
    /// The amount per second (HP healed, HP a poison takes ...).
    #[export]
    apply_status_effect_1_value: i64,
    /// A second effect it runs each second (0 = none).
    #[export]
    apply_status_effect_2: i64,
    /// Its amount per second.
    #[export]
    apply_status_effect_2_value: i64,
    /// The effect drawn on the character while it lasts (empty = none): drag a
    /// `RoseEffectFile` in from `res://data/effect_files/`.
    #[export]
    effect: Option<Gd<RoseEffectFile>>,
    /// The icon over the character frame: its number in the status icon sheet.
    #[export]
    icon_id: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseStatusEffect {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        enum_hint(property);
    }
}

/// A job class: a set of jobs that a skill or an item can require ("Soldier Job" = soldier,
/// knight, champion ...). Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseJobClass {
    /// The number (iROSE's LIST_CLASS.STB row). Skills and items link it; items, quests and
    /// saved data name it by this. A number with no job class takes every job.
    #[export]
    id: i64,
    /// The name players see ("only Soldier Job can use this").
    #[export]
    name: GString,
    /// The jobs in the class, by number: 0 visitor; 111 soldier, 121 knight, 122 champion;
    /// 211 muse, 221 mage, 222 cleric; 311 hawker, 321 raider, 322 scout; 411 dealer,
    /// 421 bourgeois, 422 artisan (131, 132, 231 ... are the unreleased third jobs). Up to 8.
    #[export]
    jobs: PackedInt32Array,
    base: Base<Resource>,
}

/// A zone's settings: its name, map, music, sky, time of day and rules. One per zone,
/// including the zones not edited as Godot scenes (their spawns and NPCs stay in the iROSE map
/// files); a zone scene shows its settings on its root node. Edit it here, then run
/// `--export-data`. Zones are described on the wiki's zone pages.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseZoneSettings {
    /// The zone's number (iROSE's LIST_ZONE.STB row; a zone scene's folder starts with it).
    /// Saved characters, warp gates and quests name the zone by it.
    #[export_group(name = "Zone")]
    #[export]
    id: i64,
    /// The name players see (loading screen, map, version line).
    #[export]
    name: GString,
    /// The text under the name on the loading screen.
    #[export(multiline)]
    description: GString,

    /// The iROSE map file (a `.zon` path in the client data). The game builds the zone from
    /// it unless the zone has Godot scenes (`godot/zones/`); the server reads its blocks'
    /// spawns and NPCs unless the zone has an exported layout (`export/zones/`).
    #[export_group(name = "Map")]
    #[export]
    map_file: GString,
    /// The named spot players arrive at when they come to the zone without a warp gate, and
    /// new characters start at (usually `start`).
    #[export]
    start_spot: GString,
    /// The named spots fallen players can get up at (the nearest one of this name; usually
    /// `restore`).
    #[export]
    revive_spot: GString,
    /// The side of the squares the server splits the zone into to decide who sees whom, in
    /// centimetres (5000 to 12000; the wiki page "Visibility" explains it).
    #[export(range = (5000.0, 12000.0, 1.0, suffix = "cm"))]
    sector_size: i64,

    /// The music by day (empty = none): an audio file, such as the imported
    /// `res://assets/rose/sound/bgm/plain01_junon.res` or one of our own in
    /// `res://assets/ours/`.
    #[export_group(name = "Look and sound")]
    #[export]
    music_day: Option<Gd<AudioStream>>,
    /// The music by night (evening and night).
    #[export]
    music_night: Option<Gd<AudioStream>>,
    /// The sky: its row in iROSE's LIST_SKY.STB (-1 = none).
    #[export]
    #[init(val = -1)]
    sky: i64,
    /// The minimap image (a DDS path in the client data, empty = none).
    #[export]
    minimap: GString,
    /// Where the minimap image starts: the block column its left edge shows.
    #[export]
    minimap_start_x: i64,
    /// Where the minimap image starts: the block row its top edge shows.
    #[export]
    minimap_start_y: i64,
    /// The iROSE model list of the zone's decorations (a ZSC file).
    #[export]
    decoration_models: GString,
    /// The iROSE model list of the zone's buildings (a ZSC file).
    #[export]
    building_models: GString,
    /// The footstep sounds walking here: a `RoseFootsteps` from `res://data/footsteps/`
    /// (empty = the same step everywhere).
    #[export]
    footsteps: Option<Gd<RoseFootsteps>>,

    /// The length of a day here in world ticks (one tick is 10 seconds).
    #[export_group(name = "Time of day")]
    #[export]
    day_cycle: i64,
    /// When in the day morning starts (ticks into the day).
    #[export]
    morning_time: i64,
    /// When day starts.
    #[export]
    day_time: i64,
    /// When evening starts.
    #[export]
    evening_time: i64,
    /// When night starts.
    #[export]
    night_time: i64,

    /// Who can fight whom here: no one, everyone except clan members, everyone except party
    /// members, everyone, or a clan zone. The wiki page "Player against player" has the rules.
    #[export_group(name = "Rules")]
    #[export(enum = (NoFighting = 0, AllButClan = 1, AllButParty = 2, Everyone = 3, ClanZone = 11))]
    pvp_state: i64,
    /// The planet the zone is on (1 Junon, 2 Luna, 3 Eldeon, 4 Orlo ...). A save point only
    /// works on the planet it was saved on.
    #[export]
    planet: i64,
    /// What can't be driven here.
    #[export(flags = (NoCarts = 1, NoCastleGear = 2))]
    vehicle_use_flags: i64,
    base: Base<Resource>,
}

/// A warp gate: where walking into it takes you. The gate models placed in a zone scene link
/// it (their `warp_gate`). Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseWarpGate {
    /// The gate's number (iROSE's WARP.STB row). The gates placed in the zones name it.
    #[export]
    id: i64,
    /// The zone it takes you to: drag its `RoseZoneSettings` in from `res://data/zones/`.
    #[export]
    target_zone: Option<Gd<RoseZoneSettings>>,
    /// The named spot of that zone you arrive at (a `RoseZoneEvent` of its scene, or a ZON
    /// event position of its map file).
    #[export]
    target_spot: GString,
    base: Base<Resource>,
}

/// A crafting or refining recipe: up to four materials, used one step after another. An
/// item names its recipe by number (its `craft_material`); recipes 1 to 10 refine weapons and
/// 11 to 20 everything else, one per grade. Edit it here, then run `--export-data`. The wiki
/// pages "Crafting" and "Refining" have the rules.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseCraftRecipe {
    /// The number (iROSE's LIST_PRODUCT.STB row). Items name their recipe by it.
    #[export]
    id: i64,
    /// For finding it only (what it makes).
    #[export]
    name: GString,
    /// When `material_1` is empty, any item of this class can be the first material (metal,
    /// leather ... for crafting; the refining recipes take chemicals).
    #[export]
    #[init(val = GString::from(NO_STAT))]
    raw_material_class: GString,
    /// The first material: an item resource (drag one in), or empty for any item of the raw
    /// material class.
    #[export_group(name = "Materials")]
    #[export]
    material_1: Option<Gd<Resource>>,
    /// How many of the first material.
    #[export]
    material_1_quantity: i64,
    /// The second material (empty = none).
    #[export]
    material_2: Option<Gd<Resource>>,
    /// How many of the second material.
    #[export]
    material_2_quantity: i64,
    /// The third material (empty = none).
    #[export]
    material_3: Option<Gd<Resource>>,
    /// How many of the third material.
    #[export]
    material_3_quantity: i64,
    /// The fourth material (empty = none).
    #[export]
    material_4: Option<Gd<Resource>>,
    /// How many of the fourth material.
    #[export]
    material_4_quantity: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseCraftRecipe {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        enum_hint(property);
    }
}

/// What a new character starts with: stats, the gear it wears and what is in its bag. One
/// for male and one for female characters. New characters also know the basic actions
/// (sit, pick up, emotes ...), which are in code. Edit it here, then run `--export-data`. The
/// wiki page "Character creation" describes it.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseNewCharacter {
    /// Which characters this is for.
    #[export]
    #[init(val = GString::from("Male"))]
    gender: GString,
    /// Starting strength.
    #[export_group(name = "Stats")]
    #[export]
    strength: i64,
    /// Starting dexterity.
    #[export]
    dexterity: i64,
    /// Starting intelligence.
    #[export]
    intelligence: i64,
    /// Starting concentration.
    #[export]
    concentration: i64,
    /// Starting charm.
    #[export]
    charm: i64,
    /// Starting sense.
    #[export]
    sense: i64,
    /// Worn on the face (an item resource, empty = nothing).
    #[export_group(name = "Wearing")]
    #[export]
    face: Option<Gd<Resource>>,
    /// Worn on the head.
    #[export]
    head: Option<Gd<Resource>>,
    /// Worn on the body.
    #[export]
    body: Option<Gd<Resource>>,
    /// Worn on the back.
    #[export]
    back: Option<Gd<Resource>>,
    /// Worn on the hands.
    #[export]
    hands: Option<Gd<Resource>>,
    /// Worn on the feet.
    #[export]
    feet: Option<Gd<Resource>>,
    /// Held as the weapon.
    #[export]
    weapon: Option<Gd<Resource>>,
    /// Held in the other hand (a shield or support tool).
    #[export]
    sub_weapon: Option<Gd<Resource>>,
    /// Equipment in the bag (up to 10 item resources).
    #[export_group(name = "Bag")]
    #[export]
    bag_equipment: Array<Option<Gd<Resource>>>,
    /// Consumables in the bag (up to 5 item resources), how many of each in
    /// `bag_consumable_counts` (same order).
    #[export]
    bag_consumables: Array<Option<Gd<Resource>>>,
    /// How many of each consumable.
    #[export]
    bag_consumable_counts: PackedInt32Array,
    /// Materials in the bag (up to 5 item resources), how many of each in
    /// `bag_material_counts`.
    #[export]
    bag_materials: Array<Option<Gd<Resource>>>,
    /// How many of each material.
    #[export]
    bag_material_counts: PackedInt32Array,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseNewCharacter {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        enum_hint(property);
    }
}

/// A sound the game plays: an audio file, by the number items, monsters, skills and the hit
/// and footstep sounds link it with. The Inspector plays `stream` (the arrow next to it).
/// Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseSound {
    /// The number (iROSE's FILE_SOUND.STB row). A new sound takes any free number.
    #[export]
    id: i64,
    /// For finding it only (the file's name).
    #[export]
    name: GString,
    /// The audio: drag an audio file in, such as the imported
    /// `res://assets/rose/sound/attack/sword_001.res` or a WAV or OGG file of our own in
    /// `res://assets/ours/` (empty = silent). A file in `res://assets/ours/` with the same
    /// path as an imported one plays instead of it.
    #[export]
    stream: Option<Gd<AudioStream>>,
    base: Base<Resource>,
}

/// What one kind of weapon sounds like when it hits: one sound for each material the target
/// can be made of (a monster's `hit_sound_material_type`; players are cloth). A weapon links
/// it as its `hit_sounds`. Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseHitSounds {
    /// The number (iROSE's LIST_HITSOUND.STB row).
    #[export]
    id: i64,
    /// For finding it only (the weapon kind).
    #[export]
    name: GString,
    /// Hitting armour (metal monsters such as golems and machines).
    #[export]
    on_armour: Option<Gd<RoseSound>>,
    /// Hitting cloth or flesh (players and most monsters).
    #[export]
    on_cloth: Option<Gd<RoseSound>>,
    /// Hitting stone.
    #[export]
    on_stone: Option<Gd<RoseSound>>,
    base: Base<Resource>,
}

/// The footstep sound of each ground texture, for the zones that link it as their
/// `footsteps`. Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseFootsteps {
    /// The number (iROSE's LIST_STEPSOUND.STB column, the zone's footstep type).
    #[export]
    id: i64,
    /// For finding it only (the zones that use it).
    #[export]
    name: GString,
    /// The step on ground texture 0, 1, 2 ... (a zone's texture number, the order of its
    /// ground textures). An empty place, a texture past the end, buildings and bridges play
    /// the solid step (sound 653).
    #[export]
    sounds: Array<Option<Gd<RoseSound>>>,
    base: Base<Resource>,
}

/// An effect the game can show, by the number items, monsters, skills, status effects and
/// attack effects link it with: an effect scene (a `RoseEffect` with its particle and mesh
/// parts). Open the scene to see it play. Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseEffectFile {
    /// The number (iROSE's FILE_EFFECT.STB row). A new effect takes any free number.
    #[export]
    id: i64,
    /// For finding it only (the file's name).
    #[export]
    name: GString,
    /// The effect scene: drag one in, such as the imported
    /// `res://assets/rose/3ddata/effect/levelup_01.scn` or a scene of our own in
    /// `res://assets/ours/` (empty = nothing shows). A scene in `res://assets/ours/` with the
    /// same path as an imported one plays instead of it.
    #[export]
    scene: Option<Gd<PackedScene>>,
    base: Base<Resource>,
}

/// How an attack looks: the effects on the weapon, the swing's trail, the projectile and its
/// flight, and the hit. Weapons, gems, ammunition, cart parts, monsters and skills link it.
/// Edit it here, then run `--export-data`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAttackEffect {
    /// The number (iROSE's LIST_EFFECT.STB row). A new one takes any free number.
    #[export]
    id: i64,
    /// For finding it only (the first thing that uses it).
    #[export]
    name: GString,
    /// Effects drawn on the weapon's effect points, in order (up to 4 `RoseEffectFile`s).
    #[export]
    point_effects: Array<Option<Gd<RoseEffectFile>>>,
    /// The colour of the swing's trail (a row of the client's trail colours, 0 = no trail).
    #[export]
    trail_colour: i64,
    /// How long the trail stays, in milliseconds.
    #[export]
    trail_milliseconds: i64,
    /// The effect on the target when it hits (empty = none).
    #[export]
    hit_effect: Option<Gd<RoseEffectFile>>,
    /// The effect on the target on a critical hit (empty = the normal one).
    #[export]
    hit_effect_critical: Option<Gd<RoseEffectFile>>,
    /// The projectile (empty = none): an arrow, a bullet, a fire ball.
    #[export]
    bullet_effect: Option<Gd<RoseEffectFile>>,
    /// How the projectile flies: Linear (straight to the target), Parabola (in an arc) or
    /// Immediate (appears on the target).
    #[export]
    bullet_move: GString,
    /// How fast the projectile flies, in centimetres per second.
    #[export]
    bullet_speed: f32,
    /// The sound when it shoots (empty = none).
    #[export]
    fire_sound: Option<Gd<RoseSound>>,
    /// The sound when it hits (empty = none).
    #[export]
    hit_sound: Option<Gd<RoseSound>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseAttackEffect {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        enum_hint(property);
    }
}

/// `find`'s index of one folder: each number's file and name, and when it was made.
type FoundIndex = (std::time::Instant, std::collections::HashMap<i64, (String, String)>);

thread_local! {
    static FOUND: std::cell::RefCell<std::collections::HashMap<String, FoundIndex>> = Default::default();
    /// The resources `find` handed out, kept loaded. Loading a monster loads everything it
    /// links (drop table, items, their sounds and effect scenes); let go, Godot frees all of
    /// it and the next marker that asks loads it again.
    static KEPT: std::cell::RefCell<std::collections::HashMap<String, Gd<Resource>>> = Default::default();
}

/// Drops the resources `find` kept while the engine is still running (a Godot object
/// freed with the thread's locals, after the engine has gone, aborts the program).
pub fn clear_cache() {
    KEPT.with_borrow_mut(|kept| kept.clear());
}

/// The number and name in a data resource's text, read without loading it (and everything
/// it links): the `id = ` and `name = "..."` lines of its `[resource]` section.
fn text_id_name(path: &str) -> Option<(i64, String)> {
    let text = std::fs::read_to_string(ProjectSettings::singleton().globalize_path(path).to_string()).ok()?;
    let body = &text[text.find("\n[resource]")?..];
    let (mut id, mut name) = (None, String::new());
    for line in body.lines().skip(2) {
        if line.starts_with('[') {
            break;
        } else if let Some(value) = line.strip_prefix("id = ") {
            id = value.trim().parse().ok();
        } else if let Some(value) = line.strip_prefix("name = \"") {
            name = value.strip_suffix('"').unwrap_or(value).replace("\\\"", "\"").replace("\\\\", "\\");
        }
    }
    Some((id?, name))
}

/// The file and name of number `id` under `dir`. Indexes the folder from the files' text
/// once, and again when a number isn't found (at most every few seconds: the markers ask
/// every frame).
fn find_path(dir: &str, id: i64) -> Option<(String, String)> {
    if id <= 0 {
        return None;
    }
    let (cached, fresh) = FOUND.with_borrow(|found| match found.get(dir) {
        Some((made, ids)) => (ids.get(&id).cloned(), made.elapsed().as_secs() < 3),
        None => (None, false),
    });
    if cached.is_some() || fresh {
        return cached;
    }
    let mut files = Vec::new();
    tres_files(dir, &mut files);
    let ids: std::collections::HashMap<_, _> = files.into_iter().filter_map(|path| text_id_name(&path).map(|(number, name)| (number, (path, name)))).collect();
    let found = ids.get(&id).cloned();
    FOUND.with_borrow_mut(|f| f.insert(dir.to_string(), (std::time::Instant::now(), ids)));
    found
}

/// The name of number `id` under `dir`, for the editor's spawn and NPC labels: from the
/// resource when it is loaded (so an edit shows at once), else from its file's text.
pub fn find_name(dir: &str, id: i64) -> Option<String> {
    let (path, name) = find_path(dir, id)?;
    let loader = ResourceLoader::singleton();
    if loader.has_cached(&path) {
        if let Some(resource) = loader.get_cached_ref(&path) {
            return Some(resource.get("name").try_to::<GString>().map(|n| n.to_string()).unwrap_or(name));
        }
    }
    Some(name)
}

/// The resource under `dir` with number `id`, and its name (the editor's spawn and NPC
/// markers, which store numbers). Kept loaded once asked for.
pub fn find(dir: &str, id: i64) -> Option<(Gd<Resource>, String)> {
    let (path, _) = find_path(dir, id)?;
    let resource = match KEPT.with_borrow(|kept| kept.get(&path).cloned()) {
        Some(resource) => resource,
        None => {
            let resource = ResourceLoader::singleton().load(&path)?;
            KEPT.with_borrow_mut(|kept| kept.insert(path.clone(), resource.clone()));
            resource
        }
    };
    if resource.get("id").try_to::<i64>().ok() != Some(id) {
        // Renumbered since the folder was indexed: index it again.
        FOUND.with_borrow_mut(|f| f.remove(dir));
        KEPT.with_borrow_mut(|kept| kept.remove(&path));
        return None;
    }
    let name = resource.get("name").try_to::<GString>().map(|n| n.to_string()).unwrap_or_default();
    Some((resource, name))
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

/// The saved properties a class declares itself: name, type and, for a link, the class it
/// links.
fn own_properties(class: &str) -> Vec<(String, VariantType, String)> {
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
            let linked = p.get("class_name").and_then(|c| c.try_to::<StringName>().ok()).map(|c| c.to_string()).unwrap_or_default();
            let linked = if linked.is_empty() {
                p.get("hint_string").and_then(|h| h.try_to::<GString>().ok()).map(|h| h.to_string()).unwrap_or_default()
            } else {
                linked
            };
            Some((name, VariantType::from_ord(ty as i32), linked))
        })
        .collect()
}

/// The kind of thing an id in a link property names (`drop_table`, `shop_tab_2`,
/// `required_skill_1` ...), as `Links` keys it.
fn link_kind(property: &str) -> &str {
    if property.starts_with("shop_tab") {
        "shop_tab"
    } else if property.starts_with("required_skill") {
        "skill"
    } else if property.starts_with("status_effect") {
        "status_effect"
    } else if property == "required_job_class" {
        "job_class"
    } else if property == "summon_monster" {
        "monster"
    } else if property == "target_zone" {
        "zone"
    } else if property.ends_with("_sound") || property.starts_with("hit_dummy_sound") || property.starts_with("on_") || property == "sounds" {
        "sound"
    } else {
        property
    }
}

/// The kind a link to one of these classes names, whatever the property is called (a skill's
/// `hit_effect` is an effect file, a cart part's an attack effect).
fn class_link_kind(class: &str) -> Option<&'static str> {
    match class {
        "RoseEffectFile" => Some("effect_file"),
        "RoseAttackEffect" => Some("attack_effect"),
        _ => None,
    }
}

/// The item kind of an item class (`RoseItemWeapon` -> `weapon`).
fn item_kind(class: &str) -> Option<&'static str> {
    ITEM_CLASSES
        .iter()
        .find(|(c, kind)| *c == class && *kind != GRADES_FILE)
        .map(|(_, kind)| *kind)
}

/// What a link holds in the export: an item as `weapon/3`, an audio file or a scene its path, anything
/// else its id.
fn link_value(resource: &Gd<Resource>) -> Value {
    if resource.is_class("AudioStream") || resource.is_class("PackedScene") {
        return Value::from(resource.get_path().to_string());
    }
    let id = resource.get("id").try_to::<i64>().unwrap_or(0);
    match item_kind(&resource.get_class().to_string()) {
        Some(kind) => Value::from(format!("{kind}/{id}")),
        None => Value::from(id),
    }
}

/// Whether an array property holds links named by number (a typed array of one of our
/// classes, `RoseFootsteps.sounds`): an empty place is then 0, not `""`.
fn numbered_links(array: &Variant) -> bool {
    let class = array.call("get_typed_class_name", &[]).to_string();
    !class.is_empty() && class != "Resource"
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
    for (name, ty, _) in own_properties(&resource.get_class().to_string()) {
        let value = resource.get(&name);
        let json = match ty {
            VariantType::INT => Value::from(value.to::<i64>()),
            // Through f32, as the export holds it, so 0.1 stays 0.1.
            VariantType::FLOAT => Value::from(value.to::<f64>() as f32),
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
                let numbered = numbered_links(&value);
                let Ok(array) = value.try_to::<AnyArray>() else { continue };
                let empty = if numbered { Value::from(0) } else { Value::from("") };
                let mut list: Vec<Value> =
                    array.iter_shared().map(|v| if v.is_nil() { empty.clone() } else { element_value(&v) }).collect();
                while list.last() == Some(&empty) {
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
fn resolve(key: &str, value: &Value, links: &Links, class: &str) -> Result<Option<Gd<Resource>>, String> {
    let link = match value {
        Value::String(s) if s.is_empty() => return Ok(None),
        // An audio file or a scene, by its path.
        Value::String(s) if s.starts_with("res://") => {
            return ResourceLoader::singleton()
                .load(s.as_str())
                .map(Some)
                .ok_or(format!("{key}: could not load {s} (run --import-rose first)"));
        }
        Value::String(s) => format!("items/{s}"),
        Value::Number(n) if n.as_i64() == Some(0) => return Ok(None),
        Value::Number(n) => format!("{}/{n}", class_link_kind(class).unwrap_or_else(|| link_kind(key))),
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
        let Some((_, ty, linked)) = properties.iter().find(|(name, _, _)| name == key) else {
            return Err(format!("{class} has no property {key}"));
        };
        let variant = match (ty, value) {
            (&VariantType::INT, Value::Number(n)) => {
                n.as_i64().ok_or(format!("{key}: {n}"))?.to_variant()
            }
            // The export's f32 as written (0.1, not 0.10000000149).
            (&VariantType::FLOAT, Value::Number(n)) => {
                let single = n.as_f64().ok_or(format!("{key}: {n}"))? as f32;
                single.to_string().parse::<f64>().unwrap_or(single as f64).to_variant()
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
            (&VariantType::OBJECT, _) => match resolve(key, value, links, linked)? {
                Some(linked) => linked.to_variant(),
                None => Variant::nil(),
            },
            (&VariantType::ARRAY, Value::Array(values)) => {
                let numbered = numbered_links(&resource.get(key));
                let element_class = resource.get(key).call("get_typed_class_name", &[]).to_string();
                let mut list = VarArray::new();
                for element in values {
                    let item = match element {
                        Value::Number(n) if !numbered => n.as_i64().ok_or(format!("{key}: {n}"))?.to_variant(),
                        _ => match resolve(key, element, links, &element_class)? {
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
/// into `res://data/items/`. Refuses when item resources
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
    let mut links = Links::new();
    index_sound_links(&mut links)?;
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
                    &links,
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
    // Tell the uid cache too, so files written later in this run link this one by its uid.
    let mut uids = ResourceUid::singleton();
    if uids.has_id(uid) {
        uids.set_id(uid, path);
    } else {
        uids.add_id(uid, path);
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
            "RoseMonster" => format!("monster/{id}"),
            "RoseSkill" => format!("skill/{id}"),
            "RoseStatusEffect" => format!("status_effect/{id}"),
            "RoseJobClass" => format!("job_class/{id}"),
            "RoseZoneSettings" => format!("zone/{id}"),
            "RoseSound" => format!("sound/{id}"),
            "RoseHitSounds" => format!("hit_sounds/{id}"),
            "RoseFootsteps" => format!("footsteps/{id}"),
            "RoseEffectFile" => format!("effect_file/{id}"),
            "RoseAttackEffect" => format!("attack_effect/{id}"),
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
    index_sound_links(&mut links)?;
    let strings =
        rose_data_irose::get_string_database(&data.vfs, 1).map_err(|e| format!("{e:#}"))?;
    let zones = rose_data_irose::get_zone_database(&data.vfs, strings.clone()).map_err(|e| format!("{e:#}"))?;
    // The zone scenes' NPCs, as exported (export/zones/).
    let project = ProjectSettings::singleton().globalize_path("res://").to_string();
    let layouts = rose_data_export::ZoneLayout::load_dir(
        &Path::new(&project).join("..").join(rose_data_export::ZONES_DIR),
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

/// The skill, status effect and job class folders, with their classes.
const SKILL_DIRS: [(&str, &str); 3] = [
    (SKILLS_DIR, "RoseSkill"),
    (STATUS_EFFECTS_DIR, "RoseStatusEffect"),
    (JOB_CLASSES_DIR, "RoseJobClass"),
];

/// The skills in an order where each comes after the skills it needs (a resource can only
/// link a file that is already written). A loop of skills needing each other is an error.
fn skills_in_link_order(skills: &[SkillExport]) -> Result<Vec<&SkillExport>, String> {
    let by_id: std::collections::HashMap<u16, &SkillExport> = skills.iter().map(|s| (s.id, s)).collect();
    // 0 = not visited, 1 = on the current path, 2 = done.
    let mut state: std::collections::HashMap<u16, u8> = std::collections::HashMap::new();
    let mut out = Vec::with_capacity(skills.len());
    fn visit<'a>(
        skill: &'a SkillExport,
        by_id: &std::collections::HashMap<u16, &'a SkillExport>,
        state: &mut std::collections::HashMap<u16, u8>,
        out: &mut Vec<&'a SkillExport>,
    ) -> Result<(), String> {
        match state.get(&skill.id) {
            Some(2) => return Ok(()),
            Some(1) => return Err(format!("skill {} needs itself through the skills it needs", skill.id)),
            _ => {}
        }
        state.insert(skill.id, 1);
        for other in skill.skill_links() {
            let other = by_id.get(&other).ok_or(format!("skill {} needs skill {other}, which isn't there", skill.id))?;
            visit(other, by_id, state, out)?;
        }
        state.insert(skill.id, 2);
        out.push(skill);
        Ok(())
    }
    for skill in skills {
        visit(skill, &by_id, &mut state, &mut out)?;
    }
    Ok(out)
}

/// `--convert-data` for skills, status effects and job classes: one resource each from the
/// iROSE tables (LIST_SKILL.STB, LIST_STATUS.STB, LIST_CLASS.STB), linking each other and the
/// monsters skills summon (which must be there). A skill's levels go in one folder,
/// `skills/<first level's id>-<name>/`.
pub fn convert_skills(force: bool) -> Result<usize, String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    for (dir, _) in SKILL_DIRS {
        tres_files(dir, &mut existing);
    }
    if !existing.is_empty() && !force {
        return Err(format!(
            "res://data/ already has {} skill, status effect or job class resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    let mut links = Links::new();
    index_links(MONSTERS_DIR, &mut links)?;
    if links.is_empty() {
        return Err(format!("no monster resources in {MONSTERS_DIR}; the summoning skills link them"));
    }
    index_sound_links(&mut links)?;
    let strings = rose_data_irose::get_string_database(&data.vfs, 1).map_err(|e| format!("{e:#}"))?;
    let skills = rose_data_irose::get_skill_database(&data.vfs, strings.clone()).map_err(|e| format!("{e:#}"))?;
    let effects =
        rose_data_irose::get_status_effect_database(&data.vfs, strings.clone()).map_err(|e| format!("{e:#}"))?;
    let classes = rose_data_irose::get_job_class_database(&data.vfs, strings).map_err(|e| format!("{e:#}"))?;
    let export = SkillsExport::from_databases(&skills, &effects, &classes);
    export.check().map_err(|e| format!("{e:#}"))?;
    let mut count = 0;
    for effect in &export.status_effects {
        let name = if effect.name.trim().is_empty() { "status-effect" } else { &effect.name };
        let path = format!("{STATUS_EFFECTS_DIR}{}.tres", file_stem(effect.id as u32, name));
        write_resource("RoseStatusEffect", &path, effect, &links)?;
        links.insert(format!("status_effect/{}", effect.id), path);
        count += 1;
    }
    for class in &export.job_classes {
        let name = if class.name.trim().is_empty() { "job-class" } else { &class.name };
        let path = format!("{JOB_CLASSES_DIR}{}.tres", file_stem(class.id as u32, name));
        write_resource("RoseJobClass", &path, class, &links)?;
        links.insert(format!("job_class/{}", class.id), path);
        count += 1;
    }
    // A skill's folder is named after its first level, or the first of its levels with a name.
    let mut names: std::collections::BTreeMap<u16, &str> = std::collections::BTreeMap::new();
    for skill in &export.skills {
        let base = if skill.base_skill_id == 0 { skill.id } else { skill.base_skill_id };
        let name = skill.name.trim();
        let entry = names.entry(base).or_insert(name);
        if entry.is_empty() || (skill.id == base && !name.is_empty()) {
            *entry = name;
        }
    }
    for skill in skills_in_link_order(&export.skills)? {
        if let Some(monster) = (skill.summon_monster != 0).then_some(skill.summon_monster) {
            if !links.contains_key(&format!("monster/{monster}")) {
                return Err(format!("skill {} summons monster {monster}, which has no resource", skill.id));
            }
        }
        let base = if skill.base_skill_id == 0 { skill.id } else { skill.base_skill_id };
        let named = |name: &str| if name.is_empty() { "skill".to_string() } else { name.to_string() };
        let folder = file_stem(base as u32, &named(names.get(&base).copied().unwrap_or_default()));
        let path = format!("{SKILLS_DIR}{folder}/{}.tres", file_stem(skill.id as u32, &named(skill.name.trim())));
        write_resource("RoseSkill", &path, skill, &links)?;
        links.insert(format!("skill/{}", skill.id), path);
        count += 1;
    }
    Ok(count)
}

/// Reads every skill, status effect and job class resource into the export. Two of one kind
/// with the same number are an error naming both files.
pub fn read_skills() -> Result<(SkillsExport, usize), String> {
    let mut export = SkillsExport::default();
    let mut seen: std::collections::HashMap<(&str, u16), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    let mut count = 0;
    for (dir, _) in SKILL_DIRS {
        register_uids(dir);
    }
    for (dir, _) in SKILL_DIRS {
        let mut files = Vec::new();
        tres_files(dir, &mut files);
        for path in files {
            count += 1;
            let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
            let class = resource.get_class().to_string();
            let mut check = |kind: &'static str, id: u16| -> Result<(), String> {
                match seen.insert((kind, id), path.clone()) {
                    Some(other) => Err(format!("{path} and {other} are both {kind} {id}")),
                    None => Ok(()),
                }
            };
            let read = |e: String| format!("{path}: {e}");
            match class.as_str() {
                "RoseSkill" => {
                    let skill: SkillExport = from_resource(&resource).map_err(read)?;
                    check("skill", skill.id)?;
                    export.skills.push(skill);
                }
                "RoseStatusEffect" => {
                    let effect: StatusEffectExport = from_resource(&resource).map_err(read)?;
                    check("status effect", effect.id)?;
                    export.status_effects.push(effect);
                }
                "RoseJobClass" => {
                    let class: JobClassExport = from_resource(&resource).map_err(read)?;
                    check("job class", class.id)?;
                    export.job_classes.push(class);
                }
                _ => return Err(format!("{path} is a {class}, not a skill, status effect or job class")),
            }
        }
    }
    export.check().map_err(|e| format!("{e:#}"))?;
    Ok((export, count))
}

const WORLD_DIRS: [(&str, &str); 4] = [
    (ZONES_DIR, "RoseZoneSettings"),
    (WARP_GATES_DIR, "RoseWarpGate"),
    (CRAFT_RECIPES_DIR, "RoseCraftRecipe"),
    (NEW_CHARACTERS_DIR, "RoseNewCharacter"),
];

/// `--convert-data` for the world: zone settings (LIST_ZONE.STB), warp gates (WARP.STB),
/// crafting recipes (LIST_PRODUCT.STB) and new characters (INIT_AVATAR.STB), one resource
/// each, linking the zones and the items (which must be there). Returns how many and what
/// couldn't be kept.
pub fn convert_world(force: bool) -> Result<(usize, Vec<String>), String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    for (dir, _) in WORLD_DIRS {
        tres_files(dir, &mut existing);
    }
    if !existing.is_empty() && !force {
        return Err(format!(
            "res://data/ already has {} zone, warp gate, recipe or new character resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    let mut links = Links::new();
    index_links(ITEMS_DIR, &mut links)?;
    if links.is_empty() {
        return Err(format!("no item resources in {ITEMS_DIR}; the recipes and new characters link them"));
    }
    index_sound_links(&mut links)?;
    let strings = rose_data_irose::get_string_database(&data.vfs, 1).map_err(|e| format!("{e:#}"))?;
    let (mut export, mut lost) =
        rose_game_data::world_export_from_irose(&data.vfs, &strings, &data.items).map_err(|e| format!("{e:#}"))?;
    // The music links the imported audio files.
    for zone in &mut export.zones {
        for music in [&mut zone.music_day, &mut zone.music_night] {
            if music.is_empty() {
                continue;
            }
            match audio_link(music) {
                Some(path) => *music = path,
                None => {
                    lost.push(format!("zone {}: the client data has no {music}", zone.id));
                    music.clear();
                }
            }
        }
    }
    let mut count = 0;
    let mut zone_names = std::collections::HashMap::new();
    for zone in &export.zones {
        let name = if zone.name.trim().is_empty() { "zone" } else { &zone.name };
        let path = format!("{ZONES_DIR}{}.tres", file_stem(zone.id as u32, name));
        write_resource("RoseZoneSettings", &path, zone, &links)?;
        links.insert(format!("zone/{}", zone.id), path);
        zone_names.insert(zone.id, name.to_string());
        count += 1;
    }
    for gate in &export.warp_gates {
        let target = zone_names.get(&gate.target_zone).cloned().unwrap_or_default();
        let path = format!("{WARP_GATES_DIR}{}.tres", file_stem(gate.id as u32, &format!("to {target}")));
        write_resource("RoseWarpGate", &path, gate, &links)?;
        count += 1;
    }
    for recipe in &export.craft_recipes {
        let name = if recipe.name.trim().is_empty() { "recipe" } else { &recipe.name };
        let path = format!("{CRAFT_RECIPES_DIR}{}.tres", file_stem(recipe.id as u32, name));
        write_resource("RoseCraftRecipe", &path, recipe, &links)?;
        count += 1;
    }
    for character in &export.new_characters {
        let name = format!("{:?}", character.gender).to_lowercase();
        let path = format!("{NEW_CHARACTERS_DIR}{name}.tres");
        write_resource("RoseNewCharacter", &path, character, &links)?;
        count += 1;
    }
    Ok((count, lost))
}

/// Reads every zone settings, warp gate, recipe and new character resource into the export.
/// Two of one kind with the same number (or gender) are an error naming both files.
pub fn read_world() -> Result<(WorldExport, usize), String> {
    let mut export = WorldExport::default();
    let mut seen: std::collections::HashMap<(&str, String), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    let mut count = 0;
    for (dir, _) in WORLD_DIRS {
        register_uids(dir);
    }
    for (dir, _) in WORLD_DIRS {
        let mut files = Vec::new();
        tres_files(dir, &mut files);
        for path in files {
            count += 1;
            let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
            let class = resource.get_class().to_string();
            let mut check = |kind: &'static str, id: String| -> Result<(), String> {
                match seen.insert((kind, id.clone()), path.clone()) {
                    Some(other) => Err(format!("{path} and {other} are both {kind} {id}")),
                    None => Ok(()),
                }
            };
            let read = |e: String| format!("{path}: {e}");
            match class.as_str() {
                "RoseZoneSettings" => {
                    let zone: ZoneExport = from_resource(&resource).map_err(read)?;
                    check("zone", zone.id.to_string())?;
                    export.zones.push(zone);
                }
                "RoseWarpGate" => {
                    let gate: WarpGateExport = from_resource(&resource).map_err(read)?;
                    check("warp gate", gate.id.to_string())?;
                    export.warp_gates.push(gate);
                }
                "RoseCraftRecipe" => {
                    let recipe: CraftRecipeExport = from_resource(&resource).map_err(read)?;
                    check("recipe", recipe.id.to_string())?;
                    export.craft_recipes.push(recipe);
                }
                "RoseNewCharacter" => {
                    let character: NewCharacterExport = from_resource(&resource).map_err(read)?;
                    check("new character", format!("{:?}", character.gender))?;
                    export.new_characters.push(character);
                }
                _ => return Err(format!("{path} is a {class}, not a zone, warp gate, recipe or new character")),
            }
        }
    }
    export.check().map_err(|e| format!("{e:#}"))?;
    Ok((export, count))
}

/// The sound, hit sound and footstep folders, with their classes.
const SOUND_DIRS: [(&str, &str); 3] = [
    (SOUNDS_DIR, "RoseSound"),
    (HIT_SOUNDS_DIR, "RoseHitSounds"),
    (FOOTSTEPS_DIR, "RoseFootsteps"),
];

/// The links to every sound, hit sound, footstep, effect file and attack effect resource.
fn index_sound_links(links: &mut Links) -> Result<(), String> {
    for (dir, _) in SOUND_DIRS.iter().chain(&EFFECT_DIRS) {
        index_links(dir, links)?;
    }
    Ok(())
}

/// The imported Godot copy of a sound file in the client data (`Sound\BGM\Prologue.ogg` ->
/// `res://assets/rose/sound/bgm/prologue.res`, or our own file at that place in
/// `res://assets/ours/`), None when there is none.
pub fn audio_link(client_path: &str) -> Option<String> {
    if client_path.trim().is_empty() {
        return None;
    }
    crate::assets::find_stem(&crate::assets::stem(client_path), crate::assets::AUDIO_EXTENSIONS)
}

/// `--convert-data` for sounds: one resource per FILE_SOUND.STB row with a file (or that an
/// item, monster or skill names), per LIST_HITSOUND.STB row and per LIST_STEPSOUND.STB
/// column, linking the imported audio files (`--import-rose` first). Sounds go in a folder
/// per client folder (`sounds/attack/37-sword-001.tres`). Returns the count and the sounds
/// whose file the client data doesn't have.
pub fn convert_sounds(force: bool) -> Result<(usize, Vec<String>), String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    for (dir, _) in SOUND_DIRS {
        tres_files(dir, &mut existing);
    }
    if !existing.is_empty() && !force {
        return Err(format!(
            "res://data/ already has {} sound, hit sound or footstep resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    if crate::assets::find_stem("sound/avata/step_solid", crate::assets::AUDIO_EXTENSIONS).is_none() {
        return Err("the sounds are not imported; run --import-rose first".into());
    }
    let sounds = rose_data_irose::get_sound_database(&data.vfs).map_err(|e| format!("{e:#}"))?;
    // The sounds and hit sounds something names, so their links survive even without a file.
    let mut used = std::collections::BTreeSet::new();
    let mut used_hits = std::collections::BTreeSet::new();
    let mut add = |id: Option<rose_data::SoundId>| {
        if let Some(id) = id {
            used.insert(id.get());
        }
    };
    use rose_data::ItemType;
    for item_type in (0..ItemType::LENGTH).map(ItemType::from_usize) {
        for item in data.items.iter_items(item_type) {
            if let Some(base) = data.items.get_base_item(item) {
                add(base.equip_sound_id);
            }
            let number = item.item_number;
            match item_type {
                ItemType::Weapon => {
                    let weapon = data.items.get_weapon_item(number).unwrap();
                    add(weapon.attack_start_sound_id);
                    add(weapon.attack_fire_sound_id);
                    used_hits.insert(weapon.attack_hit_sound_index);
                }
                ItemType::Consumable => add(data.items.get_consumable_item(number).unwrap().effect_sound_id),
                ItemType::Vehicle => {
                    let part = data.items.get_vehicle_item(number).unwrap();
                    for id in [part.ride_sound_id, part.dismount_sound_id, part.dead_sound_id, part.stop_sound_id, part.move_sound_id, part.hit_sound_id] {
                        add(id);
                    }
                }
                _ => {}
            }
        }
    }
    for npc in data.npcs.iter() {
        for id in [npc.normal_effect_sound_id, npc.attack_sound_id, npc.hitted_sound_id, npc.die_sound_id, npc.create_sound_id] {
            add(id);
        }
    }
    for skill in data.skills.iter() {
        for id in [skill.bullet_fire_sound_id, skill.hit_sound_id, skill.hit_dummy_sound_id[0], skill.hit_dummy_sound_id[1]] {
            add(id);
        }
    }
    let mut export = SoundsExport::from_database(&sounds, &used, &used_hits);
    used.extend(export.hit_sounds.iter().flat_map(|h| [h.on_armour, h.on_cloth, h.on_stone]));
    used.extend(export.footsteps.iter().flat_map(|f| f.sounds.iter().copied()));
    // FILE_SOUND names some files the client doesn't have (the game plays nothing for them):
    // a sound something links stays, silent; the others are left out.
    let mut lost = Vec::new();
    let mut left_out = 0;
    export.sounds.retain_mut(|sound| {
        if sound.stream.is_empty() {
            return true;
        }
        match audio_link(&sound.stream) {
            Some(path) => {
                sound.stream = path;
                true
            }
            None if used.contains(&sound.id) => {
                lost.push(format!("sound {}: the client data has no {}, kept silent", sound.id, sound.stream));
                sound.stream.clear();
                true
            }
            None => {
                left_out += 1;
                false
            }
        }
    });
    if left_out > 0 {
        lost.push(format!("{left_out} FILE_SOUND rows nothing links name files the client data doesn't have; left out"));
    }
    // A footstep set is named after the zones that walk on it.
    for set in &mut export.footsteps {
        let zones: Vec<&str> = data
            .zone_list
            .iter()
            .filter(|z| z.footstep_type == Some(set.id))
            .map(|z| z.name.trim())
            .filter(|n| !n.is_empty())
            .collect();
        set.name = match zones.len() {
            0 => format!("footsteps {}", set.id),
            1 => zones[0].to_string(),
            n => format!("{} and {} more", zones[0], n - 1),
        };
    }
    let mut links = Links::new();
    let mut count = 0;
    for sound in &export.sounds {
        let folder = sound
            .stream
            .strip_prefix(crate::assets::ROSE)
            .or_else(|| sound.stream.strip_prefix(crate::assets::OURS))
            .and_then(|p| p.split('/').nth(1).filter(|_| p.matches('/').count() >= 2))
            .unwrap_or("other");
        let path = format!("{SOUNDS_DIR}{folder}/{}.tres", file_stem(sound.id as u32, &sound.name));
        write_resource("RoseSound", &path, sound, &links)?;
        links.insert(format!("sound/{}", sound.id), path);
        count += 1;
    }
    for set in &export.hit_sounds {
        let path = format!("{HIT_SOUNDS_DIR}{}.tres", file_stem(set.id, &set.name));
        write_resource("RoseHitSounds", &path, set, &links)?;
        count += 1;
    }
    for set in &export.footsteps {
        let path = format!("{FOOTSTEPS_DIR}{}.tres", file_stem(set.id, &set.name));
        write_resource("RoseFootsteps", &path, set, &links)?;
        count += 1;
    }
    Ok((count, lost))
}

/// The effect file and attack effect folders, with their classes.
const EFFECT_DIRS: [(&str, &str); 2] = [(EFFECT_FILES_DIR, "RoseEffectFile"), (ATTACK_EFFECTS_DIR, "RoseAttackEffect")];

/// The fields of each exported thing that link an effect file and those that link an attack
/// effect (the same name can be either: a skill's `hit_effect` is a file, a cart part's an
/// attack effect).
type EffectFields = (&'static [&'static str], &'static [&'static str]);

/// The effect files and attack effects one exported thing (as JSON) links, the attack effects
/// with the thing's name.
fn effect_links(json: &Value, (files, attacks): EffectFields, used: &mut std::collections::BTreeSet<u16>, attack_names: &mut std::collections::BTreeMap<u16, String>) {
    let numbers = |key: &str| -> Vec<u16> {
        match json.get(key) {
            Some(Value::Number(n)) => n.as_u64().into_iter().map(|n| n as u16).collect(),
            Some(Value::Array(list)) => list.iter().filter_map(|v| v.as_u64()).map(|n| n as u16).collect(),
            _ => Vec::new(),
        }
    };
    used.extend(files.iter().flat_map(|key| numbers(key)).filter(|id| *id != 0));
    let name = json.get("name").and_then(|n| n.as_str()).unwrap_or("").trim().to_string();
    for id in attacks.iter().flat_map(|key| numbers(key)).filter(|id| *id != 0) {
        attack_names.entry(id).or_insert_with(|| name.clone());
    }
}

/// `--convert-data` for effects: one resource per FILE_EFFECT.STB row with a file (or that
/// something names) and per LIST_EFFECT.STB row with something in it (or that something
/// names), linking the imported effect scenes (`--import-rose` first) and the sound
/// resources. An attack effect is named after the first item, monster or skill that uses it.
/// Returns the count and the effects whose file the client data doesn't have.
pub fn convert_effects(force: bool) -> Result<(usize, Vec<String>), String> {
    let data = crate::data::get().ok_or("open the data first")?;
    let mut existing = Vec::new();
    for (dir, _) in EFFECT_DIRS {
        tres_files(dir, &mut existing);
    }
    if !existing.is_empty() && !force {
        return Err(format!(
            "res://data/ already has {} effect file or attack effect resources; they are the source now (use --convert-data=force to write them again from iROSE)",
            existing.len()
        ));
    }
    if crate::assets::find_stem("3ddata/effect/levelup_01", crate::effect::SCENE_EXTENSIONS).is_none() {
        return Err("the effects are not imported; run --import-rose first".into());
    }
    let effects = rose_data_irose::get_effect_database(&data.vfs).map_err(|e| format!("{e:#}"))?;
    let mut used = std::collections::BTreeSet::new();
    let mut attack_names = std::collections::BTreeMap::new();
    let mut add = |json: Result<Value, serde_json::Error>, fields: EffectFields| {
        if let Ok(json) = json {
            effect_links(&json, fields, &mut used, &mut attack_names);
        }
    };
    let items = ItemsExport::from_database(&data.items);
    for item in &items.weapon {
        add(serde_json::to_value(item), (&[], &["attack_effect", "bullet_effect"]));
    }
    for item in &items.gem {
        add(serde_json::to_value(item), (&[], &["gem_effect"]));
    }
    for item in &items.material {
        add(serde_json::to_value(item), (&[], &["bullet_effect"]));
    }
    for item in &items.consumable {
        add(serde_json::to_value(item), (&["effect"], &[]));
    }
    for item in &items.vehicle {
        add(
            serde_json::to_value(item),
            (&["ride_effect", "dismount_effect", "dead_effect", "move_effect", "dummy_effects"], &["hit_effect", "bullet_effect"]),
        );
    }
    for npc in data.npcs.iter() {
        add(serde_json::to_value(MonsterExport::from_data(npc)), (&["die_effect"], &["hand_hit_effect"]));
    }
    for skill in data.skills.iter() {
        add(
            serde_json::to_value(SkillExport::from_data(skill)),
            (
                &["casting_effect_1", "casting_effect_2", "casting_effect_3", "casting_effect_4", "hit_effect", "hit_dummy_effect_1", "hit_dummy_effect_2"],
                &["bullet_effect"],
            ),
        );
    }
    for effect in data.status_effects.iter() {
        add(serde_json::to_value(StatusEffectExport::from_data(effect)), (&["effect"], &[]));
    }
    let used_attacks = attack_names.keys().copied().collect();
    let mut export = EffectsExport::from_database(&effects, &used, &used_attacks);
    for effect in &mut export.attack_effects {
        effect.name = match attack_names.get(&effect.id) {
            Some(name) if !name.is_empty() => name.clone(),
            _ => format!("attack effect {}", effect.id),
        };
    }
    used.extend(export.attack_effects.iter().flat_map(|e| e.effect_files().collect::<Vec<_>>()));
    // FILE_EFFECT names some files the client doesn't have (or can't read): one something
    // links stays, empty; the others are left out.
    let mut lost = Vec::new();
    let mut left_out = 0;
    export.effect_files.retain_mut(|file| {
        if file.scene.is_empty() {
            return true;
        }
        let stem = crate::assets::stem(&file.scene);
        match crate::assets::find_stem(&stem, crate::effect::SCENE_EXTENSIONS) {
            Some(path) => {
                file.scene = path;
                true
            }
            None if used.contains(&file.id) => {
                lost.push(format!("effect {}: the client data has no readable {}, kept empty", file.id, file.scene));
                file.scene.clear();
                true
            }
            None => {
                left_out += 1;
                false
            }
        }
    });
    if left_out > 0 {
        lost.push(format!("{left_out} FILE_EFFECT rows nothing links name files the client data doesn't have; left out"));
    }
    let mut links = Links::new();
    for (dir, _) in SOUND_DIRS {
        index_links(dir, &mut links)?;
    }
    let mut count = 0;
    for file in &export.effect_files {
        let path = format!("{EFFECT_FILES_DIR}{}.tres", file_stem(file.id as u32, &file.name));
        write_resource("RoseEffectFile", &path, file, &links)?;
        links.insert(format!("effect_file/{}", file.id), path);
        count += 1;
    }
    for effect in &export.attack_effects {
        let path = format!("{ATTACK_EFFECTS_DIR}{}.tres", file_stem(effect.id as u32, &effect.name));
        write_resource("RoseAttackEffect", &path, effect, &links)?;
        count += 1;
    }
    Ok((count, lost))
}

/// Reads every effect file and attack effect resource into the export. Two of one kind with
/// the same number are an error naming both files.
pub fn read_effects() -> Result<(EffectsExport, usize), String> {
    let mut export = EffectsExport::default();
    let mut seen: std::collections::HashMap<(&str, u16), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    let mut count = 0;
    for (dir, _) in EFFECT_DIRS {
        register_uids(dir);
    }
    for (dir, _) in EFFECT_DIRS {
        let mut files = Vec::new();
        tres_files(dir, &mut files);
        for path in files {
            count += 1;
            let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
            let class = resource.get_class().to_string();
            let mut check = |kind: &'static str, id: u16| -> Result<(), String> {
                match seen.insert((kind, id), path.clone()) {
                    Some(other) => Err(format!("{path} and {other} are both {kind} {id}")),
                    None => Ok(()),
                }
            };
            let read = |e: String| format!("{path}: {e}");
            match class.as_str() {
                "RoseEffectFile" => {
                    let file: EffectFileExport = from_resource(&resource).map_err(read)?;
                    check("effect file", file.id)?;
                    export.effect_files.push(file);
                }
                "RoseAttackEffect" => {
                    let effect: AttackEffectExport = from_resource(&resource).map_err(read)?;
                    check("attack effect", effect.id)?;
                    export.attack_effects.push(effect);
                }
                _ => return Err(format!("{path} is a {class}, not an effect file or attack effect")),
            }
        }
    }
    export.check().map_err(|e| format!("{e:#}"))?;
    Ok((export, count))
}

/// Reads every sound, hit sound and footstep resource into the export. Two of one kind with
/// the same number are an error naming both files.
pub fn read_sounds() -> Result<(SoundsExport, usize), String> {
    let mut export = SoundsExport::default();
    let mut seen: std::collections::HashMap<(&str, u32), String> = std::collections::HashMap::new();
    let mut loader = ResourceLoader::singleton();
    let mut count = 0;
    for (dir, _) in SOUND_DIRS {
        register_uids(dir);
    }
    for (dir, _) in SOUND_DIRS {
        let mut files = Vec::new();
        tres_files(dir, &mut files);
        for path in files {
            count += 1;
            let resource = loader.load(&path).ok_or(format!("could not load {path}"))?;
            let class = resource.get_class().to_string();
            let mut check = |kind: &'static str, id: u32| -> Result<(), String> {
                match seen.insert((kind, id), path.clone()) {
                    Some(other) => Err(format!("{path} and {other} are both {kind} {id}")),
                    None => Ok(()),
                }
            };
            let read = |e: String| format!("{path}: {e}");
            match class.as_str() {
                "RoseSound" => {
                    let sound: SoundExport = from_resource(&resource).map_err(read)?;
                    check("sound", sound.id as u32)?;
                    export.sounds.push(sound);
                }
                "RoseHitSounds" => {
                    let set: HitSoundsExport = from_resource(&resource).map_err(read)?;
                    check("hit sounds", set.id)?;
                    export.hit_sounds.push(set);
                }
                "RoseFootsteps" => {
                    let set: FootstepsExport = from_resource(&resource).map_err(read)?;
                    check("footsteps", set.id)?;
                    export.footsteps.push(set);
                }
                _ => return Err(format!("{path} is a {class}, not a sound, hit sound or footstep set")),
            }
        }
    }
    export.check().map_err(|e| format!("{e:#}"))?;
    Ok((export, count))
}

/// `--export-data`: writes `export/` (in the repository) from the resources, the zone scenes
/// and the animations, or with `check` only compares. Needs the data open (the motion list and
/// the names in the zone files' comments come from it) and the imported animations. Returns
/// the files written or compared and those that differ.
pub fn export(check: bool) -> Result<ExportResult, String> {
    if crate::data::get().is_none() {
        return Err("open the data first".into());
    }
    let started = std::time::Instant::now();
    // Every kind links others (items link effects and sounds ...): the uid cache must know
    // them all before the first load, or each link loads the slow way.
    register_uids("res://data/");
    // Godot frees a loaded resource once nothing holds it, so without this every item and
    // monster would load its effects' scenes (and their textures) and sounds again.
    let mut loader = ResourceLoader::singleton();
    let mut shared = Vec::new();
    for (dir, _) in SOUND_DIRS.iter().chain(&EFFECT_DIRS) {
        tres_files(dir, &mut shared);
    }
    let _kept: Vec<Gd<Resource>> = shared.iter().filter_map(|path| loader.load(path)).collect();
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
    let (skills, skill_resources) = read_skills()?;
    if skill_resources == 0 {
        return Err(format!(
            "no skill resources in {SKILLS_DIR}; run --convert-data first"
        ));
    }
    let (world, world_resources) = read_world()?;
    if world_resources == 0 {
        return Err(format!(
            "no zone resources in {ZONES_DIR}; run --convert-data first"
        ));
    }
    let (sounds, sound_resources) = read_sounds()?;
    if sound_resources == 0 {
        return Err(format!(
            "no sound resources in {SOUNDS_DIR}; run --convert-data first"
        ));
    }
    let (effects, effect_resources) = read_effects()?;
    if effect_resources == 0 {
        return Err(format!(
            "no effect resources in {EFFECT_FILES_DIR}; run --convert-data first"
        ));
    }
    if let Some((effect, sound)) = effects
        .attack_effects
        .iter()
        .flat_map(|e| [(e.id, e.fire_sound), (e.id, e.hit_sound)])
        .find(|(_, sound)| !sounds.has_sound(*sound))
    {
        return Err(format!("attack effect {effect} plays sound {sound}, which isn't there"));
    }
    // A summon must name a monster that is there.
    let monsters: std::collections::HashSet<u16> = npcs.monsters.iter().map(|m| m.id).collect();
    if let Some(skill) = skills.skills.iter().find(|s| s.summon_monster != 0 && !monsters.contains(&s.summon_monster)) {
        return Err(format!("skill {} summons monster {}, which isn't there", skill.id, skill.summon_monster));
    }
    let read_ms = started.elapsed().as_millis();
    let project = ProjectSettings::singleton()
        .globalize_path("res://")
        .to_string();
    let dir = Path::new(&project).join("..");
    // Paths in the repository (`export/data/items/weapon.toml`) and their text.
    let in_data = |name: String| format!("{}/{name}", rose_data_export::DATA_DIR);
    let toml = rose_data_export::Format::Toml;
    let mut files: Vec<(String, String)> = items
        .to_files()
        .map_err(|e| format!("{e:#}"))?
        .into_iter()
        .map(|(name, text)| (in_data(rose_data_export::items_file(name)), text))
        .collect();
    let mut data_files = Vec::new();
    data_files.extend(npcs.to_files_as(toml).map_err(|e| format!("{e:#}"))?);
    data_files.extend(skills.to_files_as(toml).map_err(|e| format!("{e:#}"))?);
    data_files.extend(world.to_files_as(toml).map_err(|e| format!("{e:#}"))?);
    data_files.extend(sounds.to_files_as(toml).map_err(|e| format!("{e:#}"))?);
    data_files.extend(effects.to_files_as(toml).map_err(|e| format!("{e:#}"))?);
    files.extend(data_files.into_iter().map(|(name, text)| (in_data(rose_data_export::data_file_as(name, toml)), text)));
    // The written files must read back (the server and the game load them the same way).
    let find = |path: String| Ok(files.iter().find(|(n, _)| *n == in_data(path.clone())).map(|(_, t)| t.clone()));
    ItemsExport::from_files(|name| find(rose_data_export::items_file(name))).map_err(|e| format!("{e:#}"))?;
    NpcsExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the monster export doesn't read back")?
        .drop_rows(rose_game_data::encode_item_base1000)
        .map_err(|e| format!("{e:#}"))?;
    SkillsExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the skill export doesn't read back")?
        .check()
        .map_err(|e| format!("{e:#}"))?;
    let world_back = WorldExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the world export doesn't read back")?;
    rose_game_data::world_from_export(&world_back).map_err(|e| format!("{e:#}"))?;
    SoundsExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the sound export doesn't read back")?
        .to_database()
        .map_err(|e| format!("{e:#}"))?;
    EffectsExport::from_files_as(toml, |name| find(rose_data_export::data_file_as(name, toml)))
        .map_err(|e| format!("{e:#}"))?
        .ok_or("the effect export doesn't read back")?
        .to_database()
        .map_err(|e| format!("{e:#}"))?;

    // The zones edited as scenes need their settings, and their layouts go to the server.
    // Their comments name the zone, monsters and NPCs as the resources do now.
    let zone_names: std::collections::HashMap<u16, &str> = world.zones.iter().map(|z| (z.id, z.name.as_str())).collect();
    let npc_names: std::collections::HashMap<u32, String> = (npcs.monsters.iter().map(|m| (m.id as u32, m.name.clone())))
        .chain(npcs.npcs.iter().map(|n| (n.id as u32, n.name.clone())))
        .collect();
    let mut zone_scenes = 0;
    for (zone_id, folder) in crate::zone_scene::zone_folders() {
        let Some(zone_name) = zone_names.get(&zone_id) else {
            return Err(format!("{folder} is zone {zone_id}, which has no settings in {ZONES_DIR}"));
        };
        let layout = crate::zone_scene::zone_layout(zone_id)?;
        let name = format!("{}/{}", rose_data_export::ZONES_DIR, crate::zone_scene::layout_file_name(&folder));
        files.push((name, crate::zone_scene::layout_text(&layout, zone_name, &npc_names)));
        zone_scenes += 1;
    }
    // The motion timings, from the hit markers of the animations.
    let timings = crate::character::motion_timings().ok_or("open the data first")?;
    if timings.motion.is_empty() {
        return Err("no imported animations; run --import-rose first".into());
    }
    let motions = timings.motion.len();
    files.push((rose_data_export::MOTIONS_FILE.to_string(), timings.to_toml()));

    let mut result = ExportResult {
        resources: item_resources + npc_resources + skill_resources + world_resources + sound_resources + effect_resources,
        sounds: sounds.sounds.len(),
        effects: effects.effect_files.len(),
        attack_effects: effects.attack_effects.len(),
        items: items.count(),
        monsters: npcs.monsters.len(),
        npcs: npcs.npcs.len(),
        skills: skills.skills.len(),
        zones: world.zones.len(),
        zone_scenes,
        motions,
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
    pub skills: usize,
    pub zones: usize,
    pub sounds: usize,
    pub effects: usize,
    pub attack_effects: usize,
    pub zone_scenes: usize,
    pub motions: usize,
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
    /// the iROSE tables: sounds first (they link the imported audio), then effect files and
    /// attack effects (they link the imported effect scenes and the sounds), then items, then monsters, NPCs, drop tables and shop tabs, which
    /// link the items, then skills, status effects and job classes, which link the monsters,
    /// then zone settings, warp gates, recipes and new characters, which link the zones and
    /// items. Needs RoseData.open. Returns {resources, folder, skipped (kinds already there),
    /// lost (values that couldn't be kept)}, or {error}.
    #[func]
    fn convert(force: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        let mut count = 0;
        let mut kept = VarArray::new();
        let mut lost = VarArray::new();
        let mut others = Vec::new();
        for (dir, _) in SOUND_DIRS {
            tres_files(dir, &mut others);
        }
        if force || others.is_empty() {
            match convert_sounds(force) {
                Ok((n, not_kept)) => {
                    count += n;
                    for line in not_kept {
                        lost.push(&line.to_variant());
                    }
                }
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"sounds".to_variant());
        }
        let mut others = Vec::new();
        for (dir, _) in EFFECT_DIRS {
            tres_files(dir, &mut others);
        }
        if force || others.is_empty() {
            match convert_effects(force) {
                Ok((n, not_kept)) => {
                    count += n;
                    for line in not_kept {
                        lost.push(&line.to_variant());
                    }
                }
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"effects".to_variant());
        }
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
        let mut others = Vec::new();
        for (dir, _) in SKILL_DIRS {
            tres_files(dir, &mut others);
        }
        if force || others.is_empty() {
            match convert_skills(force) {
                Ok(n) => count += n,
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"skills, status effects and job classes".to_variant());
        }
        let mut others = Vec::new();
        for (dir, _) in WORLD_DIRS {
            tres_files(dir, &mut others);
        }
        if force || others.is_empty() {
            match convert_world(force) {
                Ok((n, not_kept)) => {
                    count += n;
                    for line in not_kept {
                        lost.push(&line.to_variant());
                    }
                }
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        } else {
            kept.push(&"zones, warp gates, recipes and new characters".to_variant());
        }
        let folder = ProjectSettings::singleton().globalize_path("res://data/").to_string();
        result.set("resources", count as i64);
        result.set("folder", folder.as_str());
        result.set("skipped", &kept);
        result.set("lost", &lost);
        result
    }

    /// Writes (or with `check` compares) `export/` from the resources, the zone scenes and the
    /// animations. Needs RoseData.open and the import. Returns {files, changed, resources,
    /// items, monsters, npcs, skills, zones, sounds, effects, attack_effects, zone_scenes,
    /// motions, read_ms}, or {error}.
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
                result.set("skills", out.skills as i64);
                result.set("zones", out.zones as i64);
                result.set("sounds", out.sounds as i64);
                result.set("effects", out.effects as i64);
                result.set("attack_effects", out.attack_effects as i64);
                result.set("zone_scenes", out.zone_scenes as i64);
                result.set("motions", out.motions as i64);
                result.set("read_ms", out.read_ms as i64);
            }
            Err(error) => result.set("error", error.as_str()),
        }
        result
    }
}
