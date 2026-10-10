//! Quest pages: who gives the quest, its text, and every step (quest trigger) with what it
//! checks and what it gives.

use std::{collections::BTreeSet, time::Duration};

use rose_file_readers::{
    QsdCondition, QsdConditionOperator, QsdObjectType, QsdReward, QsdRewardOperator, QsdSpawnMonsterLocation,
    QsdVariableType,
};

use crate::{
    page::{natural_key, rose_text, row, strs, Page, Pages},
    Ctx,
};

fn op(o: QsdConditionOperator) -> &'static str {
    match o {
        QsdConditionOperator::Equals => "=",
        QsdConditionOperator::GreaterThan => ">",
        QsdConditionOperator::GreaterThanEqual => "≥",
        QsdConditionOperator::LessThan => "<",
        QsdConditionOperator::LessThanEqual => "≤",
        QsdConditionOperator::NotEqual => "≠",
    }
}

fn set_op(o: QsdRewardOperator, what: &str, value: i32) -> String {
    match o {
        QsdRewardOperator::Set => format!("set {what} to {value}"),
        QsdRewardOperator::Add => format!("add {value} to {what}"),
        QsdRewardOperator::Subtract => format!("take {value} from {what}"),
        QsdRewardOperator::Zero => format!("set {what} to 0"),
        QsdRewardOperator::One => format!("set {what} to 1"),
    }
}

fn variable(t: QsdVariableType, id: usize) -> String {
    match t {
        QsdVariableType::Variable => format!("quest variable {id}"),
        QsdVariableType::Switch => format!("quest switch {id}"),
        QsdVariableType::Timer => "the quest timer".into(),
        QsdVariableType::Episode => format!("episode variable {id}"),
        QsdVariableType::Job => format!("job variable {id}"),
        QsdVariableType::Planet => format!("planet variable {id}"),
        QsdVariableType::Union => format!("union variable {id}"),
    }
}

fn object(o: QsdObjectType) -> &'static str {
    match o {
        QsdObjectType::SelectedNpc => "the NPC",
        QsdObjectType::SelectedEvent => "the event object",
        QsdObjectType::QuestOwner => "you",
    }
}

fn skill(cx: &Ctx, id: usize) -> String {
    cx.skill_link(id as u16)
}

pub fn condition(cx: &Ctx, c: &QsdCondition) -> String {
    match c {
        QsdCondition::SelectQuest { id } => format!("you have {}", cx.quest_link(*id)),
        QsdCondition::QuestVariable { variable_type, variable_id, operator, value } => {
            format!("{} {} {value}", variable(*variable_type, *variable_id), op(*operator))
        }
        QsdCondition::AbilityValue { ability_type, operator, value } => {
            format!("your {} {} {value}", cx.ability_id_name(ability_type.get()), op(*operator))
        }
        QsdCondition::QuestItem { item, equipment_index, required_count, operator } => match item {
            Some(i) => format!("you carry {} {required_count} × {}", op(*operator), cx.item_link_sn(i.to_sn())),
            None => format!("equipment slot {} is empty", equipment_index.map_or(0, |e| e.get())),
        },
        QsdCondition::Party { is_leader, level_operator, level } => {
            format!("you are in a party{} with party level {} {level}", if *is_leader { " as its leader" } else { "" }, op(*level_operator))
        }
        QsdCondition::Position { zone, x, y, distance } => format!(
            "you are within {} m of ({:.0}, {:.0}) in {}",
            distance / 100,
            x / 100.0,
            y / 100.0,
            cx.zone_link(*zone as u16)
        ),
        QsdCondition::WorldTime { range } => format!("world time is {}–{}", range.start(), range.end()),
        QsdCondition::HasSkill { id, has_skill } => {
            format!("you {} {}", if *has_skill { "have" } else { "don't have" }, skill(cx, *id))
        }
        QsdCondition::HasSkillInRange { range, has_skill } => format!(
            "you {} a skill from {} to {}",
            if *has_skill { "have" } else { "don't have" },
            skill(cx, *range.start()),
            skill(cx, *range.end())
        ),
        QsdCondition::RandomPercent { range } => format!("a random roll 0–99 lands in {}–{}", range.start(), range.end()),
        QsdCondition::ObjectVariable { object: o, variable_id, operator, value } => {
            format!("{}'s variable {variable_id} {} {value}", object(*o), op(*operator))
        }
        QsdCondition::SelectEventObject { zone, event_id, .. } => {
            format!("event object {event_id} in {}", cx.zone_link(*zone as u16))
        }
        QsdCondition::SelectNpc { id } => format!("the NPC {}", cx.npc_link(*id as u16)),
        QsdCondition::QuestSwitch { id, value } => format!("quest switch {id} is {}", if *value { "on" } else { "off" }),
        QsdCondition::PartyMemberCount { range } => format!("your party has {}–{} members", range.start(), range.end()),
        QsdCondition::ObjectZoneTime { object: o, time_range } => {
            format!("{}'s zone time is {}–{}", object(*o), time_range.start(), time_range.end())
        }
        QsdCondition::CompareNpcVariables { npc_id_1, variable_id_1, operator, npc_id_2, variable_id_2 } => format!(
            "{} variable {variable_id_1} {} {} variable {variable_id_2}",
            cx.npc_link(*npc_id_1 as u16),
            op(*operator),
            cx.npc_link(*npc_id_2 as u16)
        ),
        QsdCondition::MonthDayTime { month_day, day_minutes_range } => format!(
            "{}time of day {}–{} minutes",
            month_day.map_or(String::new(), |d| format!("day {d} of the month, ")),
            day_minutes_range.start(),
            day_minutes_range.end()
        ),
        QsdCondition::WeekDayTime { week_day, day_minutes_range } => format!(
            "week day {week_day}, time of day {}–{} minutes",
            day_minutes_range.start(),
            day_minutes_range.end()
        ),
        QsdCondition::TeamNumber { range } => format!("team number {}–{}", range.start(), range.end()),
        QsdCondition::ObjectDistance { object: o, distance } => format!("{} is within {} m", object(*o), distance / 100),
        QsdCondition::ServerChannelNumber { range } => format!("channel {}–{}", range.start(), range.end()),
        QsdCondition::HasClan { has_clan } => format!("you {} in a clan", if *has_clan { "are" } else { "are not" }),
        QsdCondition::ClanPosition { operator, value } => format!("your clan rank {} {value}", op(*operator)),
        QsdCondition::ClanPointContribution { operator, value } => format!("your clan points {} {value}", op(*operator)),
        QsdCondition::ClanLevel { operator, value } => format!("clan level {} {value}", op(*operator)),
        QsdCondition::ClanPoints { operator, value } => format!("clan points {} {value}", op(*operator)),
        QsdCondition::ClanMoney { operator, value } => format!("clan money {} {value}", op(*operator)),
        QsdCondition::ClanMemberCount { operator, value } => format!("clan members {} {value}", op(*operator)),
        QsdCondition::HasClanSkill { id, has_skill } => {
            format!("the clan {} {}", if *has_skill { "has" } else { "doesn't have" }, skill(cx, *id))
        }
        QsdCondition::HasClanSkillInRange { range, has_skill } => format!(
            "the clan {} a skill from {} to {}",
            if *has_skill { "has" } else { "doesn't have" },
            skill(cx, *range.start()),
            skill(cx, *range.end())
        ),
    }
}

/// What a QSD reward formula depends on (the formulas are on rules/quests).
fn equation(e: usize) -> String {
    let what = match e {
        0 => "base plus a bonus from Charm, smaller at higher levels",
        1 => "grows with your level and Charm",
        2 => "base × times the quest was repeated",
        3 | 5 => "base plus a bonus from Charm, smaller at higher levels",
        4 | 6 => "grows with level and Charm",
        _ => "unknown formula",
    };
    format!("reward formula {e}: {what}")
}

pub fn reward(cx: &Ctx, r: &QsdReward) -> String {
    match r {
        QsdReward::RemoveSelectedQuest => "the quest ends (removed from your list)".into(),
        QsdReward::AddQuest { id } => format!("you get the quest {}", cx.quest_link(*id)),
        QsdReward::ChangeSelectedQuest { id, keep_data } => format!(
            "the quest becomes {}{}",
            cx.quest_link(*id),
            if *keep_data { " (progress kept)" } else { "" }
        ),
        QsdReward::SelectQuest { id } => format!("works on {}", cx.quest_link(*id)),
        QsdReward::AddItem { item, quantity } => format!("you get {quantity} × {}", cx.item_link_sn(item.to_sn())),
        QsdReward::RemoveItem { item, quantity } => format!("{quantity} × {} is taken", cx.item_link_sn(item.to_sn())),
        QsdReward::QuestVariable { variable_type, variable_id, operator, value } => {
            set_op(*operator, &variable(*variable_type, *variable_id), *value)
        }
        QsdReward::AbilityValue { ability_type, operator, value } => {
            set_op(*operator, &format!("your {}", cx.ability_id_name(ability_type.get())), *value)
        }
        QsdReward::CalculatedExperiencePoints { equation: e, value } => {
            format!("experience, base {value} ({}, see [[rules/quests|Quests]])", equation(*e))
        }
        QsdReward::CalculatedMoney { equation: e, value } => {
            format!("Zuly, base {value} ({}, see [[rules/quests|Quests]])", equation(*e))
        }
        QsdReward::CalculatedItem { equation: e, value, item, .. } => {
            format!(
                "you get {}, base count {value} ({}, see [[rules/quests|Quests]])",
                cx.item_link_sn(item.to_sn()),
                equation(*e)
            )
        }
        QsdReward::SetHealthManaPercent { health_percent, mana_percent } => {
            format!("HP set to {health_percent}% and MP to {mana_percent}%")
        }
        QsdReward::Teleport { zone, x, y } => {
            format!("you are moved to ({}, {}) in {}", x / 100, y / 100, cx.zone_link(*zone as u16))
        }
        QsdReward::SpawnMonster { npc, count, location, .. } => format!(
            "{count} × {} appear{}",
            cx.npc_link(*npc as u16),
            match location {
                QsdSpawnMonsterLocation::QuestOwner => " around you".to_string(),
                QsdSpawnMonsterLocation::SelectedNpc => " around the NPC".to_string(),
                QsdSpawnMonsterLocation::SelectedEvent => " at the event object".to_string(),
                QsdSpawnMonsterLocation::Position { zone, x, y } => {
                    format!(" at ({:.0}, {:.0}) in {}", x / 100.0, y / 100.0, cx.zone_link(*zone as u16))
                }
            }
        ),
        QsdReward::Trigger { name } => format!("then runs step `{name}`"),
        QsdReward::ResetBasicStats => "your stats are reset".into(),
        QsdReward::ObjectVariable { object: o, variable_id, operator, value } => {
            set_op(*operator, &format!("{}'s variable {variable_id}", object(*o)), *value)
        }
        QsdReward::NpcMessage { string_id, .. } => format!(
            "the NPC says: \"{}\"",
            cx.game.quests.get_quest_string(*string_id as u16).map(|s| rose_text(s).replace('\n', " ")).unwrap_or_default()
        ),
        QsdReward::TriggerAfterDelay { delay, trigger, .. } => {
            format!("after {} s runs step `{trigger}`", delay.as_secs())
        }
        QsdReward::AddSkill { id } => format!("you learn {}", skill(cx, *id)),
        QsdReward::RemoveSkill { id } => format!("you lose {}", skill(cx, *id)),
        QsdReward::SetQuestSwitch { id, value } => format!("quest switch {id} {}", if *value { "on" } else { "off" }),
        QsdReward::ClearSwitchGroup { group } => format!("quest switch group {group} cleared"),
        QsdReward::ClearAllSwitches => "all quest switches cleared".into(),
        QsdReward::FormatAnnounceMessage { string_id, .. } => format!(
            "announcement: \"{}\"",
            cx.game.quests.get_quest_string(*string_id as u16).map(|s| rose_text(s).replace('\n', " ")).unwrap_or_default()
        ),
        QsdReward::TriggerForZoneTeam { zone, trigger, .. } => {
            format!("runs step `{trigger}` for a team in {}", cx.zone_link(*zone as u16))
        }
        QsdReward::SetTeamNumber { .. } => "your team number is set".into(),
        QsdReward::SetRevivePosition { x, y } => format!("your get-up point is set to ({:.0}, {:.0})", x / 100.0, y / 100.0),
        QsdReward::EnableMonsterSpawns { zone } => format!("monster spawns on in {}", cx.zone_link(*zone as u16)),
        QsdReward::DisableMonsterSpawns { zone } => format!("monster spawns off in {}", cx.zone_link(*zone as u16)),
        QsdReward::ToggleMonsterSpawns { zone } => format!("monster spawns toggled in {}", cx.zone_link(*zone as u16)),
        QsdReward::ClanLevelIncrease => "clan level +1".into(),
        QsdReward::ClanMoney { operator, value } => set_op(*operator, "clan money", *value),
        QsdReward::ClanPoints { operator, value } => set_op(*operator, "clan points", *value),
        QsdReward::AddClanSkill { id } => format!("the clan learns {}", skill(cx, *id)),
        QsdReward::RemoveClanSkill { id } => format!("the clan loses {}", skill(cx, *id)),
        QsdReward::ClanPointContribution { operator, value } => set_op(*operator, "your clan points", *value),
        QsdReward::TeleportNearbyClanMembers { zone, .. } => {
            format!("nearby clan members are moved to {}", cx.zone_link(*zone as u16))
        }
        QsdReward::CallLuaFunction { name } => format!("client script `{name}`"),
        QsdReward::ResetSkills => "your skills are reset".into(),
    }
}

pub fn write(cx: &Ctx, pages: &mut Pages) {
    let game = cx.game;
    let mut list = Page::new("quests", "list");
    list.set("title", "Quests");
    list.set("columns", strs(["given_by", "steps"].map(String::from)));
    list.line("# Quests");
    pages.add(list);

    for (&id, (path, name)) in &cx.quest_pages {
        let Some(quest) = game.quests.get_quest_data(id) else { continue };
        let mut triggers: Vec<&String> =
            cx.trigger_quests.iter().filter(|(_, q)| q.contains(&id)).map(|(t, _)| t).collect();
        triggers.sort_by_key(|t| natural_key(t));

        let mut givers = BTreeSet::new();
        let mut npcs = BTreeSet::new();
        let mut monsters = BTreeSet::new();
        for t in &triggers {
            let Some(trigger) = game.quests.get_trigger_by_name(t) else { continue };
            let adds = trigger.rewards.iter().any(|r| matches!(r, QsdReward::AddQuest { id: q } if *q == id));
            if let Some(n) = cx.trigger_npcs.get(*t) {
                npcs.extend(n.iter().copied());
                if adds {
                    givers.extend(n.iter().copied());
                }
            }
            if let Some(m) = cx.trigger_monsters.get(*t) {
                monsters.extend(m.iter().copied());
            }
        }

        let mut p = Page::new(path.clone(), "quest");
        p.set("id", id as i64);
        p.set("name", name.as_str());
        p.set("status", "in-game");
        if !givers.is_empty() {
            p.set("given_by", strs(givers.iter().map(|n| cx.npc_link(*n))));
        }
        let others: Vec<u16> = npcs.difference(&givers).copied().collect();
        if !others.is_empty() {
            p.set("npcs", strs(others.iter().map(|n| cx.npc_link(*n))));
        }
        if !monsters.is_empty() {
            p.set("monsters", strs(monsters.iter().map(|m| cx.npc_link(*m))));
        }
        if let Some(limit) = quest.time_limit {
            p.set("time_limit_minutes", (Duration::from(limit).as_secs() / 60) as i64);
        }
        p.set("steps", triggers.len() as i64);
        p.set(
            "source",
            row([
                ("data", format!("LIST_QUEST.STB row {id}; QSD triggers {}", triggers.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(", ")).into()),
                ("code", strs(["module/src/quests.rs".into(), "crates/rose-quest/src/lib.rs".into()])),
            ]),
        );

        p.line(format!("# {name}"));
        for (title, text) in [("", quest.description), ("When you take it", quest.start_message), ("When you finish it", quest.end_message)] {
            let t = rose_text(text);
            if t.is_empty() {
                continue;
            }
            p.line("");
            if !title.is_empty() {
                p.line(format!("## {title}"));
                p.line("");
            }
            for l in t.lines() {
                p.line(format!("{}  ", l.trim()));
            }
        }
        if !triggers.is_empty() {
            p.line("");
            p.line("## Steps");
            p.line("");
            p.line("Each step is a quest trigger. A step runs when all its checks pass; then it gives");
            p.line("everything under \"Then\". See [[rules/quests|Quests]].");
        }
        for t in &triggers {
            let Some(trigger) = game.quests.get_trigger_by_name(t) else { continue };
            p.line("");
            p.line(format!("### `{t}`"));
            p.line("");
            let mut by = Vec::new();
            if let Some(n) = cx.trigger_npcs.get(*t) {
                by.extend(n.iter().map(|n| format!("talking to {}", cx.npc_link(*n))));
            }
            if let Some(m) = cx.trigger_monsters.get(*t) {
                by.extend(m.iter().map(|m| format!("killing {}", cx.npc_link(*m))));
            }
            if !by.is_empty() {
                p.line(format!("Happens by {}.", by.join(", ")));
                p.line("");
            }
            if trigger.conditions.is_empty() {
                p.line("Checks: none.");
            } else {
                p.line("Checks:");
                p.line("");
                for c in &trigger.conditions {
                    p.line(format!("- {}", condition(cx, c)));
                }
            }
            if !trigger.rewards.is_empty() {
                p.line("");
                p.line("Then:");
                p.line("");
                for r in &trigger.rewards {
                    p.line(format!("- {}", reward(cx, r)));
                }
            }
            if let Some(next) = &trigger.next_trigger_name {
                p.line("");
                p.line(format!("If the checks fail, step `{next}` is tried instead."));
            }
        }
        pages.add(p);
    }
}
