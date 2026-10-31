//! Leaving the world. Logging out takes LOGOUT_US and can't start while in a fight; moving,
//! attacking, using a skill or getting into a fight on the way cancels it. A character
//! whose player crashed or lost the connection stays in the world, where it can still be
//! hit, for LINGER_US before it leaves; connecting again in that time takes it back over.

use spacetimedb::{Identity, ReducerContext, Table};

use crate::{combat, monster_ai, now_us, player, stats, Player};

/// How long a logout takes.
pub const LOGOUT_US: i64 = 15_000_000;
/// How long a character stays after its player's connection dropped.
pub const LINGER_US: i64 = 60_000_000;
/// Hitting, or being hit or missed, keeps a character in a fight this long.
pub const FIGHT_US: i64 = 10_000_000;

/// A logout on its way: the character leaves at ends_us. Public, so the game can show the
/// countdown.
#[spacetimedb::table(accessor = logout, public)]
#[derive(Clone)]
pub struct Logout {
    #[primary_key]
    pub identity: Identity,
    pub entity_id: u64,
    pub started_us: i64,
    pub ends_us: i64,
}

/// A character left behind by a dropped connection: it leaves at until_us.
#[spacetimedb::table(accessor = linger)]
#[derive(Clone)]
pub struct Linger {
    #[primary_key]
    pub identity: Identity,
    pub entity_id: u64,
    pub until_us: i64,
}

/// When a player's character last hit or was hit (or missed) by anyone.
#[spacetimedb::table(accessor = fight_tag)]
#[derive(Clone)]
pub struct FightTag {
    #[primary_key]
    pub entity_id: u64,
    pub last_us: i64,
}

/// Start logging out.
#[spacetimedb::reducer]
pub fn request_logout(ctx: &ReducerContext) -> Result<(), String> {
    let (player, id) = crate::my_player(ctx)?;
    if ctx.db.logout().identity().find(player.identity).is_some() {
        return Ok(());
    }
    let t = now_us(ctx);
    if in_fight(ctx, id, t) {
        return Err("You can't log out during a fight".into());
    }
    // Standing still from here on: a swing or a cast in progress stops.
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, t);
    crate::skills::cancel_cast(ctx, id);
    ctx.db.logout().insert(Logout { identity: player.identity, entity_id: id, started_us: t, ends_us: t + LOGOUT_US });
    Ok(())
}

/// Stop logging out.
#[spacetimedb::reducer]
pub fn cancel_logout(ctx: &ReducerContext) -> Result<(), String> {
    ctx.db.logout().identity().delete(ctx.sender());
    Ok(())
}

/// In a fight: hit someone, or was hit or missed, in the last FIGHT_US, or a monster has
/// us as its target.
pub fn in_fight(ctx: &ReducerContext, id: u64, t: i64) -> bool {
    if ctx.db.fight_tag().entity_id().find(id).is_some_and(|f| t - f.last_us < FIGHT_US) {
        return true;
    }
    ctx.db.monster_ai().iter().any(|ai| {
        ctx.db.combat().entity_id().find(ai.entity_id).is_some_and(|c| c.attack_target == Some(id) && c.dead_until_us.is_none())
    })
}

/// A hit or a miss between two characters: players among them are in a fight now, and
/// their logout stops.
pub fn mark_fight(ctx: &ReducerContext, ids: [u64; 2], t: i64) {
    for id in ids {
        let is_player = ctx.db.stats().entity_id().find(id).is_some_and(|s| s.is_player);
        if !is_player {
            continue;
        }
        let tag = FightTag { entity_id: id, last_us: t };
        if ctx.db.fight_tag().entity_id().find(id).is_some() {
            ctx.db.fight_tag().entity_id().update(tag);
        } else {
            ctx.db.fight_tag().insert(tag);
        }
        cancel_for(ctx, id, "Logout cancelled: you are in a fight");
    }
}

/// The character did something that stops a logout (moved, attacked, used a skill, got
/// into a fight).
pub fn cancel_for(ctx: &ReducerContext, id: u64, why: &str) {
    let Some(l) = ctx.db.logout().iter().find(|l| l.entity_id == id) else { return };
    ctx.db.logout().identity().delete(l.identity);
    crate::items::notify(ctx, l.identity, why);
}

/// The player's connection dropped: their character stays for LINGER_US. A logout on its
/// way keeps going and may end it sooner.
pub fn start_linger(ctx: &ReducerContext, player: &Player, id: u64) {
    let t = now_us(ctx);
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, t);
    crate::skills::cancel_cast(ctx, id);
    crate::shop::close(ctx, id);
    let row = Linger { identity: player.identity, entity_id: id, until_us: t + LINGER_US };
    if ctx.db.linger().identity().find(player.identity).is_some() {
        ctx.db.linger().identity().update(row);
    } else {
        ctx.db.linger().insert(row);
    }
}

/// The player is back before their character left.
pub fn end_linger(ctx: &ReducerContext, identity: Identity) {
    ctx.db.linger().identity().delete(identity);
}

/// From the combat tick: characters whose logout or linger time is up leave.
pub fn tick(ctx: &ReducerContext, t: i64) {
    let due: Vec<Identity> = ctx
        .db
        .logout()
        .iter()
        .filter(|l| t >= l.ends_us)
        .map(|l| l.identity)
        .chain(ctx.db.linger().iter().filter(|l| t >= l.until_us).map(|l| l.identity))
        .collect();
    for identity in due {
        ctx.db.logout().identity().delete(identity);
        ctx.db.linger().identity().delete(identity);
        if let Some(mut p) = ctx.db.player().identity().find(identity) {
            crate::leave_world(ctx, &mut p);
            ctx.db.player().identity().update(p);
        }
    }
}

/// The character left the world: forget its rows here.
pub fn forget(ctx: &ReducerContext, identity: Identity, id: u64) {
    ctx.db.logout().identity().delete(identity);
    ctx.db.linger().identity().delete(identity);
    ctx.db.fight_tag().entity_id().delete(id);
}
