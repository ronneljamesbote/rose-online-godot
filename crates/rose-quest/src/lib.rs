//! Quest trigger conditions (QSD files), shared by the server module and the client.
//!
//! Ported from rose-offline-server's quest_system: the server checks a trigger's conditions
//! and then applies its rewards, the client checks the conditions to build NPC conversation
//! menus before it asks the server to run a trigger. Both describe the character with a
//! [`QuestCharacter`] and answer questions about the world through [`QuestWorld`].

use std::ops::RangeInclusive;

use rose_data::{AbilityType, DataDecoder, QuestDatabase, QuestTrigger, QuestTriggerHash, WorldTicks};
use rose_file_readers::{QsdCondition, QsdConditionOperator, QsdItem, QsdObjectType, QsdRewardOperator, QsdVariableType};
use rose_game_common::components::{AbilityValues, BasicStats, Equipment, Inventory, QuestState, SkillList, UnionMembership};

/// Ten seconds of real time per world tick (WORLD_TICK_DURATION), counted from the Unix epoch
/// so the server and every client agree on it without syncing.
pub fn world_ticks(unix_us: i64) -> WorldTicks {
    WorldTicks((unix_us.max(0) / 10_000_000) as u64)
}

/// What the quest rules read about the character running a trigger.
#[derive(Clone, Debug)]
pub struct QuestCharacter {
    pub gender: u8,
    pub face: u8,
    pub hair: u8,
    pub job: u16,
    pub level: u32,
    pub xp: u64,
    pub stat_points: u32,
    pub skill_points: u32,
    pub basic_stats: BasicStats,
    pub ability_values: AbilityValues,
    pub hp: i32,
    pub mp: i32,
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
    pub team: u32,
    pub equipment: Equipment,
    pub inventory: Inventory,
    pub skill_list: SkillList,
    pub quest_state: QuestState,
    pub union_membership: UnionMembership,
}

/// A town NPC as the quest rules see it.
#[derive(Clone, Copy, Debug)]
pub struct QuestNpc {
    pub entity_id: u64,
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
}

/// The character's party, for the Party conditions.
#[derive(Clone, Copy, Debug)]
pub struct QuestParty {
    pub is_leader: bool,
    pub level: i32,
    pub member_count: usize,
}

/// Real-world time for the MonthDayTime and WeekDayTime conditions.
#[derive(Clone, Copy, Debug, Default)]
pub struct RealTime {
    /// Day of the month, 1-31.
    pub month_day: u32,
    /// 0 is Sunday.
    pub week_day: u32,
    /// Minutes since midnight.
    pub day_minutes: i32,
}

impl RealTime {
    /// UTC time from microseconds since the Unix epoch.
    pub fn from_unix_us(unix_us: i64) -> Self {
        let secs = unix_us.max(0) / 1_000_000;
        let days = secs / 86_400;
        let day_minutes = ((secs % 86_400) / 60) as i32;
        // 1970-01-01 was a Thursday.
        let week_day = ((days + 4) % 7) as u32;
        // Civil date from days since the epoch (Howard Hinnant's algorithm).
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let month_day = (doy - (153 * mp + 2) / 5 + 1) as u32;
        Self { month_day, week_day, day_minutes }
    }
}

/// What the quest rules ask about the world around the character.
pub trait QuestWorld {
    fn world_ticks(&self) -> WorldTicks;
    fn real_time(&self) -> RealTime;
    /// A roll for RandomPercent, 0-99, or None to let the condition pass (the client leaves
    /// random rolls to the server).
    fn random_percent(&mut self) -> Option<u8>;
    /// The NPC with this NPC id, if one is spawned.
    fn find_npc(&self, npc_id: u16) -> Option<QuestNpc>;
    /// One of an NPC's twenty object variables.
    fn npc_variable(&self, npc_entity_id: u64, variable_id: usize) -> Option<i32>;
    fn party(&self) -> Option<QuestParty>;
    /// LIST_CLASS, for job conditions.
    fn job_classes(&self) -> Option<&rose_data::JobClassDatabase>;
}

/// Whether a job belongs to a LIST_CLASS row (CUserDATA::Check_JobCollection): a row
/// whose first job is 0 (row 0, Visitor, and blank rows) takes everyone, a row past the
/// table takes no one.
pub fn job_in_class(classes: &rose_data::JobClassDatabase, class_id: i32, job: u16) -> bool {
    if class_id < 0 || class_id as usize >= classes.len() {
        return false;
    }
    match u16::try_from(class_id).ok().and_then(rose_data::JobClassId::new).and_then(|id| classes.get(id)) {
        Some(class) => class.jobs.is_empty() || class.jobs.iter().any(|j| j.get() == job),
        None => true,
    }
}

/// State carried from one condition or reward to the next while a trigger chain runs.
#[derive(Clone, Debug, Default)]
pub struct QuestContext {
    pub selected_quest_index: Option<usize>,
    pub selected_npc: Option<QuestNpc>,
    pub next_trigger_name: Option<String>,
}

pub fn condition_operator<T: PartialOrd>(operator: QsdConditionOperator, lhs: T, rhs: T) -> bool {
    match operator {
        QsdConditionOperator::Equals => lhs == rhs,
        QsdConditionOperator::GreaterThan => lhs > rhs,
        QsdConditionOperator::GreaterThanEqual => lhs >= rhs,
        QsdConditionOperator::LessThan => lhs < rhs,
        QsdConditionOperator::LessThanEqual => lhs <= rhs,
        QsdConditionOperator::NotEqual => lhs != rhs,
    }
}

pub fn reward_operator(operator: QsdRewardOperator, current: i32, value: i32) -> i32 {
    match operator {
        QsdRewardOperator::Set => value,
        QsdRewardOperator::Add => current + value,
        QsdRewardOperator::Subtract => current - value,
        QsdRewardOperator::Zero => 0,
        QsdRewardOperator::One => 1,
    }
}

/// Index into UnionMembership::points for UnionPoint1..UnionPoint10.
pub fn union_point_index(ability: AbilityType) -> Option<usize> {
    Some(match ability {
        AbilityType::UnionPoint1 => 0,
        AbilityType::UnionPoint2 => 1,
        AbilityType::UnionPoint3 => 2,
        AbilityType::UnionPoint4 => 3,
        AbilityType::UnionPoint5 => 4,
        AbilityType::UnionPoint6 => 5,
        AbilityType::UnionPoint7 => 6,
        AbilityType::UnionPoint8 => 7,
        AbilityType::UnionPoint9 => 8,
        AbilityType::UnionPoint10 => 9,
        _ => return None,
    })
}

/// A character value by AbilityType (rose-offline's ability_values_get_value). Values the game
/// does not track yet (stamina, rank, fame) read as 0.
pub fn ability_value(ch: &QuestCharacter, ability: AbilityType) -> Option<i32> {
    let av = &ch.ability_values;
    Some(match ability {
        AbilityType::Gender => ch.gender as i32,
        AbilityType::Job => ch.job as i32,
        AbilityType::Face => ch.face as i32,
        AbilityType::Hair => ch.hair as i32,
        AbilityType::Strength => av.get_strength(),
        AbilityType::Dexterity => av.get_dexterity(),
        AbilityType::Intelligence => av.get_intelligence(),
        AbilityType::Concentration => av.get_concentration(),
        AbilityType::Charm => av.get_charm(),
        AbilityType::Sense => av.get_sense(),
        AbilityType::Attack => av.get_attack_power(),
        AbilityType::Defence => av.get_defence(),
        AbilityType::Hit => av.get_hit(),
        AbilityType::Resistance => av.get_resistance(),
        AbilityType::Avoid => av.get_avoid(),
        AbilityType::AttackSpeed => av.get_attack_speed(),
        AbilityType::Critical => av.get_critical(),
        AbilityType::Speed => av.get_run_speed() as i32,
        AbilityType::Skillpoint => ch.skill_points as i32,
        AbilityType::BonusPoint => ch.stat_points as i32,
        AbilityType::Experience => ch.xp.min(i32::MAX as u64) as i32,
        AbilityType::Level => ch.level as i32,
        AbilityType::Money => ch.inventory.money.0.min(i32::MAX as i64) as i32,
        AbilityType::MaxHealth => av.get_max_health(),
        AbilityType::MaxMana => av.get_max_mana(),
        AbilityType::Health => ch.hp,
        AbilityType::Mana => ch.mp,
        AbilityType::SaveMana => av.get_save_mana(),
        AbilityType::DropRate => av.get_drop_rate(),
        AbilityType::Union => ch.union_membership.current_union.map_or(0, |u| u.get() as i32),
        AbilityType::Rank | AbilityType::Fame | AbilityType::Stamina => 0,
        other => match union_point_index(other) {
            Some(i) => ch.union_membership.points[i] as i32,
            None => return None,
        },
    })
}

pub fn get_quest_variable(
    ch: &QuestCharacter,
    cx: &QuestContext,
    now: WorldTicks,
    variable_type: QsdVariableType,
    variable_id: usize,
) -> Option<i32> {
    let quest_state = &ch.quest_state;
    let active_quest = cx.selected_quest_index.and_then(|index| quest_state.get_quest(index));
    match variable_type {
        QsdVariableType::Variable => active_quest.and_then(|q| q.variables.get(variable_id)).map(|x| *x as i32),
        QsdVariableType::Switch => active_quest.and_then(|q| q.switches.get(variable_id)).map(|x| *x as i32),
        QsdVariableType::Timer => active_quest
            .and_then(|q| q.expire_time)
            .map(|expire| expire.0.saturating_sub(now.0) as i32),
        QsdVariableType::Episode => quest_state.episode_variables.get(variable_id).map(|x| *x as i32),
        QsdVariableType::Job => quest_state.job_variables.get(variable_id).map(|x| *x as i32),
        QsdVariableType::Planet => quest_state.planet_variables.get(variable_id).map(|x| *x as i32),
        QsdVariableType::Union => quest_state.union_variables.get(variable_id).map(|x| *x as i32),
    }
}

pub fn set_quest_variable(
    ch: &mut QuestCharacter,
    cx: &QuestContext,
    variable_type: QsdVariableType,
    variable_id: usize,
    value: i32,
) {
    let quest_state = &mut ch.quest_state;
    let value16 = value.clamp(0, u16::MAX as i32) as u16;
    match variable_type {
        QsdVariableType::Variable => {
            if let Some(x) = cx
                .selected_quest_index
                .and_then(|index| quest_state.get_quest_mut(index))
                .and_then(|q| q.variables.get_mut(variable_id))
            {
                *x = value16;
            }
        }
        QsdVariableType::Switch => {
            if let Some(q) = cx.selected_quest_index.and_then(|index| quest_state.get_quest_mut(index)) {
                if let Some(mut x) = q.switches.get_mut(variable_id) {
                    *x = value != 0;
                }
            }
        }
        QsdVariableType::Episode => set(&mut quest_state.episode_variables, variable_id, value16),
        QsdVariableType::Job => set(&mut quest_state.job_variables, variable_id, value16),
        QsdVariableType::Planet => set(&mut quest_state.planet_variables, variable_id, value16),
        QsdVariableType::Union => set(&mut quest_state.union_variables, variable_id, value16),
        QsdVariableType::Timer => {}
    }
}

fn set(variables: &mut [u16], id: usize, value: u16) {
    if let Some(x) = variables.get_mut(id) {
        *x = value;
    }
}

/// How many of an item the character has: quest items count in the selected quest, other
/// items in the inventory.
pub fn item_quantity(decoder: &dyn DataDecoder, ch: &QuestCharacter, cx: &QuestContext, item: QsdItem) -> u32 {
    let Some(reference) = decoder.decode_item_reference(item.item_number, item.item_type) else { return 0 };
    if reference.item_type.is_quest_item() {
        cx.selected_quest_index
            .and_then(|index| ch.quest_state.get_quest(index))
            .and_then(|q| q.find_item(reference))
            .map_or(0, |i| i.get_quantity())
    } else {
        ch.inventory
            .find_item(reference)
            .and_then(|slot| ch.inventory.get_item(slot))
            .map_or(0, |i| i.get_quantity())
    }
}

fn has_skill(ch: &QuestCharacter, range: &RangeInclusive<usize>, have: bool) -> bool {
    for page in ch.skill_list.pages.iter() {
        for skill_id in page.skills.iter().flatten() {
            if range.contains(&(skill_id.get() as usize)) {
                return have;
            }
        }
    }
    !have
}

fn object_npc(cx: &QuestContext, object: QsdObjectType) -> Option<QuestNpc> {
    match object {
        QsdObjectType::SelectedNpc => cx.selected_npc,
        // Event objects (zone IFO event points) are not in the game yet.
        QsdObjectType::SelectedEvent | QsdObjectType::QuestOwner => None,
    }
}

/// Check every condition of one trigger, in order (quest_trigger_check_conditions).
/// SelectQuest and SelectNpc conditions select into `cx` as they pass.
pub fn check_conditions(
    decoder: &dyn DataDecoder,
    ch: &QuestCharacter,
    world: &mut dyn QuestWorld,
    cx: &mut QuestContext,
    trigger: &QuestTrigger,
) -> bool {
    for condition in trigger.conditions.iter() {
        let ok = match *condition {
            // A job condition names a LIST_CLASS row (a set of jobs) and ignores its operator
            // (Check_UserVAR in io_quest.cpp): "Job 1" means any soldier job.
            QsdCondition::AbilityValue { ability_type, value, .. }
                if decoder.decode_ability_type(ability_type.get()) == Some(AbilityType::Job) =>
            {
                world.job_classes().is_some_and(|classes| job_in_class(classes, value, ch.job))
            }
            QsdCondition::AbilityValue { ability_type, operator, value } => decoder
                .decode_ability_type(ability_type.get())
                .and_then(|ability| ability_value(ch, ability))
                .map_or(false, |current| condition_operator(operator, current, value)),
            QsdCondition::SelectQuest { id } => match ch.quest_state.find_active_quest_index(id) {
                Some(index) => {
                    cx.selected_quest_index = Some(index);
                    true
                }
                None => false,
            },
            QsdCondition::QuestItem { item, equipment_index, required_count, operator } => {
                match equipment_index.and_then(|i| decoder.decode_equipment_index(i.get())) {
                    Some(equipment_index) => {
                        item.and_then(|item| decoder.decode_item_reference(item.item_number, item.item_type))
                            == ch.equipment.get_equipment_item(equipment_index).map(|i| i.item)
                    }
                    None => {
                        let quantity = item.map_or(0, |item| item_quantity(decoder, ch, cx, item));
                        condition_operator(operator, quantity, required_count)
                    }
                }
            }
            QsdCondition::QuestSwitch { id, value } => {
                ch.quest_state.quest_switches.get(id).map_or(false, |switch| *switch == value)
            }
            QsdCondition::Position { zone, x, y, distance } => {
                ch.zone_id as usize == zone && {
                    let (dx, dy) = (ch.x - x, ch.y - y);
                    dx * dx + dy * dy < distance as f32 * distance as f32
                }
            }
            QsdCondition::QuestVariable { variable_type, variable_id, operator, value } => {
                get_quest_variable(ch, cx, world.world_ticks(), variable_type, variable_id)
                    .map_or(false, |current| condition_operator(operator, current, value))
            }
            QsdCondition::WorldTime { ref range } => range.contains(&world.world_ticks().get_world_time()),
            QsdCondition::MonthDayTime { month_day, ref day_minutes_range } => {
                let now = world.real_time();
                month_day.map_or(true, |day| day.get() as u32 == now.month_day)
                    && day_minutes_range.contains(&now.day_minutes)
            }
            QsdCondition::WeekDayTime { week_day, ref day_minutes_range } => {
                let now = world.real_time();
                week_day as u32 == now.week_day && day_minutes_range.contains(&now.day_minutes)
            }
            QsdCondition::HasSkill { id, has_skill: have } => has_skill(ch, &(id..=id), have),
            QsdCondition::HasSkillInRange { ref range, has_skill: have } => has_skill(ch, range, have),
            QsdCondition::TeamNumber { ref range } => range.contains(&(ch.team as usize)),
            // One channel.
            QsdCondition::ServerChannelNumber { ref range } => range.contains(&1),
            QsdCondition::SelectNpc { id } => {
                cx.selected_npc = world.find_npc(id as u16);
                cx.selected_npc.is_some()
            }
            QsdCondition::ObjectVariable { object, variable_id, operator, value } => object_npc(cx, object)
                .and_then(|npc| world.npc_variable(npc.entity_id, variable_id))
                .map_or(false, |current| condition_operator(operator, current, value)),
            QsdCondition::ObjectDistance { object, distance } => object_npc(cx, object)
                .filter(|npc| npc.zone_id == ch.zone_id)
                .map_or(false, |npc| {
                    let (dx, dy) = (ch.x - npc.x, ch.y - npc.y);
                    ((dx * dx + dy * dy).sqrt() as i32) < distance
                }),
            QsdCondition::ObjectZoneTime { ref time_range, .. } => {
                // Zones have no day cycle yet: use the world time.
                time_range.contains(&world.world_ticks().get_world_time())
            }
            QsdCondition::CompareNpcVariables { npc_id_1, variable_id_1, operator, npc_id_2, variable_id_2 } => {
                let value = |npc_id: usize, variable_id: usize| {
                    world
                        .find_npc(npc_id as u16)
                        .and_then(|npc| world.npc_variable(npc.entity_id, variable_id))
                        .unwrap_or(0)
                };
                condition_operator(operator, value(npc_id_1, variable_id_1), value(npc_id_2, variable_id_2))
            }
            QsdCondition::Party { is_leader, level_operator, level } => world
                .party()
                .map_or(false, |party| (!is_leader || party.is_leader) && condition_operator(level_operator, party.level, level)),
            QsdCondition::PartyMemberCount { ref range } => {
                world.party().map_or(false, |party| range.contains(&party.member_count))
            }
            QsdCondition::RandomPercent { ref range } => world.random_percent().map_or(true, |roll| range.contains(&roll)),
            QsdCondition::SelectEventObject { .. } => false,
            // No clans yet: the character is never in one.
            QsdCondition::HasClan { has_clan } => !has_clan,
            QsdCondition::HasClanSkill { has_skill, .. } | QsdCondition::HasClanSkillInRange { has_skill, .. } => !has_skill,
            QsdCondition::ClanPosition { .. }
            | QsdCondition::ClanPointContribution { .. }
            | QsdCondition::ClanLevel { .. }
            | QsdCondition::ClanPoints { .. }
            | QsdCondition::ClanMoney { .. }
            | QsdCondition::ClanMemberCount { .. } => false,
        };
        if !ok {
            log::trace!(target: "quest", "condition failed {condition:?}");
            return false;
        }
    }
    true
}

/// The first trigger of a chain, by name.
pub fn find_trigger<'a>(quests: &'a QuestDatabase, name: &str) -> Option<&'a QuestTrigger> {
    quests
        .get_trigger_by_name(name)
        .or_else(|| quests.get_trigger_by_hash(QuestTriggerHash::from(name)))
}

/// Whether a trigger chain would succeed, without applying rewards (the client's
/// quest_check_conditions): walk the chain, following a passing trigger's Trigger reward
/// or a failing trigger's next trigger.
pub fn check_trigger(
    quests: &QuestDatabase,
    decoder: &dyn DataDecoder,
    ch: &QuestCharacter,
    world: &mut dyn QuestWorld,
    name: &str,
) -> bool {
    let mut cx = QuestContext::default();
    let mut trigger = find_trigger(quests, name);
    let mut success = false;
    let mut steps = 0;
    while let Some(t) = trigger {
        steps += 1;
        if steps > 64 {
            break;
        }
        if check_conditions(decoder, ch, world, &mut cx, t) {
            success = true;
            let next = t.rewards.iter().rev().find_map(|r| match r {
                rose_file_readers::QsdReward::Trigger { name } => Some(name.clone()),
                _ => None,
            });
            trigger = next.and_then(|name| quests.get_trigger_by_name(&name));
        } else {
            trigger = t.next_trigger_name.as_ref().and_then(|name| quests.get_trigger_by_name(name));
        }
    }
    success
}
