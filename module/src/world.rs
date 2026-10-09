//! Zones, spawn points and monsters, built from the zone and NPC databases.
//! Spawning follows rose-offline's monster_spawn_system (spawn point "tactics").

use rand::Rng;
use rose_data::{NpcId, NpcMotionAction, WarpGateId};
use rose_game_common::components::StatusEffects;
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

use crate::{
    cancel_attack, combat, entity, game_data::game, monster_ai, motion, my_player, now_us, stats, Combat, Entity,
    EntityKind, MonsterAi, Motion, Player, Stats,
};

#[spacetimedb::table(accessor = zone_info, public)]
#[derive(Clone)]
pub struct ZoneInfo {
    #[primary_key]
    pub zone_id: u16,
    pub name: String,
    pub start_x: f32,
    pub start_y: f32,
    pub revive_x: f32,
    pub revive_y: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(SpacetimeType, Clone, Debug)]
pub struct SpawnEntry {
    pub npc_id: u16,
    pub count: u32,
}

#[spacetimedb::table(accessor = monster_spawn)]
#[derive(Clone)]
pub struct MonsterSpawn {
    #[primary_key]
    pub spawn_id: u32,
    #[index(btree)]
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
    /// Spawn radius in cm.
    pub range: f32,
    pub interval_secs: u32,
    pub limit_count: u32,
    pub tactic_points: u32,
    pub basic: Vec<SpawnEntry>,
    pub tactic: Vec<SpawnEntry>,
    pub current_tactics_value: u32,
    pub next_check_at_us: i64,
}

/// Replace zones and spawn points with what the zone database says.
pub fn setup_zones(ctx: &ReducerContext, game: &GameData) {
    let monsters: Vec<u64> =
        ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Monster).map(|e| e.entity_id).collect();
    for id in monsters {
        crate::despawn(ctx, id);
    }
    let zones: Vec<u16> = ctx.db.zone_info().iter().map(|z| z.zone_id).collect();
    for id in zones {
        ctx.db.zone_info().zone_id().delete(id);
    }
    let spawns: Vec<u32> = ctx.db.monster_spawn().iter().map(|s| s.spawn_id).collect();
    for id in spawns {
        ctx.db.monster_spawn().spawn_id().delete(id);
    }

    for zone in game.zones.iter() {
        let zone_id = zone.id.get();
        let size_x = zone.num_sectors_x as f32 * zone.sector_size as f32;
        let size_y = zone.num_sectors_y as f32 * zone.sector_size as f32;
        let revive = zone.get_closest_revive_position(zone.start_position).unwrap_or(zone.start_position);
        ctx.db.zone_info().insert(ZoneInfo {
            zone_id,
            name: zone.name.to_string(),
            start_x: zone.start_position.x,
            start_y: zone.start_position.y,
            revive_x: revive.x,
            revive_y: revive.y,
            min_x: zone.sectors_base_position.x,
            min_y: zone.sectors_base_position.y,
            max_x: zone.sectors_base_position.x + size_x,
            max_y: zone.sectors_base_position.y + size_y,
        });
        let entries = |list: &Vec<(NpcId, usize)>| -> Vec<SpawnEntry> {
            list.iter().map(|(id, count)| SpawnEntry { npc_id: id.get(), count: *count as u32 }).collect()
        };
        for (i, point) in zone.monster_spawns.iter().enumerate() {
            ctx.db.monster_spawn().insert(MonsterSpawn {
                spawn_id: zone_id as u32 * 10_000 + i as u32,
                zone_id,
                x: point.position.x,
                y: point.position.y,
                range: (point.range * 100) as f32,
                interval_secs: point.interval.max(1),
                limit_count: point.limit_count.max(1),
                tactic_points: point.tactic_points.max(1),
                basic: entries(&point.basic_spawns),
                tactic: entries(&point.tactic_spawns),
                current_tactics_value: 0,
                next_check_at_us: 0,
            });
        }
    }
    crate::npcs::spawn_npcs(ctx, game);
}

/// One spawn check for a spawn point: which monsters to add, as rose-offline decides it.
fn spawn_queue(spawn: &mut MonsterSpawn, live_count: u32) -> Vec<(u16, u32)> {
    if live_count >= spawn.limit_count {
        spawn.current_tactics_value = spawn.current_tactics_value.saturating_sub(1);
        return Vec::new();
    }
    let regen_value = ((spawn.limit_count * 2 - live_count) * spawn.current_tactics_value * 50)
        / (spawn.limit_count * spawn.tactic_points);
    let basic = |i: usize, adjust: i32| {
        spawn.basic.get(i).map(|e| (e.npc_id, (e.count as i32 + adjust).max(0) as u32))
    };
    let tactic = |i: usize, adjust: i32| {
        spawn.tactic.get(i).map(|e| (e.npc_id, (e.count as i32 + adjust).max(0) as u32))
    };
    let (queue, tactics): (Vec<Option<(u16, u32)>>, Result<u32, u32>) = match regen_value {
        0..=10 => (vec![basic(0, 0)], Ok(12)),
        11..=15 => (vec![basic(0, -2), basic(1, 0)], Ok(15)),
        16..=25 => (vec![basic(2, 0)], Ok(12)),
        26..=30 => (vec![basic(0, -1), basic(2, 0)], Ok(15)),
        31..=40 => (vec![basic(3, 0)], Ok(12)),
        41..=50 => (vec![basic(1, 0), basic(2, -1)], Ok(12)),
        51..=65 => (vec![basic(2, 0), basic(3, -2)], Ok(20)),
        66..=73 => (vec![basic(3, 0), basic(4, 0)], Ok(15)),
        74..=85 => (vec![basic(0, 0), basic(4, -2), tactic(0, -1)], Ok(15)),
        // Ok adds to the tactics value, Err sets it.
        86..=92 => (vec![basic(1, 0), tactic(0, 0), tactic(1, 0)], Err(1)),
        _ => (vec![basic(4, 0), tactic(0, 1), tactic(1, 0)], Err(7)),
    };
    spawn.current_tactics_value = match tactics {
        Ok(add) => spawn.current_tactics_value + add,
        Err(set) => set,
    }
    .min(500);
    queue.into_iter().flatten().collect()
}

/// Spawn checks for every spawn point in zones that have a player online.
pub fn spawn_tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let mut active_zones: Vec<u16> =
        ctx.db.player().iter().filter(|p| p.online).map(|p| p.zone_id).collect();
    active_zones.sort_unstable();
    active_zones.dedup();
    for zone_id in active_zones {
        for mut spawn in ctx.db.monster_spawn().zone_id().filter(zone_id) {
            if spawn.next_check_at_us > t {
                continue;
            }
            let interval_us = spawn.interval_secs as i64 * 1_000_000;
            spawn.next_check_at_us = if spawn.next_check_at_us == 0 { t } else { spawn.next_check_at_us } + interval_us;
            if spawn.next_check_at_us < t {
                spawn.next_check_at_us = t + interval_us;
            }
            let live = ctx.db.monster_ai().spawn_id().filter(spawn.spawn_id).count() as u32;
            for (npc_id, count) in spawn_queue(&mut spawn, live) {
                for _ in 0..count {
                    spawn_monster(ctx, game, &spawn, npc_id);
                }
            }
            ctx.db.monster_spawn().spawn_id().update(spawn);
        }
    }
}

/// Ability values, attack timing and hit count of an NPC type, as a Stats row.
pub fn npc_stats(game: &GameData, entity_id: u64, npc_id: u16, effects: &StatusEffects) -> Option<Stats> {
    npc_stats_for(game, entity_id, npc_id, effects, None)
}

/// npc_stats for a summon too: its stats grow with its owner's level and the summoning
/// skill's level (owner_level, skill_level).
pub fn npc_stats_for(
    game: &GameData,
    entity_id: u64,
    npc_id: u16,
    effects: &StatusEffects,
    summon: Option<(i32, i32)>,
) -> Option<Stats> {
    let id = NpcId::new(npc_id)?;
    let av = game.ability_value_calculator.calculate_npc(id, effects, summon.map(|s| s.0), summon.map(|s| s.1))?;
    let attack = game.npcs.get_npc_action_motion(id, NpcMotionAction::Attack);
    let attack_motion_ms = attack.map_or(1000, |m| m.duration.as_millis() as i32).max(300);
    let attack_hit_ms = attack
        .and_then(|m| m.first_attack_time)
        .map_or(attack_motion_ms / 2, |d| d.as_millis() as i32)
        .min(attack_motion_ms);
    let hit_count = attack.map_or(1, |m| m.total_attack_frames.max(1) as i32);
    Some(Stats::from_ability_values(entity_id, &av, false, attack_motion_ms, attack_hit_ms, hit_count))
}

pub fn spawn_monster(ctx: &ReducerContext, game: &GameData, spawn: &MonsterSpawn, npc_id: u16) {
    let Some(npc) = NpcId::new(npc_id).and_then(|id| game.npcs.get_npc(id)) else { return };
    let mut rng = ctx.rng();
    let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let r: f32 = rng.gen_range(0.0..spawn.range.max(1.0));
    let (x, y) = (spawn.x + angle.cos() * r, spawn.y + angle.sin() * r);
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0,
        kind: EntityKind::Monster,
        npc_id,
        zone_id: spawn.zone_id,
        name: npc.name.to_string(),
    });
    let id = entity.entity_id;
    let Some(stats) = npc_stats(game, id, npc_id, &StatusEffects::default()) else {
        ctx.db.entity().entity_id().delete(id);
        return;
    };
    let t = now_us(ctx);
    ctx.db.motion().insert(Motion {
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
    ctx.db.monster_ai().insert(MonsterAi {
        entity_id: id,
        spawn_id: spawn.spawn_id,
        home_x: x,
        home_y: y,
        aggro_range: 0.0,
        returning: false,
    });
}

use crate::player;

/// Move a player's character to another zone (or another spot in the same one).
pub fn teleport(ctx: &ReducerContext, p: &mut Player, zone_id: u16, at: (f32, f32)) {
    let t = now_us(ctx);
    if let Some(id) = p.entity_id {
        crate::shop::close(ctx, id);
        if let Ok(game) = game(ctx) {
            crate::vehicle::get_off(ctx, &game, id);
        }
        cancel_attack(ctx, id, t);
        crate::skills::cancel_cast(ctx, id);
        crate::forget_entity(ctx, id);
        crate::clear_damage_sources(ctx, id);
        if let Some(mut e) = ctx.db.entity().entity_id().find(id) {
            e.zone_id = zone_id;
            ctx.db.entity().entity_id().update(e);
        }
        if let Some(mut m) = ctx.db.motion().entity_id().find(id) {
            m.from_x = at.0;
            m.from_y = at.1;
            m.to_x = at.0;
            m.to_y = at.1;
            m.started_at_us = t;
            m.chase_target = None;
            ctx.db.motion().entity_id().update(m);
        }
    }
    p.zone_id = zone_id;
    p.last_x = at.0;
    p.last_y = at.1;
    ctx.db.player().identity().update(p.clone());
}

/// The character walked into a warp gate (the client sees the gate's collision): go where
/// WARP.STB sends it, to the target zone's event object of that name (as rose-offline does;
/// the server has no zone geometry to check the gate's position).
#[spacetimedb::reducer]
pub fn use_warp_gate(ctx: &ReducerContext, warp_id: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let gate = game.warp_gates.get_warp_gate(WarpGateId::new(warp_id)).ok_or("no such warp gate")?;
    let zone = game.zones.get_zone(gate.target_zone).ok_or("the warp gate leads nowhere")?;
    let at = zone.event_positions.get(&gate.target_event_object).ok_or("the warp gate leads nowhere")?;
    teleport(ctx, &mut p, gate.target_zone.get(), (at.x, at.y));
    Ok(())
}

/// Debug: send a player (by name, online) to a zone's start position.
#[spacetimedb::reducer]
pub fn warp_player(ctx: &ReducerContext, name: String, zone_id: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    let zone = ctx.db.zone_info().zone_id().find(zone_id).ok_or("no such zone")?;
    teleport(ctx, &mut p, zone_id, (zone.start_x, zone.start_y));
    Ok(())
}

/// Debug: put a player (by name) two metres from an NPC (by NPC id), for testing quests.
#[spacetimedb::reducer]
pub fn admin_teleport_to_npc(ctx: &ReducerContext, name: String, npc_id: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    use crate::npcs::npc;
    let npc = ctx.db.npc().iter().find(|n| n.npc_id == npc_id).ok_or("no such NPC")?;
    let at = crate::position(ctx, npc.entity_id, now_us(ctx)).ok_or("the NPC isn't placed")?;
    teleport(ctx, &mut p, npc.zone_id, (at.0 + 200.0, at.1));
    Ok(())
}
