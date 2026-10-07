//! Parties of up to five, after rose-offline's party_system: invites, the leader, kicks,
//! and the sharing rules. Members nearby share kill experience (equally or by level) and
//! the party's drops (money split or items and money in turn). Offline members stay in
//! the party.

use rose_game_common::components::DroppedItem;
use rose_game_data::GameData;
use spacetimedb::{Identity, ReducerContext, Table};

use crate::{character, distance, items, my_player, now_us, player, position, xp_event, Player, XpEvent};

pub const MAX_MEMBERS: usize = 5;
/// Members this close to a kill or a pickup share it.
const SHARE_RANGE_CM: f32 = 5000.0;
/// You invite players you can see.
const INVITE_RANGE_CM: f32 = 3000.0;
const INVITE_EXPIRE_US: i64 = 30_000_000;
/// Extra experience for each member besides the first who shares a kill (our choice; the
/// iROSE party bonus isn't in the vendored code).
const XP_BONUS_PERCENT_PER_MEMBER: u64 = 10;

pub const XP_EQUAL: u8 = 0;
pub const XP_BY_LEVEL: u8 = 1;
pub const ITEMS_EQUAL: u8 = 0;
pub const ITEMS_IN_TURN: u8 = 1;

#[spacetimedb::table(accessor = party, public)]
#[derive(Clone)]
pub struct Party {
    #[primary_key]
    #[auto_inc]
    pub party_id: u64,
    pub owner: Identity,
    /// XP_EQUAL or XP_BY_LEVEL.
    pub xp_sharing: u8,
    /// ITEMS_EQUAL (money is split, items go to who picks them up) or ITEMS_IN_TURN.
    pub item_sharing: u8,
    pub item_turn: u32,
    pub money_turn: u32,
}

#[spacetimedb::table(accessor = party_member, public)]
#[derive(Clone)]
pub struct PartyMember {
    #[primary_key]
    pub identity: Identity,
    #[index(btree)]
    pub party_id: u64,
    pub joined_us: i64,
}

#[spacetimedb::table(accessor = party_invitation, public)]
#[derive(Clone)]
pub struct PartyInvite {
    #[primary_key]
    #[auto_inc]
    pub invite_id: u64,
    pub from: Identity,
    #[index(btree)]
    pub to: Identity,
    pub expires_us: i64,
}

pub fn party_of(ctx: &ReducerContext, identity: Identity) -> Option<Party> {
    let m = ctx.db.party_member().identity().find(identity)?;
    ctx.db.party().party_id().find(m.party_id)
}

/// The members in the order they joined.
pub fn members(ctx: &ReducerContext, party_id: u64) -> Vec<PartyMember> {
    let mut members: Vec<PartyMember> = ctx.db.party_member().party_id().filter(party_id).collect();
    members.sort_by_key(|m| m.joined_us);
    members
}

/// Online members in this zone within range of a point, in joining order.
fn nearby(ctx: &ReducerContext, party_id: u64, zone_id: u16, at: (f32, f32), t: i64) -> Vec<Player> {
    members(ctx, party_id)
        .into_iter()
        .filter_map(|m| ctx.db.player().identity().find(m.identity))
        .filter(|p| p.online && p.zone_id == zone_id)
        .filter(|p| p.entity_id.and_then(|id| position(ctx, id, t)).is_some_and(|pos| distance(pos, at) <= SHARE_RANGE_CM))
        .collect()
}

pub fn same_party(ctx: &ReducerContext, a: Identity, b: Identity) -> bool {
    match (ctx.db.party_member().identity().find(a), ctx.db.party_member().identity().find(b)) {
        (Some(a), Some(b)) => a.party_id == b.party_id,
        _ => false,
    }
}

/// For quest conditions.
pub fn quest_party(ctx: &ReducerContext, identity: Identity) -> Option<rose_quest::QuestParty> {
    let party = party_of(ctx, identity)?;
    Some(rose_quest::QuestParty {
        is_leader: party.owner == identity,
        // Parties don't level up yet.
        level: 1,
        member_count: members(ctx, party.party_id).len(),
    })
}

fn tell_party(ctx: &ReducerContext, party_id: u64, text: &str) {
    for m in members(ctx, party_id) {
        items::notify(ctx, m.identity, text.to_string());
    }
}

fn name(ctx: &ReducerContext, identity: Identity) -> String {
    ctx.db.player().identity().find(identity).map_or_else(String::new, |p| p.name)
}

/// Experience a player earned from a kill at `at`, shared with the party members nearby.
pub fn reward_kill_xp(ctx: &ReducerContext, game: &GameData, p: &Player, xp: u64, at: (f32, f32), t: i64) {
    let mut shares = vec![(p.identity, xp)];
    if let Some(party) = party_of(ctx, p.identity) {
        let mut with = nearby(ctx, party.party_id, p.zone_id, at, t);
        if !with.iter().any(|m| m.identity == p.identity) {
            with.push(p.clone());
        }
        if with.len() > 1 {
            let total = xp * (100 + XP_BONUS_PERCENT_PER_MEMBER * (with.len() as u64 - 1)) / 100;
            let levels: u64 = with.iter().map(|m| m.level.max(1) as u64).sum();
            shares = with
                .iter()
                .map(|m| {
                    let share = if party.xp_sharing == XP_BY_LEVEL {
                        total * m.level.max(1) as u64 / levels
                    } else {
                        total / with.len() as u64
                    };
                    (m.identity, share.max(1))
                })
                .collect();
        }
    }
    for (identity, xp) in shares {
        character::reward_xp(ctx, game, identity, xp);
        let level = ctx.db.player().identity().find(identity).map_or(0, |p| p.level);
        ctx.db.xp_event().insert(XpEvent { identity, xp, level });
    }
}

/// Who gets a drop `picker` picks up: the picker, or with ITEMS_IN_TURN the next nearby
/// member. Money in ITEMS_EQUAL parties is split instead (`split_money`).
pub fn pickup_receiver(ctx: &ReducerContext, picker: &Player, dropped: &DroppedItem, at: (f32, f32), t: i64) -> Identity {
    let Some(mut party) = party_of(ctx, picker.identity) else { return picker.identity };
    if party.item_sharing != ITEMS_IN_TURN {
        return picker.identity;
    }
    let with = nearby(ctx, party.party_id, picker.zone_id, at, t);
    if with.len() < 2 {
        return picker.identity;
    }
    let turn = match dropped {
        DroppedItem::Item(_) => &mut party.item_turn,
        DroppedItem::Money(_) => &mut party.money_turn,
    };
    *turn = turn.wrapping_add(1);
    let receiver = with[*turn as usize % with.len()].identity;
    ctx.db.party().party_id().update(party);
    receiver
}

/// Money picked up in an ITEMS_EQUAL party: (member, share) for each nearby member, the
/// picker's share holding the remainder. None when it isn't split.
pub fn split_money(ctx: &ReducerContext, picker: &Player, amount: i64, at: (f32, f32), t: i64) -> Option<Vec<(Identity, i64)>> {
    let party = party_of(ctx, picker.identity)?;
    if party.item_sharing != ITEMS_EQUAL {
        return None;
    }
    let with = nearby(ctx, party.party_id, picker.zone_id, at, t);
    if with.len() < 2 {
        return None;
    }
    let share = amount / with.len() as i64;
    let rest = amount - share * with.len() as i64;
    Some(with.iter().map(|m| (m.identity, if m.identity == picker.identity { share + rest } else { share })).collect())
}

pub fn expire_invites(ctx: &ReducerContext, t: i64) {
    let old: Vec<u64> = ctx.db.party_invitation().iter().filter(|i| i.expires_us <= t).map(|i| i.invite_id).collect();
    for id in old {
        ctx.db.party_invitation().invite_id().delete(id);
    }
}

/// Invite the player of this entity (creating the party when they accept).
#[spacetimedb::reducer]
pub fn party_invite(ctx: &ReducerContext, target_entity_id: u64) -> Result<(), String> {
    let (p, id) = my_player(ctx)?;
    let target = ctx.db.player().iter().find(|q| q.entity_id == Some(target_entity_id)).ok_or("that isn't a player")?;
    if target.identity == p.identity {
        return Err("you can't invite yourself".into());
    }
    let t = now_us(ctx);
    let near = match (position(ctx, id, t), position(ctx, target_entity_id, t)) {
        (Some(a), Some(b)) => distance(a, b) <= INVITE_RANGE_CM,
        _ => false,
    };
    if !target.online || target.zone_id != p.zone_id || !near {
        return Err(format!("{} is too far away", target.name));
    }
    if let Some(party) = party_of(ctx, p.identity) {
        if party.owner != p.identity {
            return Err("only the party leader can invite".into());
        }
        if members(ctx, party.party_id).len() >= MAX_MEMBERS {
            return Err("your party is full".into());
        }
    }
    if ctx.db.party_member().identity().find(target.identity).is_some() {
        return Err(format!("{} is already in a party", target.name));
    }
    for old in ctx.db.party_invitation().to().filter(target.identity).filter(|i| i.from == p.identity) {
        ctx.db.party_invitation().invite_id().delete(old.invite_id);
    }
    ctx.db.party_invitation().insert(PartyInvite { invite_id: 0, from: p.identity, to: target.identity, expires_us: t + INVITE_EXPIRE_US });
    items::notify(ctx, p.identity, format!("Invited {} to your party", target.name));
    Ok(())
}

#[spacetimedb::reducer]
pub fn party_accept(ctx: &ReducerContext, invite_id: u64) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let invite = ctx.db.party_invitation().invite_id().find(invite_id).filter(|i| i.to == p.identity).ok_or("the invitation is gone")?;
    ctx.db.party_invitation().invite_id().delete(invite_id);
    if ctx.db.party_member().identity().find(p.identity).is_some() {
        return Err("you are already in a party".into());
    }
    let t = now_us(ctx);
    let party = match party_of(ctx, invite.from) {
        Some(party) if party.owner != invite.from => return Err("the invitation is gone".into()),
        Some(party) => party,
        None => {
            let party = ctx.db.party().insert(Party {
                party_id: 0,
                owner: invite.from,
                xp_sharing: XP_EQUAL,
                item_sharing: ITEMS_EQUAL,
                item_turn: 0,
                money_turn: 0,
            });
            ctx.db.party_member().insert(PartyMember { identity: invite.from, party_id: party.party_id, joined_us: t });
            party
        }
    };
    if members(ctx, party.party_id).len() >= MAX_MEMBERS {
        return Err("the party is full".into());
    }
    ctx.db.party_member().insert(PartyMember { identity: p.identity, party_id: party.party_id, joined_us: t + 1 });
    for other in ctx.db.party_invitation().to().filter(p.identity).collect::<Vec<_>>() {
        ctx.db.party_invitation().invite_id().delete(other.invite_id);
    }
    tell_party(ctx, party.party_id, &format!("{} joined the party", p.name));
    Ok(())
}

#[spacetimedb::reducer]
pub fn party_decline(ctx: &ReducerContext, invite_id: u64) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let invite = ctx.db.party_invitation().invite_id().find(invite_id).filter(|i| i.to == p.identity).ok_or("the invitation is gone")?;
    ctx.db.party_invitation().invite_id().delete(invite_id);
    items::notify(ctx, invite.from, format!("{} declined your invitation", p.name));
    Ok(())
}

/// Take a member out; the next member leads when the leader goes, and a party of one ends.
fn remove_member(ctx: &ReducerContext, party: Party, identity: Identity, text: &str) {
    tell_party(ctx, party.party_id, text);
    ctx.db.party_member().identity().delete(identity);
    let rest = members(ctx, party.party_id);
    if rest.len() < 2 {
        for m in rest {
            items::notify(ctx, m.identity, "The party has ended".to_string());
            ctx.db.party_member().identity().delete(m.identity);
        }
        ctx.db.party().party_id().delete(party.party_id);
    } else if party.owner == identity {
        let owner = rest[0].identity;
        tell_party(ctx, party.party_id, &format!("{} now leads the party", name(ctx, owner)));
        ctx.db.party().party_id().update(Party { owner, ..party });
    }
}

#[spacetimedb::reducer]
pub fn party_leave(ctx: &ReducerContext) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let party = party_of(ctx, p.identity).ok_or("you aren't in a party")?;
    remove_member(ctx, party, p.identity, &format!("{} left the party", p.name));
    Ok(())
}

#[spacetimedb::reducer]
pub fn party_kick(ctx: &ReducerContext, member: Identity) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let party = party_of(ctx, p.identity).filter(|party| party.owner == p.identity).ok_or("only the party leader can do that")?;
    if member == p.identity || !same_party(ctx, p.identity, member) {
        return Err("they aren't in your party".into());
    }
    remove_member(ctx, party, member, &format!("{} was removed from the party", name(ctx, member)));
    Ok(())
}

#[spacetimedb::reducer]
pub fn party_set_leader(ctx: &ReducerContext, member: Identity) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let party = party_of(ctx, p.identity).filter(|party| party.owner == p.identity).ok_or("only the party leader can do that")?;
    if !same_party(ctx, p.identity, member) {
        return Err("they aren't in your party".into());
    }
    tell_party(ctx, party.party_id, &format!("{} now leads the party", name(ctx, member)));
    ctx.db.party().party_id().update(Party { owner: member, ..party });
    Ok(())
}

#[spacetimedb::reducer]
pub fn party_set_rules(ctx: &ReducerContext, xp_sharing: u8, item_sharing: u8) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let party = party_of(ctx, p.identity).filter(|party| party.owner == p.identity).ok_or("only the party leader can do that")?;
    if xp_sharing > XP_BY_LEVEL || item_sharing > ITEMS_IN_TURN {
        return Err("no such rule".into());
    }
    let xp = if xp_sharing == XP_BY_LEVEL { "by level" } else { "equally" };
    let loot = if item_sharing == ITEMS_IN_TURN { "in turn" } else { "to whoever picks them up, money split" };
    tell_party(ctx, party.party_id, &format!("Party rules: experience shared {xp}, drops {loot}"));
    ctx.db.party().party_id().update(Party { xp_sharing, item_sharing, ..party });
    Ok(())
}

