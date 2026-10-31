//! Quests: `quests.toml` (one `[[quest]]` per quest: its name and journal text, iROSE's
//! LIST_QUEST.STB with LIST_QUEST_S.STL) and `quest-triggers.toml` (one `[[trigger]]` per quest
//! trigger, iROSE's QSD files). The keys are the property names of the Godot resources
//! (`RoseQuest`, `RoseQuestTrigger` and the block classes, `RoseCondition<kind>` and
//! `RoseReward<kind>`) they are exported from.
//!
//! A trigger is a step: when every condition passes, it gives every reward. Its conditions
//! are `[[trigger.condition]]` tables and its rewards `[[trigger.reward]]` tables, each with a
//! `kind` (the block: `AbilityValue`, `AddItem` ...) and that kind's fields. A trigger whose
//! conditions fail tries its `next_trigger`, if it has one. Dialogs, monsters and other
//! triggers name triggers by their name.
//!
//! A block that names a quest, a monster or an NPC, a skill or a zone holds its id, and an item
//! its export name (`quest/601`). Ability types are their names (`Level`), as in the skills.
//! The text an NPC says (`NpcMessage`) is in the block itself: the loader numbers the texts
//! for the game, which looks them up by number (iROSE's QUEST_S.STB).
//!
//! The game's quest engine (`rose-quest`) runs iROSE's blocks, so the loader turns the export
//! back into them (`QsdCondition`, `QsdReward`), with iROSE's numbers for ability and item
//! types (`IroseCodes`, from `rose-data-irose`).

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    num::{NonZeroU8, NonZeroUsize},
    sync::Arc,
    time::Duration,
};

use anyhow::{bail, Context};
use rose_data::{
    AbilityType, ItemReference, ItemType, QuestData, QuestDatabase, QuestTrigger, QuestTriggerHash,
    StringDatabase, WorldTicks, WORLD_TICK_DURATION,
};
use rose_file_readers::{
    QsdCondition, QsdConditionOperator, QsdItem, QsdNpcMessageType, QsdObjectType, QsdReward,
    QsdRewardOperator, QsdSpawnMonsterLocation, QsdTeamNumberSource, QsdVariableType,
};
use serde::{Deserialize, Serialize};

use crate::npcs::{item_name, parse_item_name};
use crate::Format;

pub const QUESTS_FILE: &str = "quests";
pub const QUEST_TRIGGERS_FILE: &str = "quest-triggers";

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn yes() -> bool {
    true
}

fn is_yes(value: &bool) -> bool {
    *value
}

/// iROSE's numbers for ability types and item types, which the quest engine reads
/// (`rose_data_irose::decode_ability_type` and the others).
#[derive(Clone, Copy)]
pub struct IroseCodes {
    pub decode_ability: fn(usize) -> Option<AbilityType>,
    pub encode_ability: fn(AbilityType) -> Option<usize>,
    pub decode_item_type: fn(usize) -> Option<ItemType>,
    pub encode_item_type: fn(ItemType) -> Option<usize>,
}

impl IroseCodes {
    fn ability(&self, number: NonZeroUsize) -> Result<AbilityType, anyhow::Error> {
        (self.decode_ability)(number.get()).with_context(|| format!("ability {number} has no name"))
    }

    fn ability_number(&self, ability: AbilityType) -> Result<NonZeroUsize, anyhow::Error> {
        (self.encode_ability)(ability)
            .and_then(NonZeroUsize::new)
            .with_context(|| format!("ability {ability:?} has no iROSE number"))
    }

    fn item(&self, item: QsdItem) -> Result<String, anyhow::Error> {
        let item_type = (self.decode_item_type)(item.item_type)
            .with_context(|| format!("item type {} has no name", item.item_type))?;
        if item.item_number == 0 {
            bail!("item {} of type {:?}: numbers start at 1", item.item_number, item_type);
        }
        Ok(item_name(ItemReference::new(item_type, item.item_number)))
    }

    fn qsd_item(&self, name: &str) -> Result<QsdItem, anyhow::Error> {
        let item = parse_item_name(name)?;
        Ok(QsdItem {
            item_number: item.item_number,
            item_type: (self.encode_item_type)(item.item_type)
                .with_context(|| format!("item type {:?} has no iROSE number", item.item_type))?,
        })
    }
}

/// How a condition compares (the condition's value on the left).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compare {
    #[default]
    #[serde(rename = "==")]
    Equals,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanEqual,
    #[serde(rename = "!=")]
    NotEqual,
}

/// The Inspector's names of `Compare`, in order.
pub const COMPARE_NAMES: [&str; 6] = ["==", ">", ">=", "<", "<=", "!="];

impl From<QsdConditionOperator> for Compare {
    fn from(o: QsdConditionOperator) -> Self {
        match o {
            QsdConditionOperator::Equals => Compare::Equals,
            QsdConditionOperator::GreaterThan => Compare::GreaterThan,
            QsdConditionOperator::GreaterThanEqual => Compare::GreaterThanEqual,
            QsdConditionOperator::LessThan => Compare::LessThan,
            QsdConditionOperator::LessThanEqual => Compare::LessThanEqual,
            QsdConditionOperator::NotEqual => Compare::NotEqual,
        }
    }
}

impl From<Compare> for QsdConditionOperator {
    fn from(o: Compare) -> Self {
        match o {
            Compare::Equals => QsdConditionOperator::Equals,
            Compare::GreaterThan => QsdConditionOperator::GreaterThan,
            Compare::GreaterThanEqual => QsdConditionOperator::GreaterThanEqual,
            Compare::LessThan => QsdConditionOperator::LessThan,
            Compare::LessThanEqual => QsdConditionOperator::LessThanEqual,
            Compare::NotEqual => QsdConditionOperator::NotEqual,
        }
    }
}

/// How a reward changes a value: set it to the reward's value, add or take the value, or set
/// it to 0 or 1 (the value is then not used).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Change {
    #[default]
    Set,
    Add,
    Subtract,
    SetZero,
    SetOne,
}

/// The Inspector's names of `Change`, in order.
pub const CHANGE_NAMES: [&str; 5] = ["Set", "Add", "Subtract", "SetZero", "SetOne"];

impl From<QsdRewardOperator> for Change {
    fn from(o: QsdRewardOperator) -> Self {
        match o {
            QsdRewardOperator::Set => Change::Set,
            QsdRewardOperator::Add => Change::Add,
            QsdRewardOperator::Subtract => Change::Subtract,
            QsdRewardOperator::Zero => Change::SetZero,
            QsdRewardOperator::One => Change::SetOne,
        }
    }
}

impl From<Change> for QsdRewardOperator {
    fn from(o: Change) -> Self {
        match o {
            Change::Set => QsdRewardOperator::Set,
            Change::Add => QsdRewardOperator::Add,
            Change::Subtract => QsdRewardOperator::Subtract,
            Change::SetZero => QsdRewardOperator::Zero,
            Change::SetOne => QsdRewardOperator::One,
        }
    }
}

/// Which of the character's quest values: a variable of the selected quest, one of its
/// switches, its timer, or a variable shared by all quests (episode, job, planet, union).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableType {
    #[default]
    Variable,
    Switch,
    Timer,
    Episode,
    Job,
    Planet,
    Union,
}

/// The Inspector's names of `VariableType`, in order.
pub const VARIABLE_TYPE_NAMES: [&str; 7] = ["Variable", "Switch", "Timer", "Episode", "Job", "Planet", "Union"];

impl From<QsdVariableType> for VariableType {
    fn from(t: QsdVariableType) -> Self {
        match t {
            QsdVariableType::Variable => VariableType::Variable,
            QsdVariableType::Switch => VariableType::Switch,
            QsdVariableType::Timer => VariableType::Timer,
            QsdVariableType::Episode => VariableType::Episode,
            QsdVariableType::Job => VariableType::Job,
            QsdVariableType::Planet => VariableType::Planet,
            QsdVariableType::Union => VariableType::Union,
        }
    }
}

impl From<VariableType> for QsdVariableType {
    fn from(t: VariableType) -> Self {
        match t {
            VariableType::Variable => QsdVariableType::Variable,
            VariableType::Switch => QsdVariableType::Switch,
            VariableType::Timer => QsdVariableType::Timer,
            VariableType::Episode => QsdVariableType::Episode,
            VariableType::Job => QsdVariableType::Job,
            VariableType::Planet => QsdVariableType::Planet,
            VariableType::Union => QsdVariableType::Union,
        }
    }
}

/// Whose value: the NPC the trigger selected (`SelectNpc`), the event object it selected, or
/// the player running the quest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Object {
    #[default]
    SelectedNpc,
    SelectedEvent,
    QuestOwner,
}

/// The Inspector's names of `Object`, in order.
pub const OBJECT_NAMES: [&str; 3] = ["SelectedNpc", "SelectedEvent", "QuestOwner"];

impl From<QsdObjectType> for Object {
    fn from(o: QsdObjectType) -> Self {
        match o {
            QsdObjectType::SelectedNpc => Object::SelectedNpc,
            QsdObjectType::SelectedEvent => Object::SelectedEvent,
            QsdObjectType::QuestOwner => Object::QuestOwner,
        }
    }
}

impl From<Object> for QsdObjectType {
    fn from(o: Object) -> Self {
        match o {
            Object::SelectedNpc => QsdObjectType::SelectedNpc,
            Object::SelectedEvent => QsdObjectType::SelectedEvent,
            Object::QuestOwner => QsdObjectType::QuestOwner,
        }
    }
}

/// How an NPC's message is shown: said nearby, shouted to the zone, or announced to everyone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    #[default]
    Chat,
    Shout,
    Announce,
}

/// The Inspector's names of `MessageType`, in order.
pub const MESSAGE_TYPE_NAMES: [&str; 3] = ["Chat", "Shout", "Announce"];

/// Where spawned monsters appear: around the player, the selected NPC or event object, or at
/// a spot (`zone`, `x`, `y`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnAt {
    #[default]
    QuestOwner,
    SelectedNpc,
    SelectedEvent,
    Position,
}

/// The Inspector's names of `SpawnAt`, in order.
pub const SPAWN_AT_NAMES: [&str; 4] = ["QuestOwner", "SelectedNpc", "SelectedEvent", "Position"];

/// Where the player's new team number comes from: a number of their own, their clan's or
/// their party's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSource {
    #[default]
    Unique,
    Clan,
    Party,
}

/// The Inspector's names of `TeamSource`, in order.
pub const TEAM_SOURCE_NAMES: [&str; 3] = ["Unique", "Clan", "Party"];

/// What a trigger checks. `from`/`to` are both included.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum ConditionExport {
    /// The player has `quest` (and the trigger works on it from now on).
    SelectQuest {
        #[serde(default)]
        quest: u16,
    },
    QuestVariable { variable_type: VariableType, variable: u32, operator: Compare, value: i32 },
    AbilityValue { ability: AbilityType, operator: Compare, value: i32 },
    /// The player carries `count` of `item` (`operator`). With `equipment_index` set to a worn
    /// slot (iROSE's number, 1 to 8), wears `item` there instead (no item: the slot is empty).
    QuestItem {
        #[serde(default, skip_serializing_if = "is_default")]
        item: String,
        #[serde(default, skip_serializing_if = "is_default")]
        equipment_index: u32,
        count: u32,
        operator: Compare,
    },
    Party { leader: bool, level_operator: Compare, level: i32 },
    Position {
        #[serde(default)]
        zone: u16,
        x: f32,
        y: f32,
        distance: i32,
    },
    WorldTime { from: u32, to: u32 },
    HasSkill {
        #[serde(default)]
        skill: u16,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        has: bool,
    },
    HasSkillInRange {
        from_skill: u16,
        to_skill: u16,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        has: bool,
    },
    RandomPercent { from: u8, to: u8 },
    ObjectVariable { object: Object, variable: u32, operator: Compare, value: i32 },
    SelectEventObject {
        #[serde(default)]
        zone: u16,
        chunk_x: u32,
        chunk_y: u32,
        event: u32,
    },
    SelectNpc {
        #[serde(default)]
        npc: u16,
    },
    QuestSwitch { switch: u32, value: bool },
    PartyMemberCount { from: u32, to: u32 },
    ObjectZoneTime { object: Object, from: u32, to: u32 },
    CompareNpcVariables {
        #[serde(default)]
        npc_1: u16,
        variable_1: u32,
        operator: Compare,
        #[serde(default)]
        npc_2: u16,
        variable_2: u32,
    },
    /// 0 = any day.
    MonthDayTime { month_day: u8, from_minute: i32, to_minute: i32 },
    WeekDayTime { week_day: u8, from_minute: i32, to_minute: i32 },
    TeamNumber { from: u32, to: u32 },
    ObjectDistance { object: Object, distance: i32 },
    ServerChannelNumber { from: u32, to: u32 },
    HasClan { has_clan: bool },
    ClanPosition { operator: Compare, value: i32 },
    ClanPointContribution { operator: Compare, value: i32 },
    ClanLevel { operator: Compare, value: i32 },
    ClanPoints { operator: Compare, value: i32 },
    ClanMoney { operator: Compare, value: i32 },
    ClanMemberCount { operator: Compare, value: i32 },
    HasClanSkill {
        #[serde(default)]
        skill: u16,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        has: bool,
    },
    HasClanSkillInRange {
        from_skill: u16,
        to_skill: u16,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        has: bool,
    },
}

/// What a trigger gives or does.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum RewardExport {
    RemoveSelectedQuest,
    AddQuest {
        #[serde(default)]
        quest: u16,
    },
    ChangeSelectedQuest {
        #[serde(default)]
        quest: u16,
        keep_data: bool,
    },
    SelectQuest {
        #[serde(default)]
        quest: u16,
    },
    AddItem {
        #[serde(default)]
        item: String,
        quantity: u32,
        /// Equipment only: the life it has left when handed out, in % (100: new).
        #[serde(default = "full_life", skip_serializing_if = "is_full_life")]
        life_percent: u8,
    },
    RemoveItem {
        #[serde(default)]
        item: String,
        quantity: u32,
    },
    QuestVariable { variable_type: VariableType, variable: u32, operator: Change, value: i32 },
    AbilityValue { ability: AbilityType, operator: Change, value: i32 },
    CalculatedExperiencePoints { equation: u32, value: i32 },
    CalculatedMoney { equation: u32, value: i32 },
    CalculatedItem {
        equation: u32,
        value: i32,
        #[serde(default)]
        item: String,
        #[serde(default, skip_serializing_if = "is_default")]
        gem: u32,
    },
    SetHealthManaPercent { health_percent: u8, mana_percent: u8 },
    Teleport {
        #[serde(default)]
        zone: u16,
        x: u32,
        y: u32,
    },
    SpawnMonster {
        #[serde(default)]
        monster: u16,
        count: u32,
        location: SpawnAt,
        #[serde(default, skip_serializing_if = "is_default")]
        zone: u16,
        #[serde(default, skip_serializing_if = "is_default")]
        x: f32,
        #[serde(default, skip_serializing_if = "is_default")]
        y: f32,
        distance: i32,
        team: u32,
    },
    Trigger {
        #[serde(default)]
        trigger: String,
    },
    ResetBasicStats,
    ObjectVariable { object: Object, variable: u32, operator: Change, value: i32 },
    NpcMessage {
        message_type: MessageType,
        #[serde(default)]
        text: String,
    },
    TriggerAfterDelay {
        object: Object,
        delay_seconds: u32,
        #[serde(default)]
        trigger: String,
    },
    AddSkill {
        #[serde(default)]
        skill: u16,
    },
    RemoveSkill {
        #[serde(default)]
        skill: u16,
    },
    SetQuestSwitch { switch: u32, value: bool },
    ClearSwitchGroup { group: u32 },
    ClearAllSwitches,
    /// `text` with the NPC variables filled in (`npcs` and `variables`, in pairs).
    FormatAnnounceMessage {
        #[serde(default)]
        text: String,
        #[serde(default, skip_serializing_if = "is_default")]
        npcs: Vec<u16>,
        #[serde(default, skip_serializing_if = "is_default")]
        variables: Vec<i32>,
    },
    TriggerForZoneTeam {
        #[serde(default)]
        zone: u16,
        team: u32,
        #[serde(default)]
        trigger: String,
    },
    SetTeamNumber { source: TeamSource },
    SetRevivePosition { x: f32, y: f32 },
    EnableMonsterSpawns {
        #[serde(default)]
        zone: u16,
    },
    DisableMonsterSpawns {
        #[serde(default)]
        zone: u16,
    },
    ToggleMonsterSpawns {
        #[serde(default)]
        zone: u16,
    },
    ClanLevelIncrease,
    ClanMoney { operator: Change, value: i32 },
    ClanPoints { operator: Change, value: i32 },
    AddClanSkill {
        #[serde(default)]
        skill: u16,
    },
    RemoveClanSkill {
        #[serde(default)]
        skill: u16,
    },
    ClanPointContribution { operator: Change, value: i32 },
    TeleportNearbyClanMembers {
        distance: i32,
        #[serde(default)]
        zone: u16,
        x: f32,
        y: f32,
    },
    CallLuaFunction {
        #[serde(default)]
        function: String,
    },
    ResetSkills,
}

/// The kind names of the conditions and the rewards, as the block classes are named
/// (`RoseCondition` + kind, `RoseReward` + kind).
pub const CONDITION_KINDS: [&str; 31] = [
    "SelectQuest", "QuestVariable", "AbilityValue", "QuestItem", "Party", "Position", "WorldTime",
    "HasSkill", "HasSkillInRange", "RandomPercent", "ObjectVariable", "SelectEventObject",
    "SelectNpc", "QuestSwitch", "PartyMemberCount", "ObjectZoneTime", "CompareNpcVariables",
    "MonthDayTime", "WeekDayTime", "TeamNumber", "ObjectDistance", "ServerChannelNumber",
    "HasClan", "ClanPosition", "ClanPointContribution", "ClanLevel", "ClanPoints", "ClanMoney",
    "ClanMemberCount", "HasClanSkill", "HasClanSkillInRange",
];
pub const REWARD_KINDS: [&str; 40] = [
    "RemoveSelectedQuest", "AddQuest", "ChangeSelectedQuest", "SelectQuest", "AddItem",
    "RemoveItem", "QuestVariable", "AbilityValue", "CalculatedExperiencePoints",
    "CalculatedMoney", "CalculatedItem", "SetHealthManaPercent", "Teleport", "SpawnMonster",
    "Trigger", "ResetBasicStats", "ObjectVariable", "NpcMessage", "TriggerAfterDelay", "AddSkill",
    "RemoveSkill", "SetQuestSwitch", "ClearSwitchGroup", "ClearAllSwitches",
    "FormatAnnounceMessage", "TriggerForZoneTeam", "SetTeamNumber", "SetRevivePosition",
    "EnableMonsterSpawns", "DisableMonsterSpawns", "ToggleMonsterSpawns", "ClanLevelIncrease",
    "ClanMoney", "ClanPoints", "AddClanSkill", "RemoveClanSkill", "ClanPointContribution",
    "TeleportNearbyClanMembers", "CallLuaFunction", "ResetSkills",
];

fn id16(value: usize, what: &str) -> Result<u16, anyhow::Error> {
    u16::try_from(value).with_context(|| format!("{what} {value} is too big"))
}

fn id32(value: usize, what: &str) -> Result<u32, anyhow::Error> {
    u32::try_from(value).with_context(|| format!("{what} {value} is too big"))
}

impl ConditionExport {
    pub fn from_qsd(c: &QsdCondition, codes: &IroseCodes) -> Result<Self, anyhow::Error> {
        use ConditionExport as E;
        use QsdCondition as C;
        Ok(match c {
            C::SelectQuest { id } => E::SelectQuest { quest: id16(*id, "quest")? },
            C::QuestVariable { variable_type, variable_id, operator, value } => E::QuestVariable {
                variable_type: (*variable_type).into(),
                variable: id32(*variable_id, "variable")?,
                operator: (*operator).into(),
                value: *value,
            },
            C::AbilityValue { ability_type, operator, value } => E::AbilityValue {
                ability: codes.ability(*ability_type)?,
                operator: (*operator).into(),
                value: *value,
            },
            C::QuestItem { item, equipment_index, required_count, operator } => E::QuestItem {
                item: item.map(|i| codes.item(i)).transpose()?.unwrap_or_default(),
                equipment_index: equipment_index.map_or(0, |e| e.get() as u32),
                count: *required_count,
                operator: (*operator).into(),
            },
            C::Party { is_leader, level_operator, level } => {
                E::Party { leader: *is_leader, level_operator: (*level_operator).into(), level: *level }
            }
            C::Position { zone, x, y, distance } => {
                E::Position { zone: id16(*zone, "zone")?, x: *x, y: *y, distance: *distance }
            }
            C::WorldTime { range } => E::WorldTime { from: *range.start(), to: *range.end() },
            C::HasSkill { id, has_skill } => E::HasSkill { skill: id16(*id, "skill")?, has: *has_skill },
            C::HasSkillInRange { range, has_skill } => E::HasSkillInRange {
                from_skill: id16(*range.start(), "skill")?,
                to_skill: id16(*range.end(), "skill")?,
                has: *has_skill,
            },
            C::RandomPercent { range } => E::RandomPercent { from: *range.start(), to: *range.end() },
            C::ObjectVariable { object, variable_id, operator, value } => E::ObjectVariable {
                object: (*object).into(),
                variable: id32(*variable_id, "variable")?,
                operator: (*operator).into(),
                value: *value,
            },
            C::SelectEventObject { zone, chunk_x, chunk_y, event_id } => E::SelectEventObject {
                zone: id16(*zone, "zone")?,
                chunk_x: id32(*chunk_x, "chunk")?,
                chunk_y: id32(*chunk_y, "chunk")?,
                event: id32(*event_id, "event")?,
            },
            C::SelectNpc { id } => E::SelectNpc { npc: id16(*id, "NPC")? },
            C::QuestSwitch { id, value } => E::QuestSwitch { switch: id32(*id, "switch")?, value: *value },
            C::PartyMemberCount { range } => E::PartyMemberCount {
                from: id32(*range.start(), "party size")?,
                to: id32(*range.end(), "party size")?,
            },
            C::ObjectZoneTime { object, time_range } => {
                E::ObjectZoneTime { object: (*object).into(), from: *time_range.start(), to: *time_range.end() }
            }
            C::CompareNpcVariables { npc_id_1, variable_id_1, operator, npc_id_2, variable_id_2 } => {
                E::CompareNpcVariables {
                    npc_1: id16(*npc_id_1, "NPC")?,
                    variable_1: id32(*variable_id_1, "variable")?,
                    operator: (*operator).into(),
                    npc_2: id16(*npc_id_2, "NPC")?,
                    variable_2: id32(*variable_id_2, "variable")?,
                }
            }
            C::MonthDayTime { month_day, day_minutes_range } => E::MonthDayTime {
                month_day: month_day.map_or(0, |d| d.get()),
                from_minute: *day_minutes_range.start(),
                to_minute: *day_minutes_range.end(),
            },
            C::WeekDayTime { week_day, day_minutes_range } => E::WeekDayTime {
                week_day: *week_day,
                from_minute: *day_minutes_range.start(),
                to_minute: *day_minutes_range.end(),
            },
            C::TeamNumber { range } => {
                E::TeamNumber { from: id32(*range.start(), "team")?, to: id32(*range.end(), "team")? }
            }
            C::ObjectDistance { object, distance } => E::ObjectDistance { object: (*object).into(), distance: *distance },
            C::ServerChannelNumber { range } => E::ServerChannelNumber {
                from: id32(*range.start(), "channel")?,
                to: id32(*range.end(), "channel")?,
            },
            C::HasClan { has_clan } => E::HasClan { has_clan: *has_clan },
            C::ClanPosition { operator, value } => {
                E::ClanPosition { operator: (*operator).into(), value: i32::try_from(*value)? }
            }
            C::ClanPointContribution { operator, value } => {
                E::ClanPointContribution { operator: (*operator).into(), value: *value }
            }
            C::ClanLevel { operator, value } => E::ClanLevel { operator: (*operator).into(), value: *value },
            C::ClanPoints { operator, value } => E::ClanPoints { operator: (*operator).into(), value: *value },
            C::ClanMoney { operator, value } => E::ClanMoney { operator: (*operator).into(), value: *value },
            C::ClanMemberCount { operator, value } => {
                E::ClanMemberCount { operator: (*operator).into(), value: i32::try_from(*value)? }
            }
            C::HasClanSkill { id, has_skill } => E::HasClanSkill { skill: id16(*id, "skill")?, has: *has_skill },
            C::HasClanSkillInRange { range, has_skill } => E::HasClanSkillInRange {
                from_skill: id16(*range.start(), "skill")?,
                to_skill: id16(*range.end(), "skill")?,
                has: *has_skill,
            },
        })
    }

    pub fn to_qsd(&self, codes: &IroseCodes) -> Result<QsdCondition, anyhow::Error> {
        use ConditionExport as E;
        use QsdCondition as C;
        let count = |v: i32, what: &str| usize::try_from(v).with_context(|| format!("{what} {v} is below 0"));
        Ok(match self {
            E::SelectQuest { quest } => C::SelectQuest { id: *quest as usize },
            E::QuestVariable { variable_type, variable, operator, value } => C::QuestVariable {
                variable_type: (*variable_type).into(),
                variable_id: *variable as usize,
                operator: (*operator).into(),
                value: *value,
            },
            E::AbilityValue { ability, operator, value } => C::AbilityValue {
                ability_type: codes.ability_number(*ability)?,
                operator: (*operator).into(),
                value: *value,
            },
            E::QuestItem { item, equipment_index, count, operator } => C::QuestItem {
                item: (!item.is_empty()).then(|| codes.qsd_item(item)).transpose()?,
                equipment_index: NonZeroUsize::new(*equipment_index as usize),
                required_count: *count,
                operator: (*operator).into(),
            },
            E::Party { leader, level_operator, level } => {
                C::Party { is_leader: *leader, level_operator: (*level_operator).into(), level: *level }
            }
            E::Position { zone, x, y, distance } => {
                C::Position { zone: *zone as usize, x: *x, y: *y, distance: *distance }
            }
            E::WorldTime { from, to } => C::WorldTime { range: *from..=*to },
            E::HasSkill { skill, has } => C::HasSkill { id: *skill as usize, has_skill: *has },
            E::HasSkillInRange { from_skill, to_skill, has } => {
                C::HasSkillInRange { range: *from_skill as usize..=*to_skill as usize, has_skill: *has }
            }
            E::RandomPercent { from, to } => C::RandomPercent { range: *from..=*to },
            E::ObjectVariable { object, variable, operator, value } => C::ObjectVariable {
                object: (*object).into(),
                variable_id: *variable as usize,
                operator: (*operator).into(),
                value: *value,
            },
            E::SelectEventObject { zone, chunk_x, chunk_y, event } => C::SelectEventObject {
                zone: *zone as usize,
                chunk_x: *chunk_x as usize,
                chunk_y: *chunk_y as usize,
                event_id: *event as usize,
            },
            E::SelectNpc { npc } => C::SelectNpc { id: *npc as usize },
            E::QuestSwitch { switch, value } => C::QuestSwitch { id: *switch as usize, value: *value },
            E::PartyMemberCount { from, to } => C::PartyMemberCount { range: *from as usize..=*to as usize },
            E::ObjectZoneTime { object, from, to } => {
                C::ObjectZoneTime { object: (*object).into(), time_range: *from..=*to }
            }
            E::CompareNpcVariables { npc_1, variable_1, operator, npc_2, variable_2 } => C::CompareNpcVariables {
                npc_id_1: *npc_1 as usize,
                variable_id_1: *variable_1 as usize,
                operator: (*operator).into(),
                npc_id_2: *npc_2 as usize,
                variable_id_2: *variable_2 as usize,
            },
            E::MonthDayTime { month_day, from_minute, to_minute } => C::MonthDayTime {
                month_day: NonZeroU8::new(*month_day),
                day_minutes_range: *from_minute..=*to_minute,
            },
            E::WeekDayTime { week_day, from_minute, to_minute } => {
                C::WeekDayTime { week_day: *week_day, day_minutes_range: *from_minute..=*to_minute }
            }
            E::TeamNumber { from, to } => C::TeamNumber { range: *from as usize..=*to as usize },
            E::ObjectDistance { object, distance } => C::ObjectDistance { object: (*object).into(), distance: *distance },
            E::ServerChannelNumber { from, to } => C::ServerChannelNumber { range: *from as usize..=*to as usize },
            E::HasClan { has_clan } => C::HasClan { has_clan: *has_clan },
            E::ClanPosition { operator, value } => {
                C::ClanPosition { operator: (*operator).into(), value: count(*value, "clan position")? }
            }
            E::ClanPointContribution { operator, value } => {
                C::ClanPointContribution { operator: (*operator).into(), value: *value }
            }
            E::ClanLevel { operator, value } => C::ClanLevel { operator: (*operator).into(), value: *value },
            E::ClanPoints { operator, value } => C::ClanPoints { operator: (*operator).into(), value: *value },
            E::ClanMoney { operator, value } => C::ClanMoney { operator: (*operator).into(), value: *value },
            E::ClanMemberCount { operator, value } => {
                C::ClanMemberCount { operator: (*operator).into(), value: count(*value, "clan member count")? }
            }
            E::HasClanSkill { skill, has } => C::HasClanSkill { id: *skill as usize, has_skill: *has },
            E::HasClanSkillInRange { from_skill, to_skill, has } => {
                C::HasClanSkillInRange { range: *from_skill as usize..=*to_skill as usize, has_skill: *has }
            }
        })
    }

    /// The quest it names, if any.
    pub fn quest(&self) -> Option<u16> {
        match self {
            ConditionExport::SelectQuest { quest } => Some(*quest),
            _ => None,
        }
    }
}

impl RewardExport {
    /// `text` gives the text of an iROSE message number (QUEST_S.STB).
    pub fn from_qsd(r: &QsdReward, codes: &IroseCodes, text: impl Fn(usize) -> String) -> Result<Self, anyhow::Error> {
        use QsdReward as R;
        use RewardExport as E;
        Ok(match r {
            R::RemoveSelectedQuest => E::RemoveSelectedQuest,
            R::AddQuest { id } => E::AddQuest { quest: id16(*id, "quest")? },
            R::ChangeSelectedQuest { id, keep_data } => {
                E::ChangeSelectedQuest { quest: id16(*id, "quest")?, keep_data: *keep_data }
            }
            R::SelectQuest { id } => E::SelectQuest { quest: id16(*id, "quest")? },
            R::AddItem { item, quantity, life_percent } => {
                E::AddItem { item: codes.item(*item)?, quantity: id32(*quantity, "quantity")?, life_percent: *life_percent }
            }
            R::RemoveItem { item, quantity } => {
                E::RemoveItem { item: codes.item(*item)?, quantity: id32(*quantity, "quantity")? }
            }
            R::QuestVariable { variable_type, variable_id, operator, value } => E::QuestVariable {
                variable_type: (*variable_type).into(),
                variable: id32(*variable_id, "variable")?,
                operator: (*operator).into(),
                value: *value,
            },
            R::AbilityValue { ability_type, operator, value } => E::AbilityValue {
                ability: codes.ability(*ability_type)?,
                operator: (*operator).into(),
                value: *value,
            },
            R::CalculatedExperiencePoints { equation, value } => {
                E::CalculatedExperiencePoints { equation: id32(*equation, "equation")?, value: *value }
            }
            R::CalculatedMoney { equation, value } => E::CalculatedMoney { equation: id32(*equation, "equation")?, value: *value },
            R::CalculatedItem { equation, value, item, gem } => E::CalculatedItem {
                equation: id32(*equation, "equation")?,
                value: *value,
                item: codes.item(*item)?,
                gem: gem.map_or(Ok(0), |g| id32(g.get(), "gem"))?,
            },
            R::SetHealthManaPercent { health_percent, mana_percent } => {
                E::SetHealthManaPercent { health_percent: *health_percent, mana_percent: *mana_percent }
            }
            R::Teleport { zone, x, y } => E::Teleport { zone: id16(*zone, "zone")?, x: *x, y: *y },
            R::SpawnMonster { npc, count, location, distance, team_number } => {
                let (location, zone, x, y) = match location {
                    QsdSpawnMonsterLocation::QuestOwner => (SpawnAt::QuestOwner, 0, 0.0, 0.0),
                    QsdSpawnMonsterLocation::SelectedNpc => (SpawnAt::SelectedNpc, 0, 0.0, 0.0),
                    QsdSpawnMonsterLocation::SelectedEvent => (SpawnAt::SelectedEvent, 0, 0.0, 0.0),
                    QsdSpawnMonsterLocation::Position { zone, x, y } => (SpawnAt::Position, id16(*zone, "zone")?, *x, *y),
                };
                E::SpawnMonster {
                    monster: id16(*npc, "monster")?,
                    count: id32(*count, "count")?,
                    location,
                    zone,
                    x,
                    y,
                    distance: *distance,
                    team: id32(*team_number, "team")?,
                }
            }
            R::Trigger { name } => E::Trigger { trigger: name.clone() },
            R::ResetBasicStats => E::ResetBasicStats,
            R::ObjectVariable { object, variable_id, operator, value } => E::ObjectVariable {
                object: (*object).into(),
                variable: id32(*variable_id, "variable")?,
                operator: (*operator).into(),
                value: *value,
            },
            R::NpcMessage { message_type, string_id } => E::NpcMessage {
                message_type: match message_type {
                    QsdNpcMessageType::Chat => MessageType::Chat,
                    QsdNpcMessageType::Shout => MessageType::Shout,
                    QsdNpcMessageType::Announce => MessageType::Announce,
                },
                text: text(*string_id),
            },
            R::TriggerAfterDelay { object, delay, trigger } => E::TriggerAfterDelay {
                object: (*object).into(),
                delay_seconds: u32::try_from(delay.as_secs())?,
                trigger: trigger.clone(),
            },
            R::AddSkill { id } => E::AddSkill { skill: id16(*id, "skill")? },
            R::RemoveSkill { id } => E::RemoveSkill { skill: id16(*id, "skill")? },
            R::SetQuestSwitch { id, value } => E::SetQuestSwitch { switch: id32(*id, "switch")?, value: *value },
            R::ClearSwitchGroup { group } => E::ClearSwitchGroup { group: id32(*group, "switch group")? },
            R::ClearAllSwitches => E::ClearAllSwitches,
            R::FormatAnnounceMessage { string_id, variables } => E::FormatAnnounceMessage {
                text: text(*string_id),
                npcs: variables.iter().map(|(n, _)| id16(*n, "NPC")).collect::<Result<_, _>>()?,
                variables: variables.iter().map(|(_, v)| i32::try_from(*v)).collect::<Result<_, _>>()?,
            },
            R::TriggerForZoneTeam { zone, team_number, trigger } => E::TriggerForZoneTeam {
                zone: id16(*zone, "zone")?,
                team: id32(*team_number, "team")?,
                trigger: trigger.clone(),
            },
            R::SetTeamNumber { source } => E::SetTeamNumber {
                source: match source {
                    QsdTeamNumberSource::Unique => TeamSource::Unique,
                    QsdTeamNumberSource::Clan => TeamSource::Clan,
                    QsdTeamNumberSource::Party => TeamSource::Party,
                },
            },
            R::SetRevivePosition { x, y } => E::SetRevivePosition { x: *x, y: *y },
            R::EnableMonsterSpawns { zone } => E::EnableMonsterSpawns { zone: id16(*zone, "zone")? },
            R::DisableMonsterSpawns { zone } => E::DisableMonsterSpawns { zone: id16(*zone, "zone")? },
            R::ToggleMonsterSpawns { zone } => E::ToggleMonsterSpawns { zone: id16(*zone, "zone")? },
            R::ClanLevelIncrease => E::ClanLevelIncrease,
            R::ClanMoney { operator, value } => E::ClanMoney { operator: (*operator).into(), value: *value },
            R::ClanPoints { operator, value } => E::ClanPoints { operator: (*operator).into(), value: *value },
            R::AddClanSkill { id } => E::AddClanSkill { skill: id16(*id, "skill")? },
            R::RemoveClanSkill { id } => E::RemoveClanSkill { skill: id16(*id, "skill")? },
            R::ClanPointContribution { operator, value } => {
                E::ClanPointContribution { operator: (*operator).into(), value: *value }
            }
            R::TeleportNearbyClanMembers { distance, zone, x, y } => {
                E::TeleportNearbyClanMembers { distance: *distance, zone: id16(*zone, "zone")?, x: *x, y: *y }
            }
            R::CallLuaFunction { name } => E::CallLuaFunction { function: name.clone() },
            R::ResetSkills => E::ResetSkills,
        })
    }

    /// `text` gives a message's number for the game (see `QuestsExport::to_database`).
    pub fn to_qsd(&self, codes: &IroseCodes, mut text: impl FnMut(&str) -> usize) -> Result<QsdReward, anyhow::Error> {
        use QsdReward as R;
        use RewardExport as E;
        let item = |name: &str, what: &str| -> Result<QsdItem, anyhow::Error> {
            if name.is_empty() {
                bail!("{what} names no item");
            }
            codes.qsd_item(name)
        };
        Ok(match self {
            E::RemoveSelectedQuest => R::RemoveSelectedQuest,
            E::AddQuest { quest } => R::AddQuest { id: *quest as usize },
            E::ChangeSelectedQuest { quest, keep_data } => R::ChangeSelectedQuest { id: *quest as usize, keep_data: *keep_data },
            E::SelectQuest { quest } => R::SelectQuest { id: *quest as usize },
            E::AddItem { item: name, quantity, life_percent } => {
                R::AddItem { item: item(name, "AddItem")?, quantity: *quantity as usize, life_percent: (*life_percent).min(100) }
            }
            E::RemoveItem { item: name, quantity } => {
                R::RemoveItem { item: item(name, "RemoveItem")?, quantity: *quantity as usize }
            }
            E::QuestVariable { variable_type, variable, operator, value } => R::QuestVariable {
                variable_type: (*variable_type).into(),
                variable_id: *variable as usize,
                operator: (*operator).into(),
                value: *value,
            },
            E::AbilityValue { ability, operator, value } => R::AbilityValue {
                ability_type: codes.ability_number(*ability)?,
                operator: (*operator).into(),
                value: *value,
            },
            E::CalculatedExperiencePoints { equation, value } => {
                R::CalculatedExperiencePoints { equation: *equation as usize, value: *value }
            }
            E::CalculatedMoney { equation, value } => R::CalculatedMoney { equation: *equation as usize, value: *value },
            E::CalculatedItem { equation, value, item: name, gem } => R::CalculatedItem {
                equation: *equation as usize,
                value: *value,
                item: item(name, "CalculatedItem")?,
                gem: NonZeroUsize::new(*gem as usize),
            },
            E::SetHealthManaPercent { health_percent, mana_percent } => {
                R::SetHealthManaPercent { health_percent: *health_percent, mana_percent: *mana_percent }
            }
            E::Teleport { zone, x, y } => R::Teleport { zone: *zone as usize, x: *x, y: *y },
            E::SpawnMonster { monster, count, location, zone, x, y, distance, team } => R::SpawnMonster {
                npc: *monster as usize,
                count: *count as usize,
                location: match location {
                    SpawnAt::QuestOwner => QsdSpawnMonsterLocation::QuestOwner,
                    SpawnAt::SelectedNpc => QsdSpawnMonsterLocation::SelectedNpc,
                    SpawnAt::SelectedEvent => QsdSpawnMonsterLocation::SelectedEvent,
                    SpawnAt::Position => QsdSpawnMonsterLocation::Position { zone: *zone as usize, x: *x, y: *y },
                },
                distance: *distance,
                team_number: *team as usize,
            },
            E::Trigger { trigger } => R::Trigger { name: trigger.clone() },
            E::ResetBasicStats => R::ResetBasicStats,
            E::ObjectVariable { object, variable, operator, value } => R::ObjectVariable {
                object: (*object).into(),
                variable_id: *variable as usize,
                operator: (*operator).into(),
                value: *value,
            },
            E::NpcMessage { message_type, text: message } => R::NpcMessage {
                message_type: match message_type {
                    MessageType::Chat => QsdNpcMessageType::Chat,
                    MessageType::Shout => QsdNpcMessageType::Shout,
                    MessageType::Announce => QsdNpcMessageType::Announce,
                },
                string_id: text(message),
            },
            E::TriggerAfterDelay { object, delay_seconds, trigger } => R::TriggerAfterDelay {
                object: (*object).into(),
                delay: Duration::from_secs(*delay_seconds as u64),
                trigger: trigger.clone(),
            },
            E::AddSkill { skill } => R::AddSkill { id: *skill as usize },
            E::RemoveSkill { skill } => R::RemoveSkill { id: *skill as usize },
            E::SetQuestSwitch { switch, value } => R::SetQuestSwitch { id: *switch as usize, value: *value },
            E::ClearSwitchGroup { group } => R::ClearSwitchGroup { group: *group as usize },
            E::ClearAllSwitches => R::ClearAllSwitches,
            E::FormatAnnounceMessage { text: message, npcs, variables } => {
                if npcs.len() != variables.len() {
                    bail!("FormatAnnounceMessage: {} NPCs but {} variables", npcs.len(), variables.len());
                }
                R::FormatAnnounceMessage {
                    string_id: text(message),
                    variables: npcs
                        .iter()
                        .zip(variables)
                        .map(|(n, v)| Ok((*n as usize, usize::try_from(*v)?)))
                        .collect::<Result<_, anyhow::Error>>()?,
                }
            }
            E::TriggerForZoneTeam { zone, team, trigger } => R::TriggerForZoneTeam {
                zone: *zone as usize,
                team_number: *team as usize,
                trigger: trigger.clone(),
            },
            E::SetTeamNumber { source } => R::SetTeamNumber {
                source: match source {
                    TeamSource::Unique => QsdTeamNumberSource::Unique,
                    TeamSource::Clan => QsdTeamNumberSource::Clan,
                    TeamSource::Party => QsdTeamNumberSource::Party,
                },
            },
            E::SetRevivePosition { x, y } => R::SetRevivePosition { x: *x, y: *y },
            E::EnableMonsterSpawns { zone } => R::EnableMonsterSpawns { zone: *zone as usize },
            E::DisableMonsterSpawns { zone } => R::DisableMonsterSpawns { zone: *zone as usize },
            E::ToggleMonsterSpawns { zone } => R::ToggleMonsterSpawns { zone: *zone as usize },
            E::ClanLevelIncrease => R::ClanLevelIncrease,
            E::ClanMoney { operator, value } => R::ClanMoney { operator: (*operator).into(), value: *value },
            E::ClanPoints { operator, value } => R::ClanPoints { operator: (*operator).into(), value: *value },
            E::AddClanSkill { skill } => R::AddClanSkill { id: *skill as usize },
            E::RemoveClanSkill { skill } => R::RemoveClanSkill { id: *skill as usize },
            E::ClanPointContribution { operator, value } => {
                R::ClanPointContribution { operator: (*operator).into(), value: *value }
            }
            E::TeleportNearbyClanMembers { distance, zone, x, y } => {
                R::TeleportNearbyClanMembers { distance: *distance, zone: *zone as usize, x: *x, y: *y }
            }
            E::CallLuaFunction { function } => R::CallLuaFunction { name: function.clone() },
            E::ResetSkills => R::ResetSkills,
        })
    }

    /// The quest it names, if any.
    pub fn quest(&self) -> Option<u16> {
        match self {
            RewardExport::AddQuest { quest }
            | RewardExport::ChangeSelectedQuest { quest, .. }
            | RewardExport::SelectQuest { quest } => Some(*quest),
            _ => None,
        }
    }

    /// The monster it names, if any.
    pub fn monster_mut(&mut self) -> Option<&mut u16> {
        match self {
            RewardExport::SpawnMonster { monster, .. } => Some(monster),
            _ => None,
        }
    }
}

/// One quest: what the quest journal shows.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuestDataExport {
    /// The number (iROSE's LIST_QUEST.STB row). Saved characters store it.
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub description: String,
    /// What the journal says while the quest is under way, and once it is done.
    #[serde(skip_serializing_if = "is_default")]
    pub start_message: String,
    #[serde(skip_serializing_if = "is_default")]
    pub end_message: String,
    /// Seconds to finish it in (0 = no limit). The game counts in steps of 10 seconds.
    #[serde(skip_serializing_if = "is_default")]
    pub time_limit: u32,
}

impl QuestDataExport {
    pub fn from_data(quest: &QuestData) -> Self {
        Self {
            id: quest.id as u16,
            name: quest.name.to_string(),
            description: quest.description.to_string(),
            start_message: quest.start_message.to_string(),
            end_message: quest.end_message.to_string(),
            time_limit: quest.time_limit.map_or(0, |t| Duration::from(t).as_secs() as u32),
        }
    }

    /// Whether it holds nothing (no text, no time limit): the same as no quest.
    pub fn is_empty(&self) -> bool {
        *self == QuestDataExport { id: self.id, ..Default::default() }
    }

    fn to_data(&self, text: &mut Vec<String>) -> QuestData {
        let ticks = (self.time_limit as u64).div_ceil(WORLD_TICK_DURATION.as_secs());
        QuestData {
            id: self.id as usize,
            name: keep(text, &self.name),
            description: keep(text, &self.description),
            start_message: keep(text, &self.start_message),
            end_message: keep(text, &self.end_message),
            time_limit: (ticks > 0).then_some(WorldTicks(ticks)),
        }
    }
}

/// Gives `text` a `'static` lifetime by keeping it in `keep`, which the database owns.
fn keep(keep: &mut Vec<String>, text: &str) -> &'static str {
    if text.is_empty() {
        return "";
    }
    let owned = text.to_string();
    // SAFETY: the String's buffer never moves or changes while it sits in `keep`, and
    // `keep` lives in the database (`with_text`) as long as the data pointing into it.
    let text: &'static str = unsafe { std::mem::transmute::<&str, &'static str>(owned.as_str()) };
    keep.push(owned);
    text
}

/// One quest trigger: its conditions, its rewards, and the trigger tried when a condition
/// fails.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuestTriggerExport {
    /// What dialogs, monsters and other triggers call it by (`124-01`).
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub next_trigger: String,
    #[serde(skip_serializing_if = "is_default")]
    pub condition: Vec<ConditionExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub reward: Vec<RewardExport>,
}

impl QuestTriggerExport {
    pub fn from_data(trigger: &QuestTrigger, codes: &IroseCodes, strings: &HashMap<u16, String>) -> Result<Self, anyhow::Error> {
        let text = |id: usize| u16::try_from(id).ok().and_then(|id| strings.get(&id)).cloned().unwrap_or_default();
        Ok(Self {
            name: trigger.name.clone(),
            next_trigger: trigger.next_trigger_name.clone().unwrap_or_default(),
            condition: trigger
                .conditions
                .iter()
                .map(|c| ConditionExport::from_qsd(c, codes))
                .collect::<Result<_, _>>()?,
            reward: trigger
                .rewards
                .iter()
                .map(|r| RewardExport::from_qsd(r, codes, text))
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct QuestFile {
    #[serde(default)]
    quest: Vec<QuestDataExport>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TriggerFile {
    #[serde(default)]
    trigger: Vec<QuestTriggerExport>,
}

/// Every quest and quest trigger, as exported.
#[derive(Debug, Default)]
pub struct QuestsExport {
    pub quests: Vec<QuestDataExport>,
    pub triggers: Vec<QuestTriggerExport>,
}

impl QuestsExport {
    /// The converter's starting point and the same-data check. Triggers sorted by name.
    pub fn from_database(db: &QuestDatabase, codes: &IroseCodes) -> Result<Self, anyhow::Error> {
        let quests = db.quests.iter().flatten().map(QuestDataExport::from_data).collect();
        let mut triggers = db
            .triggers
            .values()
            .map(|t| QuestTriggerExport::from_data(t, codes, &db.strings).with_context(|| format!("trigger {}", t.name)))
            .collect::<Result<Vec<_>, _>>()?;
        triggers.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(Self { quests, triggers })
    }

    /// The quest database. NPC messages get numbers from 1 in the order they come.
    /// `strings` is only kept for the other databases that share it.
    pub fn to_database(&self, strings: Arc<StringDatabase>, codes: &IroseCodes) -> Result<QuestDatabase, anyhow::Error> {
        self.check()?;
        let mut text = Vec::new();
        let mut quests: Vec<Option<QuestData>> = Vec::new();
        for quest in &self.quests {
            let at = quest.id as usize;
            if quests.len() <= at {
                quests.resize_with(at + 1, || None);
            }
            quests[at] = Some(quest.to_data(&mut text));
        }
        let mut messages: HashMap<u16, String> = HashMap::new();
        let mut numbers: HashMap<String, usize> = HashMap::new();
        let mut number = |message: &str| -> usize {
            if message.is_empty() {
                return 0;
            }
            let next = numbers.len() + 1;
            *numbers.entry(message.to_string()).or_insert_with(|| {
                messages.insert(next as u16, message.to_string());
                next
            })
        };
        let mut triggers = HashMap::new();
        for trigger in &self.triggers {
            let mut read = || -> Result<QuestTrigger, anyhow::Error> {
                Ok(QuestTrigger {
                    name: trigger.name.clone(),
                    conditions: trigger.condition.iter().map(|c| c.to_qsd(codes)).collect::<Result<_, _>>()?,
                    rewards: trigger.reward.iter().map(|r| r.to_qsd(codes, &mut number)).collect::<Result<_, _>>()?,
                    next_trigger_name: (!trigger.next_trigger.is_empty()).then(|| trigger.next_trigger.clone()),
                })
            };
            triggers.insert(trigger.name.clone(), read().with_context(|| format!("trigger {}", trigger.name))?);
        }
        if messages.len() > u16::MAX as usize {
            bail!("more than {} NPC messages", u16::MAX);
        }
        let triggers_by_hash = triggers.keys().map(|k| (QuestTriggerHash::from(k.as_str()), k.clone())).collect();
        Ok(QuestDatabase {
            _string_database: strings,
            quests,
            strings: messages,
            triggers,
            triggers_by_hash,
            _text: text,
        })
    }

    /// Ids and names used twice, triggers that try a next trigger that isn't there (or come
    /// round to themselves), and quests a block names that aren't there.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut quests = BTreeSet::new();
        for quest in &self.quests {
            if quest.id == 0 {
                bail!("quest 0: ids start at 1");
            }
            if !quests.insert(quest.id) {
                bail!("{QUESTS_FILE}: quest {} is there twice", quest.id);
            }
        }
        let mut next: BTreeMap<&str, &str> = BTreeMap::new();
        for trigger in &self.triggers {
            if trigger.name.is_empty() {
                bail!("{QUEST_TRIGGERS_FILE}: a trigger has no name");
            }
            if next.insert(&trigger.name, &trigger.next_trigger).is_some() {
                bail!("{QUEST_TRIGGERS_FILE}: trigger {} is there twice", trigger.name);
            }
        }
        for trigger in &self.triggers {
            let mut at = trigger.next_trigger.as_str();
            let mut steps = 0;
            while !at.is_empty() {
                let Some(after) = next.get(at) else {
                    bail!("trigger {} tries trigger {at} next, which isn't there", trigger.name);
                };
                steps += 1;
                if at == trigger.name || steps > next.len() {
                    bail!("trigger {}: its next triggers come round to it again", trigger.name);
                }
                at = after;
            }
            let named = trigger
                .condition
                .iter()
                .filter_map(ConditionExport::quest)
                .chain(trigger.reward.iter().filter_map(RewardExport::quest));
            for quest in named {
                if !quests.contains(&quest) {
                    bail!("trigger {} names quest {quest}, which isn't there", trigger.name);
                }
            }
        }
        Ok(())
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut quest = self.quests.clone();
        quest.sort_by_key(|q| q.id);
        let mut trigger = self.triggers.clone();
        trigger.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(vec![
            (QUESTS_FILE, format.write(QUESTS_FILE, &QuestFile { quest })?),
            (QUEST_TRIGGERS_FILE, format.write(QUEST_TRIGGERS_FILE, &TriggerFile { trigger })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; one missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        match (read(QUESTS_FILE)?, read(QUEST_TRIGGERS_FILE)?) {
            (None, None) => Ok(None),
            (Some(quests), Some(triggers)) => Ok(Some(Self {
                quests: format.parse::<QuestFile>(QUESTS_FILE, &quests)?.quest,
                triggers: format.parse::<TriggerFile>(QUEST_TRIGGERS_FILE, &triggers)?.trigger,
            })),
            (None, _) => bail!("the quest export has no {QUESTS_FILE}.{}", format.extension()),
            (_, None) => bail!("the quest export has no {QUEST_TRIGGERS_FILE}.{}", format.extension()),
        }
    }
}

/// Every difference between two quest databases, compared as their export (so an NPC message
/// counts by its text, not its number). A quest with no text and no time limit counts as no
/// quest, and a monster `monsters` doesn't have as none (the game can't spawn either).
pub fn compare_quest_databases(
    a: &QuestDatabase,
    b: &QuestDatabase,
    codes: &IroseCodes,
    monsters: &BTreeSet<u16>,
) -> Result<Vec<String>, anyhow::Error> {
    let normal = |db: &QuestDatabase| -> Result<QuestsExport, anyhow::Error> {
        let mut export = QuestsExport::from_database(db, codes)?;
        export.quests.retain(|q| !q.is_empty());
        for reward in export.triggers.iter_mut().flat_map(|t| t.reward.iter_mut()) {
            if let Some(monster) = reward.monster_mut().filter(|m| !monsters.contains(m)) {
                *monster = 0;
            }
        }
        Ok(export)
    };
    let (a, b) = (normal(a)?, normal(b)?);
    let mut out = Vec::new();
    let quests = |e: &QuestsExport| e.quests.iter().map(|q| (q.id.to_string(), format!("{q:#?}"))).collect::<BTreeMap<_, _>>();
    let triggers = |e: &QuestsExport| e.triggers.iter().map(|t| (t.name.clone(), format!("{t:#?}"))).collect::<BTreeMap<_, _>>();
    for (what, x, y) in [("quest", quests(&a), quests(&b)), ("quest trigger", triggers(&a), triggers(&b))] {
        let keys: BTreeSet<&String> = x.keys().chain(y.keys()).collect();
        for key in keys {
            match (x.get(key), y.get(key)) {
                (Some(_), None) => out.push(format!("{what} {key}: missing")),
                (None, Some(_)) => out.push(format!("{what} {key}: extra")),
                (Some(l), Some(r)) if l != r => {
                    for (l, r) in l.lines().zip(r.lines()).filter(|(l, r)| l != r) {
                        out.push(format!("{what} {key}: {} -> {}", l.trim(), r.trim()));
                    }
                    if l.lines().count() != r.lines().count() {
                        out.push(format!("{what} {key}: the fields differ"));
                    }
                }
                _ => {}
            }
        }
    }
    Ok(out)
}

fn full_life() -> u8 {
    100
}

fn is_full_life(life_percent: &u8) -> bool {
    *life_percent >= 100
}
