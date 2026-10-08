//! Town NPC scripts (AIP files): every idle interval an NPC runs the first event of its
//! idle trigger whose conditions pass. Town NPCs use this to switch their quests on and
//! off with the time of day (object variables the quest triggers read) and to shout.
//! Follows rose-offline's npc_ai_system for the conditions and actions town NPCs use.

use std::time::Duration;

use rand::Rng;
use rose_data::{NpcId, ZoneId};
use rose_file_readers::{
    AipAction, AipCondition, AipMessageType, AipOperatorType, AipResultOperator, AipTrigger, AipVariableType,
    QsdCondition, QsdNpcMessageType, QsdReward,
};
use rose_game_data::GameData;
use rose_quest::RealTime;
use spacetimedb::{ReducerContext, Table};

use crate::{distance, items, npc, npcs::Npc, player, position};

/// Players this close hear an NPC's local chat.
const SAY_RANGE_CM: f32 = 2000.0;
/// Longest chain of NPC quest triggers one action follows.
const MAX_CHAIN: usize = 16;

pub fn compare(operator: AipOperatorType, a: i32, b: i32) -> bool {
    match operator {
        AipOperatorType::Equals => a == b,
        AipOperatorType::GreaterThan => a > b,
        AipOperatorType::GreaterThanEqual => a >= b,
        AipOperatorType::LessThan => a < b,
        AipOperatorType::LessThanEqual => a <= b,
        AipOperatorType::NotEqual => a != b,
    }
}

/// The time of day in a zone, in world ticks since the zone's day began.
pub fn zone_time(game: &GameData, zone_id: u16, t: i64) -> u32 {
    let world_time = rose_quest::world_ticks(t).get_world_time();
    match ZoneId::new(zone_id).and_then(|z| game.zones.get_zone(z)) {
        Some(zone) if zone.day_cycle > 0 => world_time % zone.day_cycle,
        _ => world_time,
    }
}

pub fn is_daytime(game: &GameData, zone_id: u16, t: i64) -> bool {
    let Some(zone) = ZoneId::new(zone_id).and_then(|z| game.zones.get_zone(z)) else { return true };
    let time = zone_time(game, zone_id, t);
    (zone.day_time / 2..=(zone.evening_time + zone.night_time) / 2).contains(&time)
}

/// The NPC of this NPC id in the zone (or anywhere, if the zone has none).
fn find_npc(ctx: &ReducerContext, npc_id: u16, zone_id: u16) -> Option<Npc> {
    let mut found = None;
    for n in ctx.db.npc().iter().filter(|n| n.npc_id == npc_id) {
        if n.zone_id == zone_id {
            return Some(n);
        }
        found.get_or_insert(n);
    }
    found
}

struct Event<'a> {
    ctx: &'a ReducerContext,
    game: &'a GameData,
    source: Npc,
    selected: Option<u64>,
    t: i64,
}

impl Event<'_> {
    fn variables(&self, variable_type: AipVariableType) -> Option<Npc> {
        match variable_type {
            AipVariableType::LocalNpcObject => self.ctx.db.npc().entity_id().find(self.selected?),
            AipVariableType::Ai => self.ctx.db.npc().entity_id().find(self.source.entity_id),
            AipVariableType::World | AipVariableType::Economy => None,
        }
    }

    fn check(&mut self, condition: &AipCondition) -> bool {
        match *condition {
            AipCondition::SelectLocalNpc(npc_id) => {
                self.selected = find_npc(self.ctx, npc_id as u16, self.source.zone_id).map(|n| n.entity_id);
                self.selected.is_some()
            }
            AipCondition::Variable(variable_type, variable_id, operator, value) => {
                let current = self.variables(variable_type).and_then(|n| n.variables.get(variable_id).copied()).unwrap_or(0);
                compare(operator, current, value)
            }
            AipCondition::ZoneTime(ref range) => range.contains(&zone_time(self.game, self.source.zone_id, self.t)),
            AipCondition::IsDaytime(day) => is_daytime(self.game, self.source.zone_id, self.t) == day,
            AipCondition::MonthDay(ref when) => {
                let now = RealTime::from_unix_us(self.t);
                when.month_day.map_or(true, |d| d.get() as u32 == now.month_day)
                    && when.day_minutes_range.contains(&now.day_minutes)
            }
            AipCondition::WeekDay(ref when) => {
                let now = RealTime::from_unix_us(self.t);
                when.week_day as u32 == now.week_day && when.day_minutes_range.contains(&now.day_minutes)
            }
            AipCondition::WorldTime(ref range) => range.contains(&rose_quest::world_ticks(self.t).get_world_time()),
            AipCondition::Random(operator, ref range, value) => {
                let roll = if range.is_empty() { range.start } else { self.ctx.rng().gen_range(range.clone()) };
                compare(operator, roll, value)
            }
            AipCondition::ServerChannelNumber(ref range) => range.contains(&1),
            // Town NPCs don't fight or look around.
            _ => false,
        }
    }

    fn set_variable(&mut self, variable_type: AipVariableType, id: usize, operator: AipResultOperator, value: i32) {
        let Some(mut n) = self.variables(variable_type) else { return };
        let local = matches!(variable_type, AipVariableType::LocalNpcObject);
        if let Some(v) = n.variables.get_mut(id) {
            *v = match operator {
                AipResultOperator::Set => value,
                AipResultOperator::Add if local => (*v + value).min(500),
                AipResultOperator::Subtract if local => (*v - value).max(0),
                AipResultOperator::Add => *v + value,
                AipResultOperator::Subtract => *v - value,
            };
            self.ctx.db.npc().entity_id().update(n);
        }
    }

    fn act(&mut self, action: &AipAction) {
        match *action {
            AipAction::SetVariable(variable_type, id, operator, value) => self.set_variable(variable_type, id, operator, value),
            AipAction::Message(message_type, string_id) => {
                if let Some(text) = self.game.ai.get_ai_string(string_id) {
                    let text = text.to_string();
                    npc_message(self.ctx, self.game, &self.source, matches!(message_type, AipMessageType::Say), &text, self.t);
                }
            }
            AipAction::DoQuestTrigger(ref name) => run_npc_trigger(self.ctx, self.game, &self.source, name, self.t),
            // Say is client-side chatter; PvP flags and wandering come later.
            _ => {}
        }
    }
}

/// An NPC speaks: local chat reaches players nearby, a shout the whole zone.
fn npc_message(ctx: &ReducerContext, game: &GameData, source: &Npc, local: bool, text: &str, t: i64) {
    let name = NpcId::new(source.npc_id).and_then(|id| game.npcs.get_npc(id)).map_or("", |n| n.name);
    let at = position(ctx, source.entity_id, t);
    for p in ctx.db.player().iter().filter(|p| p.online && p.zone_id == source.zone_id) {
        if local {
            let near = match (at, p.entity_id.and_then(|id| position(ctx, id, t))) {
                (Some(a), Some(b)) => distance(a, b) <= SAY_RANGE_CM,
                _ => false,
            };
            if !near {
                continue;
            }
        }
        items::notify(ctx, p.identity, format!("{name}: {text}"));
    }
}

fn run_trigger(ctx: &ReducerContext, game: &GameData, source: Npc, trigger: Option<&AipTrigger>, t: i64) {
    let Some(trigger) = trigger else { return };
    for event in trigger.events.iter() {
        let mut e = Event { ctx, game, source: source.clone(), selected: None, t };
        if event.conditions.iter().all(|c| e.check(c)) {
            for action in event.actions.iter() {
                e.act(action);
            }
            break;
        }
    }
}

/// A quest trigger an NPC runs on itself (only NPC-side conditions and rewards make sense).
fn run_npc_trigger(ctx: &ReducerContext, game: &GameData, source: &Npc, name: &str, t: i64) {
    let mut trigger = rose_quest::find_trigger(&game.quests, name);
    let mut selected: Option<u64> = None;
    let mut steps = 0;
    while let Some(tr) = trigger {
        steps += 1;
        if steps > MAX_CHAIN {
            break;
        }
        let passed = tr.conditions.iter().all(|c| match *c {
            QsdCondition::SelectNpc { id } => {
                selected = find_npc(ctx, id as u16, source.zone_id).map(|n| n.entity_id);
                selected.is_some()
            }
            QsdCondition::ObjectVariable { variable_id, operator, value, .. } => selected
                .and_then(|id| ctx.db.npc().entity_id().find(id))
                .and_then(|n| n.variables.get(variable_id).copied())
                .map_or(false, |v| rose_quest::condition_operator(operator, v, value)),
            QsdCondition::RandomPercent { ref range } => range.contains(&ctx.rng().gen_range(0..100)),
            _ => false,
        });
        let mut next = None;
        if passed {
            for reward in tr.rewards.iter() {
                match *reward {
                    QsdReward::NpcMessage { message_type, string_id } => {
                        if let Some(text) = game.quests.get_quest_string(string_id as u16) {
                            let local = matches!(message_type, QsdNpcMessageType::Chat);
                            npc_message(ctx, game, source, local, text, t);
                        }
                    }
                    QsdReward::ObjectVariable { variable_id, operator, value, .. } => {
                        if let Some(mut n) = selected.and_then(|id| ctx.db.npc().entity_id().find(id)) {
                            if let Some(v) = n.variables.get_mut(variable_id) {
                                *v = rose_quest::reward_operator(operator, *v, value);
                                ctx.db.npc().entity_id().update(n);
                            }
                        }
                    }
                    QsdReward::Trigger { ref name } => next = Some(name.clone()),
                    _ => {}
                }
            }
            trigger = next.and_then(|n| game.quests.get_trigger_by_name(&n));
        } else {
            trigger = tr.next_trigger_name.as_ref().and_then(|n| game.quests.get_trigger_by_name(n));
        }
    }
}

/// A new NPC runs its created trigger and starts its idle timer.
pub fn npc_created(ctx: &ReducerContext, game: &GameData, n: &Npc, t: i64) {
    let Some(ai) = NpcId::new(n.npc_id)
        .and_then(|id| game.npcs.get_npc(id))
        .and_then(|data| game.ai.get_ai(data.ai_file_index as usize))
    else {
        return;
    };
    run_trigger(ctx, game, n.clone(), ai.trigger_on_created.as_ref(), t);
}

/// Run the idle trigger of every NPC whose interval is up.
pub fn npc_ai_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let due: Vec<Npc> = ctx.db.npc().iter().filter(|n| n.next_idle_us <= t).collect();
    for n in due {
        let ai = NpcId::new(n.npc_id)
            .and_then(|id| game.npcs.get_npc(id))
            .and_then(|data| game.ai.get_ai(data.ai_file_index as usize));
        let interval = ai.map_or(Duration::from_secs(60), |ai| ai.idle_trigger_interval).max(Duration::from_secs(1));
        if let Some(mut row) = ctx.db.npc().entity_id().find(n.entity_id) {
            row.next_idle_us = t + interval.as_micros() as i64;
            ctx.db.npc().entity_id().update(row);
        }
        if let Some(ai) = ai {
            run_trigger(ctx, game, n, ai.trigger_on_idle.as_ref(), t);
        }
    }
}

/// Debug: set an object variable on every NPC with this NPC id.
#[spacetimedb::reducer]
pub fn set_npc_variable(ctx: &ReducerContext, npc_id: u16, variable_id: u8, value: i32) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let rows: Vec<Npc> = ctx.db.npc().iter().filter(|n| n.npc_id == npc_id).collect();
    if rows.is_empty() {
        return Err("no such NPC".into());
    }
    for mut n in rows {
        let v = n.variables.get_mut(variable_id as usize).ok_or("no such variable")?;
        *v = value;
        ctx.db.npc().entity_id().update(n);
    }
    Ok(())
}
