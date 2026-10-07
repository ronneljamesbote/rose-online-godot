//! Chat, with the iROSE channels: talk to players nearby, shout to the whole zone, talk to
//! your party, or whisper to one player. Each message is stored once per player who should
//! read it and clients only see their own copies (the my_chat view), so a whisper never
//! reaches anyone else. Messages are kept for a minute; the client keeps its own log.

use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table, ViewContext};

use crate::party::{party_member, PartyMember};
use crate::{distance, entity, my_player, now_us, player, position, Player};

pub const MAX_CHAT_LEN: usize = 200;
/// Players this close hear a nearby message.
const NEARBY_CM: f32 = 5000.0;
const KEEP_US: i64 = 60_000_000;
/// At most this many messages in each window, and one shout per SHOUT_EVERY_US.
const RATE_WINDOW_US: i64 = 5_000_000;
const RATE_MAX: u32 = 6;
const SHOUT_EVERY_US: i64 = 5_000_000;

#[derive(SpacetimeType, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChatChannel {
    Nearby,
    Shout,
    Party,
    Whisper,
}

#[spacetimedb::table(accessor = chat_message)]
#[derive(Clone)]
pub struct ChatMessage {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub recipient: Identity,
    pub channel: ChatChannel,
    pub from_name: String,
    /// The speaker's entity, for the speech bubble over nearby speakers.
    pub from_entity: Option<u64>,
    /// Who a whisper went to, on the sender's own copy.
    pub to_name: String,
    pub text: String,
    pub at_us: i64,
}

#[spacetimedb::table(accessor = chat_limit)]
pub struct ChatLimit {
    #[primary_key]
    pub identity: Identity,
    pub window_start_us: i64,
    pub count: u32,
    pub last_shout_us: i64,
}

/// The messages this client may read.
#[spacetimedb::view(accessor = my_chat, public)]
pub fn my_chat(ctx: &ViewContext) -> Vec<ChatMessage> {
    ctx.db.chat_message().recipient().filter(ctx.sender()).collect()
}

/// Send a chat message. `to` is the player's name for a whisper and ignored otherwise.
#[spacetimedb::reducer]
pub fn send_chat(ctx: &ReducerContext, channel: ChatChannel, to: String, text: String) -> Result<(), String> {
    let (p, id) = my_player(ctx)?;
    let text: String = text.chars().filter(|c| !c.is_control()).collect();
    let text = text.trim();
    if text.is_empty() {
        return Ok(());
    }
    if text.chars().count() > MAX_CHAT_LEN {
        return Err(format!("messages are at most {MAX_CHAT_LEN} characters"));
    }
    let t = now_us(ctx);
    check_rate(ctx, p.identity, channel, t)?;

    let zone = ctx.db.entity().entity_id().find(id).map_or(p.zone_id, |e| e.zone_id);
    let online = || ctx.db.player().iter().filter(|q| q.online && q.entity_id.is_some());
    let mut to_name = String::new();
    let recipients: Vec<Identity> = match channel {
        ChatChannel::Nearby => {
            let here = position(ctx, id, t).unwrap_or((p.last_x, p.last_y));
            online()
                .filter(|q| in_zone(ctx, q, zone))
                .filter(|q| q.entity_id.and_then(|e| position(ctx, e, t)).map_or(false, |at| distance(here, at) <= NEARBY_CM))
                .map(|q| q.identity)
                .collect()
        }
        ChatChannel::Shout => online().filter(|q| in_zone(ctx, q, zone)).map(|q| q.identity).collect(),
        ChatChannel::Party => {
            let member = ctx.db.party_member().identity().find(p.identity).ok_or("you aren't in a party")?;
            ctx.db.party_member().party_id().filter(member.party_id).map(|m: PartyMember| m.identity).collect()
        }
        ChatChannel::Whisper => {
            let name = to.trim();
            let target = ctx
                .db
                .player()
                .iter()
                .find(|q| q.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("there is no player called {name}"))?;
            if !target.online {
                return Err(format!("{} isn't online", target.name));
            }
            to_name = target.name.clone();
            if target.identity == p.identity {
                vec![p.identity]
            } else {
                vec![p.identity, target.identity]
            }
        }
    };
    for recipient in recipients {
        ctx.db.chat_message().insert(ChatMessage {
            id: 0,
            recipient,
            channel,
            from_name: p.name.clone(),
            from_entity: Some(id),
            to_name: to_name.clone(),
            text: text.to_string(),
            at_us: t,
        });
    }
    Ok(())
}

fn in_zone(ctx: &ReducerContext, q: &Player, zone: u16) -> bool {
    q.entity_id.and_then(|e| ctx.db.entity().entity_id().find(e)).map_or(false, |e| e.zone_id == zone)
}

fn check_rate(ctx: &ReducerContext, identity: Identity, channel: ChatChannel, t: i64) -> Result<(), String> {
    let mut limit = ctx.db.chat_limit().identity().find(identity).unwrap_or(ChatLimit {
        identity,
        window_start_us: t,
        count: 0,
        last_shout_us: 0,
    });
    if t - limit.window_start_us >= RATE_WINDOW_US {
        limit.window_start_us = t;
        limit.count = 0;
    }
    if limit.count >= RATE_MAX {
        return Err("you are talking too fast".into());
    }
    if channel == ChatChannel::Shout {
        if t - limit.last_shout_us < SHOUT_EVERY_US {
            return Err("you can shout again in a few seconds".into());
        }
        limit.last_shout_us = t;
    }
    limit.count += 1;
    if ctx.db.chat_limit().identity().find(identity).is_some() {
        ctx.db.chat_limit().identity().update(limit);
    } else {
        ctx.db.chat_limit().insert(limit);
    }
    Ok(())
}

/// Drop messages older than a minute (from spawn_tick).
pub fn expire_messages(ctx: &ReducerContext, t: i64) {
    let old: Vec<u64> = ctx.db.chat_message().iter().filter(|m| t - m.at_us > KEEP_US).map(|m| m.id).collect();
    for id in old {
        ctx.db.chat_message().id().delete(id);
    }
}
