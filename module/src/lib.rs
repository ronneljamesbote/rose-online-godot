//! ROSE on SpacetimeDB: movement and combat prototype.
//!
//! Units follow rose-offline: positions in cm, speeds in cm/s, times stored as
//! microseconds since the Unix epoch so clients can evaluate motion paths too.

mod damage;

use rand::Rng;
use spacetimedb::{Identity, ReducerContext, ScheduleAt, SpacetimeType, Table};
use std::time::Duration;

const COMBAT_TICK_MS: u64 = 100;
const SPAWN_TICK_MS: u64 = 1000;
const PLAYER_RESPAWN_US: i64 = 5_000_000;
const MONSTER_LEASH_CM: f32 = 3000.0;
/// Re-path a chase only when the target has moved this far from the current destination.
const CHASE_REPATH_CM: f32 = 100.0;
/// Extra reach so an entity that stopped exactly at range still swings.
const RANGE_SLACK_CM: f32 = 50.0;
const DEFAULT_ZONE: u16 = 1;

#[derive(SpacetimeType, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Player,
    Monster,
}

#[spacetimedb::table(accessor = admin)]
pub struct Admin {
    #[primary_key]
    identity: Identity,
}

#[spacetimedb::table(accessor = player, public)]
#[derive(Clone)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub entity_id: Option<u64>,
    pub name: String,
    pub online: bool,
    pub zone_id: u16,
    pub last_x: f32,
    pub last_y: f32,
    pub last_hp: i32,
    /// Test loadout: false = melee, true = ranged (bow).
    pub ranged: bool,
}

#[spacetimedb::table(accessor = entity, public)]
#[derive(Clone)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub entity_id: u64,
    pub kind: EntityKind,
    /// NPC id for monsters, 0 for players.
    pub npc_id: u16,
    pub zone_id: u16,
    pub name: String,
}

/// A straight-line move. Position at time t is from + (to - from) * min(1, (t - started) * speed / distance).
#[spacetimedb::table(accessor = motion, public)]
#[derive(Clone)]
pub struct Motion {
    #[primary_key]
    pub entity_id: u64,
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub started_at_us: i64,
    pub speed: f32,
    /// Set while chasing an entity; a plain click-to-move leaves it empty.
    pub chase_target: Option<u64>,
}

#[spacetimedb::table(accessor = combat, public)]
#[derive(Clone)]
pub struct Combat {
    #[primary_key]
    pub entity_id: u64,
    pub hp: i32,
    pub max_hp: i32,
    pub attack_target: Option<u64>,
    pub next_attack_at_us: i64,
    pub dead_until_us: Option<i64>,
}

#[spacetimedb::table(accessor = stats)]
#[derive(Clone)]
pub struct Stats {
    #[primary_key]
    pub entity_id: u64,
    pub level: i32,
    pub attack_power: i32,
    pub hit: i32,
    pub defence: i32,
    pub avoid: i32,
    pub critical: i32,
    pub attack_speed: i32,
    /// Attack animation length at attack speed 100.
    pub attack_motion_ms: i32,
    pub attack_range: f32,
    pub move_speed: f32,
    /// Chase speed for monsters (their run speed); same as move_speed for players.
    pub run_speed: f32,
    pub is_player: bool,
}

#[spacetimedb::table(accessor = monster_ai)]
#[derive(Clone)]
pub struct MonsterAi {
    #[primary_key]
    pub entity_id: u64,
    #[index(btree)]
    pub spawn_id: u32,
    pub home_x: f32,
    pub home_y: f32,
    pub aggro_range: f32,
    pub returning: bool,
}

#[spacetimedb::table(accessor = zone_info)]
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

#[spacetimedb::table(accessor = monster_spawn)]
#[derive(Clone)]
pub struct MonsterSpawn {
    #[primary_key]
    pub spawn_id: u32,
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub npc_id: u16,
    pub max_alive: u32,
    pub respawn_secs: u32,
    pub next_spawn_at_us: i64,
}

#[spacetimedb::table(accessor = npc_data)]
#[derive(Clone)]
pub struct NpcData {
    #[primary_key]
    pub npc_id: u16,
    pub name: String,
    pub level: i32,
    pub max_hp: i32,
    pub attack_power: i32,
    pub hit: i32,
    pub defence: i32,
    pub avoid: i32,
    pub attack_speed: i32,
    pub attack_motion_ms: i32,
    pub attack_range: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    /// 0 = only fights back when hit.
    pub aggro_range: f32,
}

#[spacetimedb::table(accessor = damage_event, public, event)]
pub struct DamageEvent {
    pub attacker: u64,
    pub defender: u64,
    pub amount: i32,
    pub is_critical: bool,
    pub killed: bool,
    /// True when the attacker was walking at the moment of the swing (attack while moving).
    pub attacker_moving: bool,
    pub at_us: i64,
}

/// Per-tick timing, so the go/no-go "combat tick duration" can be read with SQL.
#[spacetimedb::table(accessor = tick_stats, public)]
pub struct TickStats {
    #[primary_key]
    pub id: u8,
    pub ticks: u64,
    pub last_tick_entities: u32,
    pub max_gap_us: i64,
    pub last_at_us: i64,
}

#[spacetimedb::table(accessor = combat_tick_timer, scheduled(combat_tick))]
pub struct CombatTickTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

#[spacetimedb::table(accessor = spawn_tick_timer, scheduled(spawn_tick))]
pub struct SpawnTickTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

// ---------------------------------------------------------------- helpers

fn now_us(ctx: &ReducerContext) -> i64 {
    ctx.timestamp.to_micros_since_unix_epoch()
}

impl Motion {
    fn position_at(&self, t_us: i64) -> (f32, f32) {
        let dx = self.to_x - self.from_x;
        let dy = self.to_y - self.from_y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 0.01 || self.speed <= 0.0 {
            return (self.to_x, self.to_y);
        }
        let elapsed = (t_us - self.started_at_us).max(0) as f32 / 1_000_000.0;
        let f = (elapsed * self.speed / dist).min(1.0);
        (self.from_x + dx * f, self.from_y + dy * f)
    }

    fn is_moving(&self, t_us: i64) -> bool {
        let (x, y) = self.position_at(t_us);
        (x - self.to_x).abs() > 0.01 || (y - self.to_y).abs() > 0.01
    }
}

fn position(ctx: &ReducerContext, entity_id: u64, t: i64) -> Option<(f32, f32)> {
    ctx.db.motion().entity_id().find(entity_id).map(|m| m.position_at(t))
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn set_motion(ctx: &ReducerContext, entity_id: u64, to: (f32, f32), speed: f32, chase: Option<u64>) {
    let t = now_us(ctx);
    let from = position(ctx, entity_id, t).unwrap_or(to);
    let row = Motion {
        entity_id,
        from_x: from.0,
        from_y: from.1,
        to_x: to.0,
        to_y: to.1,
        started_at_us: t,
        speed,
        chase_target: chase,
    };
    if ctx.db.motion().entity_id().find(entity_id).is_some() {
        ctx.db.motion().entity_id().update(row);
    } else {
        ctx.db.motion().insert(row);
    }
}

/// Stop where the entity is now. Only writes the row if it was actually moving.
fn stop_motion(ctx: &ReducerContext, entity_id: u64) {
    let t = now_us(ctx);
    if let Some(m) = ctx.db.motion().entity_id().find(entity_id) {
        if m.is_moving(t) || m.chase_target.is_some() {
            let p = m.position_at(t);
            set_motion(ctx, entity_id, p, 0.0, None);
        }
    }
}

fn require_admin(ctx: &ReducerContext) -> Result<(), String> {
    if ctx.db.admin().identity().find(ctx.sender()).is_some() {
        Ok(())
    } else {
        Err("admin only".into())
    }
}

fn attack_interval_us(stats: &Stats) -> i64 {
    // rose-offline: required_duration = attack motion / (max(attack_speed, 30) / 100)
    let speed = stats.attack_speed.max(30) as i64;
    stats.attack_motion_ms as i64 * 1000 * 100 / speed
}

/// Fixed test character until character creation exists: roughly a level 10 soldier.
fn player_stats(entity_id: u64, ranged: bool) -> Stats {
    Stats {
        entity_id,
        level: 10,
        attack_power: if ranged { 52 } else { 60 },
        hit: 70,
        defence: 35,
        avoid: 30,
        critical: 40,
        attack_speed: if ranged { 115 } else { 100 },
        attack_motion_ms: 1000,
        // Short Bow (item 202) and Short Sword (item 2) ranges, so the client agrees on reach.
        attack_range: if ranged { 2100.0 } else { 150.0 },
        move_speed: 425.0,
        run_speed: 425.0,
        is_player: true,
    }
}

fn despawn(ctx: &ReducerContext, entity_id: u64) {
    ctx.db.entity().entity_id().delete(entity_id);
    ctx.db.motion().entity_id().delete(entity_id);
    ctx.db.combat().entity_id().delete(entity_id);
    ctx.db.stats().entity_id().delete(entity_id);
    ctx.db.monster_ai().entity_id().delete(entity_id);
    // Anyone targeting it loses the target.
    for mut c in ctx.db.combat().iter().filter(|c| c.attack_target == Some(entity_id)) {
        c.attack_target = None;
        ctx.db.combat().entity_id().update(c);
    }
    for m in ctx.db.motion().iter().filter(|m| m.chase_target == Some(entity_id)) {
        stop_motion(ctx, m.entity_id);
    }
}

fn spawn_player_entity(ctx: &ReducerContext, player: &mut Player) {
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0,
        kind: EntityKind::Player,
        npc_id: 0,
        zone_id: player.zone_id,
        name: player.name.clone(),
    });
    let id = entity.entity_id;
    let stats = ctx.db.stats().insert(player_stats(id, player.ranged));
    let max_hp = 300;
    let hp = if player.last_hp > 0 { player.last_hp.min(max_hp) } else { max_hp };
    let t = now_us(ctx);
    ctx.db.motion().insert(Motion {
        entity_id: id,
        from_x: player.last_x,
        from_y: player.last_y,
        to_x: player.last_x,
        to_y: player.last_y,
        started_at_us: t,
        speed: stats.move_speed,
        chase_target: None,
    });
    ctx.db.combat().insert(Combat {
        entity_id: id,
        hp,
        max_hp,
        attack_target: None,
        next_attack_at_us: t,
        dead_until_us: None,
    });
    player.entity_id = Some(id);
}

fn spawn_monster(ctx: &ReducerContext, spawn: &MonsterSpawn, npc: &NpcData) {
    let mut rng = ctx.rng();
    let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let r: f32 = rng.gen_range(0.0..spawn.radius.max(1.0));
    let (x, y) = (spawn.x + angle.cos() * r, spawn.y + angle.sin() * r);
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0,
        kind: EntityKind::Monster,
        npc_id: npc.npc_id,
        zone_id: spawn.zone_id,
        name: npc.name.clone(),
    });
    let id = entity.entity_id;
    let t = now_us(ctx);
    ctx.db.stats().insert(Stats {
        entity_id: id,
        level: npc.level,
        attack_power: npc.attack_power,
        hit: npc.hit,
        defence: npc.defence,
        avoid: npc.avoid,
        critical: (npc.level as f32 * 2.5) as i32,
        attack_speed: npc.attack_speed,
        attack_motion_ms: npc.attack_motion_ms,
        attack_range: npc.attack_range,
        move_speed: npc.walk_speed,
        run_speed: npc.run_speed,
        is_player: false,
    });
    ctx.db.motion().insert(Motion {
        entity_id: id,
        from_x: x,
        from_y: y,
        to_x: x,
        to_y: y,
        started_at_us: t,
        speed: npc.walk_speed,
        chase_target: None,
    });
    ctx.db.combat().insert(Combat {
        entity_id: id,
        hp: npc.max_hp,
        max_hp: npc.max_hp,
        attack_target: None,
        next_attack_at_us: t,
        dead_until_us: None,
    });
    ctx.db.monster_ai().insert(MonsterAi {
        entity_id: id,
        spawn_id: spawn.spawn_id,
        home_x: x,
        home_y: y,
        aggro_range: npc.aggro_range,
        returning: false,
    });
}

fn my_player(ctx: &ReducerContext) -> Result<(Player, u64), String> {
    let player = ctx.db.player().identity().find(ctx.sender()).ok_or("not connected")?;
    let id = player.entity_id.ok_or("no character in the world")?;
    Ok((player, id))
}

fn is_alive(ctx: &ReducerContext, entity_id: u64) -> bool {
    ctx.db
        .combat()
        .entity_id()
        .find(entity_id)
        .map_or(false, |c| c.dead_until_us.is_none() && c.hp > 0)
}

// ---------------------------------------------------------------- lifecycle

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.admin().insert(Admin { identity: ctx.sender() });
    ctx.db.tick_stats().insert(TickStats {
        id: 0,
        ticks: 0,
        last_tick_entities: 0,
        max_gap_us: 0,
        last_at_us: 0,
    });
    ctx.db.combat_tick_timer().insert(CombatTickTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(COMBAT_TICK_MS).into()),
    });
    ctx.db.spawn_tick_timer().insert(SpawnTickTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(SPAWN_TICK_MS).into()),
    });
}

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    // The admin identity (the CLI that publishes and runs SQL) is not a game client.
    if ctx.db.admin().identity().find(ctx.sender()).is_some() {
        return;
    }
    let zone = ctx.db.zone_info().zone_id().find(DEFAULT_ZONE);
    let mut player = ctx.db.player().identity().find(ctx.sender()).unwrap_or_else(|| {
        let (x, y) = zone.as_ref().map_or((520000.0, 520000.0), |z| (z.start_x, z.start_y));
        let hex = ctx.sender().to_hex().to_string();
        ctx.db.player().insert(Player {
            identity: ctx.sender(),
            entity_id: None,
            name: format!("Tester{}", &hex[hex.len() - 4..]),
            online: true,
            zone_id: DEFAULT_ZONE,
            last_x: x,
            last_y: y,
            last_hp: 0,
            ranged: false,
        })
    });
    player.online = true;
    if player.entity_id.is_none() {
        spawn_player_entity(ctx, &mut player);
    }
    ctx.db.player().identity().update(player);
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    let Some(mut player) = ctx.db.player().identity().find(ctx.sender()) else { return };
    if let Some(id) = player.entity_id.take() {
        let t = now_us(ctx);
        if let Some(p) = position(ctx, id, t) {
            player.last_x = p.0;
            player.last_y = p.1;
        }
        if let Some(c) = ctx.db.combat().entity_id().find(id) {
            player.last_hp = c.hp;
        }
        despawn(ctx, id);
    }
    player.online = false;
    ctx.db.player().identity().update(player);
}

// ---------------------------------------------------------------- player reducers

#[spacetimedb::reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = name.trim().to_string();
    if name.is_empty() || name.len() > 20 {
        return Err("name must be 1-20 characters".into());
    }
    let (mut player, id) = my_player(ctx)?;
    player.name = name.clone();
    ctx.db.player().identity().update(player);
    if let Some(mut e) = ctx.db.entity().entity_id().find(id) {
        e.name = name;
        ctx.db.entity().entity_id().update(e);
    }
    Ok(())
}

/// Click-to-move. Keeps the attack target, which is what makes attack-while-moving work.
#[spacetimedb::reducer]
pub fn move_to(ctx: &ReducerContext, x: f32, y: f32) -> Result<(), String> {
    let (player, id) = my_player(ctx)?;
    if !is_alive(ctx, id) {
        return Err("dead".into());
    }
    if !x.is_finite() || !y.is_finite() {
        return Err("bad position".into());
    }
    if let Some(z) = ctx.db.zone_info().zone_id().find(player.zone_id) {
        if x < z.min_x || x > z.max_x || y < z.min_y || y > z.max_y {
            return Err("outside the zone".into());
        }
    }
    let speed = ctx.db.stats().entity_id().find(id).map_or(425.0, |s| s.move_speed);
    set_motion(ctx, id, (x, y), speed, None);
    Ok(())
}

/// Start auto-attacking. If out of range, chase like ROSE does.
#[spacetimedb::reducer]
pub fn attack(ctx: &ReducerContext, target: u64) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    if target == id {
        return Err("can't attack yourself".into());
    }
    if !is_alive(ctx, id) {
        return Err("dead".into());
    }
    let target_entity = ctx.db.entity().entity_id().find(target).ok_or("no such target")?;
    if target_entity.kind != EntityKind::Monster {
        return Err("PvP is not enabled".into());
    }
    if !is_alive(ctx, target) {
        return Err("target is dead".into());
    }
    let mut c = ctx.db.combat().entity_id().find(id).ok_or("no combat row")?;
    c.attack_target = Some(target);
    ctx.db.combat().entity_id().update(c);

    let t = now_us(ctx);
    let stats = ctx.db.stats().entity_id().find(id).ok_or("no stats")?;
    let (me, them) = (position(ctx, id, t).unwrap(), position(ctx, target, t).unwrap());
    let moving = ctx.db.motion().entity_id().find(id).map_or(false, |m| m.is_moving(t));
    if distance(me, them) > stats.attack_range + RANGE_SLACK_CM && !moving {
        set_motion(ctx, id, them, stats.move_speed, Some(target));
    }
    Ok(())
}

#[spacetimedb::reducer]
pub fn stop(ctx: &ReducerContext) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    stop_motion(ctx, id);
    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
        if c.attack_target.is_some() {
            c.attack_target = None;
            ctx.db.combat().entity_id().update(c);
        }
    }
    Ok(())
}

/// Test helper: switch between a melee and a ranged loadout.
#[spacetimedb::reducer]
pub fn set_loadout(ctx: &ReducerContext, ranged: bool) -> Result<(), String> {
    let (mut player, id) = my_player(ctx)?;
    player.ranged = ranged;
    ctx.db.player().identity().update(player);
    ctx.db.stats().entity_id().update(player_stats(id, ranged));
    Ok(())
}

// ---------------------------------------------------------------- admin reducers

#[spacetimedb::reducer]
pub fn import_npcs(ctx: &ReducerContext, npcs: Vec<NpcData>) -> Result<(), String> {
    require_admin(ctx)?;
    for npc in npcs {
        if ctx.db.npc_data().npc_id().find(npc.npc_id).is_some() {
            ctx.db.npc_data().npc_id().update(npc);
        } else {
            ctx.db.npc_data().insert(npc);
        }
    }
    Ok(())
}

#[spacetimedb::reducer]
pub fn import_zone(ctx: &ReducerContext, zone: ZoneInfo, spawns: Vec<MonsterSpawn>) -> Result<(), String> {
    require_admin(ctx)?;
    let zone_id = zone.zone_id;
    if ctx.db.zone_info().zone_id().find(zone_id).is_some() {
        ctx.db.zone_info().zone_id().update(zone);
    } else {
        ctx.db.zone_info().insert(zone);
    }
    for s in ctx.db.monster_spawn().iter().filter(|s| s.zone_id == zone_id) {
        ctx.db.monster_spawn().spawn_id().delete(s.spawn_id);
    }
    for s in spawns {
        ctx.db.monster_spawn().insert(s);
    }
    Ok(())
}

/// Make a monster type attack players that come within `range` cm (0 = only fights back).
#[spacetimedb::reducer]
pub fn set_aggro_range(ctx: &ReducerContext, npc_id: u16, range: f32) -> Result<(), String> {
    require_admin(ctx)?;
    let mut npc = ctx.db.npc_data().npc_id().find(npc_id).ok_or("no such npc")?;
    npc.aggro_range = range;
    ctx.db.npc_data().npc_id().update(npc);
    for e in ctx.db.entity().iter().filter(|e| e.npc_id == npc_id && e.kind == EntityKind::Monster) {
        if let Some(mut ai) = ctx.db.monster_ai().entity_id().find(e.entity_id) {
            ai.aggro_range = range;
            ctx.db.monster_ai().entity_id().update(ai);
        }
    }
    Ok(())
}

/// Debug: move a player (online or not) to a spot, e.g. next to monsters for a demo.
#[spacetimedb::reducer]
pub fn place_player(ctx: &ReducerContext, name: String, x: f32, y: f32) -> Result<(), String> {
    require_admin(ctx)?;
    let players: Vec<Player> = ctx.db.player().iter().filter(|p| p.name == name).collect();
    if players.is_empty() {
        return Err("no such player".into());
    }
    let t = now_us(ctx);
    for mut player in players {
        player.last_x = x;
        player.last_y = y;
        // Offline players come back at full HP.
        player.last_hp = 0;
        if let Some(mut m) = player.entity_id.and_then(|id| ctx.db.motion().entity_id().find(id)) {
            m.from_x = x;
            m.from_y = y;
            m.to_x = x;
            m.to_y = y;
            m.started_at_us = t;
            m.chase_target = None;
            ctx.db.motion().entity_id().update(m);
        }
        ctx.db.player().identity().update(player);
    }
    // Clear any entity an earlier admin connection left behind.
    let ghosts: Vec<Player> = ctx.db.player().iter().filter(|p| ctx.db.admin().identity().find(p.identity).is_some()).collect();
    for mut g in ghosts {
        if let Some(id) = g.entity_id.take() {
            despawn(ctx, id);
        }
        g.online = false;
        ctx.db.player().identity().update(g);
    }
    Ok(())
}

/// Remove all monsters so spawn_tick refills them from the spawn table.
#[spacetimedb::reducer]
pub fn reset_monsters(ctx: &ReducerContext) -> Result<(), String> {
    require_admin(ctx)?;
    let ids: Vec<u64> = ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Monster).map(|e| e.entity_id).collect();
    for id in ids {
        despawn(ctx, id);
    }
    for mut s in ctx.db.monster_spawn().iter() {
        s.next_spawn_at_us = 0;
        ctx.db.monster_spawn().spawn_id().update(s);
    }
    Ok(())
}

// ---------------------------------------------------------------- timers

#[spacetimedb::reducer]
pub fn combat_tick(ctx: &ReducerContext, _timer: CombatTickTimer) -> Result<(), String> {
    if ctx.sender() != ctx.database_identity() {
        return Err("timer only".into());
    }
    let t = now_us(ctx);
    let mut rng = ctx.rng();

    // Monster AI first: aggro, leash, chase.
    for ai in ctx.db.monster_ai().iter() {
        let id = ai.entity_id;
        let Some(c) = ctx.db.combat().entity_id().find(id) else { continue };
        if c.dead_until_us.is_some() {
            continue;
        }
        let Some(pos) = position(ctx, id, t) else { continue };
        let home = (ai.home_x, ai.home_y);
        if ai.returning {
            if distance(pos, home) < 10.0 {
                let mut ai = ai.clone();
                ai.returning = false;
                ctx.db.monster_ai().entity_id().update(ai);
            }
            continue;
        }
        if distance(pos, home) > MONSTER_LEASH_CM {
            // Give up: walk home at run speed and heal.
            let mut c = c.clone();
            c.attack_target = None;
            c.hp = c.max_hp;
            ctx.db.combat().entity_id().update(c);
            let speed = ctx.db.stats().entity_id().find(id).map_or(300.0, |s| s.run_speed);
            set_motion(ctx, id, home, speed, None);
            let mut ai = ai.clone();
            ai.returning = true;
            ctx.db.monster_ai().entity_id().update(ai);
            continue;
        }
        if c.attack_target.is_none() && ai.aggro_range > 0.0 {
            let nearest = ctx
                .db
                .entity()
                .iter()
                .filter(|e| e.kind == EntityKind::Player && is_alive(ctx, e.entity_id))
                .filter_map(|e| position(ctx, e.entity_id, t).map(|p| (e.entity_id, distance(pos, p))))
                .filter(|(_, d)| *d <= ai.aggro_range)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((target, _)) = nearest {
                let mut c = c.clone();
                c.attack_target = Some(target);
                ctx.db.combat().entity_id().update(c);
            }
        }
    }

    // Attacks for everyone with a target.
    let attackers: Vec<Combat> = ctx.db.combat().iter().filter(|c| c.attack_target.is_some()).collect();
    let mut processed = 0u32;
    for c in attackers {
        // Re-read: an earlier hit this tick may have killed this attacker.
        let Some(mut c) = ctx.db.combat().entity_id().find(c.entity_id) else { continue };
        let Some(target) = c.attack_target else { continue };
        if c.dead_until_us.is_some() {
            continue;
        }
        processed += 1;
        let id = c.entity_id;
        if !is_alive(ctx, target) {
            c.attack_target = None;
            ctx.db.combat().entity_id().update(c);
            stop_chase(ctx, id, target);
            continue;
        }
        let Some(stats) = ctx.db.stats().entity_id().find(id) else { continue };
        let (Some(me), Some(them)) = (position(ctx, id, t), position(ctx, target, t)) else { continue };
        let dist = distance(me, them);

        if dist <= stats.attack_range + RANGE_SLACK_CM {
            stop_chase(ctx, id, target);
            if t >= c.next_attack_at_us {
                let defender_stats = ctx.db.stats().entity_id().find(target).unwrap();
                let dmg = damage::roll(&mut rng, &stats, &defender_stats);
                let mut dc = ctx.db.combat().entity_id().find(target).unwrap();
                dc.hp = (dc.hp - dmg.amount).max(0);
                let killed = dc.hp == 0;
                // Monsters fight back when hit.
                if !defender_stats.is_player && dc.attack_target.is_none() && !killed {
                    dc.attack_target = Some(id);
                }
                ctx.db.combat().entity_id().update(dc);
                ctx.db.damage_event().insert(DamageEvent {
                    attacker: id,
                    defender: target,
                    amount: dmg.amount,
                    is_critical: dmg.is_critical,
                    killed,
                    attacker_moving: ctx.db.motion().entity_id().find(id).map_or(false, |m| m.is_moving(t)),
                    at_us: t,
                });
                // Schedule from the previous swing so ticks don't add drift.
                let interval = attack_interval_us(&stats);
                c.next_attack_at_us = (c.next_attack_at_us + interval).max(t + interval - COMBAT_TICK_MS as i64 * 1000);
                if killed {
                    c.attack_target = None;
                    kill(ctx, target, t);
                }
                ctx.db.combat().entity_id().update(c);
            }
        } else {
            let motion = ctx.db.motion().entity_id().find(id);
            let chasing = motion.as_ref().map_or(false, |m| m.chase_target == Some(target));
            let moving = motion.as_ref().map_or(false, |m| m.is_moving(t));
            // Monsters always chase. Players chase only if they asked to (attack out of range);
            // a player who clicked to move keeps walking and keeps the target (attack while moving).
            if !stats.is_player || chasing || !moving {
                let needs_repath = match &motion {
                    Some(m) if m.chase_target == Some(target) => distance((m.to_x, m.to_y), them) > CHASE_REPATH_CM,
                    _ => true,
                };
                if needs_repath {
                    let speed = stats.run_speed;
                    set_motion(ctx, id, them, speed, Some(target));
                }
            }
        }
    }

    if let Some(mut s) = ctx.db.tick_stats().id().find(0) {
        if s.last_at_us > 0 {
            s.max_gap_us = s.max_gap_us.max(t - s.last_at_us);
        }
        s.ticks += 1;
        s.last_tick_entities = processed;
        s.last_at_us = t;
        ctx.db.tick_stats().id().update(s);
    }
    Ok(())
}

fn stop_chase(ctx: &ReducerContext, id: u64, target: u64) {
    if let Some(m) = ctx.db.motion().entity_id().find(id) {
        if m.chase_target == Some(target) {
            stop_motion(ctx, id);
        }
    }
}

fn kill(ctx: &ReducerContext, id: u64, t: i64) {
    let Some(entity) = ctx.db.entity().entity_id().find(id) else { return };
    match entity.kind {
        EntityKind::Monster => {
            if let Some(ai) = ctx.db.monster_ai().entity_id().find(id) {
                if let Some(mut s) = ctx.db.monster_spawn().spawn_id().find(ai.spawn_id) {
                    s.next_spawn_at_us = t + s.respawn_secs as i64 * 1_000_000;
                    ctx.db.monster_spawn().spawn_id().update(s);
                }
            }
            despawn(ctx, id);
        }
        EntityKind::Player => {
            stop_motion(ctx, id);
            if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
                c.attack_target = None;
                c.dead_until_us = Some(t + PLAYER_RESPAWN_US);
                ctx.db.combat().entity_id().update(c);
            }
            for mut c in ctx.db.combat().iter().filter(|c| c.attack_target == Some(id)) {
                c.attack_target = None;
                ctx.db.combat().entity_id().update(c);
            }
        }
    }
}

#[spacetimedb::reducer]
pub fn spawn_tick(ctx: &ReducerContext, _timer: SpawnTickTimer) -> Result<(), String> {
    if ctx.sender() != ctx.database_identity() {
        return Err("timer only".into());
    }
    let t = now_us(ctx);

    // Revive dead players at the zone's revive point.
    for c in ctx.db.combat().iter().filter(|c| c.dead_until_us.map_or(false, |d| d <= t)) {
        let Some(e) = ctx.db.entity().entity_id().find(c.entity_id) else { continue };
        let revive = ctx.db.zone_info().zone_id().find(e.zone_id).map_or((520000.0, 520000.0), |z| (z.revive_x, z.revive_y));
        let speed = ctx.db.stats().entity_id().find(c.entity_id).map_or(425.0, |s| s.move_speed);
        ctx.db.motion().entity_id().update(Motion {
            entity_id: c.entity_id,
            from_x: revive.0,
            from_y: revive.1,
            to_x: revive.0,
            to_y: revive.1,
            started_at_us: t,
            speed,
            chase_target: None,
        });
        let mut c = c.clone();
        c.hp = c.max_hp;
        c.dead_until_us = None;
        ctx.db.combat().entity_id().update(c);
    }

    // Only zones with a player in them get monsters.
    let active_zones: Vec<u16> = ctx.db.player().iter().filter(|p| p.online).map(|p| p.zone_id).collect();
    let mut rng = ctx.rng();
    for spawn in ctx.db.monster_spawn().iter().filter(|s| active_zones.contains(&s.zone_id)) {
        if spawn.next_spawn_at_us > t {
            continue;
        }
        let Some(npc) = ctx.db.npc_data().npc_id().find(spawn.npc_id) else { continue };
        let alive = ctx.db.monster_ai().spawn_id().filter(spawn.spawn_id).count() as u32;
        if alive < spawn.max_alive {
            spawn_monster(ctx, &spawn, &npc);
            // One per spawn point per second, like a trickle rather than a burst.
        }
    }

    // Idle wandering: about one in eight idle monsters takes a short walk each second.
    for ai in ctx.db.monster_ai().iter() {
        let id = ai.entity_id;
        let Some(c) = ctx.db.combat().entity_id().find(id) else { continue };
        if c.attack_target.is_some() || ai.returning || !rng.gen_ratio(1, 8) {
            continue;
        }
        if ctx.db.motion().entity_id().find(id).map_or(true, |m| m.is_moving(t)) {
            continue;
        }
        let Some(spawn) = ctx.db.monster_spawn().spawn_id().find(ai.spawn_id) else { continue };
        let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let r: f32 = rng.gen_range(0.0..spawn.radius.max(100.0));
        let to = (spawn.x + angle.cos() * r, spawn.y + angle.sin() * r);
        let speed = ctx.db.stats().entity_id().find(id).map_or(200.0, |s| s.move_speed);
        set_motion(ctx, id, to, speed, None);
    }
    Ok(())
}
