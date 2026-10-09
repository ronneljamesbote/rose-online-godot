//! Stamina, the iROSE value scrolls and some item skills spend (skills 3101-3219, the
//! Purify, HP, Strength... scrolls and the Icicles). It is 0 to 5000. Killing monsters
//! earns it, by rose-offline's calculate_give_stamina (as rose-next's classUSER::Add_EXP
//! with the world stamina rate 100); Vital Jam and Stamina items add it.

use rose_game_common::components::MAX_STAMINA;
use spacetimedb::{Identity, ReducerContext, Table};

use rose_game_data::GameData;

use crate::player;

/// iROSE's world stamina rate (WORLD_VAR_STAMINA, 100 by default).
const WORLD_STAMINA_RATE: i32 = 100;

#[spacetimedb::table(accessor = stamina, public)]
#[derive(Clone)]
pub struct Stamina {
    #[primary_key]
    pub identity: Identity,
    pub value: u32,
}

pub fn get(ctx: &ReducerContext, identity: Identity) -> u32 {
    ctx.db.stamina().identity().find(identity).map_or(0, |s| s.value)
}

/// Add (or with a negative value, take) stamina, kept within 0 and 5000.
pub fn add(ctx: &ReducerContext, identity: Identity, value: i32) {
    let now = get(ctx, identity);
    let new = (now as i64 + value as i64).clamp(0, MAX_STAMINA as i64) as u32;
    if new == now {
        return;
    }
    if ctx.db.stamina().identity().find(identity).is_some() {
        ctx.db.stamina().identity().update(Stamina { identity, value: new });
    } else {
        ctx.db.stamina().insert(Stamina { identity, value: new });
    }
}

/// Stamina earned with experience from a kill.
pub fn reward(ctx: &ReducerContext, game: &GameData, identity: Identity, xp: u64, level: u32) {
    let gain = game.ability_value_calculator.calculate_give_stamina(xp.min(i32::MAX as u64) as i32, level as i32, WORLD_STAMINA_RATE);
    if gain > 0 {
        add(ctx, identity, gain);
    }
}

/// A character moved to another account (assign_character): its stamina follows.
pub fn rekey(ctx: &ReducerContext, old: Identity, new: Identity) {
    if let Some(mut s) = ctx.db.stamina().identity().find(old) {
        ctx.db.stamina().identity().delete(old);
        s.identity = new;
        ctx.db.stamina().insert(s);
    }
}

/// Debug: set a character's stamina, e.g. to test the scrolls.
#[spacetimedb::reducer]
pub fn set_stamina(ctx: &ReducerContext, name: String, value: u32) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let identity = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such character")?.identity;
    add(ctx, identity, value as i32 - get(ctx, identity) as i32);
    Ok(())
}
