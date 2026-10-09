//! Skills: learning them from skill books, levelling them with skill points, casting, and the
//! status effects (buffs and debuffs) they leave on characters and monsters. Rules follow
//! rose-offline's skill_list and skill_use bundles, command_system (casting),
//! skill_effect_system and status_effect_system.

use enum_map::Enum;
use rand::Rng;
use rose_data::{
    AbilityType, EquipmentIndex, Item, JobId, SkillActionMode, SkillBasicCommand, SkillCooldown, SkillData,
    SkillId, SkillTargetFilter, SkillType, StatusEffectClearedByType, StatusEffectId, StatusEffectType,
};
use rose_game_common::components::{AbilityValues, ActiveStatusEffect, ItemSlot, Money, SkillSlot, StatusEffects};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::{
    ability, character, combat, distance, entity, game_data::game, items, items::ground_item, motion, my_player, now_us, player, position,
    set_motion, stats, stop_motion, world, Combat, EntityKind, Player, RANGE_SLACK_CM,
};

/// No skill can follow another sooner than this (rose-offline's GLOBAL_SKILL_COOLDOWN).
const GLOBAL_COOLDOWN_US: i64 = 250_000;
/// Cooldown keys: 0 is the global cooldown, a skill id its own, GROUP_KEY + n a group's.
const GROUP_KEY: u32 = 100_000;
/// Casts that have no motion still take this long, so clients see them.
const MIN_CAST_US: i64 = 300_000;

/// A skill being cast: walking into range (started_at_us empty), then the casting motion
/// until effect_at_us (when it takes effect), then the action motion until ends_at_us.
#[spacetimedb::table(accessor = skill_cast, public)]
#[derive(Clone)]
pub struct SkillCast {
    #[primary_key]
    pub entity_id: u64,
    pub skill_id: u16,
    pub target: Option<u64>,
    /// Ground target of an area skill, in cm.
    pub target_x: f32,
    pub target_y: f32,
    pub at_position: bool,
    pub started_at_us: Option<i64>,
    pub effect_at_us: i64,
    pub ends_at_us: i64,
    pub applied: bool,
    /// A scroll the cast uses up: its inventory slot (page * 100 + index) and the item as
    /// JSON, or -1.
    pub use_item_slot: i32,
    pub use_item: String,
}

#[spacetimedb::table(accessor = skill_cooldown, public)]
#[derive(Clone)]
pub struct SkillCooldownRow {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub entity_id: u64,
    pub key: u32,
    pub until_us: i64,
}

/// A buff or debuff on an entity (one per StatusEffectType, as rose-offline's StatusEffects).
#[spacetimedb::table(accessor = status_effect, public)]
#[derive(Clone)]
pub struct StatusEffectRow {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub entity_id: u64,
    /// StatusEffectType as its enum index.
    pub effect_type: u8,
    pub status_effect_id: u16,
    pub value: i32,
    pub expires_at_us: i64,
}

/// Who taunted a monster: while its Taunt status lasts it only attacks them.
#[spacetimedb::table(accessor = taunt)]
pub struct Taunt {
    #[primary_key]
    pub entity_id: u64,
    pub taunter: u64,
}

/// The motions of a monster's skill cast (NPC motions from its AIP UseSkill action), for
/// clients; player casts take theirs from the skill.
#[spacetimedb::table(accessor = npc_cast_motion, public)]
pub struct NpcCastMotion {
    #[primary_key]
    pub entity_id: u64,
    pub cast_motion: i32,
    pub action_motion: i32,
}

/// A monster summoned by a player's skill: it fights for its owner, follows them, and
/// leaves when they die, leave or change zone.
#[spacetimedb::table(accessor = summon, public)]
#[derive(Clone)]
pub struct Summon {
    #[primary_key]
    pub entity_id: u64,
    #[index(btree)]
    pub owner: u64,
    pub owner_level: i32,
    pub skill_level: i32,
    /// Summon points it takes from the owner's capacity (LIST_NPC.STB).
    pub points: u32,
}

/// Summon capacity before passive skills (iROSE's 50 + AT_PSV_SUMMON_MOB_CNT).
const BASE_SUMMON_POINTS: i32 = 50;
/// A summon farther than this from its owner runs back to within SUMMON_FOLLOW_TO_CM.
const SUMMON_FOLLOW_CM: f32 = 550.0;
const SUMMON_FOLLOW_TO_CM: f32 = 350.0;
/// Resurrection brings a player back with this share of their maximum HP (iROSE).
const RESURRECT_HP_PERCENT: i32 = 30;

// ---------------------------------------------------------------- status effects

fn has_status(ctx: &ReducerContext, entity_id: u64, effect_type: StatusEffectType) -> bool {
    let effect_type = effect_type.into_usize() as u8;
    ctx.db.status_effect().entity_id().filter(entity_id).any(|r| r.effect_type == effect_type)
}

fn status_value(ctx: &ReducerContext, entity_id: u64, effect_type: StatusEffectType) -> Option<i32> {
    let effect_type = effect_type.into_usize() as u8;
    ctx.db.status_effect().entity_id().filter(entity_id).find(|r| r.effect_type == effect_type).map(|r| r.value)
}

/// Remove status effects of these types; true if any was there.
fn remove_status(ctx: &ReducerContext, entity_id: u64, types: &[StatusEffectType]) -> bool {
    let types: Vec<u8> = types.iter().map(|t| t.into_usize() as u8).collect();
    let rows: Vec<u64> =
        ctx.db.status_effect().entity_id().filter(entity_id).filter(|r| types.contains(&r.effect_type)).map(|r| r.id).collect();
    for id in rows.iter() {
        ctx.db.status_effect().id().delete(*id);
    }
    !rows.is_empty()
}

/// Why a stunned or sleeping character can't act, or None when it can. Sleep and stun block
/// moving, attacking, skills, items and picking up (iROSE's IsIgnoreSTATUS).
pub fn disabled_reason(ctx: &ReducerContext, entity_id: u64) -> Option<&'static str> {
    if has_status(ctx, entity_id, StatusEffectType::Fainting) {
        Some("you are stunned")
    } else if has_status(ctx, entity_id, StatusEffectType::Sleep) {
        Some("you are asleep")
    } else {
        None
    }
}

pub fn is_disabled(ctx: &ReducerContext, entity_id: u64) -> bool {
    disabled_reason(ctx, entity_id).is_some()
}

/// Hidden by Stealth (Transparent) or a disguise: nobody can start attacking it.
pub fn is_invisible(ctx: &ReducerContext, entity_id: u64) -> bool {
    has_status(ctx, entity_id, StatusEffectType::Disguise) || has_status(ctx, entity_id, StatusEffectType::Transparent)
}

/// A disguise ends when its wearer attacks or uses a skill; Stealth doesn't.
pub fn break_disguise(ctx: &ReducerContext, game: &GameData, entity_id: u64) {
    if remove_status(ctx, entity_id, &[StatusEffectType::Disguise]) {
        refresh_entity(ctx, game, entity_id);
    }
}

/// Any hit or miss that doesn't kill wakes a sleeping target (iROSE's Apply_DAMAGE).
pub fn wake(ctx: &ReducerContext, game: &GameData, entity_id: u64) {
    if remove_status(ctx, entity_id, &[StatusEffectType::Sleep]) {
        refresh_entity(ctx, game, entity_id);
    }
}

/// The one a taunted monster must attack, while the taunt lasts and the taunter is still
/// around in the same zone.
pub fn taunter(ctx: &ReducerContext, entity_id: u64) -> Option<u64> {
    let taunter = ctx.db.taunt().entity_id().find(entity_id)?.taunter;
    if !has_status(ctx, entity_id, StatusEffectType::Taunt) {
        return None;
    }
    let zone = |id| ctx.db.entity().entity_id().find(id).map(|e| e.zone_id);
    (zone(entity_id).is_some() && zone(entity_id) == zone(taunter) && crate::is_alive(ctx, taunter)).then_some(taunter)
}

/// The player a summon fights for.
pub fn owner_of(ctx: &ReducerContext, entity_id: u64) -> Option<u64> {
    ctx.db.summon().entity_id().find(entity_id).map(|s| s.owner)
}

/// The player behind an entity: the entity itself, or a summon's owner.
pub fn controller(ctx: &ReducerContext, entity_id: u64) -> u64 {
    owner_of(ctx, entity_id).unwrap_or(entity_id)
}

/// Stop whatever the entity is doing (a stun or sleep landing on it).
fn interrupt(ctx: &ReducerContext, entity_id: u64, t: i64) {
    cancel_cast(ctx, entity_id);
    crate::cancel_attack(ctx, entity_id, t);
    stop_motion(ctx, entity_id);
}

/// An entity's active status effects, for the ability value formulas.
pub fn status_effects(ctx: &ReducerContext, entity_id: u64) -> StatusEffects {
    let mut effects = StatusEffects::default();
    for row in ctx.db.status_effect().entity_id().filter(entity_id) {
        let Some(id) = StatusEffectId::new(row.status_effect_id) else { continue };
        let effect_type = StatusEffectType::from_usize(row.effect_type as usize);
        effects.active[effect_type] = Some(ActiveStatusEffect { id, value: row.value });
    }
    effects
}

/// Recalculate an entity's stats after its status effects changed.
fn refresh_entity(ctx: &ReducerContext, game: &GameData, entity_id: u64) {
    let Some(e) = ctx.db.entity().entity_id().find(entity_id) else { return };
    match e.kind {
        EntityKind::Player => {
            if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(entity_id)) {
                character::refresh_player(ctx, game, &p, false);
            }
        }
        EntityKind::Monster => {
            let effects = status_effects(ctx, entity_id);
            let summon = ctx.db.summon().entity_id().find(entity_id).map(|s| (s.owner_level, s.skill_level));
            let Some(stats) = world::npc_stats_for(game, entity_id, e.npc_id, &effects, summon) else { return };
            if let Some(mut c) = ctx.db.combat().entity_id().find(entity_id) {
                c.max_hp = stats.max_hp;
                c.max_mp = stats.max_mp;
                c.hp = c.hp.min(c.max_hp);
                c.mp = c.mp.min(c.max_mp);
                ctx.db.combat().entity_id().update(c);
            }
            ctx.db.stats().entity_id().update(stats);
        }
        EntityKind::Npc => {}
    }
}

/// Expire status effects and apply poison, once a second (rose-offline's status_effect_system).
pub fn status_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let mut changed: Vec<u64> = Vec::new();
    for row in ctx.db.status_effect().iter() {
        if row.expires_at_us <= t {
            ctx.db.status_effect().id().delete(row.id);
            changed.push(row.entity_id);
            continue;
        }
        if matches!(StatusEffectType::from_usize(row.effect_type as usize), StatusEffectType::Poisoned) {
            let per_second = StatusEffectId::new(row.status_effect_id)
                .and_then(|id| game.status_effects.get_status_effect(id))
                .map_or(0, |d| d.apply_per_second_value);
            if let Some(mut c) = ctx.db.combat().entity_id().find(row.entity_id) {
                if c.hp > 0 {
                    c.hp = (c.hp - per_second).max(1);
                    ctx.db.combat().entity_id().update(c);
                }
            }
        }
    }
    changed.sort_unstable();
    changed.dedup();
    for id in changed {
        refresh_entity(ctx, game, id);
    }
}

/// Forget everything about an entity that left the world.
pub fn clear_entity(ctx: &ReducerContext, entity_id: u64) {
    ctx.db.skill_cast().entity_id().delete(entity_id);
    ctx.db.npc_cast_motion().entity_id().delete(entity_id);
    ctx.db.taunt().entity_id().delete(entity_id);
    ctx.db.summon().entity_id().delete(entity_id);
    let rows: Vec<u64> = ctx.db.status_effect().entity_id().filter(entity_id).map(|r| r.id).collect();
    for id in rows {
        ctx.db.status_effect().id().delete(id);
    }
    let rows: Vec<u64> = ctx.db.skill_cooldown().entity_id().filter(entity_id).map(|r| r.id).collect();
    for id in rows {
        ctx.db.skill_cooldown().id().delete(id);
    }
}

// ---------------------------------------------------------------- learning

fn skill(game: &GameData, id: u16) -> Result<&SkillData, String> {
    SkillId::new(id).and_then(|id| game.skills.get_skill(id)).ok_or_else(|| "unknown skill".into())
}

/// The checks rose-offline's can_learn_skill and can_level_up_skill share.
fn check_requirements(ctx: &ReducerContext, game: &GameData, p: &Player, skill: &SkillData) -> Result<(), String> {
    if p.skill_points < skill.learn_point_cost {
        return Err(format!("needs {} skill points", skill.learn_point_cost));
    }
    if let Some(job_class) = skill.required_job_class.and_then(|id| game.job_class.get(id)) {
        if !job_class.jobs.is_empty() && !job_class.jobs.contains(&JobId::new(p.job)) {
            return Err(format!("only {} can learn this", job_class.name));
        }
    }
    let skill_list = p.skill_list();
    for &(required_id, required_level) in skill.required_skills.iter() {
        let Some(required) = SkillId::new(required_id.get() + required_level.max(1) as u16 - 1)
            .and_then(|id| game.skills.get_skill(id))
        else {
            continue;
        };
        let level = skill_list
            .find_skill_level(&game.skills, required.base_skill_id.unwrap_or(required_id))
            .map_or(0, |(_, _, level)| level);
        if level < required_level as u32 {
            return Err(format!("needs {} level {}", required.name, required_level));
        }
    }
    let av = items::player_ability_values(ctx, game, p);
    let c = p.entity_id.and_then(|id| ctx.db.combat().entity_id().find(id));
    for &(ability_type, value) in skill.required_ability.iter() {
        if ability::get_value(ctx, p, c.as_ref(), &av, ability_type).unwrap_or(0) < value {
            return Err(format!("needs {ability_type:?} {value}"));
        }
    }
    Ok(())
}

/// Learn a skill (from a skill book or a quest).
pub fn learn_skill(ctx: &ReducerContext, game: &GameData, p: &mut Player, skill_id: SkillId) -> Result<(), String> {
    let data = game.skills.get_skill(skill_id).ok_or("unknown skill")?;
    let mut skill_list = p.skill_list();
    if skill_list.find_skill_exact(data).is_some() {
        return Err("you already know that skill".into());
    }
    check_requirements(ctx, game, p, data)?;
    skill_list.add_skill(data).ok_or("no room for more skills")?;
    p.skill_points -= data.learn_point_cost;
    p.set_skill_list(&skill_list);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, game, p, false);
    items::notify(ctx, p.identity, format!("Learned {}", data.name));
    Ok(())
}

/// Spend skill points on the next level of a learned skill.
#[spacetimedb::reducer]
pub fn level_up_skill(ctx: &ReducerContext, page: u8, index: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let slot = SkillSlot(page as usize, index as usize);
    let mut skill_list = p.skill_list();
    let current_id = skill_list.get_skill(slot).ok_or("no skill there")?;
    let current = game.skills.get_skill(current_id).ok_or("unknown skill")?;
    let next = SkillId::new(current_id.get() + 1)
        .and_then(|id| game.skills.get_skill(id))
        .filter(|next| next.base_skill_id == current.base_skill_id && next.level == current.level + 1)
        .ok_or("that skill is at its highest level")?;
    check_requirements(ctx, &game, &p, next)?;
    *skill_list.get_slot_mut(slot).ok_or("no skill there")? = Some(next.id);
    p.skill_points -= next.learn_point_cost;
    p.set_skill_list(&skill_list);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    items::notify(ctx, p.identity, format!("{} is now level {}", next.name, next.level));
    Ok(())
}

// ---------------------------------------------------------------- casting

fn team(kind: EntityKind) -> u32 {
    match kind {
        EntityKind::Npc => 1,
        EntityKind::Player => 2,
        EntityKind::Monster => 100,
    }
}

/// An entity's team: summons are on their owner's.
fn team_of(ctx: &ReducerContext, e: &crate::Entity) -> u32 {
    if e.kind == EntityKind::Monster && owner_of(ctx, e.entity_id).is_some() {
        team(EntityKind::Player)
    } else {
        team(e.kind)
    }
}

fn identity_of(ctx: &ReducerContext, entity_id: u64) -> Option<spacetimedb::Identity> {
    ctx.db.player().iter().find(|p| p.entity_id == Some(entity_id)).map(|p| p.identity)
}

/// Whether a skill may hit or help this target (rose-offline's check_skill_target_filter).
/// Group skills reach the caster's party, Guild skills only the caster (no clans yet).
pub(crate) fn target_allowed(ctx: &ReducerContext, caster: u64, target: u64, skill: &SkillData) -> bool {
    let (Some(ce), Some(te)) = (ctx.db.entity().entity_id().find(caster), ctx.db.entity().entity_id().find(target))
    else {
        return false;
    };
    if ce.zone_id != te.zone_id {
        return false;
    }
    let alive = ctx.db.combat().entity_id().find(target).is_some_and(|c| c.hp > 0 && c.dead_until_us.is_none());
    let is_caster = caster == target;
    let (caster_team, target_team) = (team_of(ctx, &ce), team_of(ctx, &te));
    let character = te.kind == EntityKind::Player;
    let monster = te.kind == EntityKind::Monster;
    // Players (and their summons) are one team, except where two players may fight (PvP
    // zones).
    let (caster_player, target_player) = (controller(ctx, caster), controller(ctx, target));
    let pvp_enemy = caster_team == team(EntityKind::Player)
        && target_team == team(EntityKind::Player)
        && crate::game_data::game(ctx).is_ok_and(|game| crate::pvp::players_hostile(ctx, &game, caster_player, target_player));
    let enemy = caster_team != target_team || pvp_enemy;
    let allied = caster_team == target_team && !pvp_enemy;
    // Hidden characters can't be picked by enemies, except by skills that reveal them.
    if enemy && !is_caster && is_invisible(ctx, target) && !crate::game_data::game(ctx).is_ok_and(|game| reveals(&game, skill)) {
        return false;
    }
    match skill.target_filter {
        SkillTargetFilter::OnlySelf | SkillTargetFilter::Guild => alive && is_caster,
        SkillTargetFilter::Group => {
            alive
                && (is_caster
                    || (character
                        && ce.kind == EntityKind::Player
                        && match (identity_of(ctx, caster), identity_of(ctx, target)) {
                            (Some(a), Some(b)) => crate::party::same_party(ctx, a, b),
                            _ => false,
                        }))
        }
        SkillTargetFilter::Allied => alive && allied,
        SkillTargetFilter::Monster => alive && monster,
        SkillTargetFilter::Enemy => alive && target_team != 1 && enemy,
        SkillTargetFilter::EnemyCharacter => alive && enemy && character,
        SkillTargetFilter::Character => alive && character,
        SkillTargetFilter::CharacterOrMonster => alive && (character || monster),
        SkillTargetFilter::DeadAlliedCharacter => !alive && !is_caster && allied && character,
        SkillTargetFilter::EnemyMonster => alive && caster_team != target_team && monster,
    }
}

/// Whether a skill ends Stealth and disguises (Detect).
fn reveals(game: &GameData, skill: &SkillData) -> bool {
    skill.status_effects.iter().flatten().filter_map(|&id| game.status_effects.get_status_effect(id)).any(|d| {
        matches!(d.status_effect_type, StatusEffectType::ClearInvisible)
    })
}

fn cooldown_keys(skill: &SkillData) -> (u32, i64) {
    match skill.cooldown {
        SkillCooldown::Skill { duration } => (skill.id.get() as u32, duration.as_micros() as i64),
        SkillCooldown::Group { group, duration } => (GROUP_KEY + group.get() as u32, duration.as_micros() as i64),
    }
}

fn cooldown_until(ctx: &ReducerContext, entity_id: u64, key: u32) -> i64 {
    ctx.db.skill_cooldown().entity_id().filter(entity_id).find(|r| r.key == key).map_or(0, |r| r.until_us)
}

fn set_cooldown(ctx: &ReducerContext, entity_id: u64, key: u32, until_us: i64) {
    if let Some(mut r) = ctx.db.skill_cooldown().entity_id().filter(entity_id).find(|r| r.key == key) {
        r.until_us = until_us;
        ctx.db.skill_cooldown().id().update(r);
    } else {
        ctx.db.skill_cooldown().insert(SkillCooldownRow { id: 0, entity_id, key, until_us });
    }
}

/// Mana a skill costs after the caster's save-mana (rose-offline's use_mana_rate).
fn use_cost(av: &AbilityValues, ability_type: AbilityType, value: i32) -> i32 {
    if ability_type == AbilityType::Mana {
        (value as f32 * (100 - av.get_save_mana()) as f32 / 100.0) as i32
    } else {
        value
    }
}

/// rose-offline's skill_can_use for a player: cooldowns, costs and the weapon it needs.
pub(crate) fn check_can_use(ctx: &ReducerContext, game: &GameData, p: &Player, id: u64, skill: &SkillData, t: i64) -> Result<(), String> {
    if let Some(reason) = disabled_reason(ctx, id).or(crate::vehicle::passenger_refusal(ctx, id)) {
        return Err(reason.into());
    }
    if has_status(ctx, id, StatusEffectType::Dumb) {
        return Err("you can't use skills while silenced".into());
    }
    if cooldown_until(ctx, id, 0) > t || cooldown_until(ctx, id, cooldown_keys(skill).0) > t {
        return Err(format!("{} isn't ready yet", skill.name));
    }
    let av = items::player_ability_values(ctx, game, p);
    let c = ctx.db.combat().entity_id().find(id);
    for &(ability_type, value) in skill.use_ability.iter() {
        let have = ability::get_value(ctx, p, c.as_ref(), &av, ability_type).unwrap_or(0);
        if have < use_cost(&av, ability_type, value) {
            return Err(match ability_type {
                AbilityType::Mana => "not enough MP".into(),
                AbilityType::Health => "not enough HP".into(),
                AbilityType::Stamina => format!("needs {value} Stamina (you have {have})"),
                other => format!("needs {other:?} {value}"),
            });
        }
    }
    if let Some(refusal) = crate::vehicle::skill_refusal(ctx, game, p, id, skill) {
        return Err(refusal);
    }
    let vehicle_skill = skill
        .required_equipment_class
        .iter()
        .any(|c| matches!(c, rose_data::ItemClass::CartBody | rose_data::ItemClass::CastleGearBody));
    if !skill.required_equipment_class.is_empty() && !vehicle_skill {
        let equipment = p.equipment();
        let class = |index| {
            equipment
                .get_equipment_item(index)
                .and_then(|item| game.items.get_base_item(item.item))
                .map(|data| data.class)
        };
        let (weapon, sub_weapon) = (class(EquipmentIndex::Weapon), class(EquipmentIndex::SubWeapon));
        if !skill.required_equipment_class.iter().any(|&c| Some(c) == weapon || Some(c) == sub_weapon) {
            return Err(format!("{} needs another weapon", skill.name));
        }
    }
    Ok(())
}

/// Pay a skill's costs and start its cooldowns (rose-offline's subtract_skill_use_cost).
pub(crate) fn pay_costs(ctx: &ReducerContext, game: &GameData, id: u64, skill: &SkillData, t: i64) {
    set_cooldown(ctx, id, 0, t + GLOBAL_COOLDOWN_US);
    let (key, duration) = cooldown_keys(skill);
    if duration > 0 {
        set_cooldown(ctx, id, key, t + duration);
    }
    let Some(mut p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) else { return };
    let av = items::player_ability_values(ctx, game, &p);
    let mut c = ctx.db.combat().entity_id().find(id);
    let mut player_changed = false;
    for &(ability_type, value) in skill.use_ability.iter() {
        let cost = use_cost(&av, ability_type, value);
        match ability_type {
            AbilityType::Health => {
                if let Some(c) = c.as_mut() {
                    c.hp = (c.hp - cost).max(1);
                }
            }
            AbilityType::Mana => {
                if let Some(c) = c.as_mut() {
                    c.mp = (c.mp - cost).max(0);
                }
            }
            AbilityType::Experience => {
                p.xp = p.xp.saturating_sub(cost.max(0) as u64);
                player_changed = true;
            }
            AbilityType::Stamina => crate::stamina::add(ctx, p.identity, -cost),
            AbilityType::Fuel => {
                let mut fuel_user = p.clone();
                crate::vehicle::use_fuel(ctx, game, &mut fuel_user, Some(cost));
                p = fuel_user;
            }
            AbilityType::Money => {
                let mut inventory = p.inventory();
                inventory.money = Money((inventory.money.0 - cost as i64).max(0));
                p.set_inventory(&inventory);
                player_changed = true;
            }
            _ => {}
        }
    }
    if let Some(c) = c {
        ctx.db.combat().entity_id().update(c);
    }
    if player_changed {
        ctx.db.player().identity().update(p);
    }
}

/// Length of a character motion for this skill, scaled by its speed.
/// A driver's skill plays on the vehicle, whose arms part picks the motion column.
fn motion_us(ctx: &ReducerContext, game: &GameData, p: &Player, id: u64, motion_id: Option<rose_data::MotionId>, speed: f32) -> i64 {
    let equipment = p.equipment();
    let (weapon_motion, gender) = if crate::vehicle::is_driving(ctx, id) {
        let arms = equipment
            .get_vehicle_item(rose_data::VehiclePartIndex::Arms)
            .and_then(|a| game.items.get_vehicle_item(a.item.item_number))
            .map_or(0, |v| v.base_motion_index as usize);
        (arms, 0)
    } else {
        let weapon = equipment
            .get_equipment_item(EquipmentIndex::Weapon)
            .and_then(|item| game.items.get_weapon_item(item.item.item_number))
            .map_or(0, |weapon| weapon.motion_type as usize);
        (weapon, p.gender as usize)
    };
    motion_id
        .and_then(|id| game.motions.find_first_character_motion(id, weapon_motion, gender))
        .map_or(0, |m| (m.duration.as_micros() as f32 * if speed > 0.0 { speed } else { 1.0 }) as i64)
}

/// The ground point a cast aims at: its target entity, its ground target, or the caster.
fn cast_point(ctx: &ReducerContext, cast: &SkillCast, t: i64) -> Option<(f32, f32)> {
    if cast.at_position {
        Some((cast.target_x, cast.target_y))
    } else {
        position(ctx, cast.target.unwrap_or(cast.entity_id), t)
    }
}

/// A queued cast: walk into range, then start casting (pay, start cooldowns, time it).
/// Err cancels the cast.
fn advance_cast(ctx: &ReducerContext, game: &GameData, mut cast: SkillCast, t: i64) -> Result<(), String> {
    let id = cast.entity_id;
    let skill = skill(game, cast.skill_id)?;
    let p = ctx.db.player().iter().find(|p| p.entity_id == Some(id)).ok_or("not a player")?;
    if let Some(target) = cast.target {
        if target != id && !target_allowed(ctx, id, target, skill) {
            return Err("you lost your target".into());
        }
    }
    let me = position(ctx, id, t).ok_or("no position")?;
    let there = cast_point(ctx, &cast, t).ok_or("you lost your target")?;
    let range = if skill.cast_range > 0 {
        skill.cast_range as f32
    } else {
        ctx.db.stats().entity_id().find(id).map_or(100.0, |s| s.attack_range)
    };
    if distance(me, there) > range + RANGE_SLACK_CM {
        let speed = ctx.db.stats().entity_id().find(id).map_or(450.0, |s| s.move_speed);
        let chase = cast.target.filter(|&target| target != id);
        let repath = match ctx.db.motion().entity_id().find(id) {
            Some(m) => distance((m.to_x, m.to_y), there) > crate::CHASE_REPATH_CM,
            None => true,
        };
        if repath {
            set_motion(ctx, id, there, speed, chase);
        }
        return Ok(());
    }
    stop_motion(ctx, id);
    check_can_use(ctx, game, &p, id, skill, t)?;
    pay_costs(ctx, game, id, skill, t);
    let casting = motion_us(ctx, game, &p, id, skill.casting_motion_id, skill.casting_motion_speed);
    let action = motion_us(ctx, game, &p, id, skill.action_motion_id, skill.action_motion_speed);
    cast.started_at_us = Some(t);
    cast.effect_at_us = t + casting;
    cast.ends_at_us = t + (casting + action).max(MIN_CAST_US);
    ctx.db.skill_cast().entity_id().update(cast);
    Ok(())
}

/// Queue a cast for a character (it starts once in range). The skill's checks run now too,
/// so a refused cast doesn't walk anywhere.
pub fn start_cast(
    ctx: &ReducerContext,
    game: &GameData,
    p: &Player,
    id: u64,
    skill: &SkillData,
    target: Option<u64>,
    at: Option<(f32, f32)>,
    use_item: Option<(i32, &Item)>,
) -> Result<(), String> {
    let t = now_us(ctx);
    check_can_use(ctx, game, p, id, skill, t)?;
    crate::stand_up(ctx, id);
    break_disguise(ctx, game, id);
    let self_skill = skill.skill_type.is_self_skill();
    let target = if self_skill { None } else { target };
    if let Some(target) = target {
        if !target_allowed(ctx, id, target, skill) {
            return Err(format!("{} can't be used on that", skill.name));
        }
    } else if at.is_none() && !self_skill && !target_allowed(ctx, id, id, skill) {
        return Err(format!("{} needs a target", skill.name));
    }
    crate::cancel_attack(ctx, id, t);
    ctx.db.skill_cast().entity_id().delete(id);
    let (x, y) = at.unwrap_or((0.0, 0.0));
    let cast = ctx.db.skill_cast().insert(SkillCast {
        entity_id: id,
        skill_id: skill.id.get(),
        target,
        target_x: x,
        target_y: y,
        at_position: at.is_some(),
        started_at_us: None,
        effect_at_us: 0,
        ends_at_us: 0,
        applied: false,
        use_item_slot: use_item.map_or(-1, |(slot, _)| slot),
        use_item: use_item.map_or(String::new(), |(_, item)| serde_json::to_string(item).unwrap_or_default()),
    });
    if let Err(e) = advance_cast(ctx, game, cast, t) {
        ctx.db.skill_cast().entity_id().delete(id);
        return Err(e);
    }
    Ok(())
}

pub fn is_casting(ctx: &ReducerContext, id: u64) -> bool {
    ctx.db.skill_cast().entity_id().find(id).is_some()
}

/// Moving, attacking or stopping drops a cast that hasn't taken effect (its cost is spent).
pub fn cancel_cast(ctx: &ReducerContext, id: u64) {
    ctx.db.skill_cast().entity_id().delete(id);
    ctx.db.npc_cast_motion().entity_id().delete(id);
}

/// A cast that only plays its motions on the caster (emotes, jumping): no walking, no
/// target, nothing taking effect.
fn self_motion_cast(ctx: &ReducerContext, game: &GameData, p: &Player, id: u64, skill: &SkillData) -> Result<(), String> {
    let t = now_us(ctx);
    check_can_use(ctx, game, p, id, skill, t)?;
    crate::stand_up(ctx, id);
    crate::cancel_attack(ctx, id, t);
    cancel_cast(ctx, id);
    stop_motion(ctx, id);
    pay_costs(ctx, game, id, skill, t);
    let casting = motion_us(ctx, game, p, id, skill.casting_motion_id, skill.casting_motion_speed);
    let action = motion_us(ctx, game, p, id, skill.action_motion_id, skill.action_motion_speed);
    ctx.db.skill_cast().insert(SkillCast {
        entity_id: id,
        skill_id: skill.id.get(),
        target: None,
        target_x: 0.0,
        target_y: 0.0,
        at_position: false,
        started_at_us: Some(t),
        effect_at_us: t + casting,
        ends_at_us: t + (casting + action).max(MIN_CAST_US),
        applied: false,
        use_item_slot: -1,
        use_item: String::new(),
    });
    Ok(())
}

/// Pick up the nearest item on the ground in reach (the Pick Up action).
fn pickup_nearest(ctx: &ReducerContext, p: &Player, id: u64) -> Result<(), String> {
    let t = now_us(ctx);
    let me = position(ctx, id, t).ok_or("no position")?;
    let nearest = ctx
        .db
        .ground_item()
        .zone_id()
        .filter(p.zone_id)
        .filter(|g| g.owner.is_none_or(|o| o == p.identity || crate::party::same_party(ctx, o, p.identity)) || t >= g.owner_until_us)
        .map(|g| (g.drop_id, distance(me, (g.x, g.y))))
        .filter(|(_, d)| *d <= items::PICKUP_RANGE_CM)
        .min_by(|a, b| a.1.total_cmp(&b.1));
    let (drop_id, _) = nearest.ok_or("nothing to pick up nearby")?;
    items::pickup_item(ctx, drop_id)
}

/// Use the skill in a skill list slot on a target entity, a ground point, or yourself.
#[spacetimedb::reducer]
pub fn cast_skill(ctx: &ReducerContext, page: u8, index: u16, target: Option<u64>, x: f32, y: f32) -> Result<(), String> {
    let game = game(ctx)?;
    let (p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let skill_id = p.skill_list().get_skill(SkillSlot(page as usize, index as usize)).ok_or("no skill there")?;
    let skill = game.skills.get_skill(skill_id).ok_or("unknown skill")?;
    if let Some(reason) = disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    // Sitting (standing up) is the only action while our shop is open.
    if crate::shop::is_open(ctx, id) && !matches!(skill.basic_command, Some(SkillBasicCommand::Sit)) {
        return Err("close your shop first".into());
    }
    match skill.skill_type {
        SkillType::Passive => return Err(format!("{} works on its own", skill.name)),
        SkillType::BasicAction => {
            use SkillBasicCommand as B;
            return match (skill.basic_command, target) {
                (Some(B::Sit), _) => crate::sit(ctx),
                (Some(B::DriveVehicle), _) => crate::vehicle::drive_toggle(ctx),
                (Some(B::Attack), Some(target)) => crate::attack(ctx, target),
                (Some(B::PickupItem), _) => pickup_nearest(ctx, &p, id),
                (Some(B::Jump | B::AirJump), _) => self_motion_cast(ctx, &game, &p, id, skill),
                (Some(B::PartyInvite), Some(target)) => crate::party::party_invite(ctx, target),
                (Some(B::Trade), Some(target)) => crate::trade::trade_ask(ctx, target),
                (Some(B::AddFriend), Some(target)) => crate::friends::friend_ask_entity(ctx, target),
                (Some(B::VehiclePassengerInvite), Some(target)) => crate::vehicle::ride_offer_to(ctx, target),
                // Ride Request (skill 25) has no command in this client's data.
                (None, Some(target)) if skill.id.get() == 25 => crate::vehicle::ride_offer_to(ctx, target),
                (Some(B::Attack | B::PartyInvite | B::Trade | B::AddFriend | B::VehiclePassengerInvite), None) => Err("pick a target first".into()),
                // Picking a target is done by the game client.
                (Some(B::AutoTarget | B::SelfTarget), _) => Ok(()),
                _ => Err(format!("{} isn't in the game yet", skill.name)),
            };
        }
        SkillType::Emote => return self_motion_cast(ctx, &game, &p, id, skill),
        SkillType::CreateWindow => return Err(format!("{} isn't in the game yet", skill.name)),
        _ => {}
    }
    let at = (matches!(skill.skill_type, SkillType::AreaTarget) && target.is_none() && x.is_finite() && y.is_finite())
        .then_some((x, y));
    start_cast(ctx, &game, &p, id, skill, target, at, None)
}

/// Use a scroll (a MagicItem consumable) from the inventory: it casts its skill on the
/// caster, or warps for return scrolls. The scroll is used up when the skill takes effect.
pub fn use_scroll(ctx: &ReducerContext, game: &GameData, p: &Player, id: u64, slot: ItemSlot, item: &Item, skill_id: SkillId) -> Result<(), String> {
    let skill = game.skills.get_skill(skill_id).ok_or("unknown skill")?;
    let encoded = match slot {
        ItemSlot::Inventory(page, index) => page as i32 * 100 + index as i32,
        _ => return Err("not an inventory item".into()),
    };
    let target = ctx.db.combat().entity_id().find(id).and_then(|c| c.attack_target);
    if skill.skill_type.is_target_skill() && target.is_none() {
        return Err("pick a target first".into());
    }
    start_cast(ctx, game, p, id, skill, target, None, Some((encoded, item)))
}

/// Advance every cast: walk into range, start, take effect, finish (from combat_tick).
pub fn cast_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let casts: Vec<SkillCast> = ctx.db.skill_cast().iter().collect();
    for cast in casts {
        let id = cast.entity_id;
        let notify = |text: String| {
            if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) {
                items::notify(ctx, p.identity, text);
            }
        };
        if !crate::is_alive(ctx, id) {
            cancel_cast(ctx, id);
            continue;
        }
        if cast.started_at_us.is_none() {
            if let Err(e) = advance_cast(ctx, game, cast, t) {
                cancel_cast(ctx, id);
                notify(capitalise(&e));
            }
            continue;
        }
        if !cast.applied && t >= cast.effect_at_us {
            let mut cast = cast.clone();
            cast.applied = true;
            ctx.db.skill_cast().entity_id().update(cast.clone());
            if let Err(e) = take_effect(ctx, game, &cast, t) {
                notify(capitalise(&e));
            }
            continue;
        }
        if cast.applied && t >= cast.ends_at_us {
            cancel_cast(ctx, id);
            // Melee skills go back to attacking their target (SkillActionMode::Attack).
            if let (Ok(skill), Some(target)) = (skill(game, cast.skill_id), cast.target) {
                if matches!(skill.action_mode, SkillActionMode::Attack)
                    && target != id
                    && crate::pvp::can_attack(ctx, game, id, target)
                    && crate::is_alive(ctx, target)
                {
                    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
                        c.attack_target = Some(target);
                        ctx.db.combat().entity_id().update(c);
                    }
                }
            }
        }
    }
}

/// Whether the entity has a buff (good: Some(true)), a debuff (Some(false)) or any status
/// effect (None) on it, for the monster AI's status checks.
pub fn has_status_effects(ctx: &ReducerContext, entity_id: u64, good: Option<bool>) -> bool {
    ctx.db.status_effect().entity_id().filter(entity_id).any(|r| {
        let effect_type = StatusEffectType::from_usize(r.effect_type as usize);
        match good {
            Some(true) => effect_type.is_good(),
            Some(false) => effect_type.is_bad(),
            None => true,
        }
    })
}

fn capitalise(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or(String::new(), |c| c.to_uppercase().collect::<String>() + chars.as_str())
}

/// Take a used scroll out of the inventory (it may have been moved or sold since).
fn consume_scroll(ctx: &ReducerContext, cast: &SkillCast) -> Result<(), String> {
    if cast.use_item_slot < 0 {
        return Ok(());
    }
    let mut p = ctx.db.player().iter().find(|p| p.entity_id == Some(cast.entity_id)).ok_or("not a player")?;
    let item: Item = serde_json::from_str(&cast.use_item).map_err(|_| "the scroll is gone")?;
    let slot = items::inventory_slot((cast.use_item_slot / 100) as u8, (cast.use_item_slot % 100) as u16)?;
    let mut inventory = p.inventory();
    let held = inventory.get_item(slot).ok_or("the scroll is gone")?;
    if !item.is_same_item(held) {
        return Err("the scroll is gone".into());
    }
    inventory.try_take_quantity(slot, 1).ok_or("the scroll is gone")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p);
    Ok(())
}

/// The cast reached its effect frame: damage, heal, buff or warp (skill_effect_system).
fn take_effect(ctx: &ReducerContext, game: &GameData, cast: &SkillCast, t: i64) -> Result<(), String> {
    let skill = skill(game, cast.skill_id)?;
    consume_scroll(ctx, cast)?;
    let caster = cast.entity_id;
    match skill.skill_type {
        SkillType::Immediate
        | SkillType::EnforceWeapon
        | SkillType::EnforceBullet
        | SkillType::FireBullet
        | SkillType::AreaTarget
        | SkillType::SelfDamage => {
            for target in targets(ctx, cast, skill, t) {
                if damage(ctx, game, caster, target, skill, t).is_some() {
                    apply_effects(ctx, game, caster, target, skill, t);
                }
            }
            weapon_wear(ctx, game, caster, skill);
            Ok(())
        }
        SkillType::SelfBoundDuration
        | SkillType::SelfStateDuration
        | SkillType::TargetBoundDuration
        | SkillType::TargetStateDuration
        | SkillType::SelfBound
        | SkillType::TargetBound => {
            for target in targets(ctx, cast, skill, t) {
                apply_effects(ctx, game, caster, target, skill, t);
            }
            Ok(())
        }
        SkillType::SelfAndTarget => {
            let target = cast.target.ok_or("you lost your target")?;
            if damage(ctx, game, caster, target, skill, t).is_some_and(|amount| amount > 0) {
                apply_effects(ctx, game, caster, target, skill, t);
            }
            weapon_wear(ctx, game, caster, skill);
            Ok(())
        }
        SkillType::Resurrection => {
            for target in targets(ctx, cast, skill, t) {
                if resurrect(ctx, game, target) {
                    // Some of the experience the death cost comes back (Cancel_PenalEXP).
                    crate::death::refund(ctx, target, skill.power as u64);
                }
            }
            Ok(())
        }
        SkillType::SummonPet => summon(ctx, game, caster, skill, t),
        SkillType::Warp => {
            let zone = skill.warp_zone_id.ok_or("the scroll leads nowhere")?;
            let mut p = ctx.db.player().iter().find(|p| p.entity_id == Some(caster)).ok_or("not a player")?;
            world::teleport(ctx, &mut p, zone.get(), (skill.warp_zone_x, skill.warp_zone_y));
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Who a cast reaches: everyone in its scope around the aim point, or its one target.
fn targets(ctx: &ReducerContext, cast: &SkillCast, skill: &SkillData, t: i64) -> Vec<u64> {
    let caster = cast.entity_id;
    if skill.scope > 0 {
        let Some(centre) = cast_point(ctx, cast, t) else { return Vec::new() };
        let Some(zone) = ctx.db.entity().entity_id().find(caster).map(|e| e.zone_id) else { return Vec::new() };
        ctx.db
            .entity()
            .iter()
            .filter(|e| e.zone_id == zone && e.kind != EntityKind::Npc)
            .filter(|e| position(ctx, e.entity_id, t).is_some_and(|p| distance(p, centre) <= skill.scope as f32))
            .filter(|e| target_allowed(ctx, caster, e.entity_id, skill))
            .map(|e| e.entity_id)
            .collect()
    } else {
        let target = cast.target.unwrap_or(caster);
        if target_allowed(ctx, caster, target, skill) {
            vec![target]
        } else {
            Vec::new()
        }
    }
}

fn ability_values_of(ctx: &ReducerContext, id: u64) -> Option<AbilityValues> {
    ctx.db.stats().entity_id().find(id).and_then(|s| serde_json::from_str(&s.ability_values).ok())
}

/// A damaging skill wears the caster's weapon, except natural magic (as iROSE's
/// SKILL_DAMAGE_TYPE 3 check before Dec_WeaponLife).
fn weapon_wear(ctx: &ReducerContext, game: &GameData, caster: u64, skill: &SkillData) {
    if !matches!(skill.damage_type, rose_data::SkillDamageType::NaturalMagic) {
        crate::durability::weapon_used(ctx, game, caster);
    }
}

/// Skill damage on one target; Some(amount) when it landed.
fn damage(ctx: &ReducerContext, game: &GameData, caster: u64, target: u64, skill: &SkillData, t: i64) -> Option<i32> {
    let (a, d) = (ability_values_of(ctx, caster)?, ability_values_of(ctx, target)?);
    let dmg = game.ability_value_calculator.calculate_skill_damage(&a, &d, skill, 1);
    crate::deal_damage(ctx, game, caster, target, dmg.amount as i32, dmg.is_critical, t);
    Some(dmg.amount as i32)
}

/// A value of the target that a skill's add_ability scales with.
fn target_value(av: &AbilityValues, c: Option<&Combat>, ability_type: AbilityType) -> i32 {
    match ability_type {
        AbilityType::Health => c.map_or(0, |c| c.hp),
        AbilityType::Mana => c.map_or(0, |c| c.mp),
        AbilityType::MaxHealth => av.get_max_health(),
        AbilityType::MaxMana => av.get_max_mana(),
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
        AbilityType::Level => av.get_level(),
        _ => 0,
    }
}

/// A skill's status effects and instant heals on one target
/// (rose-offline's apply_skill_status_effects_to_entity).
fn apply_effects(ctx: &ReducerContext, game: &GameData, caster: u64, target: u64, skill: &SkillData, t: i64) {
    if !target_allowed(ctx, caster, target, skill) {
        return;
    }
    let (Some(a), Some(d)) = (ability_values_of(ctx, caster), ability_values_of(ctx, target)) else { return };
    let mut rng = ctx.rng();
    let mut effects = status_effects(ctx, target);
    let mut changed = false;
    for (index, data) in skill
        .status_effects
        .iter()
        .enumerate()
        .filter_map(|(i, id)| id.and_then(|id| game.status_effects.get_status_effect(id)).map(|d| (i, d)))
    {
        if skill.success_ratio > 0 {
            let missed = match data.cleared_by_type {
                StatusEffectClearedByType::ClearGood => {
                    skill.success_ratio < d.get_level() - a.get_level() + rng.gen_range(1..=100)
                }
                _ => {
                    skill.success_ratio as f32 * (a.get_level() * 2 + a.get_intelligence() + 20) as f32
                        / (d.get_resistance() as f32 * 0.6 + 5.0 + d.get_avoid() as f32)
                        <= rng.gen_range(1..=100) as f32
                }
            };
            if missed {
                continue;
            }
        }
        let value = if matches!(data.status_effect_type, StatusEffectType::AdditionalDamageRate | StatusEffectType::ShieldDamage) {
            skill.power as i32
        } else if let Some(add) = skill.add_ability[index].as_ref() {
            let c = ctx.db.combat().entity_id().find(target);
            let current = target_value(&d, c.as_ref(), add.ability_type);
            game.ability_value_calculator.calculate_skill_adjust_value(add, a.get_intelligence(), current)
        } else {
            0
        };
        match data.status_effect_type {
            clear @ (StatusEffectType::ClearGood
            | StatusEffectType::ClearBad
            | StatusEffectType::ClearAll
            | StatusEffectType::ClearInvisible) => {
                if clear_status(ctx, game, target, clear) {
                    effects = status_effects(ctx, target);
                    changed = true;
                }
                continue;
            }
            StatusEffectType::DecreaseLifeTime => continue,
            _ => {}
        }
        if !effects.can_apply(data, value) {
            continue;
        }
        let effect_type = data.status_effect_type.into_usize() as u8;
        let old: Vec<u64> = ctx
            .db
            .status_effect()
            .entity_id()
            .filter(target)
            .filter(|r| r.effect_type == effect_type)
            .map(|r| r.id)
            .collect();
        for id in old {
            ctx.db.status_effect().id().delete(id);
        }
        ctx.db.status_effect().insert(StatusEffectRow {
            id: 0,
            entity_id: target,
            effect_type,
            status_effect_id: data.id.get(),
            value,
            expires_at_us: t + skill.status_effect_duration.as_micros() as i64,
        });
        effects.active[data.status_effect_type] = Some(ActiveStatusEffect { id: data.id, value });
        changed = true;
        match data.status_effect_type {
            // Stunned or asleep: whatever it was doing stops.
            StatusEffectType::Fainting | StatusEffectType::Sleep => interrupt(ctx, target, t),
            // A taunted monster turns on the taunter.
            StatusEffectType::Taunt => {
                if ctx.db.entity().entity_id().find(target).is_some_and(|e| e.kind == EntityKind::Monster) {
                    ctx.db.taunt().entity_id().delete(target);
                    ctx.db.taunt().insert(Taunt { entity_id: target, taunter: caster });
                    if let Some(mut c) = ctx.db.combat().entity_id().find(target) {
                        if c.attack_target != Some(caster) {
                            c.attack_target = Some(caster);
                            c.swing_hit_at_us = None;
                            ctx.db.combat().entity_id().update(c);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if changed {
        refresh_entity(ctx, game, target);
    }
    for add in skill.add_ability.iter().flatten() {
        let Some(mut c) = ctx.db.combat().entity_id().find(target) else { continue };
        match add.ability_type {
            AbilityType::Health => {
                let heal = game.ability_value_calculator.calculate_skill_adjust_value(add, a.get_intelligence(), c.hp);
                c.hp = (c.hp + heal).min(c.max_hp);
            }
            AbilityType::Mana => c.mp = (c.mp + add.value).min(c.max_mp),
            _ => continue,
        }
        ctx.db.combat().entity_id().update(c);
    }
}

/// Remove an entity's buffs (getting on or off a vehicle, iROSE's ClearAllGOOD).
pub fn clear_good(ctx: &ReducerContext, game: &GameData, entity_id: u64) {
    if clear_status(ctx, game, entity_id, StatusEffectType::ClearGood) {
        refresh_entity(ctx, game, entity_id);
    }
}

/// A cleansing effect: ClearGood takes buffs, ClearBad debuffs and taunts, ClearAll both
/// (not taunts), ClearInvisible ends Stealth and disguises (iROSE's CEndurePACK::ClearSTATUS).
/// True if anything was removed.
fn clear_status(ctx: &ReducerContext, game: &GameData, entity_id: u64, clear: StatusEffectType) -> bool {
    use StatusEffectClearedByType as By;
    use StatusEffectType as S;
    let rows: Vec<u64> = ctx
        .db
        .status_effect()
        .entity_id()
        .filter(entity_id)
        .filter(|r| {
            let effect_type = S::from_usize(r.effect_type as usize);
            let cleared_by = StatusEffectId::new(r.status_effect_id)
                .and_then(|id| game.status_effects.get_status_effect(id))
                .map(|d| &d.cleared_by_type);
            match clear {
                S::ClearGood => matches!(cleared_by, Some(By::ClearGood)),
                S::ClearBad => matches!(cleared_by, Some(By::ClearBad)) || matches!(effect_type, S::Taunt),
                S::ClearAll => {
                    matches!(cleared_by, Some(By::ClearGood | By::ClearBad))
                        && !matches!(effect_type, S::Taunt | S::Revive | S::DecreaseLifeTime)
                }
                S::ClearInvisible => matches!(effect_type, S::Disguise | S::Transparent),
                _ => false,
            }
        })
        .map(|r| r.id)
        .collect();
    for id in rows.iter() {
        ctx.db.status_effect().id().delete(*id);
    }
    !rows.is_empty()
}

/// Remove every status effect on an entity and recalculate its stats.
pub fn clear_all_status(ctx: &ReducerContext, game: &GameData, entity_id: u64) {
    let rows: Vec<u64> = ctx.db.status_effect().entity_id().filter(entity_id).map(|r| r.id).collect();
    for id in rows {
        ctx.db.status_effect().id().delete(id);
    }
    refresh_entity(ctx, game, entity_id);
}

/// Bring a dead player back where they fell, with some of their HP (iROSE's resurrection).
/// True if they were dead.
fn resurrect(ctx: &ReducerContext, game: &GameData, target: u64) -> bool {
    if ctx.db.combat().entity_id().find(target).is_none_or(|c| c.dead_until_us.is_none()) {
        return false;
    }
    // The share of HP is of the maximum without the buffs that just ended.
    clear_all_status(ctx, game, target);
    let Some(mut c) = ctx.db.combat().entity_id().find(target) else { return false };
    c.dead_until_us = None;
    c.hp = (c.max_hp * RESURRECT_HP_PERCENT / 100).max(1);
    c.attack_target = None;
    c.swing_hit_at_us = None;
    ctx.db.combat().entity_id().update(c);
    true
}

/// How many summon points a player has: 50 plus what their passive skills add.
fn summon_capacity(game: &GameData, p: &Player) -> i32 {
    let skill_list = p.skill_list();
    let passive: i32 = skill_list
        .pages
        .iter()
        .flat_map(|page| page.skills.iter().flatten())
        .filter_map(|&id| game.skills.get_skill(id))
        .flat_map(|data| data.add_ability.iter().flatten())
        .filter(|add| add.ability_type == AbilityType::PassiveMaxSummons)
        .map(|add| add.value)
        .sum();
    BASE_SUMMON_POINTS + passive
}

/// A summoning skill took effect: its monster appears next to the caster, on their side.
fn summon(ctx: &ReducerContext, game: &GameData, caster: u64, skill: &SkillData, t: i64) -> Result<(), String> {
    let p = ctx.db.player().iter().find(|p| p.entity_id == Some(caster)).ok_or("not a player")?;
    let npc_id = skill.summon_npc_id.ok_or("nothing to summon")?;
    let npc = game.npcs.get_npc(npc_id).ok_or("nothing to summon")?;
    let points = npc.summon_point_requirement;
    let used: u32 = ctx.db.summon().owner().filter(caster).map(|s| s.points).sum();
    if (used + points) as i32 > summon_capacity(game, &p) {
        return Err("you can't summon any more".into());
    }
    let zone_id = ctx.db.entity().entity_id().find(caster).ok_or("no character")?.zone_id;
    let at = position(ctx, caster, t).ok_or("no position")?;
    let mut rng = ctx.rng();
    let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let r: f32 = rng.gen_range(50.0..150.0);
    let (x, y) = (at.0 + angle.cos() * r, at.1 + angle.sin() * r);
    let entity = ctx.db.entity().insert(crate::Entity {
        entity_id: 0,
        kind: EntityKind::Monster,
        npc_id: npc_id.get(),
        zone_id,
        name: npc.name.to_string(),
    });
    let id = entity.entity_id;
    let (owner_level, skill_level) = (p.level as i32, skill.level as i32);
    let Some(stats) =
        world::npc_stats_for(game, id, npc_id.get(), &StatusEffects::default(), Some((owner_level, skill_level)))
    else {
        ctx.db.entity().entity_id().delete(id);
        return Err("nothing to summon".into());
    };
    ctx.db.motion().insert(crate::Motion {
        entity_id: id,
        from_x: x,
        from_y: y,
        to_x: x,
        to_y: y,
        started_at_us: t,
        speed: stats.move_speed,
        chase_target: None,
    });
    ctx.db.combat().insert(Combat {
        entity_id: id,
        hp: stats.max_hp,
        max_hp: stats.max_hp,
        mp: stats.max_mp,
        max_mp: stats.max_mp,
        attack_target: None,
        next_attack_at_us: t,
        swing_hit_at_us: None,
        dead_until_us: None,
    });
    ctx.db.stats().insert(stats);
    ctx.db.summon().insert(Summon { entity_id: id, owner: caster, owner_level, skill_level, points });
    Ok(())
}

/// Summons follow their owner and fight what the owner fights or what attacks either of
/// them (iROSE's summon AI: MoveNearOwner, AttackOwnerTarget). They leave when the owner
/// dies, logs out or changes zone. Every combat tick.
pub fn summon_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let summons: Vec<Summon> = ctx.db.summon().iter().collect();
    for s in summons {
        let id = s.entity_id;
        let zone = |id| ctx.db.entity().entity_id().find(id).map(|e| e.zone_id);
        if zone(s.owner).is_none() || zone(s.owner) != zone(id) || !crate::is_alive(ctx, s.owner) {
            crate::despawn(ctx, id);
            continue;
        }
        let Some(mut c) = ctx.db.combat().entity_id().find(id) else { continue };
        if c.dead_until_us.is_some() || is_disabled(ctx, id) {
            continue;
        }
        let (Some(me), Some(owner_at)) = (position(ctx, id, t), position(ctx, s.owner, t)) else { continue };
        let hostile = |target: u64| {
            target != id && target != s.owner && crate::is_alive(ctx, target) && crate::pvp::can_attack(ctx, game, id, target)
        };
        // The owner's target comes first; then whoever is fighting the owner or the summon.
        let owner_target = ctx
            .db
            .combat()
            .entity_id()
            .find(s.owner)
            .and_then(|oc| oc.attack_target)
            .or_else(|| ctx.db.skill_cast().entity_id().find(s.owner).and_then(|cast| cast.target))
            .filter(|&target| hostile(target));
        let wanted = owner_target.or(c.attack_target.filter(|&target| hostile(target))).or_else(|| {
            ctx.db
                .combat()
                .iter()
                .find(|other| {
                    (other.attack_target == Some(s.owner) || other.attack_target == Some(id)) && hostile(other.entity_id)
                })
                .map(|other| other.entity_id)
        });
        // Too far from the owner: stop fighting and catch up.
        let far = distance(me, owner_at) > SUMMON_FOLLOW_CM * 4.0;
        let wanted = wanted.filter(|_| !far);
        if c.attack_target != wanted {
            c.attack_target = wanted;
            c.swing_hit_at_us = None;
            ctx.db.combat().entity_id().update(c);
        }
        if wanted.is_none() && distance(me, owner_at) > SUMMON_FOLLOW_CM {
            let d = distance(me, owner_at).max(1.0);
            let k = SUMMON_FOLLOW_TO_CM / d;
            let to = (owner_at.0 + (me.0 - owner_at.0) * k, owner_at.1 + (me.1 - owner_at.1) * k);
            let repath = ctx.db.motion().entity_id().find(id).is_none_or(|m| !m.is_moving(t) || distance((m.to_x, m.to_y), to) > crate::CHASE_REPATH_CM * 2.0);
            if repath {
                let speed = ctx.db.stats().entity_id().find(id).map_or(450.0, |st| st.run_speed);
                set_motion(ctx, id, to, speed, None);
            }
        }
    }
}

/// A taunted monster keeps attacking its taunter; a taunt that ended is forgotten.
pub fn taunt_tick(ctx: &ReducerContext) {
    let rows: Vec<Taunt> = ctx.db.taunt().iter().collect();
    for row in rows {
        match taunter(ctx, row.entity_id) {
            Some(taunter) => {
                if let Some(mut c) = ctx.db.combat().entity_id().find(row.entity_id) {
                    if c.attack_target != Some(taunter) && c.dead_until_us.is_none() {
                        c.attack_target = Some(taunter);
                        c.swing_hit_at_us = None;
                        ctx.db.combat().entity_id().update(c);
                    }
                }
            }
            None => {
                ctx.db.taunt().entity_id().delete(row.entity_id);
            }
        }
    }
}

/// A monster uses a skill from its AIP UseSkill action, with the NPC motion it names for
/// casting and the next one for the action. Monsters pay nothing and have no cooldowns;
/// a sleeping, stunned or silenced one can't (iROSE's CObjMOB skill use).
pub fn npc_cast(ctx: &ReducerContext, game: &GameData, id: u64, skill_id: u16, target: u64, motion_id: i32, t: i64) {
    if is_disabled(ctx, id) || has_status(ctx, id, StatusEffectType::Dumb) || ctx.db.skill_cast().entity_id().find(id).is_some() {
        return;
    }
    let Ok(skill) = skill(game, skill_id) else { return };
    let Some(e) = ctx.db.entity().entity_id().find(id) else { return };
    let target = if skill.skill_type.is_self_skill() { id } else { target };
    if !target_allowed(ctx, id, target, skill) {
        return;
    }
    if target != id {
        let range = if skill.cast_range > 0 {
            skill.cast_range as f32
        } else {
            ctx.db.stats().entity_id().find(id).map_or(100.0, |s| s.attack_range)
        };
        match (position(ctx, id, t), position(ctx, target, t)) {
            (Some(a), Some(b)) if distance(a, b) <= range + RANGE_SLACK_CM => {}
            _ => return,
        }
    }
    let Some(npc_id) = rose_data::NpcId::new(e.npc_id) else { return };
    let motion = |m: i32| {
        u16::try_from(m)
            .ok()
            .and_then(|m| game.npcs.get_npc_motion(npc_id, rose_data::MotionId::new(m)))
            .map_or(0, |m| m.duration.as_micros() as i64)
    };
    let (casting, action) = (motion(motion_id), motion(motion_id + 1));
    stop_motion(ctx, id);
    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
        if c.swing_hit_at_us.take().is_some() {
            c.next_attack_at_us = t;
            ctx.db.combat().entity_id().update(c);
        }
    }
    ctx.db.skill_cast().insert(SkillCast {
        entity_id: id,
        skill_id,
        target: (target != id).then_some(target),
        target_x: 0.0,
        target_y: 0.0,
        at_position: false,
        started_at_us: Some(t),
        effect_at_us: t + casting,
        ends_at_us: t + (casting + action).max(MIN_CAST_US),
        applied: false,
        use_item_slot: -1,
        use_item: String::new(),
    });
    ctx.db.npc_cast_motion().insert(NpcCastMotion { entity_id: id, cast_motion: motion_id, action_motion: motion_id + 1 });
}

/// The share of a hit a ShieldDamage buff sends back to the attacker, or 0.
pub fn shield_reflect(ctx: &ReducerContext, defender: u64, amount: i32) -> i32 {
    status_value(ctx, defender, StatusEffectType::ShieldDamage).map_or(0, |power| amount * power / 100)
}

/// Debug: teach a player (by name) a skill, skipping its requirements.
#[spacetimedb::reducer]
pub fn give_skill(ctx: &ReducerContext, name: String, skill_id: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = game(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    let data = skill(&game, skill_id)?;
    let mut skill_list = p.skill_list();
    skill_list.add_skill(data).ok_or("no room for more skills")?;
    p.set_skill_list(&skill_list);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Debug: set a player's job (0 visitor, 111 soldier, 211 muse, 311 hawker, 411 dealer, ...).
#[spacetimedb::reducer]
pub fn set_job(ctx: &ReducerContext, name: String, job: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = game(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    p.job = job;
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Put an inventory item (kind 1: page, index) or a skill (kind 2: page, index) on a hotbar
/// slot (0-31, eight per page), or clear it (kind 0).
#[spacetimedb::reducer]
pub fn set_hotbar_slot(ctx: &ReducerContext, index: u8, kind: u8, page: u8, slot: u16) -> Result<(), String> {
    use rose_game_common::components::{Hotbar, HotbarSlot};
    let (mut p, _) = my_player(ctx)?;
    let value = match kind {
        0 => None,
        1 => Some(HotbarSlot::Inventory(items::inventory_slot(page, slot)?)),
        2 => {
            let skill_slot = SkillSlot(page as usize, slot as usize);
            p.skill_list().get_skill(skill_slot).ok_or("no skill there")?;
            Some(HotbarSlot::Skill(skill_slot))
        }
        _ => return Err("unknown hotbar slot kind".into()),
    };
    let mut hotbar: Hotbar = serde_json::from_str(&p.hotbar).unwrap_or_default();
    hotbar.set_slot(index as usize, value).ok_or("no such hotbar slot")?;
    p.hotbar = serde_json::to_string(&hotbar).unwrap_or_default();
    ctx.db.player().identity().update(p);
    Ok(())
}
