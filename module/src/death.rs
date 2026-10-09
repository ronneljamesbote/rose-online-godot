//! Dying and getting back up, as iROSE does it (rose-next's classUSER::Dead,
//! Recv_cli_REVIVE_REQ and Set_PenalEXP).
//!
//! A character killed by a monster at level 10 or above loses 3% of the experience its level
//! needs. When it has less than that, the rest becomes a debt that is added to what the next
//! level needs (at most half of it); levelling up clears the debt. Deaths to other players
//! cost nothing. The fallen character chooses when to get up: at its save point (when that
//! is on the same planet) or at the revive point of the zone nearest to where it fell, with
//! 30% of its HP. In a PvP zone it can't be hurt for 30 seconds after getting up.
//! Resurrection gives back the skill's power in percent of the experience lost.

use glam::Vec3;
use rose_game_data::GameData;
use spacetimedb::{Identity, ReducerContext, Table};

use crate::{
    combat, entity, game_data::game, items, motion, my_player, now_us, player, position, skills, stats, world,
    zone_info, EntityKind, Motion, Player,
};

/// Below this level dying costs no experience (Set_PenalEXP).
const PENALTY_MIN_LEVEL: u32 = 10;
/// PENALTY_EXP_TOWN: the percent of the level's experience a death costs.
const PENALTY_PERCENT: u64 = 3;
/// A fallen character that hasn't chosen gets up at the zone's revive point after this long.
/// iROSE waits forever; this keeps an idle fallen character from lying there all day.
pub const AUTO_REVIVE_US: i64 = 10 * 60 * 1_000_000;
/// Getting up lands within this many cm of the revive point (RANDOM(1001) - 500).
const REVIVE_SPREAD: f32 = 500.0;
/// HP after getting up, in percent of the maximum.
const REVIVE_HP_PERCENT: i32 = 30;
/// Time a character can't be hurt after getting up in a PvP zone (status row 57, 30 s).
const PVP_REVIVE_SHIELD_US: i64 = 30 * 1_000_000;

/// A fallen character: where it fell and the experience its death cost (for Resurrection).
#[spacetimedb::table(accessor = fallen, public)]
#[derive(Clone)]
pub struct Fallen {
    #[primary_key]
    pub entity_id: u64,
    pub identity: Identity,
    pub died_at_us: i64,
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
    /// Experience taken from the bar plus debt added.
    pub penalty_xp: u64,
    /// When it gets up by itself.
    pub auto_revive_at_us: i64,
}

/// Where a character gets up when it chooses its save point (set by talking to the NPCs
/// that offer it, or by a quest).
#[spacetimedb::table(accessor = save_point, public)]
#[derive(Clone)]
pub struct SavePoint {
    #[primary_key]
    pub identity: Identity,
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
}

/// Experience owed from deaths: added to what the next level needs, cleared on level up.
#[spacetimedb::table(accessor = xp_debt, public)]
#[derive(Clone)]
pub struct XpDebt {
    #[primary_key]
    pub identity: Identity,
    pub xp: u64,
}

/// Can't be hurt until then (after getting up in a PvP zone).
#[spacetimedb::table(accessor = revive_shield, public)]
#[derive(Clone)]
pub struct ReviveShield {
    #[primary_key]
    pub entity_id: u64,
    pub until_us: i64,
}

pub fn debt(ctx: &ReducerContext, identity: Identity) -> u64 {
    ctx.db.xp_debt().identity().find(identity).map_or(0, |d| d.xp)
}

pub fn clear_debt(ctx: &ReducerContext, identity: Identity) {
    ctx.db.xp_debt().identity().delete(identity);
}

pub fn is_shielded(ctx: &ReducerContext, entity_id: u64, t: i64) -> bool {
    ctx.db.revive_shield().entity_id().find(entity_id).is_some_and(|s| s.until_us > t)
}

/// A player character just fell. `killer` is who gets the credit (a summon's owner).
pub fn player_died(ctx: &ReducerContext, game: &GameData, id: u64, killer: u64, t: i64) {
    let Some(mut p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) else { return };
    let at = position(ctx, id, t).unwrap_or((p.last_x, p.last_y));
    let by_monster = ctx.db.entity().entity_id().find(killer).is_some_and(|e| e.kind == EntityKind::Monster);
    let mut penalty = 0;
    if by_monster && p.level >= PENALTY_MIN_LEVEL {
        let need = game.ability_value_calculator.calculate_levelup_require_xp(p.level);
        let lose = need * PENALTY_PERCENT / 100;
        let from_bar = lose.min(p.xp);
        p.xp -= from_bar;
        let owed = debt(ctx, p.identity);
        let new_debt = (owed + lose - from_bar).min(need / 2);
        penalty = from_bar + new_debt.saturating_sub(owed);
        if new_debt > owed {
            ctx.db.xp_debt().identity().delete(p.identity);
            ctx.db.xp_debt().insert(XpDebt { identity: p.identity, xp: new_debt });
        }
        ctx.db.player().identity().update(p.clone());
        if penalty > 0 {
            items::notify(ctx, p.identity, format!("You lost {penalty} experience"));
        }
    }
    ctx.db.fallen().entity_id().delete(id);
    ctx.db.fallen().insert(Fallen {
        entity_id: id,
        identity: p.identity,
        died_at_us: t,
        zone_id: p.zone_id,
        x: at.0,
        y: at.1,
        penalty_xp: penalty,
        auto_revive_at_us: t + AUTO_REVIVE_US,
    });
}

/// Resurrection: give back `percent` of the experience the death cost (Cancel_PenalEXP).
pub fn refund(ctx: &ReducerContext, id: u64, percent: u64) {
    let Some(f) = ctx.db.fallen().entity_id().find(id) else { return };
    ctx.db.fallen().entity_id().delete(id);
    let back = f.penalty_xp * percent.min(100) / 100;
    if back == 0 {
        return;
    }
    let Some(mut p) = ctx.db.player().identity().find(f.identity) else { return };
    p.xp += back;
    ctx.db.player().identity().update(p.clone());
    items::notify(ctx, p.identity, format!("You got {back} experience back"));
}

/// The zone's revive point nearest to `at`.
fn revive_point(ctx: &ReducerContext, game: &GameData, zone_id: u16, at: (f32, f32)) -> (f32, f32) {
    rose_data::ZoneId::new(zone_id)
        .and_then(|z| game.zones.get_zone(z))
        .and_then(|z| z.get_closest_revive_position(Vec3::new(at.0, at.1, 0.0)).or(Some(z.start_position)))
        .map(|v| (v.x, v.y))
        .or_else(|| ctx.db.zone_info().zone_id().find(zone_id).map(|z| (z.revive_x, z.revive_y)))
        .unwrap_or(at)
}

fn planet(game: &GameData, zone_id: u16) -> u32 {
    rose_data::ZoneId::new(zone_id).and_then(|z| game.zones.get_zone(z)).map_or(0, |z| z.planet)
}

/// Get a fallen character up at its save point or the zone's revive point.
pub fn revive(ctx: &ReducerContext, game: &GameData, p: &mut Player, id: u64, at_save_point: bool, t: i64) {
    let fallen = ctx.db.fallen().entity_id().find(id);
    let fell_at = fallen.as_ref().map_or((p.last_x, p.last_y), |f| (f.x, f.y));
    let save = ctx
        .db
        .save_point()
        .identity()
        .find(p.identity)
        .filter(|s| at_save_point && planet(game, s.zone_id) == planet(game, p.zone_id));
    let (zone_id, mut to) = match save {
        Some(s) => (s.zone_id, (s.x, s.y)),
        None => (p.zone_id, revive_point(ctx, game, p.zone_id, fell_at)),
    };
    let mut rng = ctx.rng();
    use rand::Rng;
    to.0 += rng.gen_range(-REVIVE_SPREAD..=REVIVE_SPREAD);
    to.1 += rng.gen_range(-REVIVE_SPREAD..=REVIVE_SPREAD);
    ctx.db.fallen().entity_id().delete(id);

    // Whatever was on the character when it fell is gone.
    skills::clear_all_status(ctx, game, id);
    if zone_id != p.zone_id {
        world::teleport(ctx, p, zone_id, to);
    } else {
        let speed = ctx.db.stats().entity_id().find(id).map_or(425.0, |s| s.move_speed);
        ctx.db.motion().entity_id().update(Motion {
            entity_id: id,
            from_x: to.0,
            from_y: to.1,
            to_x: to.0,
            to_y: to.1,
            started_at_us: t,
            speed,
            chase_target: None,
        });
    }
    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
        c.dead_until_us = None;
        c.hp = (c.max_hp * REVIVE_HP_PERCENT / 100).max(1);
        c.attack_target = None;
        c.swing_hit_at_us = None;
        ctx.db.combat().entity_id().update(c);
    }
    if crate::pvp::zone_pvp(game, zone_id) != 0 {
        ctx.db.revive_shield().entity_id().delete(id);
        ctx.db.revive_shield().insert(ReviveShield { entity_id: id, until_us: t + PVP_REVIVE_SHIELD_US });
    }
}

/// Fallen characters that waited too long get up by themselves; old shields go.
pub fn tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let due: Vec<Fallen> = ctx.db.fallen().iter().filter(|f| f.auto_revive_at_us <= t).collect();
    for f in due {
        let Some(mut p) = ctx.db.player().identity().find(f.identity).filter(|p| p.entity_id == Some(f.entity_id)) else {
            ctx.db.fallen().entity_id().delete(f.entity_id);
            continue;
        };
        revive(ctx, game, &mut p, f.entity_id, false, t);
    }
    let old: Vec<u64> = ctx.db.revive_shield().iter().filter(|s| s.until_us <= t).map(|s| s.entity_id).collect();
    for id in old {
        ctx.db.revive_shield().entity_id().delete(id);
    }
}

/// Get up: at the save point (`at_save_point`) or at this zone's revive point.
#[spacetimedb::reducer]
pub fn revive_player(ctx: &ReducerContext, at_save_point: bool) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if ctx.db.combat().entity_id().find(id).is_none_or(|c| c.dead_until_us.is_none()) {
        return Err("you are not dead".into());
    }
    revive(ctx, &game, &mut p, id, at_save_point, now_us(ctx));
    Ok(())
}

/// Save this zone as where to get up (GF_setRevivePosition: the revive point nearest to us).
#[spacetimedb::reducer]
pub fn set_save_point(ctx: &ReducerContext) -> Result<(), String> {
    let game = game(ctx)?;
    let (p, id) = my_player(ctx)?;
    let at = position(ctx, id, now_us(ctx)).unwrap_or((p.last_x, p.last_y));
    let to = revive_point(ctx, &game, p.zone_id, at);
    save(ctx, p.identity, p.zone_id, to);
    let zone = ctx.db.zone_info().zone_id().find(p.zone_id).map_or_else(String::new, |z| z.name);
    items::notify(ctx, p.identity, format!("Save point set: {zone}"));
    Ok(())
}

pub fn save(ctx: &ReducerContext, identity: Identity, zone_id: u16, at: (f32, f32)) {
    ctx.db.save_point().identity().delete(identity);
    ctx.db.save_point().insert(SavePoint { identity, zone_id, x: at.0, y: at.1 });
}

/// A character moved to another account (assign_character): its save point and debt follow.
pub fn rekey(ctx: &ReducerContext, old: Identity, new: Identity) {
    if let Some(mut s) = ctx.db.save_point().identity().find(old) {
        ctx.db.save_point().identity().delete(old);
        s.identity = new;
        ctx.db.save_point().insert(s);
    }
    if let Some(mut d) = ctx.db.xp_debt().identity().find(old) {
        ctx.db.xp_debt().identity().delete(old);
        d.identity = new;
        ctx.db.xp_debt().insert(d);
    }
}
