//! Monster AI from the monsters' AIP files, as rose-offline's npc_ai_system runs it: every
//! idle interval an idle monster runs the first event of its idle trigger whose conditions
//! pass (most walk a few metres, aggressive ones look for a player to attack), and when hit
//! it runs its damaged trigger (fight back, call nearby friends). Monsters only think in
//! zones with a player in them.

use std::collections::HashSet;
use std::time::Duration;

use rand::Rng;
use rose_data::NpcId;
use rose_file_readers::{
    AipAbilityType, AipAction, AipAttackNearbyStat, AipCondition, AipConditionFindNearbyEntities, AipDamageType,
    AipDistanceOrigin, AipFile, AipHaveStatusTarget, AipHaveStatusType, AipMoveMode, AipMoveOrigin, AipNearbyAlly, AipSkillTarget, AipTrigger,
};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::npc_ai::{compare, is_daytime, zone_time};
use crate::{combat, distance, entity, monster_ai, motion, position, set_motion, skills, stats, stop_motion};
use crate::{EntityKind, MonsterAi, MONSTER_LEASH_CM};

/// When each monster next runs its idle trigger.
#[spacetimedb::table(accessor = monster_idle)]
pub struct MonsterIdle {
    #[primary_key]
    pub entity_id: u64,
    pub next_idle_us: i64,
}

/// What the AI needs to know about an entity near a monster.
#[derive(Clone)]
struct Body {
    id: u64,
    kind: EntityKind,
    npc_id: u16,
    pos: (f32, f32),
    level: i32,
    hp: i32,
    attack: i32,
    defence: i32,
    resistance: i32,
    target: Option<u64>,
    /// A player's summon: on the players' side, not the monsters'.
    summoned: bool,
    /// Hidden (Stealth or a disguise): monsters can't pick it as a new target.
    hidden: bool,
}

impl Body {
    fn ability(&self, ability: AipAbilityType) -> i32 {
        match ability {
            AipAbilityType::Level => self.level,
            AipAbilityType::Attack => self.attack,
            AipAbilityType::Defence => self.defence,
            AipAbilityType::Resistance => self.resistance,
            AipAbilityType::HealthPoints => self.hp,
            AipAbilityType::Charm => 0,
        }
    }
}

fn body(ctx: &ReducerContext, id: u64, t: i64) -> Option<Body> {
    let e = ctx.db.entity().entity_id().find(id)?;
    let c = ctx.db.combat().entity_id().find(id)?;
    let s = ctx.db.stats().entity_id().find(id)?;
    let pos = position(ctx, id, t)?;
    Some(Body {
        id,
        kind: e.kind,
        npc_id: e.npc_id,
        pos,
        level: s.level,
        hp: if c.dead_until_us.is_some() { 0 } else { c.hp },
        attack: s.attack_power,
        defence: s.defence,
        resistance: s.resistance,
        target: c.attack_target,
        summoned: skills::owner_of(ctx, id).is_some(),
        hidden: skills::is_invisible(ctx, id),
    })
}

/// Everyone in one zone, read once per tick and only when a trigger needs it.
struct Zone {
    zone_id: u16,
    bodies: Option<Vec<Body>>,
}

impl Zone {
    fn bodies(&mut self, ctx: &ReducerContext, t: i64) -> &[Body] {
        let zone_id = self.zone_id;
        self.bodies.get_or_insert_with(|| {
            ctx.db
                .entity()
                .iter()
                .filter(|e| e.zone_id == zone_id && e.kind != EntityKind::Npc)
                .filter_map(|e| body(ctx, e.entity_id, t))
                .collect()
        })
    }
}

struct Run<'a> {
    ctx: &'a ReducerContext,
    game: &'a GameData,
    t: i64,
    me: Body,
    ai: MonsterAi,
    zone_id: u16,
    attacker: Option<Body>,
    damage: i32,
    find_char: Option<(u64, (f32, f32))>,
    near_char: Option<(u64, (f32, f32))>,
}

impl Run<'_> {
    fn target(&self) -> Option<Body> {
        self.me.target.and_then(|id| body(self.ctx, id, self.t))
    }

    fn find_nearby(&mut self, zone: &mut Zone, f: &AipConditionFindNearbyEntities) -> bool {
        let range = f.distance as f32;
        let mut found = None;
        let mut nearest: Option<(u64, (f32, f32), f32)> = None;
        let mut count = 0;
        for b in zone.bodies(self.ctx, self.t) {
            if b.id == self.me.id || b.hp <= 0 {
                continue;
            }
            let allied = b.kind == EntityKind::Monster && !b.summoned;
            if !allied && b.hidden {
                continue;
            }
            if allied != f.is_allied || !f.level_diff_range.contains(&(self.me.level - b.level)) {
                continue;
            }
            let d = distance(self.me.pos, b.pos);
            if d > range {
                continue;
            }
            if nearest.map_or(true, |(_, _, nd)| d < nd) {
                nearest = Some((b.id, b.pos, d));
            }
            count += 1;
            if f.count_operator_type.is_none() && count >= f.count {
                found = Some((b.id, b.pos));
                break;
            }
        }
        self.near_char = nearest.map(|(id, pos, _)| (id, pos));
        if let Some(operator) = f.count_operator_type {
            if compare(operator, count, f.count) {
                found = self.near_char;
            }
        }
        self.find_char = found;
        found.is_some()
    }

    fn check(&mut self, zone: &mut Zone, condition: &AipCondition) -> bool {
        match condition {
            AipCondition::FindNearbyEntities(f) => self.find_nearby(zone, f),
            AipCondition::Damage(AipDamageType::Received, operator, value) => compare(*operator, self.damage, *value),
            AipCondition::Damage(AipDamageType::Given, ..) => false,
            AipCondition::Distance(origin, operator, value) => {
                let other = match origin {
                    AipDistanceOrigin::Spawn => Some((self.ai.home_x, self.ai.home_y)),
                    AipDistanceOrigin::Target => self.target().map(|b| b.pos),
                    AipDistanceOrigin::Owner => None,
                };
                other.is_some_and(|p| compare(*operator, distance(self.me.pos, p) as i32, *value))
            }
            AipCondition::HasNoOwner => true,
            AipCondition::OwnerHasTarget => false,
            AipCondition::HealthPercent(operator, value) => {
                let max = self.ctx.db.combat().entity_id().find(self.me.id).map_or(1, |c| c.max_hp.max(1));
                compare(*operator, 100 * self.me.hp / max, *value)
            }
            AipCondition::IsAttackerCurrentTarget => {
                self.attacker.as_ref().is_some_and(|a| Some(a.id) == self.me.target)
            }
            AipCondition::NoTargetAndCompareAttackerAbilityValue(operator, ability, value) => {
                self.me.target.is_none() && self.attacker.as_ref().is_some_and(|a| compare(*operator, a.ability(*ability), *value))
            }
            AipCondition::CompareAttackerAndTargetAbilityValue(operator, ability) => {
                match (self.attacker.as_ref(), self.target()) {
                    (Some(a), Some(target)) => compare(*operator, a.ability(*ability), target.ability(*ability)),
                    _ => false,
                }
            }
            AipCondition::Random(operator, range, value) => {
                !range.is_empty() && compare(*operator, self.ctx.rng().gen_range(range.clone()), *value)
            }
            AipCondition::SelfAbilityValue(operator, ability, value) => compare(*operator, self.me.ability(*ability), *value),
            AipCondition::TargetAbilityValue(operator, ability, value) => {
                self.target().is_some_and(|b| compare(*operator, b.ability(*ability), *value))
            }
            AipCondition::HasStatusEffect(who, kind, have) => {
                let id = match who {
                    AipHaveStatusTarget::This => Some(self.me.id),
                    AipHaveStatusTarget::Target => self.me.target,
                };
                let kind = match kind {
                    AipHaveStatusType::Good => Some(true),
                    AipHaveStatusType::Bad => Some(false),
                    AipHaveStatusType::Any => None,
                };
                id.is_some_and(|id| skills::has_status_effects(self.ctx, id, kind) == *have)
            }
            AipCondition::IsDaytime(day) => is_daytime(self.game, self.zone_id, self.t) == *day,
            AipCondition::ZoneTime(range) => range.contains(&zone_time(self.game, self.zone_id, self.t)),
            AipCondition::WorldTime(range) => range.contains(&rose_quest::world_ticks(self.t).get_world_time()),
            // Clans, channels, owners, calendar days and monster variables aren't kept.
            _ => false,
        }
    }

    fn attack(&mut self, target: u64) {
        if target == self.me.id || !crate::is_alive(self.ctx, target) {
            return;
        }
        // A taunted monster only fights its taunter; hidden characters can't be picked.
        if skills::taunter(self.ctx, self.me.id).is_some_and(|taunter| taunter != target)
            || (self.me.target != Some(target) && skills::is_invisible(self.ctx, target))
        {
            return;
        }
        if let Some(mut c) = self.ctx.db.combat().entity_id().find(self.me.id) {
            c.attack_target = Some(target);
            self.ctx.db.combat().entity_id().update(c);
            self.me.target = Some(target);
        }
    }

    fn walk(&self, to: (f32, f32), mode: AipMoveMode) {
        let Some(s) = self.ctx.db.stats().entity_id().find(self.me.id) else { return };
        let speed = match mode {
            AipMoveMode::Run => s.run_speed,
            AipMoveMode::Walk => s.move_speed,
        };
        // Stay near home: past the leash the monster would only turn back.
        let home = (self.ai.home_x, self.ai.home_y);
        let limit = MONSTER_LEASH_CM * 0.8;
        let d = distance(home, to);
        let to = if d > limit { (home.0 + (to.0 - home.0) * limit / d, home.1 + (to.1 - home.1) * limit / d) } else { to };
        set_motion(self.ctx, self.me.id, to, speed, None);
    }

    fn act(&mut self, zone: &mut Zone, action: &AipAction) {
        match action {
            AipAction::Stop => stop_motion(self.ctx, self.me.id),
            AipAction::MoveRandomDistance(origin, mode, range) => {
                let origin = match origin {
                    AipMoveOrigin::CurrentPosition => Some(self.me.pos),
                    AipMoveOrigin::Spawn => Some((self.ai.home_x, self.ai.home_y)),
                    AipMoveOrigin::FindChar => self.find_char.map(|(_, p)| p),
                };
                if let (Some(o), true) = (origin, *range > 0) {
                    let mut rng = self.ctx.rng();
                    let to = (o.0 + rng.gen_range(-*range..*range) as f32, o.1 + rng.gen_range(-*range..*range) as f32);
                    self.walk(to, *mode);
                }
            }
            AipAction::MoveAwayFromTarget(mode, range) => {
                if let Some(target) = self.target() {
                    let (dx, dy) = (self.me.pos.0 - target.pos.0, self.me.pos.1 - target.pos.1);
                    let d = (dx * dx + dy * dy).sqrt().max(1.0);
                    let r = *range as f32;
                    self.walk((self.me.pos.0 + dx / d * r, self.me.pos.1 + dy / d * r), *mode);
                }
            }
            AipAction::AttackNearChar => {
                if let Some((id, _)) = self.near_char {
                    self.attack(id);
                }
            }
            AipAction::AttackFindChar => {
                if let Some((id, _)) = self.find_char {
                    self.attack(id);
                }
            }
            AipAction::AttackAttacker => {
                if let Some(id) = self.attacker.as_ref().map(|a| a.id) {
                    self.attack(id);
                }
            }
            AipAction::AttackNearbyEntityByStat(range, ability, choice) => {
                let me = self.me.clone();
                let pick = zone
                    .bodies(self.ctx, self.t)
                    .iter()
                    .filter(|b| (b.kind == EntityKind::Player || b.summoned) && !b.hidden && b.hp > 0)
                    .filter(|b| distance(me.pos, b.pos) <= *range as f32)
                    .map(|b| (b.id, b.ability(*ability)));
                let chosen = match choice {
                    AipAttackNearbyStat::Lowest => pick.min_by_key(|(_, v)| *v),
                    AipAttackNearbyStat::Highest => pick.max_by_key(|(_, v)| *v),
                };
                if let Some((id, _)) = chosen {
                    self.attack(id);
                }
            }
            AipAction::NearbyAlliesAttackTarget(range, ally, limit) => {
                let Some(target) = self.me.target else { return };
                let me = self.me.clone();
                let allies: Vec<u64> = zone
                    .bodies(self.ctx, self.t)
                    .iter()
                    .filter(|b| b.id != me.id && b.kind == EntityKind::Monster && !b.summoned && b.hp > 0 && b.target.is_none())
                    // This one distance is in metres in the AIP files.
                    .filter(|b| distance(me.pos, b.pos) <= *range as f32 * 100.0)
                    .filter(|b| match ally {
                        AipNearbyAlly::Ally => true,
                        AipNearbyAlly::WithNpcId(npc_id) => b.npc_id as i32 == *npc_id,
                        AipNearbyAlly::WithSameNpcId => b.npc_id == me.npc_id,
                    })
                    .take(limit.unwrap_or(usize::MAX))
                    .map(|b| b.id)
                    .collect();
                for id in allies {
                    if let Some(mut c) = self.ctx.db.combat().entity_id().find(id) {
                        c.attack_target = Some(target);
                        self.ctx.db.combat().entity_id().update(c);
                    }
                }
                // The zone list is read again next time, with the new targets.
                zone.bodies = None;
            }
            AipAction::UseSkill(target, skill_id, motion_id) => {
                let target = match target {
                    AipSkillTarget::FindChar => self.find_char.map(|(id, _)| id),
                    AipSkillTarget::Target => self.me.target,
                    AipSkillTarget::This => Some(self.me.id),
                    AipSkillTarget::NearChar => self.near_char.map(|(id, _)| id),
                };
                if let (Some(target), Ok(skill_id)) = (target, u16::try_from(*skill_id)) {
                    skills::npc_cast(self.ctx, self.game, self.me.id, skill_id, target, *motion_id, self.t);
                }
            }
            // Speech and emotes are shown by the client; summons, transformations, item
            // drops and the rest come later.
            _ => {}
        }
    }

    /// The first event whose conditions all pass runs its actions.
    fn run(&mut self, zone: &mut Zone, trigger: &AipTrigger) {
        for event in &trigger.events {
            if event.conditions.iter().all(|c| self.check(zone, c)) {
                for action in &event.actions {
                    self.act(zone, action);
                }
                return;
            }
        }
    }
}

fn program<'a>(game: &'a GameData, npc_id: u16) -> Option<&'a AipFile> {
    let data = NpcId::new(npc_id).and_then(|id| game.npcs.get_npc(id))?;
    game.ai.get_ai(data.ai_file_index as usize)
}

/// Zones with a living player in them.
fn busy_zones(ctx: &ReducerContext) -> HashSet<u16> {
    ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Player).map(|e| e.zone_id).collect()
}

/// Called every combat tick: idle monsters whose interval is up run their idle trigger.
pub fn idle_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let zones = busy_zones(ctx);
    if zones.is_empty() {
        return;
    }
    let mut rng = ctx.rng();
    let mut zone_cache: Vec<Zone> = Vec::new();
    let candidates: Vec<(MonsterAi, u16, u16)> = ctx
        .db
        .monster_ai()
        .iter()
        .filter(|ai| !ai.returning)
        .filter_map(|ai| {
            let e = ctx.db.entity().entity_id().find(ai.entity_id)?;
            zones.contains(&e.zone_id).then_some((ai, e.zone_id, e.npc_id))
        })
        .collect();
    for (ai, zone_id, npc_id) in candidates {
        let id = ai.entity_id;
        let Some(program) = program(game, npc_id) else { continue };
        let interval = program.idle_trigger_interval.max(Duration::from_secs(1)).as_micros() as i64;
        let Some(next) = ctx.db.monster_idle().entity_id().find(id).map(|r| r.next_idle_us) else {
            // Spread new monsters over their first interval.
            ctx.db.monster_idle().insert(MonsterIdle { entity_id: id, next_idle_us: t + rng.gen_range(0..interval) });
            continue;
        };
        if next > t {
            continue;
        }
        ctx.db.monster_idle().entity_id().update(MonsterIdle { entity_id: id, next_idle_us: t + interval });
        let Some(trigger) = program.trigger_on_idle.as_ref() else { continue };
        let Some(me) = body(ctx, id, t) else { continue };
        // Idle means standing still with nothing to fight.
        if me.hp <= 0
            || me.target.is_some()
            || skills::is_disabled(ctx, id)
            || ctx.db.motion().entity_id().find(id).is_some_and(|m| m.is_moving(t))
        {
            continue;
        }
        let zone = match zone_cache.iter().position(|z| z.zone_id == zone_id) {
            Some(i) => &mut zone_cache[i],
            None => {
                zone_cache.push(Zone { zone_id, bodies: None });
                zone_cache.last_mut().unwrap()
            }
        };
        let mut run = Run { ctx, game, t, me, ai, zone_id, attacker: None, damage: 0, find_char: None, near_char: None };
        run.run(zone, trigger);
    }
}

/// A monster was hit and lived: its damaged trigger decides what it does (most fight back,
/// some call friends, timid ones run from strong players). Without one it fights back.
pub fn on_damaged(ctx: &ReducerContext, game: &GameData, id: u64, attacker: u64, damage: i32, t: i64) {
    if skills::is_disabled(ctx, id) {
        return;
    }
    let Some(me) = body(ctx, id, t) else { return };
    let Some(e) = ctx.db.entity().entity_id().find(id) else { return };
    // Summons have no AI program of their own (skills::summon_tick): they fight back.
    let ai = ctx.db.monster_ai().entity_id().find(id);
    if let (Some(ai), Some(program)) = (ai, program(game, e.npc_id).filter(|p| p.trigger_on_damaged.is_some())) {
        // A monster already fighting only sometimes reacts to someone new.
        let busy = me.target.is_some() && (program.damage_trigger_new_target_chance as i32) < ctx.rng().gen_range(0..100);
        if !busy {
            let attacker = body(ctx, attacker, t);
            let mut zone = Zone { zone_id: e.zone_id, bodies: None };
            let mut run =
                Run { ctx, game, t, me, ai, zone_id: e.zone_id, attacker, damage, find_char: None, near_char: None };
            run.run(&mut zone, program.trigger_on_damaged.as_ref().unwrap());
        }
        return;
    }
    // No damaged trigger: just fight back.
    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
        if c.attack_target.is_none() && c.hp > 0 {
            c.attack_target = Some(attacker);
            ctx.db.combat().entity_id().update(c);
        }
    }
}

/// A monster's swing landed: the skills in its attack trigger (iROSE's attack_move trigger).
/// Only the events that use a skill are run, and only their skills: the trigger's other
/// events mostly make the monster step about mid-fight, which rose-offline doesn't do
/// either.
pub fn on_attack(ctx: &ReducerContext, game: &GameData, id: u64, t: i64) {
    let Some(ai) = ctx.db.monster_ai().entity_id().find(id) else { return };
    let Some(e) = ctx.db.entity().entity_id().find(id) else { return };
    let Some(trigger) = program(game, e.npc_id).and_then(|p| p.trigger_on_attack_move.as_ref()) else { return };
    let Some(me) = body(ctx, id, t) else { return };
    let mut zone = Zone { zone_id: e.zone_id, bodies: None };
    let mut run = Run { ctx, game, t, me, ai, zone_id: e.zone_id, attacker: None, damage: 0, find_char: None, near_char: None };
    let uses_skill = |a: &AipAction| matches!(a, AipAction::UseSkill(..));
    for event in trigger.events.iter().filter(|e| e.actions.iter().any(uses_skill)) {
        if event.conditions.iter().all(|c| run.check(&mut zone, c)) {
            for action in event.actions.iter().filter(|a| uses_skill(a)) {
                run.act(&mut zone, action);
            }
            return;
        }
    }
}

pub fn forget(ctx: &ReducerContext, id: u64) {
    ctx.db.monster_idle().entity_id().delete(id);
}
