//! Monster AI: `monster-ai.toml`, one `[[ai]]` per AI (iROSE's AIP files, listed by
//! FILE_AI.STB, with the text of AI_S.STB). The keys are the property names of the Godot
//! resources they are exported from: `RoseAi`, `RoseAiEvent` and the blocks, `RoseAiIf<kind>`
//! (conditions) and `RoseAiDo<kind>` (actions).
//!
//! An AI has six lists of events, one per moment the server asks it what to do: `on_created`
//! (it appears), `on_idle` (every `idle_interval` seconds while it stands with nothing to
//! fight), `on_attack` (after one of its swings), `on_damaged` (it is hit), `on_kill` (it
//! killed its target) and `on_dead` (it dies). Each event (`[[ai.on_idle]]` ...) has
//! `[[...condition]]` and `[[...action]]` tables, each with a `kind` and that kind's fields.
//! The first event whose conditions all pass runs its actions, from the top.
//!
//! Distances are in centimetres, except `CallAllies`, whose distance is in metres (as in the
//! AIP files). A monster or NPC is named by its number (`npc_id`), a skill and a zone too, a
//! quest step by its name (`trigger_name`, `124-01`) and an item by its export name
//! (`quest/601`). Monsters, NPCs, skills and quest steps are numbers and names in the
//! resources too (the Inspector shows them as resources), because they can link back to the
//! AI: a monster links its AI, and a skill or a quest step can summon that monster. The text a monster says
//! (`Say`, `Message`) is in the block itself: the loader numbers the texts for the server.
//!
//! The server runs iROSE's blocks (`AipCondition`, `AipAction`), so the loader turns the
//! export back into them.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    num::NonZeroU8,
    time::Duration,
};

use anyhow::{bail, Context};
use rose_data::{AiDatabase, ItemReference};
use rose_file_readers::{
    AipAbilityType, AipAction, AipAttackNearbyStat, AipCondition, AipConditionFindNearbyEntities,
    AipConditionMonthDayTime, AipConditionWeekDayTime, AipDamageType, AipDistanceOrigin, AipEvent, AipFile,
    AipHaveStatusTarget, AipHaveStatusType, AipMessageType, AipMonsterSpawnState, AipMoveMode, AipMoveOrigin,
    AipNearbyAlly, AipOperatorType, AipResultOperator, AipSkillTarget, AipSpawnNpcOrigin, AipTrigger,
    AipVariableType,
};
use serde::{Deserialize, Serialize};

use crate::npcs::{item_name, parse_item_name};
use crate::quests::Compare;
use crate::Format;

pub const AI_FILE: &str = "monster-ai";

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn no_limit() -> i32 {
    -1
}

fn is_no_limit(value: &i32) -> bool {
    *value == -1
}

/// iROSE's item numbers (type × 1000 + number), which the AIP files hold
/// (`rose_data_irose::decode_item_base1000` and `rose_game_data::encode_item_base1000`).
#[derive(Clone, Copy)]
pub struct ItemCodes {
    pub decode: fn(usize) -> Option<ItemReference>,
    pub encode: fn(ItemReference) -> Option<i32>,
}

impl ItemCodes {
    /// The export name of an item number, `""` for 0.
    fn name(&self, number: u32) -> Result<String, anyhow::Error> {
        if number == 0 {
            return Ok(String::new());
        }
        let item = (self.decode)(number as usize).with_context(|| format!("item number {number} is no item"))?;
        Ok(item_name(item))
    }

    fn number(&self, name: &str) -> Result<u32, anyhow::Error> {
        if name.is_empty() {
            return Ok(0);
        }
        let item = parse_item_name(name)?;
        let number = (self.encode)(item).with_context(|| format!("item {name} has no iROSE number"))?;
        Ok(number as u32)
    }
}

impl From<AipOperatorType> for Compare {
    fn from(o: AipOperatorType) -> Self {
        match o {
            AipOperatorType::Equals => Compare::Equals,
            AipOperatorType::GreaterThan => Compare::GreaterThan,
            AipOperatorType::GreaterThanEqual => Compare::GreaterThanEqual,
            AipOperatorType::LessThan => Compare::LessThan,
            AipOperatorType::LessThanEqual => Compare::LessThanEqual,
            AipOperatorType::NotEqual => Compare::NotEqual,
        }
    }
}

impl From<Compare> for AipOperatorType {
    fn from(o: Compare) -> Self {
        match o {
            Compare::Equals => AipOperatorType::Equals,
            Compare::GreaterThan => AipOperatorType::GreaterThan,
            Compare::GreaterThanEqual => AipOperatorType::GreaterThanEqual,
            Compare::LessThan => AipOperatorType::LessThan,
            Compare::LessThanEqual => AipOperatorType::LessThanEqual,
            Compare::NotEqual => AipOperatorType::NotEqual,
        }
    }
}

/// Declares an enum of the export, its Inspector names (in order) and the conversions to and
/// from the AIP reader's enum of the same values.
macro_rules! names {
    ($(#[$meta:meta])* $name:ident, $names:ident, $aip:ident { $first:ident = $first_aip:ident $(, $variant:ident = $aip_variant:ident)* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            #[default]
            $first,
            $($variant,)*
        }

        pub const $names: &[&str] = &[stringify!($first) $(, stringify!($variant))*];

        impl From<$aip> for $name {
            fn from(value: $aip) -> Self {
                match value {
                    $aip::$first_aip => $name::$first,
                    $($aip::$aip_variant => $name::$variant,)*
                }
            }
        }

        impl From<$name> for $aip {
            fn from(value: $name) -> Self {
                match value {
                    $name::$first => $aip::$first_aip,
                    $($name::$variant => $aip::$aip_variant,)*
                }
            }
        }
    };
}

names!(
    /// One of a character's values.
    Ability, ABILITY_NAMES, AipAbilityType {
        Level = Level, Attack = Attack, Defence = Defence, Resistance = Resistance,
        HealthPoints = HealthPoints, Charm = Charm,
    }
);
names!(
    /// How `SetVariable` changes a variable.
    Change, CHANGE_NAMES, AipResultOperator { Set = Set, Add = Add, Subtract = Subtract }
);
names!(
    /// Whose variables: the NPC `SelectNpc` picked, the monster's own, or the world's and
    /// the economy's (which the server doesn't keep).
    VariableType, VARIABLE_TYPE_NAMES, AipVariableType {
        SelectedNpc = LocalNpcObject, Own = Ai, World = World, Economy = Economy,
    }
);
names!(
    /// What a distance is measured to.
    DistanceTo, DISTANCE_TO_NAMES, AipDistanceOrigin { Spawn = Spawn, Owner = Owner, Target = Target }
);
names!(
    /// Who `HasStatusEffect` looks at.
    Who, WHO_NAMES, AipHaveStatusTarget { Me = This, Target = Target }
);
names!(
    /// Which status effects count: good ones (buffs), bad ones (debuffs) or any.
    StatusKind, STATUS_KIND_NAMES, AipHaveStatusType { Good = Good, Bad = Bad, Any = Any }
);
names!(
    /// The damage `Damage` compares: the hit it just took, or one it gave.
    DamageType, DAMAGE_TYPE_NAMES, AipDamageType { Received = Received, Given = Given }
);
names!(
    /// Where `MoveRandomly` picks its spot around.
    MoveFrom, MOVE_FROM_NAMES, AipMoveOrigin { Spawn = Spawn, Here = CurrentPosition, FoundCharacter = FindChar }
);
names!(
    /// Walk or run.
    MoveMode, MOVE_MODE_NAMES, AipMoveMode { Walk = Walk, Run = Run }
);
names!(
    /// Which character `AttackByValue` picks.
    Pick, PICK_NAMES, AipAttackNearbyStat { Lowest = Lowest, Highest = Highest }
);
names!(
    /// Where `Spawn` puts the new monster.
    SpawnAt, SPAWN_AT_NAMES, AipSpawnNpcOrigin { Here = CurrentPosition, Attacker = AttackerPosition, Target = TargetPosition }
);
names!(
    /// Who `UseSkill` casts at: the character `FindCharacter` found, the current target, itself,
    /// or the nearest character `FindCharacter` saw.
    SkillTarget, SKILL_TARGET_NAMES, AipSkillTarget {
        FoundCharacter = FindChar, Target = Target, Me = This, NearestCharacter = NearChar,
    }
);
names!(
    /// Who hears a `Message`: players nearby, the whole zone, or everyone.
    MessageType, MESSAGE_TYPE_NAMES, AipMessageType { Say = Say, Shout = Shout, Announce = Announce }
);
names!(
    /// What `SetMonsterSpawns` does to a zone's spawns.
    SpawnState, SPAWN_STATE_NAMES, AipMonsterSpawnState { Disabled = Disabled, Enabled = Enabled, Toggle = Toggle }
);

/// How `FindCharacter` counts: `Nth` stops at the `count`-th character it finds and picks that
/// one (most AIP files); the others compare how many it found with `count` and pick the
/// nearest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CountCompare {
    #[default]
    Nth,
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

pub const COUNT_COMPARE_NAMES: [&str; 7] = ["Nth", "==", ">", ">=", "<", "<=", "!="];

impl From<Option<AipOperatorType>> for CountCompare {
    fn from(o: Option<AipOperatorType>) -> Self {
        match o.map(Compare::from) {
            None => CountCompare::Nth,
            Some(Compare::Equals) => CountCompare::Equals,
            Some(Compare::GreaterThan) => CountCompare::GreaterThan,
            Some(Compare::GreaterThanEqual) => CountCompare::GreaterThanEqual,
            Some(Compare::LessThan) => CountCompare::LessThan,
            Some(Compare::LessThanEqual) => CountCompare::LessThanEqual,
            Some(Compare::NotEqual) => CountCompare::NotEqual,
        }
    }
}

impl From<CountCompare> for Option<AipOperatorType> {
    fn from(o: CountCompare) -> Self {
        let compare = match o {
            CountCompare::Nth => return None,
            CountCompare::Equals => Compare::Equals,
            CountCompare::GreaterThan => Compare::GreaterThan,
            CountCompare::GreaterThanEqual => Compare::GreaterThanEqual,
            CountCompare::LessThan => Compare::LessThan,
            CountCompare::LessThanEqual => Compare::LessThanEqual,
            CountCompare::NotEqual => Compare::NotEqual,
        };
        Some(compare.into())
    }
}

/// Which monsters `CallAllies` calls: any monster, those of one kind (`npc`), or those of its
/// own kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Allies {
    #[default]
    Any,
    OfMonster,
    SameMonster,
}

pub const ALLIES_NAMES: [&str; 3] = ["Any", "OfMonster", "SameMonster"];

/// What an event checks. `from`/`to` are both included, except `Random`'s (`to` not).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum AiConditionExport {
    /// The attacker's value compared with its current target's.
    AttackerVsTarget { operator: Compare, ability: Ability },
    /// Characters within `distance` whose level is `from_level_diff` to `to_level_diff` below
    /// its own (its level minus theirs): monsters if `allied`, else players and their summons.
    FindCharacter {
        distance: i32,
        allied: bool,
        from_level_diff: i32,
        to_level_diff: i32,
        count_operator: CountCompare,
        count: i32,
    },
    Damage { damage_type: DamageType, operator: Compare, value: i32 },
    Distance { to: DistanceTo, operator: Compare, distance: i32 },
    HasNoOwner,
    HasStatusEffect { who: Who, status: StatusKind, has: bool },
    HealthPercent { operator: Compare, value: i32 },
    AttackerIsClanMaster,
    AttackerIsTarget,
    Daytime { day: bool },
    TargetIsClanMaster,
    /// 0 = any day of the month; minutes from midnight.
    MonthDay { month_day: u8, from_minute: i32, to_minute: i32 },
    /// No target yet, and the attacker's value compares to `value`.
    NoTargetAttackerValue { operator: Compare, ability: Ability, value: i32 },
    OwnerHasTarget,
    /// A number from `from` (included) to `to` (not) compared with `value`.
    Random { operator: Compare, from: i32, to: i32, value: i32 },
    SelectNpc {
        #[serde(default)]
        npc_id: u16,
    },
    OwnValue { operator: Compare, ability: Ability, value: i32 },
    ServerChannel { from: u16, to: u16 },
    TargetValue { operator: Compare, ability: Ability, value: i32 },
    Variable { variable_type: VariableType, variable: u32, operator: Compare, value: i32 },
    /// Day 0 is Sunday; minutes from midnight.
    WeekDay { week_day: u8, from_minute: i32, to_minute: i32 },
    WorldTime { from: u32, to: u32 },
    ZoneTime { from: u32, to: u32 },
}

/// What an event does.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum AiActionExport {
    Stop,
    Emote { motion: i32 },
    Say {
        #[serde(default, skip_serializing_if = "is_default")]
        text: String,
    },
    MoveRandomly { from: MoveFrom, mode: MoveMode, distance: i32 },
    AttackByValue { distance: i32, ability: Ability, pick: Pick },
    SpecialAttack,
    MoveAway { mode: MoveMode, distance: i32 },
    Transform {
        #[serde(default)]
        npc_id: u16,
    },
    Spawn {
        #[serde(default)]
        npc_id: u16,
        distance: i32,
        at: SpawnAt,
        owner: bool,
    },
    /// Monsters within `distance` metres that fight no one attack its target: at most `count`
    /// (-1 = all).
    CallAllies {
        distance: i32,
        allies: Allies,
        #[serde(default, skip_serializing_if = "is_default")]
        npc_id: u16,
        #[serde(default = "no_limit", skip_serializing_if = "is_no_limit")]
        count: i32,
    },
    AttackNearest,
    AttackFound,
    AttackAttacker,
    RunAway { distance: i32 },
    /// Drops one of `items`, picked at random from five places: an empty place (or one past
    /// the end of a shorter list) drops nothing.
    DropRandomItem {
        #[serde(default, skip_serializing_if = "is_default")]
        items: Vec<String>,
    },
    KillSelf,
    UseSkill {
        #[serde(default)]
        skill_id: u16,
        target: SkillTarget,
        motion: i32,
    },
    SetVariable { variable_type: VariableType, variable: u32, operator: Change, value: i32 },
    Message {
        message_type: MessageType,
        #[serde(default, skip_serializing_if = "is_default")]
        text: String,
    },
    MoveNearOwner,
    QuestStep {
        #[serde(default)]
        trigger_name: String,
    },
    AttackOwnerTarget,
    /// Zone 0 = the zone it is in.
    SetPvp {
        #[serde(default)]
        zone: u16,
        on: bool,
    },
    SetMonsterSpawns {
        #[serde(default)]
        zone: u16,
        state: SpawnState,
    },
    GiveItemToOwner {
        #[serde(default)]
        item: String,
        count: u32,
    },
}

/// The kind names of the conditions and the actions, as the block classes are named
/// (`RoseAiIf` + kind, `RoseAiDo` + kind).
pub const CONDITION_KINDS: [&str; 23] = [
    "AttackerVsTarget", "FindCharacter", "Damage", "Distance", "HasNoOwner", "HasStatusEffect",
    "HealthPercent", "AttackerIsClanMaster", "AttackerIsTarget", "Daytime", "TargetIsClanMaster",
    "MonthDay", "NoTargetAttackerValue", "OwnerHasTarget", "Random", "SelectNpc", "OwnValue",
    "ServerChannel", "TargetValue", "Variable", "WeekDay", "WorldTime", "ZoneTime",
];
pub const ACTION_KINDS: [&str; 25] = [
    "Stop", "Emote", "Say", "MoveRandomly", "AttackByValue", "SpecialAttack", "MoveAway",
    "Transform", "Spawn", "CallAllies", "AttackNearest", "AttackFound", "AttackAttacker",
    "RunAway", "DropRandomItem", "KillSelf", "UseSkill", "SetVariable", "Message",
    "MoveNearOwner", "QuestStep", "AttackOwnerTarget", "SetPvp", "SetMonsterSpawns",
    "GiveItemToOwner",
];

/// How many places a `DropRandomItem` picks from (at least).
pub const DROP_PLACES: usize = 5;

fn npc16(value: i32) -> Result<u16, anyhow::Error> {
    u16::try_from(value).with_context(|| format!("monster or NPC {value} is no number"))
}

fn zone16(zone: Option<usize>) -> Result<u16, anyhow::Error> {
    zone.map_or(Ok(0), |z| u16::try_from(z).with_context(|| format!("zone {z} is too big")))
}

fn zone_of(zone: u16) -> Option<usize> {
    (zone != 0).then_some(zone as usize)
}

impl AiConditionExport {
    pub fn from_aip(c: &AipCondition) -> Result<Self, anyhow::Error> {
        use AiConditionExport as E;
        use AipCondition as C;
        Ok(match c {
            C::CompareAttackerAndTargetAbilityValue(o, a) => E::AttackerVsTarget { operator: (*o).into(), ability: (*a).into() },
            C::FindNearbyEntities(f) => E::FindCharacter {
                distance: f.distance,
                allied: f.is_allied,
                from_level_diff: *f.level_diff_range.start(),
                to_level_diff: *f.level_diff_range.end(),
                count_operator: f.count_operator_type.into(),
                count: f.count,
            },
            C::Damage(t, o, v) => E::Damage { damage_type: (*t).into(), operator: (*o).into(), value: *v },
            C::Distance(t, o, d) => E::Distance { to: (*t).into(), operator: (*o).into(), distance: *d },
            C::HasNoOwner => E::HasNoOwner,
            C::HasStatusEffect(w, s, h) => E::HasStatusEffect { who: (*w).into(), status: (*s).into(), has: *h },
            C::HealthPercent(o, v) => E::HealthPercent { operator: (*o).into(), value: *v },
            C::IsAttackerClanMaster => E::AttackerIsClanMaster,
            C::IsAttackerCurrentTarget => E::AttackerIsTarget,
            C::IsDaytime(day) => E::Daytime { day: *day },
            C::IsTargetClanMaster => E::TargetIsClanMaster,
            C::MonthDay(m) => E::MonthDay {
                month_day: m.month_day.map_or(0, |d| d.get()),
                from_minute: *m.day_minutes_range.start(),
                to_minute: *m.day_minutes_range.end(),
            },
            C::NoTargetAndCompareAttackerAbilityValue(o, a, v) => {
                E::NoTargetAttackerValue { operator: (*o).into(), ability: (*a).into(), value: *v }
            }
            C::OwnerHasTarget => E::OwnerHasTarget,
            C::Random(o, r, v) => E::Random { operator: (*o).into(), from: r.start, to: r.end, value: *v },
            C::SelectLocalNpc(npc) => E::SelectNpc { npc_id: npc16(*npc)? },
            C::SelfAbilityValue(o, a, v) => E::OwnValue { operator: (*o).into(), ability: (*a).into(), value: *v },
            C::ServerChannelNumber(r) => E::ServerChannel { from: *r.start(), to: *r.end() },
            C::TargetAbilityValue(o, a, v) => E::TargetValue { operator: (*o).into(), ability: (*a).into(), value: *v },
            C::Variable(t, id, o, v) => E::Variable {
                variable_type: (*t).into(),
                variable: u32::try_from(*id).context("variable")?,
                operator: (*o).into(),
                value: *v,
            },
            C::WeekDay(w) => E::WeekDay {
                week_day: w.week_day,
                from_minute: *w.day_minutes_range.start(),
                to_minute: *w.day_minutes_range.end(),
            },
            C::WorldTime(r) => E::WorldTime { from: *r.start(), to: *r.end() },
            C::ZoneTime(r) => E::ZoneTime { from: *r.start(), to: *r.end() },
        })
    }

    pub fn to_aip(&self) -> AipCondition {
        use AiConditionExport as E;
        use AipCondition as C;
        match self {
            E::AttackerVsTarget { operator, ability } => C::CompareAttackerAndTargetAbilityValue((*operator).into(), (*ability).into()),
            E::FindCharacter { distance, allied, from_level_diff, to_level_diff, count_operator, count } => {
                C::FindNearbyEntities(AipConditionFindNearbyEntities {
                    distance: *distance,
                    is_allied: *allied,
                    level_diff_range: *from_level_diff..=*to_level_diff,
                    count_operator_type: (*count_operator).into(),
                    count: *count,
                })
            }
            E::Damage { damage_type, operator, value } => C::Damage((*damage_type).into(), (*operator).into(), *value),
            E::Distance { to, operator, distance } => C::Distance((*to).into(), (*operator).into(), *distance),
            E::HasNoOwner => C::HasNoOwner,
            E::HasStatusEffect { who, status, has } => C::HasStatusEffect((*who).into(), (*status).into(), *has),
            E::HealthPercent { operator, value } => C::HealthPercent((*operator).into(), *value),
            E::AttackerIsClanMaster => C::IsAttackerClanMaster,
            E::AttackerIsTarget => C::IsAttackerCurrentTarget,
            E::Daytime { day } => C::IsDaytime(*day),
            E::TargetIsClanMaster => C::IsTargetClanMaster,
            E::MonthDay { month_day, from_minute, to_minute } => C::MonthDay(AipConditionMonthDayTime {
                month_day: NonZeroU8::new(*month_day),
                day_minutes_range: *from_minute..=*to_minute,
            }),
            E::NoTargetAttackerValue { operator, ability, value } => {
                C::NoTargetAndCompareAttackerAbilityValue((*operator).into(), (*ability).into(), *value)
            }
            E::OwnerHasTarget => C::OwnerHasTarget,
            E::Random { operator, from, to, value } => C::Random((*operator).into(), *from..*to, *value),
            E::SelectNpc { npc_id } => C::SelectLocalNpc(*npc_id as i32),
            E::OwnValue { operator, ability, value } => C::SelfAbilityValue((*operator).into(), (*ability).into(), *value),
            E::ServerChannel { from, to } => C::ServerChannelNumber(*from..=*to),
            E::TargetValue { operator, ability, value } => C::TargetAbilityValue((*operator).into(), (*ability).into(), *value),
            E::Variable { variable_type, variable, operator, value } => {
                C::Variable((*variable_type).into(), *variable as usize, (*operator).into(), *value)
            }
            E::WeekDay { week_day, from_minute, to_minute } => C::WeekDay(AipConditionWeekDayTime {
                week_day: *week_day,
                day_minutes_range: *from_minute..=*to_minute,
            }),
            E::WorldTime { from, to } => C::WorldTime(*from..=*to),
            E::ZoneTime { from, to } => C::ZoneTime(*from..=*to),
        }
    }

    /// The monster or NPC it names, for the links.
    pub fn npc_mut(&mut self) -> Option<&mut u16> {
        match self {
            AiConditionExport::SelectNpc { npc_id } => Some(npc_id),
            _ => None,
        }
    }
}

impl AiActionExport {
    pub fn from_aip(a: &AipAction, codes: &ItemCodes, text: impl Fn(usize) -> String) -> Result<Self, anyhow::Error> {
        use AiActionExport as E;
        use AipAction as A;
        Ok(match a {
            A::Stop => E::Stop,
            A::Emote(motion) => E::Emote { motion: *motion },
            A::Say(id) => E::Say { text: text(*id) },
            A::MoveRandomDistance(from, mode, distance) => {
                E::MoveRandomly { from: (*from).into(), mode: (*mode).into(), distance: *distance }
            }
            A::AttackNearbyEntityByStat(distance, ability, pick) => {
                E::AttackByValue { distance: *distance, ability: (*ability).into(), pick: (*pick).into() }
            }
            A::SpecialAttack => E::SpecialAttack,
            A::MoveAwayFromTarget(mode, distance) => E::MoveAway { mode: (*mode).into(), distance: *distance },
            A::TransformNpc(npc) => E::Transform { npc_id: npc16(*npc)? },
            A::SpawnNpc(npc, distance, at, owner) => {
                E::Spawn { npc_id: npc16(*npc)?, distance: *distance, at: (*at).into(), owner: *owner }
            }
            A::NearbyAlliesAttackTarget(distance, allies, count) => {
                let (allies, npc) = match allies {
                    AipNearbyAlly::Ally => (Allies::Any, 0),
                    AipNearbyAlly::WithNpcId(npc) => (Allies::OfMonster, npc16(*npc)?),
                    AipNearbyAlly::WithSameNpcId => (Allies::SameMonster, 0),
                };
                let count = match count {
                    None => -1,
                    Some(c) => i32::try_from(*c).context("count")?,
                };
                E::CallAllies { distance: *distance, allies, npc_id: npc, count }
            }
            A::AttackNearChar => E::AttackNearest,
            A::AttackFindChar => E::AttackFound,
            A::AttackAttacker => E::AttackAttacker,
            A::RunAway(distance) => E::RunAway { distance: *distance },
            A::DropRandomItem(items) => {
                let mut items = items.iter().map(|i| codes.name(*i)).collect::<Result<Vec<_>, _>>()?;
                // As the resources hold them: no empty places at the end.
                while items.last().is_some_and(String::is_empty) {
                    items.pop();
                }
                E::DropRandomItem { items }
            }
            A::KillSelf => E::KillSelf,
            A::UseSkill(target, skill, motion) => E::UseSkill {
                skill_id: u16::try_from(*skill).with_context(|| format!("skill {skill}"))?,
                target: (*target).into(),
                motion: *motion,
            },
            A::SetVariable(t, id, operator, value) => E::SetVariable {
                variable_type: (*t).into(),
                variable: u32::try_from(*id).context("variable")?,
                operator: (*operator).into(),
                value: *value,
            },
            A::Message(message_type, id) => E::Message { message_type: (*message_type).into(), text: text(*id) },
            A::MoveNearOwner => E::MoveNearOwner,
            A::DoQuestTrigger(name) => E::QuestStep { trigger_name: name.clone() },
            A::AttackOwnerTarget => E::AttackOwnerTarget,
            A::SetPvpFlag(zone, on) => E::SetPvp { zone: zone16(*zone)?, on: *on },
            A::SetMonsterSpawnState(zone, state) => E::SetMonsterSpawns { zone: zone16(*zone)?, state: (*state).into() },
            A::GiveItemToOwner(item, count) => E::GiveItemToOwner {
                item: codes.name(*item)?,
                count: u32::try_from(*count).context("count")?,
            },
        })
    }

    /// The AIP action, with `number` giving each text its number.
    pub fn to_aip(&self, codes: &ItemCodes, mut number: impl FnMut(&str) -> usize) -> Result<AipAction, anyhow::Error> {
        use AiActionExport as E;
        use AipAction as A;
        Ok(match self {
            E::Stop => A::Stop,
            E::Emote { motion } => A::Emote(*motion),
            E::Say { text } => A::Say(number(text)),
            E::MoveRandomly { from, mode, distance } => A::MoveRandomDistance((*from).into(), (*mode).into(), *distance),
            E::AttackByValue { distance, ability, pick } => {
                A::AttackNearbyEntityByStat(*distance, (*ability).into(), (*pick).into())
            }
            E::SpecialAttack => A::SpecialAttack,
            E::MoveAway { mode, distance } => A::MoveAwayFromTarget((*mode).into(), *distance),
            E::Transform { npc_id } => A::TransformNpc(*npc_id as i32),
            E::Spawn { npc_id, distance, at, owner } => A::SpawnNpc(*npc_id as i32, *distance, (*at).into(), *owner),
            E::CallAllies { distance, allies, npc_id, count } => {
                let allies = match allies {
                    Allies::Any => AipNearbyAlly::Ally,
                    Allies::OfMonster => AipNearbyAlly::WithNpcId(*npc_id as i32),
                    Allies::SameMonster => AipNearbyAlly::WithSameNpcId,
                };
                let count = if *count < 0 { None } else { Some(*count as usize) };
                A::NearbyAlliesAttackTarget(*distance, allies, count)
            }
            E::AttackNearest => A::AttackNearChar,
            E::AttackFound => A::AttackFindChar,
            E::AttackAttacker => A::AttackAttacker,
            E::RunAway { distance } => A::RunAway(*distance),
            E::DropRandomItem { items } => {
                let mut numbers = items.iter().map(|i| codes.number(i)).collect::<Result<Vec<_>, _>>()?;
                // Five places, as in the AIP files: one is picked, an empty one drops nothing.
                if numbers.len() < DROP_PLACES {
                    numbers.resize(DROP_PLACES, 0);
                }
                A::DropRandomItem(numbers)
            }
            E::KillSelf => A::KillSelf,
            E::UseSkill { skill_id, target, motion } => A::UseSkill((*target).into(), *skill_id as i32, *motion),
            E::SetVariable { variable_type, variable, operator, value } => {
                A::SetVariable((*variable_type).into(), *variable as usize, (*operator).into(), *value)
            }
            E::Message { message_type, text } => A::Message((*message_type).into(), number(text)),
            E::MoveNearOwner => A::MoveNearOwner,
            E::QuestStep { trigger_name } => A::DoQuestTrigger(trigger_name.clone()),
            E::AttackOwnerTarget => A::AttackOwnerTarget,
            E::SetPvp { zone, on } => A::SetPvpFlag(zone_of(*zone), *on),
            E::SetMonsterSpawns { zone, state } => A::SetMonsterSpawnState(zone_of(*zone), (*state).into()),
            E::GiveItemToOwner { item, count } => A::GiveItemToOwner(codes.number(item)?, *count as usize),
        })
    }

    /// The monster or NPC it names, for the links (0 = none).
    pub fn npc_mut(&mut self) -> Option<&mut u16> {
        match self {
            AiActionExport::Transform { npc_id } | AiActionExport::Spawn { npc_id, .. } => Some(npc_id),
            AiActionExport::CallAllies { npc_id, allies: Allies::OfMonster, .. } => Some(npc_id),
            _ => None,
        }
    }
}

/// One event: when its conditions all pass, its actions run (and no later event of the list).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AiEventExport {
    /// A label (most have none).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub condition: Vec<AiConditionExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub action: Vec<AiActionExport>,
}

/// One AI: how a kind of monster (or town NPC) behaves.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AiFileExport {
    /// The number (iROSE's FILE_AI.STB row). Monsters and NPCs name their AI by it.
    pub id: u16,
    /// The AIP file's name (`de_zs`).
    pub name: String,
    /// Seconds between two runs of `on_idle`.
    pub idle_interval: u32,
    /// Chance in percent that a monster already fighting runs `on_damaged` when someone else
    /// hits it.
    pub new_target_chance: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub on_created: Vec<AiEventExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_idle: Vec<AiEventExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_attack: Vec<AiEventExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_damaged: Vec<AiEventExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_kill: Vec<AiEventExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_dead: Vec<AiEventExport>,
}

/// The event lists' keys, in the AIP files' order of triggers.
pub const EVENT_LISTS: [&str; 6] = ["on_created", "on_idle", "on_attack", "on_damaged", "on_kill", "on_dead"];

impl AiFileExport {
    pub fn lists(&self) -> [&Vec<AiEventExport>; 6] {
        [&self.on_created, &self.on_idle, &self.on_attack, &self.on_damaged, &self.on_kill, &self.on_dead]
    }

    pub fn lists_mut(&mut self) -> [&mut Vec<AiEventExport>; 6] {
        [
            &mut self.on_created,
            &mut self.on_idle,
            &mut self.on_attack,
            &mut self.on_damaged,
            &mut self.on_kill,
            &mut self.on_dead,
        ]
    }

    pub fn events(&self) -> impl Iterator<Item = &AiEventExport> {
        self.lists().into_iter().flatten()
    }

    pub fn from_aip(id: u16, name: &str, aip: &AipFile, codes: &ItemCodes, strings: &HashMap<u16, String>) -> Result<Self, anyhow::Error> {
        let text = |id: usize| u16::try_from(id).ok().and_then(|id| strings.get(&id)).cloned().unwrap_or_default();
        let events = |trigger: &Option<AipTrigger>| -> Result<Vec<AiEventExport>, anyhow::Error> {
            let Some(trigger) = trigger else { return Ok(Vec::new()) };
            trigger
                .events
                .iter()
                .map(|e| {
                    Ok(AiEventExport {
                        name: e.name.clone(),
                        condition: e.conditions.iter().map(AiConditionExport::from_aip).collect::<Result<_, _>>()?,
                        action: e.actions.iter().map(|a| AiActionExport::from_aip(a, codes, text)).collect::<Result<_, _>>()?,
                    })
                })
                .collect()
        };
        Ok(Self {
            id,
            name: name.to_string(),
            idle_interval: aip.idle_trigger_interval.as_secs() as u32,
            new_target_chance: aip.damage_trigger_new_target_chance,
            on_created: events(&aip.trigger_on_created).context("on_created")?,
            on_idle: events(&aip.trigger_on_idle).context("on_idle")?,
            on_attack: events(&aip.trigger_on_attack_move).context("on_attack")?,
            on_damaged: events(&aip.trigger_on_damaged).context("on_damaged")?,
            on_kill: events(&aip.trigger_on_kill).context("on_kill")?,
            on_dead: events(&aip.trigger_on_dead).context("on_dead")?,
        })
    }

    /// The AIP file the server runs. Every trigger is there, even with no events (a monster
    /// with no `on_damaged` events doesn't fight back).
    fn to_aip(&self, codes: &ItemCodes, number: &mut impl FnMut(&str) -> usize) -> Result<AipFile, anyhow::Error> {
        let mut trigger = |name: &str, events: &[AiEventExport]| -> Result<Option<AipTrigger>, anyhow::Error> {
            let events = events
                .iter()
                .map(|e| {
                    Ok(AipEvent {
                        name: e.name.clone(),
                        conditions: e.condition.iter().map(AiConditionExport::to_aip).collect(),
                        actions: e.action.iter().map(|a| a.to_aip(codes, &mut *number)).collect::<Result<_, anyhow::Error>>()?,
                    })
                })
                .collect::<Result<_, anyhow::Error>>()
                .with_context(|| name.to_string())?;
            Ok(Some(AipTrigger { name: name.to_string(), events }))
        };
        Ok(AipFile {
            idle_trigger_interval: Duration::from_secs(self.idle_interval as u64),
            damage_trigger_new_target_chance: self.new_target_chance,
            trigger_on_created: trigger(EVENT_LISTS[0], &self.on_created)?,
            trigger_on_idle: trigger(EVENT_LISTS[1], &self.on_idle)?,
            trigger_on_attack_move: trigger(EVENT_LISTS[2], &self.on_attack)?,
            trigger_on_damaged: trigger(EVENT_LISTS[3], &self.on_damaged)?,
            trigger_on_kill: trigger(EVENT_LISTS[4], &self.on_kill)?,
            trigger_on_dead: trigger(EVENT_LISTS[5], &self.on_dead)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AiFile {
    #[serde(default)]
    ai: Vec<AiFileExport>,
}

/// The monsters and NPCs, skills and quest steps in the game, for `without_missing`.
#[derive(Debug, Default)]
pub struct AiKnown {
    pub npcs: BTreeSet<u16>,
    pub skills: BTreeSet<u16>,
    pub steps: BTreeSet<String>,
}

/// What the AIs name, for the export's checks.
#[derive(Debug, Default)]
pub struct AiNames<'a> {
    pub npcs: BTreeSet<u16>,
    pub skills: BTreeSet<u16>,
    pub zones: BTreeSet<u16>,
    pub items: BTreeSet<&'a str>,
    pub steps: BTreeSet<&'a str>,
}

/// Every AI, as exported.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AiExport {
    pub ai: Vec<AiFileExport>,
}

impl AiExport {
    /// The converter's starting point and the same-data check. The AIs sorted by number, each
    /// named after its AIP file (`names`, by number).
    pub fn from_database(db: &AiDatabase, names: &BTreeMap<u16, String>, codes: &ItemCodes) -> Result<Self, anyhow::Error> {
        let mut ai = db
            .aips
            .iter()
            .map(|(id, aip)| {
                let name = names.get(id).cloned().unwrap_or_default();
                AiFileExport::from_aip(*id, &name, aip, codes, &db.strings).with_context(|| format!("AI {id} ({name})"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        ai.sort_by_key(|a| a.id);
        Ok(Self { ai })
    }

    /// The AI database the server runs. The texts get numbers from 1 in the order they come.
    pub fn to_database(&self, codes: &ItemCodes) -> Result<AiDatabase, anyhow::Error> {
        self.check()?;
        let mut strings: HashMap<u16, String> = HashMap::new();
        let mut numbers: HashMap<String, usize> = HashMap::new();
        let mut number = |text: &str| -> usize {
            if text.is_empty() {
                return 0;
            }
            let next = numbers.len() + 1;
            *numbers.entry(text.to_string()).or_insert_with(|| {
                strings.insert(next as u16, text.to_string());
                next
            })
        };
        let mut aips = HashMap::new();
        for ai in &self.ai {
            let aip = ai.to_aip(codes, &mut number).with_context(|| format!("AI {} ({})", ai.id, ai.name))?;
            aips.insert(ai.id, aip);
        }
        if strings.len() > u16::MAX as usize {
            bail!("more than {} AI texts", u16::MAX);
        }
        Ok(AiDatabase { strings, aips })
    }

    /// Numbers used twice or 0, and names missing.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut ids = BTreeSet::new();
        for ai in &self.ai {
            if ai.id == 0 {
                bail!("{AI_FILE}: AI 0: numbers start at 1 (0 is no AI)");
            }
            if !ids.insert(ai.id) {
                bail!("{AI_FILE}: AI {} is there twice", ai.id);
            }
            for event in ai.events() {
                for action in &event.action {
                    if let AiActionExport::QuestStep { trigger_name } = action {
                        if trigger_name.is_empty() {
                            bail!("AI {} ({}): a QuestStep action names no quest step", ai.id, ai.name);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Folds away what isn't in the game, with what the game did with it: a monster or NPC a
    /// block names becomes 0 (none: nothing is found, spawned or called), and an action
    /// casting a missing skill or running a missing quest step goes (it did nothing). Returns
    /// what was changed.
    pub fn without_missing(&mut self, known: &AiKnown) -> Vec<String> {
        let mut changed = Vec::new();
        for ai in &mut self.ai {
            let (id, name) = (ai.id, ai.name.clone());
            for event in ai.lists_mut().into_iter().flatten() {
                event.action.retain(|action| match action {
                    AiActionExport::UseSkill { skill_id, .. } if !known.skills.contains(skill_id) => {
                        changed.push(format!("AI {id} ({name}) uses skill {skill_id}, which isn't in the game (left out)"));
                        false
                    }
                    AiActionExport::QuestStep { trigger_name } if !known.steps.contains(trigger_name) => {
                        changed.push(format!("AI {id} ({name}) runs quest step {trigger_name}, which isn't there (left out)"));
                        false
                    }
                    _ => true,
                });
                let npcs = event
                    .condition
                    .iter_mut()
                    .filter_map(AiConditionExport::npc_mut)
                    .chain(event.action.iter_mut().filter_map(AiActionExport::npc_mut));
                for npc in npcs.filter(|n| **n != 0 && !known.npcs.contains(*n)) {
                    changed.push(format!("AI {id} ({name}) names monster or NPC {npc}, which isn't in the game (left empty)"));
                    *npc = 0;
                }
            }
        }
        changed
    }

    /// The monsters and NPCs, skills, zones, items and quest steps the AIs name.
    pub fn named(&self) -> AiNames<'_> {
        let mut names = AiNames::default();
        for event in self.ai.iter().flat_map(|a| a.events()) {
            for condition in &event.condition {
                if let AiConditionExport::SelectNpc { npc_id } = condition {
                    names.npcs.insert(*npc_id);
                }
            }
            for action in &event.action {
                match action {
                    AiActionExport::Transform { npc_id } | AiActionExport::Spawn { npc_id, .. } => {
                        names.npcs.insert(*npc_id);
                    }
                    AiActionExport::CallAllies { npc_id, allies: Allies::OfMonster, .. } => {
                        names.npcs.insert(*npc_id);
                    }
                    AiActionExport::UseSkill { skill_id, .. } => {
                        names.skills.insert(*skill_id);
                    }
                    AiActionExport::SetPvp { zone, .. } | AiActionExport::SetMonsterSpawns { zone, .. } => {
                        names.zones.insert(*zone);
                    }
                    AiActionExport::DropRandomItem { items } => names.items.extend(items.iter().map(String::as_str)),
                    AiActionExport::GiveItemToOwner { item, .. } => {
                        names.items.insert(item);
                    }
                    AiActionExport::QuestStep { trigger_name } => {
                        names.steps.insert(trigger_name);
                    }
                    _ => {}
                }
            }
        }
        for set in [&mut names.npcs, &mut names.skills, &mut names.zones] {
            set.remove(&0);
        }
        names.items.remove("");
        names
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut ai = self.ai.clone();
        ai.sort_by_key(|a| a.id);
        Ok(vec![(AI_FILE, format.write(AI_FILE, &AiFile { ai })?)])
    }

    /// Reads the export's file through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when it isn't there.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let Some(text) = read(AI_FILE)? else { return Ok(None) };
        Ok(Some(Self { ai: format.parse::<AiFile>(AI_FILE, &text)?.ai }))
    }
}

/// Every difference between two AI databases, compared as their export (so a text counts by
/// what it says, not its number). AIs are matched by number; names are left out (the AI
/// database doesn't keep them). What `known` doesn't have is folded away on both sides, as the
/// converter does (`AiExport::without_missing`).
pub fn compare_ai_databases(
    a: &AiDatabase,
    b: &AiDatabase,
    codes: &ItemCodes,
    known: &AiKnown,
) -> Result<Vec<String>, anyhow::Error> {
    let names = BTreeMap::new();
    let normal = |db: &AiDatabase| -> Result<AiExport, anyhow::Error> {
        let mut export = AiExport::from_database(db, &names, codes)?;
        export.without_missing(known);
        Ok(export)
    };
    let (a, b) = (normal(a)?, normal(b)?);
    let by_id = |e: &AiExport| e.ai.iter().map(|ai| (ai.id, format!("{ai:#?}"))).collect::<BTreeMap<_, _>>();
    let (x, y) = (by_id(&a), by_id(&b));
    let mut out = Vec::new();
    let keys: BTreeSet<&u16> = x.keys().chain(y.keys()).collect();
    for key in keys {
        match (x.get(key), y.get(key)) {
            (Some(_), None) => out.push(format!("AI {key}: missing")),
            (None, Some(_)) => out.push(format!("AI {key}: extra")),
            (Some(l), Some(r)) if l != r => {
                for (l, r) in l.lines().zip(r.lines()).filter(|(l, r)| l != r) {
                    out.push(format!("AI {key}: {} -> {}", l.trim(), r.trim()));
                }
                if l.lines().count() != r.lines().count() {
                    out.push(format!("AI {key}: the events differ"));
                }
            }
            _ => {}
        }
    }
    Ok(out)
}
