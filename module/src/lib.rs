//! ROSE on SpacetimeDB.
//!
//! Units follow rose-offline: positions in cm, speeds in cm/s, times stored as
//! microseconds since the Unix epoch so clients can evaluate motion paths too.
//! Game rules come from rose-offline's crates (vendored in ../crates); the game databases
//! are built from client files the host uploads (see game_data.rs).

mod ability;
mod account;
mod bank;
mod character;
mod chat;
mod craft;
mod death;
mod durability;
mod friends;
mod shop;
mod vehicle;
mod game_data;
mod items;
mod monster_brain;
mod npcs;
use npcs::npc;
mod quests;
mod npc_ai;
mod party;
mod pvp;
mod skills;
mod stamina;
mod trade;
mod world;

use rand::Rng;
use rose_game_common::components::AbilityValues;
use rose_game_data::GameData;
use spacetimedb::{ConnectionId, Identity, ReducerContext, ScheduleAt, SpacetimeType, Table};
use std::time::Duration;

pub use world::{monster_spawn, zone_info};

const COMBAT_TICK_MS: u64 = 100;
const SPAWN_TICK_MS: u64 = 1000;
pub(crate) const MONSTER_LEASH_CM: f32 = 3000.0;
/// Re-path a chase only when the target has moved this far from the current destination.
const CHASE_REPATH_CM: f32 = 100.0;
/// Extra reach so an entity that stopped exactly at range still swings.
const RANGE_SLACK_CM: f32 = 50.0;
/// New characters start where rose-offline starts them, on Birth Island.
const START_ZONE: u16 = 20;
const START_POSITION: (f32, f32) = (530500.0, 539500.0);
/// Damage older than this earns no experience (rose-offline's DAMAGE_REWARD_EXPIRE_TIME).
const DAMAGE_REWARD_EXPIRE_US: i64 = 5 * 60 * 1_000_000;
/// Passive HP and MP recovery interval (rose-offline's passive_recovery_system).
const RECOVERY_INTERVAL_US: i64 = 4_000_000;

#[derive(SpacetimeType, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Player,
    Monster,
    Npc,
}

#[spacetimedb::table(accessor = admin)]
pub struct Admin {
    #[primary_key]
    identity: Identity,
}

/// One character per identity. Components that rose-offline keeps as structs (equipment,
/// inventory, skills, quests, hotbar) are stored as their serde JSON.
#[spacetimedb::table(accessor = player, public)]
#[derive(Clone)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub entity_id: Option<u64>,
    pub name: String,
    pub online: bool,
    /// The connection playing this character. A second client on the same identity takes
    /// the character over, and the first one's disconnect then leaves it alone.
    pub connection: Option<ConnectionId>,
    pub zone_id: u16,
    pub last_x: f32,
    pub last_y: f32,
    pub last_hp: i32,
    pub last_mp: i32,
    /// 0 male, 1 female.
    pub gender: u8,
    pub face: u8,
    pub hair: u8,
    pub job: u16,
    pub level: u32,
    pub xp: u64,
    pub stat_points: u32,
    pub skill_points: u32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub concentration: i32,
    pub charm: i32,
    pub sense: i32,
    pub equipment: String,
    pub inventory: String,
    pub skill_list: String,
    pub quest_state: String,
    pub hotbar: String,
    pub union_membership: String,
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
    pub mp: i32,
    pub max_mp: i32,
    pub attack_target: Option<u64>,
    /// Earliest start of the next swing (attack speed cooldown).
    pub next_attack_at_us: i64,
    /// Set while a swing is winding up: when its hit frame lands. Moving or stopping before
    /// then cancels the swing and refunds the cooldown (animation cancelling).
    pub swing_hit_at_us: Option<i64>,
    pub dead_until_us: Option<i64>,
}

/// An entity's ability values (rose-game-common's AbilityValues), with the ones combat and
/// the HUD use as columns and the whole struct as JSON for the damage formulas.
#[spacetimedb::table(accessor = stats, public)]
#[derive(Clone)]
pub struct Stats {
    #[primary_key]
    pub entity_id: u64,
    pub level: i32,
    pub max_hp: i32,
    pub max_mp: i32,
    pub attack_power: i32,
    pub hit: i32,
    pub defence: i32,
    pub resistance: i32,
    pub avoid: i32,
    pub critical: i32,
    pub attack_speed: i32,
    /// Attack animation length at attack speed 100.
    pub attack_motion_ms: i32,
    /// Time from the start of the attack animation to its hit frame, at attack speed 100.
    pub attack_hit_ms: i32,
    /// Attack frames in the attack animation (the hit count the damage formula takes).
    pub hit_count: i32,
    pub attack_range: f32,
    pub move_speed: f32,
    /// Chase speed for monsters (their run speed); same as move_speed for players.
    pub run_speed: f32,
    pub is_player: bool,
    pub ability_values: String,
}

impl Stats {
    pub fn from_ability_values(
        entity_id: u64,
        av: &AbilityValues,
        is_player: bool,
        attack_motion_ms: i32,
        attack_hit_ms: i32,
        hit_count: i32,
    ) -> Self {
        let run_speed = av.get_run_speed();
        Stats {
            entity_id,
            level: av.get_level(),
            max_hp: av.get_max_health(),
            max_mp: av.get_max_mana(),
            attack_power: av.get_attack_power(),
            hit: av.get_hit(),
            defence: av.get_defence(),
            resistance: av.get_resistance(),
            avoid: av.get_avoid(),
            critical: av.get_critical(),
            attack_speed: av.get_attack_speed(),
            attack_motion_ms,
            attack_hit_ms,
            hit_count,
            attack_range: av.get_attack_range() as f32,
            // Players always run; monsters walk unless they chase.
            move_speed: if is_player { run_speed } else { av.get_walk_speed() },
            run_speed,
            is_player,
            ability_values: serde_json::to_string(av).unwrap_or_default(),
        }
    }

    fn ability(&self) -> Option<AbilityValues> {
        serde_json::from_str(&self.ability_values).ok()
    }
}

/// Players sitting down (rose-offline's Sit command). HP comes back faster while sitting and
/// MP only comes back while sitting. Moving, attacking or using a skill stands them up.
#[spacetimedb::table(accessor = sitting, public)]
pub struct Sitting {
    #[primary_key]
    pub entity_id: u64,
    pub since_us: i64,
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

/// Who hurt a monster and how much, for the experience share when it dies.
#[spacetimedb::table(accessor = damage_source)]
pub struct DamageSource {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub defender: u64,
    pub attacker: u64,
    pub total_damage: u64,
    pub last_at_us: i64,
}

#[spacetimedb::table(accessor = damage_event, public, event)]
pub struct DamageEvent {
    pub attacker: u64,
    pub defender: u64,
    pub amount: i32,
    pub is_critical: bool,
    pub killed: bool,
    pub at_us: i64,
}

/// Experience gained, so clients can show "+N XP".
#[spacetimedb::table(accessor = xp_event, public, event)]
pub struct XpEvent {
    pub identity: Identity,
    pub xp: u64,
    pub level: u32,
}

/// Server rates in percent, as rose-offline's WorldRates (its defaults).
#[spacetimedb::table(accessor = world_rates, public)]
#[derive(Clone)]
pub struct WorldRates {
    #[primary_key]
    pub id: u8,
    pub xp_rate: i32,
    pub drop_rate: i32,
    pub drop_money_rate: i32,
    pub reward_rate: i32,
    /// Store prices (rose-offline's world_price_rate, item_price_rate, town_price_rate).
    pub world_price_rate: i32,
    pub item_price_rate: i32,
    pub town_price_rate: i32,
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
    pub next_recovery_at_us: i64,
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
    vehicle::sync_passenger(ctx, entity_id);
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

pub(crate) fn require_admin(ctx: &ReducerContext) -> Result<(), String> {
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

/// Wind-up of a swing: start of the attack animation to its hit frame, scaled like the motion.
fn attack_windup_us(stats: &Stats) -> i64 {
    let speed = stats.attack_speed.max(30) as i64;
    stats.attack_hit_ms as i64 * 1000 * 100 / speed
}

/// Drop the attack target. A swing that has not reached its hit frame yet is cancelled and its
/// cooldown refunded; after the hit, the rest of the animation is simply skipped.
fn cancel_attack(ctx: &ReducerContext, id: u64, t: i64) {
    let Some(mut c) = ctx.db.combat().entity_id().find(id) else { return };
    if c.attack_target.is_none() && c.swing_hit_at_us.is_none() {
        return;
    }
    if c.swing_hit_at_us.take().is_some() {
        c.next_attack_at_us = t;
    }
    c.attack_target = None;
    ctx.db.combat().entity_id().update(c);
}

fn despawn(ctx: &ReducerContext, entity_id: u64) {
    shop::close(ctx, entity_id);
    vehicle::forget(ctx, entity_id);
    ctx.db.entity().entity_id().delete(entity_id);
    ctx.db.motion().entity_id().delete(entity_id);
    ctx.db.combat().entity_id().delete(entity_id);
    ctx.db.stats().entity_id().delete(entity_id);
    ctx.db.monster_ai().entity_id().delete(entity_id);
    monster_brain::forget(ctx, entity_id);
    ctx.db.npc().entity_id().delete(entity_id);
    ctx.db.sitting().entity_id().delete(entity_id);
    skills::clear_entity(ctx, entity_id);
    clear_damage_sources(ctx, entity_id);
    forget_entity(ctx, entity_id);
}

/// Anyone targeting or chasing the entity loses it (it died, left or changed zone).
fn forget_entity(ctx: &ReducerContext, entity_id: u64) {
    for mut c in ctx.db.combat().iter().filter(|c| c.attack_target == Some(entity_id)) {
        c.attack_target = None;
        ctx.db.combat().entity_id().update(c);
    }
    for m in ctx.db.motion().iter().filter(|m| m.chase_target == Some(entity_id)) {
        stop_motion(ctx, m.entity_id);
    }
}

fn clear_damage_sources(ctx: &ReducerContext, defender: u64) {
    let ids: Vec<u64> = ctx.db.damage_source().defender().filter(defender).map(|d| d.id).collect();
    for id in ids {
        ctx.db.damage_source().id().delete(id);
    }
}

pub(crate) fn spawn_player_entity(ctx: &ReducerContext, game: &GameData, player: &mut Player) {
    let entity = ctx.db.entity().insert(Entity {
        entity_id: 0,
        kind: EntityKind::Player,
        npc_id: 0,
        zone_id: player.zone_id,
        name: player.name.clone(),
    });
    let id = entity.entity_id;
    let stats = ctx.db.stats().insert(character::player_stats(ctx, game, id, player));
    let hp = if player.last_hp > 0 { player.last_hp.min(stats.max_hp) } else { stats.max_hp };
    let mp = if player.last_hp > 0 { player.last_mp.clamp(0, stats.max_mp) } else { stats.max_mp };
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
        max_hp: stats.max_hp,
        mp,
        max_mp: stats.max_mp,
        attack_target: None,
        next_attack_at_us: t,
        swing_hit_at_us: None,
        dead_until_us: None,
    });
    player.entity_id = Some(id);
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

fn world_rates_row(ctx: &ReducerContext) -> WorldRates {
    ctx.db.world_rates().id().find(0).unwrap_or(WorldRates {
        id: 0,
        xp_rate: 300,
        drop_rate: 300,
        drop_money_rate: 300,
        reward_rate: 300,
        world_price_rate: 100,
        item_price_rate: 50,
        town_price_rate: 100,
    })
}

fn start_timers(ctx: &ReducerContext) {
    ctx.db.combat_tick_timer().insert(CombatTickTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(COMBAT_TICK_MS).into()),
    });
    ctx.db.spawn_tick_timer().insert(SpawnTickTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(Duration::from_millis(SPAWN_TICK_MS).into()),
    });
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
        next_recovery_at_us: 0,
    });
    ctx.db.world_rates().insert(world_rates_row(ctx));
    start_timers(ctx);
}

/// Puts the connecting player's character in the world, creating it on first connect.
/// Without game data nobody can play yet; the client shows game_data_status instead.
/// With accounts on (account::set_auth_issuer), only players who signed in on the website get
/// in, and a new account makes its character with create_character. Without them every new
/// identity gets a test character right away.
#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) -> Result<(), String> {
    // The admin identity (the CLI that publishes and runs SQL) is not a game client.
    if ctx.db.admin().identity().find(ctx.sender()).is_some() {
        return Ok(());
    }
    account::check_connection(ctx)?;
    if account::is_service(ctx) {
        return Ok(());
    }
    let Ok(game) = game_data::game(ctx) else { return Ok(()) };
    let existing = ctx.db.player().identity().find(ctx.sender());
    if existing.is_none() && account::accounts_required(ctx) {
        return Ok(());
    }
    let mut player = existing.unwrap_or_else(|| {
        let hex = ctx.sender().to_hex().to_string();
        let name = format!("Tester{}", &hex[hex.len() - 4..]);
        let (x, y) = START_POSITION;
        ctx.db.player().insert(character::new_player(&game, ctx.sender(), name, 0, (START_ZONE, x, y)))
    });
    if !player.online {
        friends::announce(ctx, &player, true);
    }
    player.online = true;
    player.connection = ctx.connection_id();
    if player.entity_id.is_none() {
        spawn_player_entity(ctx, &game, &mut player);
    }
    ctx.db.player().identity().update(player);
    Ok(())
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    let Some(mut player) = ctx.db.player().identity().find(ctx.sender()) else { return };
    if player.connection.is_some() && player.connection != ctx.connection_id() {
        return;
    }
    player.connection = None;
    trade::cancel_for(ctx, player.identity);
    // Leaving while fallen gets up at the zone's revive point first (iROSE saves the
    // character there), so logging out isn't a way back up on the spot with full HP.
    if let (Some(id), Ok(game)) = (player.entity_id, game_data::game(ctx)) {
        if ctx.db.combat().entity_id().find(id).is_some_and(|c| c.dead_until_us.is_some()) {
            death::revive(ctx, &game, &mut player, id, false, now_us(ctx));
        }
    }
    if let Some(id) = player.entity_id.take() {
        let t = now_us(ctx);
        if let Some(p) = position(ctx, id, t) {
            player.last_x = p.0;
            player.last_y = p.1;
        }
        if let Some(c) = ctx.db.combat().entity_id().find(id) {
            player.last_hp = c.hp;
            player.last_mp = c.mp;
        }
        despawn(ctx, id);
    }
    player.online = false;
    friends::announce(ctx, &player, false);
    ctx.db.player().identity().update(player);
}

// ---------------------------------------------------------------- player reducers

#[spacetimedb::reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    if account::accounts_required(ctx) {
        return Err("names are chosen when the character is made".into());
    }
    let name = name.trim().to_string();
    let (mut player, id) = my_player(ctx)?;
    if player.name == name {
        return Ok(());
    }
    if let Some(problem) = account::name_problem(ctx, &name, Some(player.identity)) {
        return Err(problem);
    }
    player.name = name.clone();
    ctx.db.player().identity().update(player);
    if let Some(mut e) = ctx.db.entity().entity_id().find(id) {
        e.name = name;
        ctx.db.entity().entity_id().update(e);
    }
    Ok(())
}

/// Click-to-move. Moving ends the attack (no attacking while moving).
#[spacetimedb::reducer]
pub fn move_to(ctx: &ReducerContext, x: f32, y: f32) -> Result<(), String> {
    let (player, id) = my_player(ctx)?;
    if !is_alive(ctx, id) {
        return Err("dead".into());
    }
    if let Some(reason) = vehicle::passenger_refusal(ctx, id) {
        return Err(reason.into());
    }
    if let Some(reason) = skills::disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    if shop::is_open(ctx, id) {
        return Err("close your shop first".into());
    }
    if vehicle::is_driving(ctx, id) && vehicle::engine_life(&player) <= 0 {
        return Err("out of fuel".into());
    }
    if !x.is_finite() || !y.is_finite() {
        return Err("bad position".into());
    }
    if let Some(z) = ctx.db.zone_info().zone_id().find(player.zone_id) {
        if x < z.min_x || x > z.max_x || y < z.min_y || y > z.max_y {
            return Err("outside the zone".into());
        }
    }
    // It also cancels a swing or a cast in progress.
    stand_up(ctx, id);
    cancel_attack(ctx, id, now_us(ctx));
    skills::cancel_cast(ctx, id);
    let speed = ctx.db.stats().entity_id().find(id).map_or(425.0, |s| s.move_speed);
    set_motion(ctx, id, (x, y), speed, None);
    Ok(())
}

/// Start auto-attacking. If out of range, chase like ROSE does.
#[spacetimedb::reducer]
pub fn attack(ctx: &ReducerContext, target: u64) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    if shop::is_open(ctx, id) {
        return Err("close your shop first".into());
    }
    if target == id {
        return Err("can't attack yourself".into());
    }
    if !is_alive(ctx, id) {
        return Err("dead".into());
    }
    let target_entity = ctx.db.entity().entity_id().find(target).ok_or("no such target")?;
    let game = game_data::game(ctx)?;
    if !pvp::can_attack(ctx, &game, id, target) {
        return Err(match target_entity.kind {
            EntityKind::Player => "you can't fight players here".into(),
            _ => "you can't attack that".to_string(),
        });
    }
    if !is_alive(ctx, target) {
        return Err("target is dead".into());
    }
    if let Some(reason) = vehicle::passenger_refusal(ctx, id) {
        return Err(reason.into());
    }
    if let Some(reason) = skills::disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    let t = now_us(ctx);
    // Nobody can start a fight with a hidden character.
    let current = ctx.db.combat().entity_id().find(id).and_then(|c| c.attack_target);
    if current != Some(target) && skills::is_invisible(ctx, target) {
        return Err("you can't see them".into());
    }
    stand_up(ctx, id);
    skills::cancel_cast(ctx, id);
    skills::break_disguise(ctx, &game, id);
    let mut c = ctx.db.combat().entity_id().find(id).ok_or("no combat row")?;
    if c.attack_target == Some(target) {
        return Ok(());
    }
    // Switching targets cancels a swing that hasn't landed yet.
    if c.swing_hit_at_us.take().is_some() {
        c.next_attack_at_us = t;
    }
    c.attack_target = Some(target);
    ctx.db.combat().entity_id().update(c);

    // Stand still to attack; out of range, run to the target first like ROSE does.
    let stats = ctx.db.stats().entity_id().find(id).ok_or("no stats")?;
    let (me, them) = (position(ctx, id, t).unwrap(), position(ctx, target, t).unwrap());
    if distance(me, them) > stats.attack_range + RANGE_SLACK_CM {
        set_motion(ctx, id, them, stats.move_speed, Some(target));
    } else {
        stop_motion(ctx, id);
    }
    Ok(())
}

#[spacetimedb::reducer]
pub fn stop(ctx: &ReducerContext) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    stop_motion(ctx, id);
    cancel_attack(ctx, id, now_us(ctx));
    skills::cancel_cast(ctx, id);
    Ok(())
}

/// Sit down, or stand up when sitting. Sitting stops the character and any attack or cast.
#[spacetimedb::reducer]
pub fn sit(ctx: &ReducerContext) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    if ctx.db.sitting().entity_id().find(id).is_some() {
        stand_up(ctx, id);
        return Ok(());
    }
    if vehicle::is_driving(ctx, id) {
        return Err("you can't sit while driving".into());
    }
    if !is_alive(ctx, id) {
        return Err("dead".into());
    }
    if let Some(reason) = vehicle::passenger_refusal(ctx, id) {
        return Err(reason.into());
    }
    if let Some(reason) = skills::disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    let t = now_us(ctx);
    stop_motion(ctx, id);
    cancel_attack(ctx, id, t);
    skills::cancel_cast(ctx, id);
    ctx.db.sitting().insert(Sitting { entity_id: id, since_us: t });
    Ok(())
}

pub(crate) fn stand_up(ctx: &ReducerContext, id: u64) {
    ctx.db.sitting().entity_id().delete(id);
    // A personal shop is open only while its owner sits.
    shop::close(ctx, id);
}

/// How far off its straight path a client may say it hit something.
const COLLISION_PATH_SLACK_CM: f32 = 100.0;
/// How far ahead of the server's own position on the path the reported point may be, to
/// allow for the client starting the move before the server (latency).
const COLLISION_AHEAD_SLACK_CM: f32 = 300.0;

/// The client's character ran into a wall or an object at (x, y): stop the move there.
/// The server has no zone geometry, so it only checks that the point lies on the current
/// move and not ahead of where the server has the character (like rose-next's CANTMOVE).
#[spacetimedb::reducer]
pub fn move_collision(ctx: &ReducerContext, x: f32, y: f32) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    if !x.is_finite() || !y.is_finite() {
        return Err("bad position".into());
    }
    let t = now_us(ctx);
    let m = ctx.db.motion().entity_id().find(id).ok_or("no motion")?;
    let (dx, dy) = (m.to_x - m.from_x, m.to_y - m.from_y);
    let length = (dx * dx + dy * dy).sqrt();
    if length < 1.0 {
        return Err("not moving".into());
    }
    // Position of the reported point along the path, and its distance from the path.
    let along = ((x - m.from_x) * dx + (y - m.from_y) * dy) / length;
    let off_path = ((x - m.from_x) * dy - (y - m.from_y) * dx).abs() / length;
    let server_along = ((t - m.started_at_us).max(0) as f32 / 1e6 * m.speed).min(length);
    if off_path > COLLISION_PATH_SLACK_CM
        || along < -COLLISION_PATH_SLACK_CM
        || along > length + COLLISION_PATH_SLACK_CM
        || along > server_along + COLLISION_AHEAD_SLACK_CM
    {
        return Err("collision point is not on the current move".into());
    }
    let along = along.clamp(0.0, length);
    let stop = (m.from_x + dx / length * along, m.from_y + dy / length * along);
    ctx.db.motion().entity_id().update(Motion {
        from_x: stop.0,
        from_y: stop.1,
        to_x: stop.0,
        to_y: stop.1,
        started_at_us: t,
        chase_target: None,
        ..m
    });
    vehicle::sync_passenger(ctx, id);
    Ok(())
}

// ---------------------------------------------------------------- admin reducers

/// Set the server rates in percent (rose-offline's defaults are 300).
#[spacetimedb::reducer]
pub fn set_world_rates(ctx: &ReducerContext, xp_rate: i32, drop_rate: i32, drop_money_rate: i32, reward_rate: i32) -> Result<(), String> {
    require_admin(ctx)?;
    let rates = world_rates_row(ctx);
    ctx.db.world_rates().id().update(WorldRates { id: 0, xp_rate, drop_rate, drop_money_rate, reward_rate, ..rates });
    Ok(())
}

/// Set the store price rates in percent (rose-offline's defaults: world 100, item 50, town 100).
#[spacetimedb::reducer]
pub fn set_price_rates(ctx: &ReducerContext, world_price_rate: i32, item_price_rate: i32, town_price_rate: i32) -> Result<(), String> {
    require_admin(ctx)?;
    let rates = world_rates_row(ctx);
    ctx.db.world_rates().id().update(WorldRates { world_price_rate, item_price_rate, town_price_rate, ..rates });
    Ok(())
}

/// Make a monster type attack players that come within `range` cm (0 = only fights back).
/// Lasts until the monsters respawn; monster AI scripts are not ported yet.
#[spacetimedb::reducer]
pub fn set_aggro_range(ctx: &ReducerContext, npc_id: u16, range: f32) -> Result<(), String> {
    require_admin(ctx)?;
    for e in ctx.db.entity().iter().filter(|e| e.npc_id == npc_id && e.kind == EntityKind::Monster) {
        if let Some(mut ai) = ctx.db.monster_ai().entity_id().find(e.entity_id) {
            ai.aggro_range = range;
            ctx.db.monster_ai().entity_id().update(ai);
        }
    }
    Ok(())
}

/// Debug: set an online player's HP and MP (clamped to their maximum), e.g. to show recovery.
#[spacetimedb::reducer]
pub fn set_hp_mp(ctx: &ReducerContext, name: String, hp: i32, mp: i32) -> Result<(), String> {
    require_admin(ctx)?;
    let id = ctx.db.player().iter().find(|p| p.name == name).and_then(|p| p.entity_id).ok_or("no such player online")?;
    let mut c = ctx.db.combat().entity_id().find(id).ok_or("no combat row")?;
    c.hp = hp.clamp(1, c.max_hp);
    c.mp = mp.clamp(0, c.max_mp);
    ctx.db.combat().entity_id().update(c);
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
        if let Some(id) = player.entity_id {
            if let Some(mut m) = ctx.db.motion().entity_id().find(id) {
                m.from_x = x;
                m.from_y = y;
                m.to_x = x;
                m.to_y = y;
                m.started_at_us = t;
                m.chase_target = None;
                ctx.db.motion().entity_id().update(m);
                vehicle::sync_passenger(ctx, id);
            }
            if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
                c.hp = c.max_hp;
                ctx.db.combat().entity_id().update(c);
            }
        }
        ctx.db.player().identity().update(player);
    }
    Ok(())
}

/// Debug: give a player experience (by name).
#[spacetimedb::reducer]
pub fn give_xp(ctx: &ReducerContext, name: String, xp: u64) -> Result<(), String> {
    require_admin(ctx)?;
    let game = game_data::game(ctx)?;
    let player = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    character::reward_xp(ctx, &game, player.identity, xp);
    ctx.db.xp_event().insert(XpEvent { identity: player.identity, xp, level: 0 });
    Ok(())
}

/// Remove all monsters so the spawn points refill.
#[spacetimedb::reducer]
pub fn reset_monsters(ctx: &ReducerContext) -> Result<(), String> {
    require_admin(ctx)?;
    let ids: Vec<u64> = ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Monster).map(|e| e.entity_id).collect();
    for id in ids {
        despawn(ctx, id);
    }
    for mut s in ctx.db.monster_spawn().iter() {
        s.next_check_at_us = 0;
        s.current_tactics_value = 0;
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
    let Ok(game) = game_data::game(ctx) else { return Ok(()) };
    let t = now_us(ctx);
    rose_game_irose::rng::reseed(ctx.rng().gen());

    // Monster AI first: aggro, leash, chase.
    for ai in ctx.db.monster_ai().iter() {
        let id = ai.entity_id;
        let Some(c) = ctx.db.combat().entity_id().find(id) else { continue };
        if c.dead_until_us.is_some() || skills::is_disabled(ctx, id) {
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
            clear_damage_sources(ctx, id);
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
                .filter(|e| e.kind == EntityKind::Player && is_alive(ctx, e.entity_id) && !skills::is_invisible(ctx, e.entity_id))
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

    skills::taunt_tick(ctx);
    skills::summon_tick(ctx, &game, t);

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
        // Stunned or asleep: no swinging. A monster busy with a skill swings after it.
        if skills::is_disabled(ctx, id) || skills::is_casting(ctx, id) {
            continue;
        }
        let Some(stats) = ctx.db.stats().entity_id().find(id) else { continue };
        // A fight between players (or their summons) ends when the target dies or stops
        // being an enemy (left the PvP zone, joined our party).
        let player_side = |id| {
            ctx.db.entity().entity_id().find(id).is_some_and(|e| e.kind == EntityKind::Player)
                || skills::owner_of(ctx, id).is_some()
        };
        let player_fight = player_side(id) && player_side(target);
        let hostile = || pvp::players_hostile(ctx, &game, skills::controller(ctx, id), skills::controller(ctx, target));
        if !is_alive(ctx, target) || (player_fight && !hostile()) {
            c.attack_target = None;
            ctx.db.combat().entity_id().update(c);
            stop_chase(ctx, id, target);
            continue;
        }
        let (Some(me), Some(them)) = (position(ctx, id, t), position(ctx, target, t)) else { continue };
        let dist = distance(me, them);

        if dist <= stats.attack_range + RANGE_SLACK_CM {
            stop_chase(ctx, id, target);
            match c.swing_hit_at_us {
                // Hit frame reached: resolve the swing.
                Some(hit_at) if t >= hit_at => {
                    if stats.is_player {
                        take_player_ammo(ctx, &game, id, stats.hit_count);
                    }
                    let defender_stats = ctx.db.stats().entity_id().find(target).unwrap();
                    let (amount, is_critical) = match (stats.ability(), defender_stats.ability()) {
                        (Some(a), Some(d)) => {
                            let dmg = game.ability_value_calculator.calculate_damage(&a, &d, stats.hit_count);
                            (dmg.amount as i32, dmg.is_critical)
                        }
                        _ => (0, false),
                    };
                    c.swing_hit_at_us = None;
                    // Write our row first: a kill updates everyone who targets the dead.
                    let will_kill = ctx.db.combat().entity_id().find(target).is_some_and(|d| d.hp <= amount);
                    if will_kill {
                        c.attack_target = None;
                    }
                    ctx.db.combat().entity_id().update(c);
                    durability::weapon_used(ctx, &game, id);
                    let killed = deal_damage(ctx, &game, id, target, amount, is_critical, t);
                    // Monsters run their attack trigger after each swing (some use skills).
                    if !killed && !stats.is_player {
                        monster_brain::on_attack(ctx, &game, id, t);
                    }
                }
                Some(_) => {}
                // Ready: start a swing. Schedule from the previous one so ticks don't add drift.
                None if t >= c.next_attack_at_us => {
                    // A vehicle attack uses fuel; with none left it can't attack.
                    if stats.is_player && vehicle::is_driving(ctx, id) {
                        let Some(mut p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) else { continue };
                        if vehicle::engine_life(&p) <= 0 {
                            c.attack_target = None;
                            ctx.db.combat().entity_id().update(c);
                            items::notify(ctx, p.identity, "Out of fuel");
                            continue;
                        }
                        vehicle::use_fuel(ctx, &game, &mut p, None);
                    }
                    // Bows, guns and launchers need ammo for every hit of the swing.
                    else if stats.is_player && !player_has_ammo(ctx, &game, id, stats.hit_count) {
                        c.attack_target = None;
                        ctx.db.combat().entity_id().update(c);
                        if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) {
                            items::notify(ctx, p.identity, "Out of ammo");
                        }
                        continue;
                    }
                    let interval = attack_interval_us(&stats);
                    let late = t - c.next_attack_at_us;
                    let start = if late < COMBAT_TICK_MS as i64 * 1000 { c.next_attack_at_us } else { t };
                    c.next_attack_at_us = start + interval;
                    c.swing_hit_at_us = Some(start + attack_windup_us(&stats));
                    ctx.db.combat().entity_id().update(c);
                }
                None => {}
            }
        } else {
            // Target left range mid-swing: the swing misses its moment, no cooldown spent.
            if c.swing_hit_at_us.take().is_some() {
                c.next_attack_at_us = t;
                ctx.db.combat().entity_id().update(c);
            }
            let motion = ctx.db.motion().entity_id().find(id);
            let needs_repath = match &motion {
                Some(m) if m.chase_target == Some(target) => distance((m.to_x, m.to_y), them) > CHASE_REPATH_CM,
                _ => true,
            };
            if needs_repath {
                set_motion(ctx, id, them, stats.run_speed, Some(target));
            }
        }
    }

    skills::cast_tick(ctx, &game, t);

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

fn player_has_ammo(ctx: &ReducerContext, game: &GameData, id: u64, hit_count: i32) -> bool {
    let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) else { return true };
    match items::weapon_ammo(game, &p.equipment()) {
        Some(ammo) => items::has_ammo(&p, ammo, hit_count.max(1) as u32),
        None => true,
    }
}

fn take_player_ammo(ctx: &ReducerContext, game: &GameData, id: u64, hit_count: i32) {
    if vehicle::is_driving(ctx, id) {
        return;
    }
    let Some(mut p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) else { return };
    if let Some(ammo) = items::weapon_ammo(game, &p.equipment()) {
        items::use_ammo(ctx, &mut p, ammo, hit_count.max(1) as u32);
    }
}

fn add_damage_source(ctx: &ReducerContext, defender: u64, attacker: u64, amount: u64, t: i64) {
    if let Some(mut d) = ctx.db.damage_source().defender().filter(defender).find(|d| d.attacker == attacker) {
        d.total_damage += amount;
        d.last_at_us = t;
        ctx.db.damage_source().id().update(d);
    } else {
        ctx.db.damage_source().insert(DamageSource { id: 0, defender, attacker, total_damage: amount, last_at_us: t });
    }
}

fn stop_chase(ctx: &ReducerContext, id: u64, target: u64) {
    if let Some(m) = ctx.db.motion().entity_id().find(id) {
        if m.chase_target == Some(target) {
            stop_motion(ctx, id);
        }
    }
}

/// Experience for everyone who hurt a monster in the last five minutes, by rose-offline's
/// calculate_give_xp (parties come later).
fn reward_kill(ctx: &ReducerContext, game: &GameData, monster: u64, monster_stats: &Stats, npc_id: u16, t: i64) {
    let Some(npc) = rose_data::NpcId::new(npc_id).and_then(|id| game.npcs.get_npc(id)) else { return };
    let rates = world_rates_row(ctx);
    let at = position(ctx, monster, t).unwrap_or_default();
    for source in ctx.db.damage_source().defender().filter(monster) {
        if t - source.last_at_us > DAMAGE_REWARD_EXPIRE_US {
            continue;
        }
        let Some(player) = ctx.db.player().iter().find(|p| p.entity_id == Some(source.attacker)) else { continue };
        let xp = game.ability_value_calculator.calculate_give_xp(
            player.level as i32,
            source.total_damage as i32,
            monster_stats.level,
            monster_stats.max_hp,
            npc.reward_xp as i32,
            rates.xp_rate,
        );
        if xp > 0 {
            party::reward_kill_xp(ctx, game, &player, xp as u64, at, t);
        }
    }
}

/// Damage from an attack or a skill: lower HP, make a monster fight back, record who did how
/// much (for the experience share), tell clients, and kill at 0 HP. True if it killed.
fn deal_damage(ctx: &ReducerContext, game: &GameData, attacker: u64, defender: u64, amount: i32, is_critical: bool, t: i64) -> bool {
    let Some(defender_stats) = ctx.db.stats().entity_id().find(defender) else { return false };
    let Some(mut dc) = ctx.db.combat().entity_id().find(defender) else { return false };
    if dc.hp <= 0 || dc.dead_until_us.is_some() || death::is_shielded(ctx, defender, t) {
        return false;
    }
    let dealt = amount.min(dc.hp);
    dc.hp = (dc.hp - amount).max(0);
    let killed = dc.hp == 0;
    ctx.db.combat().entity_id().update(dc);
    if defender_stats.is_player && attacker != defender {
        durability::hit_taken(ctx, game, defender, amount);
    }
    // A summon's damage counts as its owner's (experience, drops, quests).
    let credited = skills::controller(ctx, attacker);
    if !defender_stats.is_player && dealt > 0 {
        add_damage_source(ctx, defender, credited, dealt as u64, t);
    }
    ctx.db.damage_event().insert(DamageEvent { attacker, defender, amount, is_critical, killed, at_us: t });
    if killed {
        kill(ctx, game, defender, &defender_stats, credited, t);
        return true;
    }
    // Any hit or miss wakes a sleeper.
    skills::wake(ctx, game, defender);
    // A damage shield sends part of the hit back (never lethal).
    let reflect = if attacker != defender { skills::shield_reflect(ctx, defender, dealt) } else { 0 };
    if reflect > 0 {
        if let Some(mut ac) = ctx.db.combat().entity_id().find(attacker).filter(|ac| ac.hp > 1 && ac.dead_until_us.is_none()) {
            ac.hp = (ac.hp - reflect).max(1);
            ctx.db.combat().entity_id().update(ac);
            ctx.db.damage_event().insert(DamageEvent {
                attacker: defender,
                defender: attacker,
                amount: reflect,
                is_critical: false,
                killed: false,
                at_us: t,
            });
        }
    }
    if !defender_stats.is_player {
        // The monster's damaged trigger: fight back, call friends for help.
        monster_brain::on_damaged(ctx, game, defender, attacker, dealt, t);
    }
    false
}

fn kill(ctx: &ReducerContext, game: &GameData, id: u64, stats: &Stats, killer: u64, t: i64) {
    let Some(entity) = ctx.db.entity().entity_id().find(id) else { return };
    items::clear_regen(ctx, id);
    match entity.kind {
        // A summon just leaves: no experience, drops or quest triggers.
        EntityKind::Monster if skills::owner_of(ctx, id).is_some() => despawn(ctx, id),
        EntityKind::Monster => {
            reward_kill(ctx, game, id, stats, entity.npc_id, t);
            // The killer runs the monster's death trigger (quest kill counts and quest drops).
            if let Some(npc) = rose_data::NpcId::new(entity.npc_id).and_then(|n| game.npcs.get_npc(n)) {
                if !npc.death_quest_trigger_name.is_empty() {
                    if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(killer)) {
                        quests::run_trigger(ctx, game, p.identity, &npc.death_quest_trigger_name);
                    }
                }
            }
            if let Some(at) = position(ctx, id, t) {
                items::monster_drop(ctx, game, entity.npc_id, entity.zone_id, at, killer, stats.level);
            }
            despawn(ctx, id);
        }
        EntityKind::Player => {
            let name = |id| ctx.db.player().iter().find(|p| p.entity_id == Some(id));
            if let (Some(winner), Some(loser)) = (name(killer), name(id)) {
                let text = format!("{} defeated {}", winner.name, loser.name);
                items::notify(ctx, winner.identity, text.clone());
                items::notify(ctx, loser.identity, text);
            }
            stop_motion(ctx, id);
            stand_up(ctx, id);
            vehicle::get_off(ctx, game, id);
            vehicle::leave_seat(ctx, id);
            death::player_died(ctx, game, id, killer, t);
            if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
                c.attack_target = None;
                c.dead_until_us = Some(t + death::AUTO_REVIVE_US);
                ctx.db.combat().entity_id().update(c);
            }
            for mut c in ctx.db.combat().iter().filter(|c| c.attack_target == Some(id)) {
                c.attack_target = None;
                ctx.db.combat().entity_id().update(c);
            }
        }
        EntityKind::Npc => {}
    }
}

#[spacetimedb::reducer]
pub fn spawn_tick(ctx: &ReducerContext, _timer: SpawnTickTimer) -> Result<(), String> {
    if ctx.sender() != ctx.database_identity() {
        return Err("timer only".into());
    }
    let Ok(game) = game_data::game(ctx) else { return Ok(()) };
    let t = now_us(ctx);

    // Fallen players who waited too long get up at the zone's revive point.
    death::tick(ctx, &game, t);
    vehicle::tick(ctx, &game, t);

    passive_recovery(ctx, &game, t);
    items::regen_tick(ctx);
    items::expire_drops(ctx, t);
    skills::status_tick(ctx, &game, t);
    npc_ai::npc_ai_tick(ctx, &game, t);
    party::expire_invites(ctx, t);
    trade::expire_requests(ctx, t);
    friends::expire_requests(ctx, t);
    chat::expire_messages(ctx, t);
    world::spawn_tick(ctx, &game, t);

    // Idle monsters run their AIP idle trigger: most wander, aggressive ones look for players.
    monster_brain::idle_tick(ctx, &game, t);
    Ok(())
}

/// Every four seconds living players regain HP, and MP too while sitting (rose-offline's
/// passive_recovery_system).
fn passive_recovery(ctx: &ReducerContext, game: &GameData, t: i64) {
    let Some(mut s) = ctx.db.tick_stats().id().find(0) else { return };
    if t < s.next_recovery_at_us {
        return;
    }
    s.next_recovery_at_us = t + RECOVERY_INTERVAL_US;
    ctx.db.tick_stats().id().update(s);
    for e in ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Player) {
        let Some(mut c) = ctx.db.combat().entity_id().find(e.entity_id) else { continue };
        if c.hp <= 0 || c.dead_until_us.is_some() || (c.hp >= c.max_hp && c.mp >= c.max_mp) {
            continue;
        }
        // No recovery while driving (Check_PerFRAME spends that time on fuel).
        if vehicle::is_driving(ctx, e.entity_id) {
            continue;
        }
        let Some(av) = ctx.db.stats().entity_id().find(e.entity_id).and_then(|s| s.ability()) else { continue };
        let state = if ctx.db.sitting().entity_id().find(e.entity_id).is_some() {
            rose_game_common::data::PassiveRecoveryState::Sitting
        } else {
            rose_game_common::data::PassiveRecoveryState::Normal
        };
        c.hp = (c.hp + game.ability_value_calculator.calculate_passive_recover_hp(&av, state)).min(c.max_hp);
        c.mp = (c.mp + game.ability_value_calculator.calculate_passive_recover_mp(&av, state)).min(c.max_mp);
        ctx.db.combat().entity_id().update(c);
    }
}
