//! The monster AI as Godot resources: `RoseAi` (one per AI in `res://data/ai/`, which monsters
//! and NPCs link as their `ai`) holding six lists of events (`RoseAiEvent`), each made of
//! blocks, one class per kind: `RoseAiIf<kind>` (conditions, 23) and `RoseAiDo<kind>` (actions,
//! 25), the kinds of iROSE's AIP files. Their properties are the export's keys
//! (`crates/rose-data-export/src/ai.rs`); `data_resources.rs` reads and writes them. The server
//! runs them (`module/src/monster_brain.rs` for monsters, `npc_ai.rs` for town NPCs).
//!
//! A block that names a monster, an NPC, a skill or a quest step stores its number or name
//! (`npc_id`, `skill_id`, `trigger_name`) and shows it in the Inspector as the resource
//! (`npc`, `skill`, `trigger`): a link would let a monster's AI link back to the monster
//! (through a skill or a quest step that summons it), which Godot can't load.

use godot::{
    classes::{IResource, Resource},
    prelude::*,
    register::info::{PropertyHint, PropertyInfo},
};
use rose_data_export::ai::{
    ABILITY_NAMES, ACTION_KINDS, ALLIES_NAMES, CHANGE_NAMES, CONDITION_KINDS, COUNT_COMPARE_NAMES, DAMAGE_TYPE_NAMES,
    DISTANCE_TO_NAMES, MESSAGE_TYPE_NAMES, MOVE_FROM_NAMES, MOVE_MODE_NAMES, PICK_NAMES, SKILL_TARGET_NAMES,
    SPAWN_AT_NAMES, SPAWN_STATE_NAMES, STATUS_KIND_NAMES, VARIABLE_TYPE_NAMES, WHO_NAMES,
};
use rose_data_export::quests::COMPARE_NAMES;

use crate::data_resources::{self, RoseSkill, RoseZoneSettings};
use crate::quest_blocks::RoseQuestTrigger;

/// The class name prefixes of the blocks.
pub const CONDITION_PREFIX: &str = "RoseAiIf";
pub const ACTION_PREFIX: &str = "RoseAiDo";

/// The Inspector's list of block classes for an array of conditions or actions.
fn block_classes(action: bool) -> String {
    let (prefix, kinds): (&str, &[&str]) =
        if action { (ACTION_PREFIX, &ACTION_KINDS) } else { (CONDITION_PREFIX, &CONDITION_KINDS) };
    kinds.iter().map(|kind| format!("{prefix}{kind}")).collect::<Vec<_>>().join(",")
}

/// An array property of resources of `classes` ("<object type>/<resource hint>:<classes>").
fn array_hint(property: &mut PropertyInfo, classes: &str) {
    property.hint_info.hint_string = GString::from(format!("24/17:{classes}").as_str());
}

/// The item classes an item link takes.
const ITEM_CLASSES: &str = "RoseItemFace,RoseItemHead,RoseItemBody,RoseItemHands,RoseItemFeet,RoseItemBack,RoseItemJewellery,RoseItemWeapon,RoseItemSubWeapon,RoseItemConsumable,RoseItemGem,RoseItemMaterial,RoseItemQuest,RoseItemVehicle";

/// The dropdowns of a block's text fields, and the classes its item links take. `action`: the
/// block is an action (`operator` then changes a value instead of comparing).
fn block_hint(property: &mut PropertyInfo, action: bool) {
    let name = property.property_name.to_string();
    if property.variant_type == VariantType::OBJECT && name == "item" {
        property.hint_info.hint_string = GString::from(ITEM_CLASSES);
        return;
    }
    if property.variant_type == VariantType::ARRAY && name == "items" {
        array_hint(property, ITEM_CLASSES);
        return;
    }
    if property.variant_type != VariantType::STRING {
        return;
    }
    let options: &[&str] = match name.as_str() {
        "operator" if action => CHANGE_NAMES,
        "operator" => &COMPARE_NAMES,
        "count_operator" => &COUNT_COMPARE_NAMES,
        "ability" => ABILITY_NAMES,
        "variable_type" => VARIABLE_TYPE_NAMES,
        "to" => DISTANCE_TO_NAMES,
        "who" => WHO_NAMES,
        "status" => STATUS_KIND_NAMES,
        "damage_type" => DAMAGE_TYPE_NAMES,
        "from" => MOVE_FROM_NAMES,
        "mode" => MOVE_MODE_NAMES,
        "pick" => PICK_NAMES,
        "at" => SPAWN_AT_NAMES,
        "target" => SKILL_TARGET_NAMES,
        "message_type" => MESSAGE_TYPE_NAMES,
        "state" => SPAWN_STATE_NAMES,
        "allies" => &ALLIES_NAMES,
        _ => return,
    };
    property.hint_info.hint = PropertyHint::ENUM;
    property.hint_info.hint_string = GString::from(options.join(",").as_str());
}

/// The monster (or town NPC) with this number, for the Inspector.
fn npc_resource(id: i64) -> Option<Gd<Resource>> {
    data_resources::find(data_resources::MONSTERS_DIR, id)
        .or_else(|| data_resources::find(data_resources::NPCS_DIR, id))
        .map(|(resource, _)| resource)
}

/// The number a monster or NPC set in the Inspector stands for; `None` keeps the number
/// there (cleared, but it names one that isn't made in Godot).
fn npc_number(npc: Option<Gd<Resource>>, current: i64) -> i64 {
    match npc {
        Some(npc) => npc.get("id").try_to::<i64>().unwrap_or(0),
        None if npc_resource(current).is_none() => current,
        None => 0,
    }
}

/// An AI: how a kind of monster (or town NPC) behaves. Monsters and NPCs link it as their `ai`;
/// several can share one. The server asks it what to do at six moments, each a list of events:
/// it runs the **first** event of the list whose conditions all pass (from the top), and that
/// event's actions, from the top. Edit it here, then run `--export-data`; the wiki page
/// "Monster AI" has the rules and what the server runs so far.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAi {
    /// The number (iROSE's FILE_AI.STB row). Keep it once monsters use it; a new AI takes any
    /// free number.
    #[export]
    id: i64,
    /// What it is called here (iROSE's AIP file name, such as `de_zs`).
    #[export]
    name: GString,
    /// Seconds between two runs of `on_idle`.
    #[export(range = (1.0, 600.0, 1.0, or_greater, suffix = "s"))]
    #[init(val = 4)]
    idle_interval: i64,
    /// When it is already fighting and someone else hits it: the chance in percent that it runs
    /// `on_damaged` (and so may turn on them).
    #[export(range = (0.0, 100.0, 1.0, suffix = "%"))]
    new_target_chance: i64,
    /// When it appears (`RoseAiEvent`s).
    #[export]
    on_created: Array<Option<Gd<Resource>>>,
    /// Every `idle_interval` while it stands with nothing to fight: wander, look for a player.
    #[export]
    on_idle: Array<Option<Gd<Resource>>>,
    /// After each of its swings: the server only runs the events here that use a skill.
    #[export]
    on_attack: Array<Option<Gd<Resource>>>,
    /// When it is hit and lives: fight back, call friends, run. With no events here it doesn't
    /// fight back.
    #[export]
    on_damaged: Array<Option<Gd<Resource>>>,
    /// When it killed its target (not run by the server yet).
    #[export]
    on_kill: Array<Option<Gd<Resource>>>,
    /// When it dies (not run by the server yet).
    #[export]
    on_dead: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseAi {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        if property.property_name.to_string().starts_with("on_") {
            array_hint(property, "RoseAiEvent");
        }
    }
}

/// What an event shows as in the Inspector's lists: its blocks' kinds (`If Random, HealthPercent:
/// UseSkill`, `Always: MoveRandomly`), after its label if it has one.
pub fn event_label(name: &str, conditions: &[String], actions: &[String]) -> String {
    let short = |kinds: &[String]| if kinds.is_empty() { "-".to_string() } else { kinds.join(", ") };
    let label = if conditions.is_empty() {
        format!("Always: {}", short(actions))
    } else {
        format!("If {}: {}", short(conditions), short(actions))
    };
    let label: String = if label.chars().count() > 72 { format!("{}…", label.chars().take(72).collect::<String>()) } else { label };
    if name.trim().is_empty() {
        label
    } else {
        format!("{} ({label})", name.trim())
    }
}

/// One event of an AI (`RoseAi`): when every condition passes (from the top), its actions run
/// (from the top) and the rest of the list is skipped. No conditions: it always runs.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAiEvent {
    /// A label (not used by the game).
    #[export]
    #[var(get = get_name, set = set_name)]
    name: GString,
    /// What must all be true (`RoseAiIf...` blocks).
    #[export]
    #[var(get = get_conditions, set = set_conditions)]
    conditions: Array<Option<Gd<Resource>>>,
    /// What it does (`RoseAiDo...` blocks).
    #[export]
    #[var(get = get_actions, set = set_actions)]
    actions: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

/// Changing the name or the blocks renames the event, so the AI's lists show it as it is.
#[godot_api]
impl RoseAiEvent {
    #[func]
    fn get_name(&self) -> GString {
        self.name.clone()
    }

    #[func]
    fn set_name(&mut self, name: GString) {
        self.name = name;
        self.relabel();
    }

    #[func]
    fn get_conditions(&self) -> Array<Option<Gd<Resource>>> {
        self.conditions.clone()
    }

    #[func]
    fn set_conditions(&mut self, conditions: Array<Option<Gd<Resource>>>) {
        self.conditions = conditions;
        self.relabel();
    }

    #[func]
    fn get_actions(&self) -> Array<Option<Gd<Resource>>> {
        self.actions.clone()
    }

    #[func]
    fn set_actions(&mut self, actions: Array<Option<Gd<Resource>>>) {
        self.actions = actions;
        self.relabel();
    }

    fn relabel(&mut self) {
        let kinds = |blocks: &Array<Option<Gd<Resource>>>, prefix: &str| -> Vec<String> {
            blocks
                .iter_shared()
                .flatten()
                .map(|block| block.get_class().to_string().trim_start_matches(prefix).to_string())
                .collect()
        };
        let label = event_label(
            &self.name.to_string(),
            &kinds(&self.conditions, CONDITION_PREFIX),
            &kinds(&self.actions, ACTION_PREFIX),
        );
        let label = GString::from(label.as_str());
        if self.base().get_name() != label {
            self.base_mut().set_name(&label);
        }
    }
}

#[godot_api]
impl IResource for RoseAiEvent {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        match property.property_name.to_string().as_str() {
            "conditions" => array_hint(property, &block_classes(false)),
            "actions" => array_hint(property, &block_classes(true)),
            _ => {}
        }
    }
}

macro_rules! ai_block {
    ($action:literal, $(#[$doc:meta])* $name:ident { $($(#[$fdoc:meta])* $field:ident: $ty:ty $(= $init:expr)?,)* }) => {
        $(#[$doc])*
        #[derive(GodotClass)]
        #[class(tool, base=Resource, init)]
        pub struct $name {
            $(
                $(#[$fdoc])*
                #[export]
                $(#[init(val = $init)])?
                $field: $ty,
            )*
            base: Base<Resource>,
        }

        #[godot_api]
        impl IResource for $name {
            fn on_validate_property(&self, property: &mut PropertyInfo) {
                block_hint(property, $action);
            }
        }
    };
}

/// A block naming a monster or NPC by number, shown as the resource (`npc`).
macro_rules! npc_block {
    ($action:literal, $(#[$doc:meta])* $name:ident { $($(#[$fdoc:meta])* $field:ident: $ty:ty $(= $init:expr)?,)* }) => {
        $(#[$doc])*
        #[derive(GodotClass)]
        #[class(tool, base=Resource, init)]
        pub struct $name {
            /// The monster or NPC (drag a RoseMonster or RoseNpc in). Stored as its number
            /// (`npc_id`).
            #[var(get = get_npc, set = set_npc, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseMonster,RoseNpc")]
            npc: PhantomVar<Option<Gd<Resource>>>,
            /// The monster's or NPC's number, as the file stores it.
            #[export(storage)]
            npc_id: i64,
            $(
                $(#[$fdoc])*
                #[export]
                $(#[init(val = $init)])?
                $field: $ty,
            )*
            base: Base<Resource>,
        }

        #[godot_api]
        impl $name {
            #[func]
            fn get_npc(&self) -> Option<Gd<Resource>> {
                npc_resource(self.npc_id)
            }

            #[func]
            fn set_npc(&mut self, npc: Option<Gd<Resource>>) {
                self.npc_id = npc_number(npc, self.npc_id);
            }
        }

        #[godot_api]
        impl IResource for $name {
            fn on_validate_property(&self, property: &mut PropertyInfo) {
                block_hint(property, $action);
            }
        }
    };
}

ai_block!(false,
    /// The attacker's value compared with its current target's (attacker on the left). A
    /// condition of an AI event (`RoseAiEvent`).
    RoseAiIfAttackerVsTarget {
        operator: GString = GString::from(">"),
        ability: GString = GString::from("Level"),
    }
);

ai_block!(false,
    /// Looks for characters around it and remembers one: the found character (`AttackFound`,
    /// `MoveRandomly` from it, `UseSkill` at it) and the nearest one (`AttackNearest`). Passes
    /// when it found one. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfFindCharacter {
        /// How far it looks, in centimetres.
        distance: i64 = 1000,
        /// On: other monsters. Off: players and their summons (not hidden ones).
        allied: bool,
        /// Their level, as its own level minus theirs: from this ...
        from_level_diff: i64 = -100,
        /// ... to this (both included).
        to_level_diff: i64 = 100,
        /// `Nth`: passes at the `count`-th one it finds and remembers that one. The others
        /// compare how many it found with `count` and remember the nearest.
        count_operator: GString = GString::from("Nth"),
        count: i64 = 1,
    }
);

ai_block!(false,
    /// The damage of the hit it just took (`Received`) compares to a number; `Given` (damage it
    /// dealt) never passes yet. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfDamage {
        damage_type: GString = GString::from("Received"),
        operator: GString = GString::from(">="),
        value: i64,
    }
);

ai_block!(false,
    /// How far it is from its spawn point or its target, in centimetres, compares to a number
    /// (`Owner` never passes: monsters have no owner yet). A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfDistance {
        to: GString = GString::from("Spawn"),
        operator: GString = GString::from(">="),
        distance: i64 = 1000,
    }
);

ai_block!(false,
    /// It has no owner (always true: monsters have no owner yet). A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfHasNoOwner {}
);

ai_block!(false,
    /// It (`Me`) or its target has a good (buff), bad (debuff) or any status effect (`has` on),
    /// or none (off). A condition of an AI event (`RoseAiEvent`).
    RoseAiIfHasStatusEffect {
        who: GString = GString::from("Me"),
        status: GString = GString::from("Any"),
        has: bool = true,
    }
);

ai_block!(false,
    /// Its health in percent compares to a number. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfHealthPercent {
        operator: GString = GString::from("<="),
        value: i64 = 50,
    }
);

ai_block!(false,
    /// The attacker leads a clan (never passes: clans aren't in the game yet). A condition of
    /// an AI event (`RoseAiEvent`).
    RoseAiIfAttackerIsClanMaster {}
);

ai_block!(false,
    /// The one who hit it is its current target. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfAttackerIsTarget {}
);

ai_block!(false,
    /// It is day in its zone (`day` on) or night (off). A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfDaytime {
        day: bool = true,
    }
);

ai_block!(false,
    /// Its target leads a clan (never passes: clans aren't in the game yet). A condition of an
    /// AI event (`RoseAiEvent`).
    RoseAiIfTargetIsClanMaster {}
);

ai_block!(false,
    /// The real date and time: a day of the month (0 = any) between two times, in minutes from
    /// midnight. Town NPCs only (monsters never pass it). A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfMonthDay {
        month_day: i64,
        from_minute: i64,
        to_minute: i64 = 1439,
    }
);

ai_block!(false,
    /// It has no target yet, and the attacker's value compares to a number. A condition of an
    /// AI event (`RoseAiEvent`).
    RoseAiIfNoTargetAttackerValue {
        operator: GString = GString::from(">="),
        ability: GString = GString::from("Level"),
        value: i64,
    }
);

ai_block!(false,
    /// Its owner has a target (never passes: monsters have no owner yet). A condition of an AI
    /// event (`RoseAiEvent`).
    RoseAiIfOwnerHasTarget {}
);

ai_block!(false,
    /// A random number from `from` (included) to `to` (not) compares to `value`: `< 30` from 0
    /// to 100 passes 30 % of the time. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfRandom {
        operator: GString = GString::from("<"),
        from: i64,
        to: i64 = 100,
        value: i64 = 50,
    }
);

npc_block!(false,
    /// Picks the town NPC (in its zone, else anywhere) whose variables the next `Variable`
    /// condition or `SetVariable` action of type `SelectedNpc` use; passes when there is one.
    /// Town NPCs only. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfSelectNpc {}
);

ai_block!(false,
    /// Its own value compares to a number. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfOwnValue {
        operator: GString = GString::from(">="),
        ability: GString = GString::from("Level"),
        value: i64,
    }
);

ai_block!(false,
    /// The server's channel is from `from` to `to` (the game has one channel, number 1). Town
    /// NPCs only. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfServerChannel {
        from: i64 = 1,
        to: i64 = 1,
    }
);

ai_block!(false,
    /// Its target's value compares to a number. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfTargetValue {
        operator: GString = GString::from(">="),
        ability: GString = GString::from("Level"),
        value: i64,
    }
);

ai_block!(false,
    /// A variable (of the NPC `SelectNpc` picked, or its own) compares to a number; World and
    /// Economy variables aren't kept (they read 0). Town NPCs only. A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfVariable {
        variable_type: GString = GString::from("SelectedNpc"),
        /// Which one (from 0).
        variable: i64,
        operator: GString = GString::from("=="),
        value: i64,
    }
);

ai_block!(false,
    /// The real day of the week (0 = Sunday) between two times, in minutes from midnight. Town
    /// NPCs only. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfWeekDay {
        week_day: i64,
        from_minute: i64,
        to_minute: i64 = 1439,
    }
);

ai_block!(false,
    /// The world's time (in world ticks) is from `from` to `to`. A condition of an AI event
    /// (`RoseAiEvent`).
    RoseAiIfWorldTime {
        from: i64,
        to: i64,
    }
);

ai_block!(false,
    /// The time of day in its zone (in ticks since the zone's day began) is from `from` to
    /// `to`. A condition of an AI event (`RoseAiEvent`).
    RoseAiIfZoneTime {
        from: i64,
        to: i64,
    }
);

ai_block!(true,
    /// Stops where it is. An action of an AI event (`RoseAiEvent`).
    RoseAiDoStop {}
);

ai_block!(true,
    /// Plays an emote motion (not shown yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoEmote {
        motion: i64,
    }
);

ai_block!(true,
    /// Says something over its head (not shown yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoSay {
        text: GString,
    }
);

ai_block!(true,
    /// Walks or runs to a random spot up to `distance` centimetres from its spawn point, where
    /// it stands, or the character `FindCharacter` found (never past its leash). An action of
    /// an AI event (`RoseAiEvent`).
    RoseAiDoMoveRandomly {
        from: GString = GString::from("Spawn"),
        mode: GString = GString::from("Walk"),
        distance: i64 = 500,
    }
);

ai_block!(true,
    /// Attacks the player (or player's summon) within `distance` centimetres with the lowest
    /// or highest value. An action of an AI event (`RoseAiEvent`).
    RoseAiDoAttackByValue {
        distance: i64 = 1000,
        ability: GString = GString::from("HealthPoints"),
        pick: GString = GString::from("Lowest"),
    }
);

ai_block!(true,
    /// A special attack (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoSpecialAttack {}
);

ai_block!(true,
    /// Walks or runs `distance` centimetres away from its target. An action of an AI event
    /// (`RoseAiEvent`).
    RoseAiDoMoveAway {
        mode: GString = GString::from("Run"),
        distance: i64 = 500,
    }
);

npc_block!(true,
    /// Turns into another monster (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoTransform {}
);

npc_block!(true,
    /// Spawns a monster `distance` centimetres from it, its attacker or its target, as its own
    /// (`owner`) or not (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoSpawn {
        distance: i64 = 150,
        at: GString = GString::from("Here"),
        owner: bool,
    }
);

/// Monsters near it that fight no one attack its target: any monster, those of one kind
/// (`OfMonster`, drag it into `npc`), or those of its own kind. An action of an AI event
/// (`RoseAiEvent`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAiDoCallAllies {
    /// How far it calls, in **metres** (as in iROSE).
    #[export]
    #[init(val = 10)]
    distance: i64,
    #[export]
    #[init(val = GString::from("SameMonster"))]
    allies: GString,
    /// For `OfMonster`: the monster kind (drag a RoseMonster in). Stored as its number
    /// (`npc_id`).
    #[var(get = get_npc, set = set_npc, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseMonster,RoseNpc")]
    npc: PhantomVar<Option<Gd<Resource>>>,
    /// The monster's number, as the file stores it.
    #[export(storage)]
    npc_id: i64,
    /// At most this many (-1 = all).
    #[export]
    #[init(val = -1)]
    count: i64,
    base: Base<Resource>,
}

#[godot_api]
impl RoseAiDoCallAllies {
    #[func]
    fn get_npc(&self) -> Option<Gd<Resource>> {
        npc_resource(self.npc_id)
    }

    #[func]
    fn set_npc(&mut self, npc: Option<Gd<Resource>>) {
        self.npc_id = npc_number(npc, self.npc_id);
    }
}

#[godot_api]
impl IResource for RoseAiDoCallAllies {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

ai_block!(true,
    /// Attacks the nearest character `FindCharacter` saw. An action of an AI event
    /// (`RoseAiEvent`).
    RoseAiDoAttackNearest {}
);

ai_block!(true,
    /// Attacks the character `FindCharacter` found. An action of an AI event (`RoseAiEvent`).
    RoseAiDoAttackFound {}
);

ai_block!(true,
    /// Attacks whoever hit it. An action of an AI event (`RoseAiEvent`).
    RoseAiDoAttackAttacker {}
);

ai_block!(true,
    /// Runs away `distance` centimetres (not run yet). An action of an AI event
    /// (`RoseAiEvent`).
    RoseAiDoRunAway {
        distance: i64 = 1000,
    }
);

ai_block!(true,
    /// Drops one of `items`, picked from five places: an empty place (or one past the end of a
    /// shorter list) drops nothing (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoDropRandomItem {
        /// Drag item resources in.
        items: Array<Option<Gd<Resource>>>,
    }
);

ai_block!(true,
    /// Dies on the spot (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoKillSelf {}
);

/// Casts a skill (it pays nothing) at the character `FindCharacter` found, its target, itself
/// or the nearest character, with one of its motions. An action of an AI event
/// (`RoseAiEvent`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAiDoUseSkill {
    /// The skill (drag a RoseSkill in: the level's own resource). Stored as its number
    /// (`skill_id`).
    #[var(get = get_skill, set = set_skill, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseSkill")]
    skill: PhantomVar<Option<Gd<RoseSkill>>>,
    /// The skill's number, as the file stores it.
    #[export(storage)]
    skill_id: i64,
    #[export]
    #[init(val = GString::from("Target"))]
    target: GString,
    /// The motion it casts with (its model's motion number).
    #[export]
    motion: i64,
    base: Base<Resource>,
}

#[godot_api]
impl RoseAiDoUseSkill {
    #[func]
    fn get_skill(&self) -> Option<Gd<RoseSkill>> {
        data_resources::find(data_resources::SKILLS_DIR, self.skill_id).and_then(|(r, _)| r.try_cast::<RoseSkill>().ok())
    }

    #[func]
    fn set_skill(&mut self, skill: Option<Gd<RoseSkill>>) {
        match skill {
            Some(skill) => self.skill_id = skill.upcast::<Resource>().get("id").try_to::<i64>().unwrap_or(0),
            // Cleared in the Inspector: keep a number that has no resource.
            None if self.get_skill().is_none() => {}
            None => self.skill_id = 0,
        }
    }
}

#[godot_api]
impl IResource for RoseAiDoUseSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

ai_block!(true,
    /// Changes a variable (of the NPC `SelectNpc` picked, or its own; World and Economy
    /// variables aren't kept). An NPC's variables stay from 0 to 500. Town NPCs only. An action
    /// of an AI event (`RoseAiEvent`).
    RoseAiDoSetVariable {
        variable_type: GString = GString::from("SelectedNpc"),
        /// Which one (from 0).
        variable: i64,
        operator: GString = GString::from("Set"),
        value: i64,
    }
);

ai_block!(true,
    /// Says something in the chat: to players nearby (`Say`), or to the whole zone. Town NPCs
    /// only. An action of an AI event (`RoseAiEvent`).
    RoseAiDoMessage {
        message_type: GString = GString::from("Shout"),
        text: GString,
    }
);

ai_block!(true,
    /// Walks back to its owner (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoMoveNearOwner {}
);

/// Runs a quest step on itself (only its NPC parts: picking an NPC, its variables, messages,
/// a random roll and the next step). Town NPCs only. An action of an AI event (`RoseAiEvent`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseAiDoQuestStep {
    /// The step (drag a RoseQuestTrigger in from `res://data/quest_triggers/`). Stored as its
    /// name (`trigger_name`).
    #[var(get = get_trigger, set = set_trigger, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseQuestTrigger")]
    trigger: PhantomVar<Option<Gd<RoseQuestTrigger>>>,
    /// The step's name, as the file stores it.
    #[export(storage)]
    trigger_name: GString,
    base: Base<Resource>,
}

#[godot_api]
impl RoseAiDoQuestStep {
    #[func]
    fn get_trigger(&self) -> Option<Gd<RoseQuestTrigger>> {
        let path = data_resources::find_quest_trigger(&self.trigger_name.to_string())?;
        godot::classes::ResourceLoader::singleton().load(&path)?.try_cast::<RoseQuestTrigger>().ok()
    }

    #[func]
    fn set_trigger(&mut self, trigger: Option<Gd<RoseQuestTrigger>>) {
        match trigger {
            Some(trigger) => self.trigger_name = trigger.upcast::<Resource>().get("name").to::<GString>(),
            // Cleared in the Inspector: keep a name that has no resource.
            None if self.get_trigger().is_none() => {}
            None => self.trigger_name = GString::new(),
        }
    }
}

#[godot_api]
impl IResource for RoseAiDoQuestStep {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

ai_block!(true,
    /// Attacks its owner's target (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoAttackOwnerTarget {}
);

ai_block!(true,
    /// Turns PvP on or off in a zone (empty: its own) (not run yet). An action of an AI event
    /// (`RoseAiEvent`).
    RoseAiDoSetPvp {
        /// The zone (drag a RoseZoneSettings in; empty = its own).
        zone: Option<Gd<RoseZoneSettings>>,
        on: bool,
    }
);

ai_block!(true,
    /// Turns a zone's monster spawns off, on, or the other way round (empty: its own zone) (not
    /// run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoSetMonsterSpawns {
        /// The zone (drag a RoseZoneSettings in; empty = its own).
        zone: Option<Gd<RoseZoneSettings>>,
        state: GString = GString::from("Enabled"),
    }
);

ai_block!(true,
    /// Gives its owner an item (not run yet). An action of an AI event (`RoseAiEvent`).
    RoseAiDoGiveItemToOwner {
        /// Drag an item resource in.
        item: Option<Gd<Resource>>,
        count: i64 = 1,
    }
);
