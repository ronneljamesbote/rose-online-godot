//! The friends list (iROSE's messenger; rose-offline has none). Follows rose-next's world
//! server: you ask a player who is online by name, they accept or decline (an unanswered
//! request lapses after 30 seconds), and the friendship is stored for both. Each side can
//! have up to 35 friends. Removing a friend removes you from their list too. Friends are
//! told when you come online or leave. Messages to friends use the whisper channel.
//!
//! The tables are private: each client reads only its own list and requests through the
//! my_friends and my_friend_requests views.

use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table, ViewContext};

#[allow(unused_imports)]
use crate::{items, my_player, now_us, player, player__view, world::zone_info__view, zone_info, Player};

/// MAX_FRIEND_COUNT in rose-next's messenger.
pub const MAX_FRIENDS: usize = 35;
const REQUEST_EXPIRE_US: i64 = 30_000_000;

/// One direction of a friendship: `owner` has `friend` on their list.
#[spacetimedb::table(accessor = friend)]
#[derive(Clone)]
pub struct Friend {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub owner: Identity,
    pub friend: Identity,
}

#[spacetimedb::table(accessor = friend_request)]
#[derive(Clone)]
pub struct FriendRequest {
    #[primary_key]
    #[auto_inc]
    pub request_id: u64,
    #[index(btree)]
    pub to: Identity,
    pub from: Identity,
    pub from_name: String,
    pub expires_us: i64,
}

/// A friend as our list shows them.
#[derive(SpacetimeType, Clone)]
pub struct FriendEntry {
    pub id: u64,
    pub name: String,
    pub online: bool,
    pub level: u32,
    pub job: u16,
    pub zone: String,
}

/// Our friends, with whether they are online and where.
#[spacetimedb::view(accessor = my_friends, public)]
pub fn my_friends(ctx: &ViewContext) -> Vec<FriendEntry> {
    ctx.db
        .friend()
        .owner()
        .filter(ctx.sender())
        .filter_map(|f| {
            let p = ctx.db.player().identity().find(f.friend)?;
            let zone = if p.online { ctx.db.zone_info().zone_id().find(p.zone_id).map_or_else(String::new, |z| z.name) } else { String::new() };
            Some(FriendEntry { id: f.id, name: p.name, online: p.online, level: p.level, job: p.job, zone })
        })
        .collect()
}

/// Requests other players sent us.
#[spacetimedb::view(accessor = my_friend_requests, public)]
pub fn my_friend_requests(ctx: &ViewContext) -> Vec<FriendRequest> {
    ctx.db.friend_request().to().filter(ctx.sender()).collect()
}

fn count(ctx: &ReducerContext, owner: Identity) -> usize {
    ctx.db.friend().owner().filter(owner).count()
}

fn are_friends(ctx: &ReducerContext, a: Identity, b: Identity) -> bool {
    ctx.db.friend().owner().filter(a).any(|f| f.friend == b)
}

/// Ask a player to be friends, by name. They must be online.
#[spacetimedb::reducer]
pub fn friend_ask(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let name = name.trim();
    let target = ctx
        .db
        .player()
        .iter()
        .find(|q| q.name.eq_ignore_ascii_case(name) && q.online)
        .ok_or_else(|| format!("{name} isn't online"))?;
    ask(ctx, &p, &target)
}

/// Ask the player we have selected (the Add Friend action).
pub fn friend_ask_entity(ctx: &ReducerContext, target_entity_id: u64) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let target = ctx.db.player().iter().find(|q| q.entity_id == Some(target_entity_id)).ok_or("that isn't a player")?;
    ask(ctx, &p, &target)
}

fn ask(ctx: &ReducerContext, p: &Player, target: &Player) -> Result<(), String> {
    if target.identity == p.identity {
        return Err("you can't add yourself".into());
    }
    if are_friends(ctx, p.identity, target.identity) {
        return Err(format!("{} is already your friend", target.name));
    }
    if count(ctx, p.identity) >= MAX_FRIENDS {
        return Err(format!("your friends list is full ({MAX_FRIENDS})"));
    }
    let old: Vec<u64> = ctx
        .db
        .friend_request()
        .to()
        .filter(target.identity)
        .filter(|r| r.from == p.identity)
        .map(|r| r.request_id)
        .collect();
    for id in old {
        ctx.db.friend_request().request_id().delete(id);
    }
    // Both asked each other: that is a yes.
    if let Some(theirs) = ctx.db.friend_request().to().filter(p.identity).find(|r| r.from == target.identity) {
        return answer(ctx, p, theirs.request_id, true);
    }
    ctx.db.friend_request().insert(FriendRequest {
        request_id: 0,
        to: target.identity,
        from: p.identity,
        from_name: p.name.clone(),
        expires_us: now_us(ctx) + REQUEST_EXPIRE_US,
    });
    items::notify(ctx, p.identity, format!("Asked {} to be friends", target.name));
    items::notify(ctx, target.identity, format!("{} wants to be friends", p.name));
    Ok(())
}

/// Accept or decline a friend request sent to us.
#[spacetimedb::reducer]
pub fn friend_answer(ctx: &ReducerContext, request_id: u64, accept: bool) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    answer(ctx, &p, request_id, accept)
}

fn answer(ctx: &ReducerContext, p: &Player, request_id: u64, accept: bool) -> Result<(), String> {
    let request = ctx
        .db
        .friend_request()
        .request_id()
        .find(request_id)
        .filter(|r| r.to == p.identity)
        .ok_or("that request is gone")?;
    ctx.db.friend_request().request_id().delete(request_id);
    if !accept {
        items::notify(ctx, request.from, format!("{} declined your friend request", p.name));
        return Ok(());
    }
    if are_friends(ctx, p.identity, request.from) {
        return Ok(());
    }
    if count(ctx, p.identity) >= MAX_FRIENDS {
        return Err(format!("your friends list is full ({MAX_FRIENDS})"));
    }
    if count(ctx, request.from) >= MAX_FRIENDS {
        return Err(format!("{}'s friends list is full", request.from_name));
    }
    ctx.db.friend().insert(Friend { id: 0, owner: p.identity, friend: request.from });
    ctx.db.friend().insert(Friend { id: 0, owner: request.from, friend: p.identity });
    items::notify(ctx, p.identity, format!("{} is now your friend", request.from_name));
    items::notify(ctx, request.from, format!("{} is now your friend", p.name));
    Ok(())
}

/// Remove a friend (by their name); we leave their list as well.
#[spacetimedb::reducer]
pub fn friend_remove(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let p = ctx.db.player().identity().find(ctx.sender()).ok_or("not connected")?;
    let other = ctx
        .db
        .friend()
        .owner()
        .filter(p.identity)
        .find(|f| ctx.db.player().identity().find(f.friend).is_some_and(|q| q.name == name))
        .ok_or_else(|| format!("{name} isn't on your list"))?;
    let rows: Vec<u64> = ctx
        .db
        .friend()
        .owner()
        .filter(p.identity)
        .filter(|f| f.friend == other.friend)
        .chain(ctx.db.friend().owner().filter(other.friend).filter(|f| f.friend == p.identity))
        .map(|f| f.id)
        .collect();
    for id in rows {
        ctx.db.friend().id().delete(id);
    }
    items::notify(ctx, p.identity, format!("Removed {name} from your friends"));
    items::notify(ctx, other.friend, format!("{} removed you from their friends", p.name));
    Ok(())
}

/// Tell a player's friends that they came online or left.
pub fn announce(ctx: &ReducerContext, p: &Player, online: bool) {
    let text = if online { format!("{} is online", p.name) } else { format!("{} went offline", p.name) };
    for f in ctx.db.friend().owner().filter(p.identity) {
        if ctx.db.player().identity().find(f.friend).is_some_and(|q| q.online) {
            items::notify(ctx, f.friend, text.clone());
        }
    }
}

pub fn expire_requests(ctx: &ReducerContext, t: i64) {
    let old: Vec<u64> = ctx.db.friend_request().iter().filter(|r| r.expires_us <= t).map(|r| r.request_id).collect();
    for id in old {
        ctx.db.friend_request().request_id().delete(id);
    }
}

/// A character moved to another account (assign_character): its friendships follow it.
pub fn rekey(ctx: &ReducerContext, old: Identity, new: Identity) {
    let rows: Vec<Friend> = ctx.db.friend().iter().filter(|f| f.owner == old || f.friend == old).collect();
    for mut f in rows {
        if f.owner == old {
            f.owner = new;
        }
        if f.friend == old {
            f.friend = new;
        }
        ctx.db.friend().id().update(f);
    }
}
