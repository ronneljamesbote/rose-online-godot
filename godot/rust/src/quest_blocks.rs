//! The quests as Godot resources: `RoseQuest` (a quest's journal text, one per quest in
//! `res://data/quests/`), `RoseQuestTrigger` (a step: conditions and rewards, one per trigger in
//! `res://data/quest_triggers/<QSD file>/`) and the blocks a trigger is made of, one class per
//! kind: `RoseCondition<kind>` (31) and `RoseReward<kind>` (40), the kinds of iROSE's QSD files
//! that the quest engine (`crates/rose-quest`) runs. Their properties are the export's keys
//! (`crates/rose-data-export/src/quests.rs`); `data_resources.rs` reads and writes them.

use godot::{
    classes::{IResource, Resource},
    prelude::*,
    register::info::{PropertyHint, PropertyInfo},
};
use rose_data::AbilityType;
use rose_data_export::quests::{
    CHANGE_NAMES, COMPARE_NAMES, CONDITION_KINDS, MESSAGE_TYPE_NAMES, OBJECT_NAMES, REWARD_KINDS, SPAWN_AT_NAMES,
    TEAM_SOURCE_NAMES, VARIABLE_TYPE_NAMES,
};

use crate::data_resources::{RoseSkill, RoseZoneSettings};

/// The class name prefixes of the blocks.
pub const CONDITION_PREFIX: &str = "RoseCondition";
pub const REWARD_PREFIX: &str = "RoseReward";

/// The Inspector's list of block classes for an array of conditions or rewards.
fn block_classes(reward: bool) -> String {
    let (prefix, kinds): (&str, &[&str]) =
        if reward { (REWARD_PREFIX, &REWARD_KINDS) } else { (CONDITION_PREFIX, &CONDITION_KINDS) };
    kinds.iter().map(|kind| format!("{prefix}{kind}")).collect::<Vec<_>>().join(",")
}

/// The item classes an item link takes.
const ITEM_CLASSES: &str = "RoseItemFace,RoseItemHead,RoseItemBody,RoseItemHands,RoseItemFeet,RoseItemBack,RoseItemJewellery,RoseItemWeapon,RoseItemSubWeapon,RoseItemConsumable,RoseItemGem,RoseItemMaterial,RoseItemQuest,RoseItemVehicle";

/// The dropdowns of a block's text fields, and the classes its item and NPC links take.
fn block_hint(property: &mut PropertyInfo, reward: bool) {
    let name = property.property_name.to_string();
    if property.variant_type == VariantType::OBJECT || property.variant_type == VariantType::ARRAY {
        let classes = match name.as_str() {
            "item" => ITEM_CLASSES,
            "npc" | "npc_1" | "npc_2" => "RoseNpc,RoseMonster",
            "monster" => "RoseMonster",
            _ => return,
        };
        property.hint_info.hint_string = if property.variant_type == VariantType::ARRAY {
            // An array of resources: "<object type>/<resource hint>:<classes>".
            GString::from(format!("24/17:{classes}").as_str())
        } else {
            GString::from(classes)
        };
        return;
    }
    if property.variant_type != VariantType::STRING {
        return;
    }
    let options: Vec<String> = match name.as_str() {
        "operator" if reward => CHANGE_NAMES.iter().map(|n| n.to_string()).collect(),
        "operator" | "level_operator" => COMPARE_NAMES.iter().map(|n| n.to_string()).collect(),
        "variable_type" => VARIABLE_TYPE_NAMES.iter().map(|n| n.to_string()).collect(),
        "object" => OBJECT_NAMES.iter().map(|n| n.to_string()).collect(),
        "message_type" => MESSAGE_TYPE_NAMES.iter().map(|n| n.to_string()).collect(),
        "location" => SPAWN_AT_NAMES.iter().map(|n| n.to_string()).collect(),
        "source" => TEAM_SOURCE_NAMES.iter().map(|n| n.to_string()).collect(),
        "ability" => (0..<AbilityType as enum_map::Enum>::LENGTH)
            .map(|i| format!("{:?}", <AbilityType as enum_map::Enum>::from_usize(i)))
            .collect(),
        _ => return,
    };
    property.hint_info.hint = PropertyHint::ENUM;
    property.hint_info.hint_string = GString::from(options.join(",").as_str());
}

/// A quest: what the quest journal shows (press Q in the game). Its steps are the quest
/// triggers (`RoseQuestTrigger`) whose blocks name it. Edit it here, then run `--export-data`
/// (see `godot/data/README.md`). How quests work is on the wiki page "Quests".
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseQuest {
    /// The number (iROSE's LIST_QUEST.STB row). Saved characters store it, so don't change it
    /// once it is in use. A new quest takes any free number.
    #[export]
    id: i64,
    /// The name in the journal.
    #[export]
    name: GString,
    /// The journal text: what to do.
    #[export(multiline)]
    description: GString,
    /// What the journal says while the quest is under way (often empty).
    #[export(multiline)]
    start_message: GString,
    /// What the journal says once it is done (often empty).
    #[export(multiline)]
    end_message: GString,
    /// How long the player has to finish it (0 = no limit). The game counts in steps of 10 s.
    #[export(range = (0.0, 86400.0, 10.0, or_greater, suffix = "s"))]
    time_limit: i64,
    base: Base<Resource>,
}

/// A quest trigger: one step of a quest. When every condition passes (from the top), it gives
/// every reward (from the top); when one fails, it tries `next_trigger` instead, if it has one.
/// NPC dialogs, monsters (their `death_quest_trigger_name`), level-ups and other triggers run it
/// by its `name`. The game checks the conditions first, then the server checks them again and
/// gives the rewards. Add a block to `conditions` or `rewards` with the arrow of a new element
/// ("New RoseCondition..."). Edit it here, then run `--export-data`; the wiki page "Quests" has
/// the rules.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseQuestTrigger {
    /// What dialogs, monsters and other triggers call it (`124-01`: quest 124, step 1). Keep
    /// it unique; the file is named after it.
    #[export]
    name: GString,
    /// What must all be true, checked from the top (`RoseCondition...` blocks).
    #[export]
    conditions: Array<Option<Gd<Resource>>>,
    /// What it gives or does, from the top (`RoseReward...` blocks).
    #[export]
    rewards: Array<Option<Gd<Resource>>>,
    /// The trigger tried when a condition fails (empty = none): the next way the step can go.
    #[export]
    next_trigger: Option<Gd<RoseQuestTrigger>>,
    /// Where its box sits in the Graphs screen, once you have moved it there ((0, 0): the
    /// graph places it). Only for the editor; the game and `--export-data` don't use it.
    #[export]
    graph_position: Vector2,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseQuestTrigger {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        let name = property.property_name.to_string();
        if name == "conditions" || name == "rewards" {
            property.hint_info.hint_string = GString::from(format!("24/17:{}", block_classes(name == "rewards")).as_str());
        }
    }
}

/// The player has this quest in their journal. The trigger then works on it: its variables,
/// switches and items. A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionSelectQuest {
    /// The quest (drag a RoseQuest in).
    #[export]
    quest: Option<Gd<RoseQuest>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionSelectQuest {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// A quest value compares to a number. A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionQuestVariable {
    /// Which kind of quest value: the selected quest's own variables, switches or timer, or the
    /// ones all quests share (Episode, Job, Planet, Union).
    #[export]
    #[init(val = GString::from("Variable"))]
    variable_type: GString,
    /// Which one (from 0).
    #[export]
    variable: i64,
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionQuestVariable {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// One of the player's values (level, job, a stat, money ...) compares to a number. A condition
/// of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionAbilityValue {
    /// Which value.
    #[export]
    #[init(val = GString::from("Level"))]
    ability: GString,
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionAbilityValue {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player carries a number of an item (a quest item counts in the selected quest's own item
/// list). With `equipment_index` set to a worn slot (1 to 8), the player wears the item there
/// instead (1 to 11); no item means the slot is empty. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionQuestItem {
    /// The item (drag an item resource in).
    #[export]
    item: Option<Gd<Resource>>,
    /// iROSE's slot number: 0 = carried; 1 face, 2 head, 3 body, 4 back, 5 hands, 6 feet, 7 weapon,
    /// 8 shield, 9 necklace, 10 ring, 11 earring. Other numbers count as carried.
    #[export]
    equipment_index: i64,
    /// How many.
    #[export]
    count: i64,
    /// How to compare: what the player carries on the left, `count` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionQuestItem {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player is in a party (as its leader if `leader`), whose level compares to a number. A
/// condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionParty {
    /// The player must lead the party.
    #[export]
    leader: bool,
    /// How to compare the party level with `level`.
    #[export]
    #[init(val = GString::from("=="))]
    level_operator: GString,
    /// The party level to compare with.
    #[export]
    level: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionParty {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player is in a zone, within a distance of a spot. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionPosition {
    /// The zone (drag its RoseZoneSettings in).
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// The spot, east-west.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    x: f64,
    /// The spot, north-south.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    y: f64,
    /// How near.
    #[export(range = (0.0, 100000.0, 100.0, or_greater, suffix = "cm"))]
    distance: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionPosition {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The world's time is from `from` to `to` (both included). A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionWorldTime {
    /// From (world time).
    #[export]
    from: i64,
    /// To (world time).
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionWorldTime {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player knows a skill (or, with `has` off, does not). A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionHasSkill {
    /// The skill (drag a RoseSkill in).
    #[export]
    skill: Option<Gd<RoseSkill>>,
    /// On: must know it. Off: must not.
    #[export]
    #[init(val = true)]
    has: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionHasSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player knows a skill numbered from `from_skill` to `to_skill` (or, with `has` off, none
/// of them). A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionHasSkillInRange {
    /// The first skill number.
    #[export]
    from_skill: i64,
    /// The last skill number.
    #[export]
    to_skill: i64,
    /// On: must know one. Off: must know none.
    #[export]
    #[init(val = true)]
    has: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionHasSkillInRange {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// A random number from 0 to 99 lands from `from` to `to` (both included). A condition of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionRandomPercent {
    /// From.
    #[export]
    from: i64,
    /// To.
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionRandomPercent {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// A variable of the selected NPC, event object or the player compares to a number. A condition
/// of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionObjectVariable {
    /// Whose: the NPC the trigger selected (SelectNpc), the event object it selected, or the player
    /// (QuestOwner).
    #[export]
    #[init(val = GString::from("SelectedNpc"))]
    object: GString,
    /// Which variable (from 0).
    #[export]
    variable: i64,
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionObjectVariable {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// Selects an event object of a zone (found by its map block and number), for later blocks.
/// (Not in the game yet: it always fails.) A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionSelectEventObject {
    /// The zone (drag its RoseZoneSettings in).
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// The map block, east-west.
    #[export]
    chunk_x: i64,
    /// The map block, north-south.
    #[export]
    chunk_y: i64,
    /// The event object's number in the block.
    #[export]
    event: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionSelectEventObject {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// Selects an NPC (or monster) for later blocks (ObjectVariable, SpawnMonster at SelectedNpc
/// ...). Passes when it is in the world. A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionSelectNpc {
    /// The NPC or monster (drag a RoseNpc or RoseMonster in).
    #[export]
    npc: Option<Gd<Resource>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionSelectNpc {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// A quest switch the player has is on (or off). A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionQuestSwitch {
    /// Which switch (from 0).
    #[export]
    switch: i64,
    /// On or off.
    #[export]
    value: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionQuestSwitch {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player's party has from `from` to `to` members. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionPartyMemberCount {
    /// At least.
    #[export]
    from: i64,
    /// At most.
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionPartyMemberCount {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The zone time of the selected NPC, event object or the player's zone is from `from` to `to`.
/// A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionObjectZoneTime {
    /// Whose: the NPC the trigger selected (SelectNpc), the event object it selected, or the player
    /// (QuestOwner).
    #[export]
    #[init(val = GString::from("SelectedNpc"))]
    object: GString,
    /// From.
    #[export]
    from: i64,
    /// To.
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionObjectZoneTime {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// A variable of one NPC compares to a variable of another. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionCompareNpcVariables {
    /// The first NPC.
    #[export]
    npc_1: Option<Gd<Resource>>,
    /// Its variable.
    #[export]
    variable_1: i64,
    /// How to compare: the first NPC's variable on the left.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The second NPC.
    #[export]
    npc_2: Option<Gd<Resource>>,
    /// Its variable.
    #[export]
    variable_2: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionCompareNpcVariables {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// It is a day of the month (0 = any day) and the time of day is from `from_minute` to
/// `to_minute`. A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionMonthDayTime {
    /// The day of the month (0 = any).
    #[export]
    month_day: i64,
    /// From (minutes after midnight).
    #[export(range = (0.0, 1440.0, 1.0, suffix = "min"))]
    from_minute: i64,
    /// To (minutes after midnight).
    #[export(range = (0.0, 1440.0, 1.0, suffix = "min"))]
    to_minute: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionMonthDayTime {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// It is a day of the week and the time of day is from `from_minute` to `to_minute`. A
/// condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionWeekDayTime {
    /// The day of the week.
    #[export]
    week_day: i64,
    /// From (minutes after midnight).
    #[export(range = (0.0, 1440.0, 1.0, suffix = "min"))]
    from_minute: i64,
    /// To (minutes after midnight).
    #[export(range = (0.0, 1440.0, 1.0, suffix = "min"))]
    to_minute: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionWeekDayTime {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player's team number is from `from` to `to`. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionTeamNumber {
    /// From.
    #[export]
    from: i64,
    /// To.
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionTeamNumber {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The selected NPC or event object is within a distance of the player. A condition of a quest
/// trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionObjectDistance {
    /// Whose: the NPC the trigger selected (SelectNpc), the event object it selected, or the player
    /// (QuestOwner).
    #[export]
    #[init(val = GString::from("SelectedNpc"))]
    object: GString,
    /// How near.
    #[export(range = (0.0, 100000.0, 100.0, or_greater, suffix = "cm"))]
    distance: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionObjectDistance {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The game channel is from `from` to `to`. A condition of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionServerChannelNumber {
    /// From.
    #[export]
    from: i64,
    /// To.
    #[export]
    to: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionServerChannelNumber {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player is in a clan (or not). A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionHasClan {
    /// On: in a clan. Off: in none.
    #[export]
    has_clan: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionHasClan {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player's rank in the clan compares to a number. (Clans are not in the game yet.) A
/// condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanPosition {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The rank to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanPosition {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The clan points the player gave compare to a number. (Clans are not in the game yet.) A
/// condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanPointContribution {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanPointContribution {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The clan's level compares to a number. (Clans are not in the game yet.) A condition of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanLevel {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The level to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanLevel {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The clan's points compare to a number. (Clans are not in the game yet.) A condition of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanPoints {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanPoints {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The clan's money compares to a number. (Clans are not in the game yet.) A condition of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanMoney {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanMoney {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The clan's member count compares to a number. (Clans are not in the game yet.) A condition
/// of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionClanMemberCount {
    /// How to compare: the current value on the left, `value` on the right.
    #[export]
    #[init(val = GString::from("=="))]
    operator: GString,
    /// The number to compare with.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionClanMemberCount {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player's clan knows a skill (or not). (Clans are not in the game yet.) A condition of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionHasClanSkill {
    /// The skill (drag a RoseSkill in).
    #[export]
    skill: Option<Gd<RoseSkill>>,
    /// On: must know it. Off: must not.
    #[export]
    #[init(val = true)]
    has: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionHasClanSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// The player's clan knows a skill numbered from `from_skill` to `to_skill` (or none of them).
/// (Clans are not in the game yet.) A condition of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseConditionHasClanSkillInRange {
    /// The first skill number.
    #[export]
    from_skill: i64,
    /// The last skill number.
    #[export]
    to_skill: i64,
    /// On: must know one. Off: must know none.
    #[export]
    #[init(val = true)]
    has: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseConditionHasClanSkillInRange {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, false);
    }
}

/// Takes the selected quest out of the journal. A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardRemoveSelectedQuest {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardRemoveSelectedQuest {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Puts a quest in the journal and selects it. A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardAddQuest {
    /// The quest (drag a RoseQuest in).
    #[export]
    quest: Option<Gd<RoseQuest>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardAddQuest {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns the selected quest into another (the next part of a story). A reward of a quest
/// trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardChangeSelectedQuest {
    /// The quest it becomes.
    #[export]
    quest: Option<Gd<RoseQuest>>,
    /// Keep its variables, switches, timer and items.
    #[export]
    keep_data: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardChangeSelectedQuest {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Selects a quest in the journal for the blocks after it. A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSelectQuest {
    /// The quest.
    #[export]
    quest: Option<Gd<RoseQuest>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSelectQuest {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives an item (a quest item goes in the selected quest's item list). A reward of a quest
/// trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardAddItem {
    /// The item (drag an item resource in).
    #[export]
    item: Option<Gd<Resource>>,
    /// How many.
    #[export]
    quantity: i64,
    /// Equipment only: how much of its life the item has left when it is handed out, in %
    /// (100: new; 30: worn, so it can be repaired at once).
    #[export(range = (0.0, 100.0))]
    #[init(val = 100)]
    life_percent: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardAddItem {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Takes an item away. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardRemoveItem {
    /// The item.
    #[export]
    item: Option<Gd<Resource>>,
    /// How many.
    #[export]
    quantity: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardRemoveItem {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes a quest value. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardQuestVariable {
    /// Which kind of quest value: the selected quest's own variables, switches or timer, or the
    /// ones all quests share (Episode, Job, Planet, Union).
    #[export]
    #[init(val = GString::from("Variable"))]
    variable_type: GString,
    /// Which one (from 0).
    #[export]
    variable: i64,
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardQuestVariable {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes one of the player's values (a stat, money, skill points ...). A reward of a quest
/// trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardAbilityValue {
    /// Which value.
    #[export]
    #[init(val = GString::from("Level"))]
    ability: GString,
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardAbilityValue {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives experience points worked out by a formula (`equation`, see the wiki page "Quests")
/// from `value` and the player. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardCalculatedExperiencePoints {
    /// Which formula.
    #[export]
    equation: i64,
    /// The number the formula starts from.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardCalculatedExperiencePoints {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives money worked out by a formula (`equation`) from `value` and the player. A reward of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardCalculatedMoney {
    /// Which formula.
    #[export]
    equation: i64,
    /// The number the formula starts from.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardCalculatedMoney {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives a number of an item worked out by a formula (`equation`) from `value`. A reward of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardCalculatedItem {
    /// Which formula.
    #[export]
    equation: i64,
    /// The number the formula starts from.
    #[export]
    value: i64,
    /// The item.
    #[export]
    item: Option<Gd<Resource>>,
    /// A gem number for the item (0 = none), as iROSE has it.
    #[export]
    gem: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardCalculatedItem {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Sets the player's health and mana to a share of their maximum. A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSetHealthManaPercent {
    /// Health.
    #[export(range = (0.0, 100.0, 1.0, suffix = "%"))]
    health_percent: i64,
    /// Mana.
    #[export(range = (0.0, 100.0, 1.0, suffix = "%"))]
    mana_percent: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSetHealthManaPercent {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Moves the player to a spot in a zone. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardTeleport {
    /// The zone (drag its RoseZoneSettings in).
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// East-west.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    x: i64,
    /// North-south.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    y: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardTeleport {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Puts monsters in the world: around the player, the selected NPC or event object, or at a
/// spot. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSpawnMonster {
    /// The monster (drag a RoseMonster in).
    #[export]
    monster: Option<Gd<Resource>>,
    /// How many.
    #[export]
    count: i64,
    /// Where (Position: at `zone`, `x`, `y`).
    #[export]
    #[init(val = GString::from("QuestOwner"))]
    location: GString,
    /// For Position: the zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// For Position: east-west.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    x: f64,
    /// For Position: north-south.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    y: f64,
    /// How far around the spot they may appear.
    #[export(range = (0.0, 100000.0, 100.0, or_greater, suffix = "cm"))]
    distance: i64,
    /// Their team number (100 = monsters).
    #[export]
    team: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSpawnMonster {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Runs another trigger after this one (by its name: triggers can run each other in a loop,
/// which links cannot). A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardTrigger {
    /// The trigger's name (`124-01`).
    #[export]
    trigger: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardTrigger {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives back the player's stat points. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardResetBasicStats {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardResetBasicStats {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes a variable of the selected NPC, event object or the player. A reward of a quest
/// trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardObjectVariable {
    /// Whose: the NPC the trigger selected (SelectNpc), the event object it selected, or the player
    /// (QuestOwner).
    #[export]
    #[init(val = GString::from("SelectedNpc"))]
    object: GString,
    /// Which variable (from 0).
    #[export]
    variable: i64,
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardObjectVariable {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// The selected NPC says something: nearby (Chat), to the zone (Shout) or to everyone
/// (Announce). A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardNpcMessage {
    /// Who hears it.
    #[export]
    #[init(val = GString::from("Chat"))]
    message_type: GString,
    /// What it says.
    #[export(multiline)]
    text: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardNpcMessage {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Runs a trigger on the selected NPC, event object or the player after a delay. (The server
/// does not run this one yet.) A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardTriggerAfterDelay {
    /// Whose: the NPC the trigger selected (SelectNpc), the event object it selected, or the player
    /// (QuestOwner).
    #[export]
    #[init(val = GString::from("SelectedNpc"))]
    object: GString,
    /// The delay.
    #[export(range = (0.0, 86400.0, 1.0, or_greater, suffix = "s"))]
    delay_seconds: i64,
    /// The trigger's name.
    #[export]
    trigger: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardTriggerAfterDelay {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Teaches the player a skill. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardAddSkill {
    /// The skill (drag a RoseSkill in).
    #[export]
    skill: Option<Gd<RoseSkill>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardAddSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Makes the player forget a skill. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardRemoveSkill {
    /// The skill.
    #[export]
    skill: Option<Gd<RoseSkill>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardRemoveSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns a quest switch on or off. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSetQuestSwitch {
    /// Which switch (from 0).
    #[export]
    switch: i64,
    /// On or off.
    #[export]
    value: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSetQuestSwitch {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns off a group of 32 quest switches. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClearSwitchGroup {
    /// Which group (from 0).
    #[export]
    group: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClearSwitchGroup {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns off every quest switch. A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClearAllSwitches {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClearAllSwitches {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Announces a text to everyone, with NPC variables filled in (`npcs` and `variables`, in
/// pairs). (The server does not run this one yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardFormatAnnounceMessage {
    /// The text.
    #[export(multiline)]
    text: GString,
    /// The NPC number of each value.
    #[export]
    npcs: PackedInt32Array,
    /// Which variable of that NPC.
    #[export]
    variables: PackedInt32Array,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardFormatAnnounceMessage {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Runs a trigger for everyone of a team in a zone. (The server does not run this one yet.) A
/// reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardTriggerForZoneTeam {
    /// The zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// The team number.
    #[export]
    team: i64,
    /// The trigger's name.
    #[export]
    trigger: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardTriggerForZoneTeam {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Gives the player a team number: their own, their clan's or their party's. (Teams are not in
/// the game yet: it does nothing.) A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSetTeamNumber {
    /// Where the number comes from.
    #[export]
    #[init(val = GString::from("Unique"))]
    source: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSetTeamNumber {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Sets where the player gets up after falling (in their zone). A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardSetRevivePosition {
    /// East-west.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    x: f64,
    /// North-south.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    y: f64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardSetRevivePosition {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns a zone's monster spawns on. (The server does not run this one yet.) A reward of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardEnableMonsterSpawns {
    /// The zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardEnableMonsterSpawns {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns a zone's monster spawns off. (The server does not run this one yet.) A reward of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardDisableMonsterSpawns {
    /// The zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardDisableMonsterSpawns {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Turns a zone's monster spawns on if off, off if on. (The server does not run this one yet.)
/// A reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardToggleMonsterSpawns {
    /// The zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardToggleMonsterSpawns {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Raises the clan's level by one. (Clans are not in the game yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClanLevelIncrease {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClanLevelIncrease {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes the clan's money. (Clans are not in the game yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClanMoney {
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClanMoney {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes the clan's points. (Clans are not in the game yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClanPoints {
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClanPoints {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Teaches the clan a skill. (Clans are not in the game yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardAddClanSkill {
    /// The skill.
    #[export]
    skill: Option<Gd<RoseSkill>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardAddClanSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Makes the clan forget a skill. (Clans are not in the game yet.) A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardRemoveClanSkill {
    /// The skill.
    #[export]
    skill: Option<Gd<RoseSkill>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardRemoveClanSkill {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Changes the clan points the player gave. (Clans are not in the game yet.) A reward of a
/// quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardClanPointContribution {
    /// How to change it: Set to `value`, Add or Subtract `value`, or SetZero / SetOne (then `value`
    /// is not used).
    #[export]
    #[init(val = GString::from("Set"))]
    operator: GString,
    /// The number.
    #[export]
    value: i64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardClanPointContribution {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Moves the player's clan members near them to a spot. (Clans are not in the game yet.) A
/// reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardTeleportNearbyClanMembers {
    /// How near they must be.
    #[export(range = (0.0, 100000.0, 100.0, or_greater, suffix = "cm"))]
    distance: i64,
    /// The zone.
    #[export]
    zone: Option<Gd<RoseZoneSettings>>,
    /// East-west.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    x: f64,
    /// North-south.
    #[export(range = (0.0, 1000000.0, 1.0, or_greater, suffix = "cm"))]
    y: f64,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardTeleportNearbyClanMembers {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Runs a function of the client's scripts (iROSE's Lua; the server does nothing with it). A
/// reward of a quest trigger (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardCallLuaFunction {
    /// The function's name.
    #[export]
    function: GString,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardCallLuaFunction {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}

/// Makes the player forget their skills and gives back the points. A reward of a quest trigger
/// (`RoseQuestTrigger`).
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseRewardResetSkills {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseRewardResetSkills {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        block_hint(property, true);
    }
}
