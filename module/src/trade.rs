//! Trading between two players standing near each other. rose-offline has no trading, so
//! this follows the iROSE flow: one player asks, the other accepts, both put up to ten
//! items and some Zuly on the table, lock their offer, and the trade happens when both
//! press Trade. Changing an offer unlocks both sides, so nobody accepts a swapped offer.

use rose_data::Item;
use rose_game_common::components::{Inventory, ItemSlot, Money};
use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

use crate::{distance, items, my_player, now_us, player, position, Player};

pub const MAX_TRADE_ITEMS: usize = 10;
const TRADE_RANGE_CM: f32 = 1500.0;
const REQUEST_EXPIRE_US: i64 = 30_000_000;

#[spacetimedb::table(accessor = trade, public)]
#[derive(Clone)]
pub struct Trade {
    #[primary_key]
    #[auto_inc]
    pub trade_id: u64,
    #[index(btree)]
    pub a: Identity,
    #[index(btree)]
    pub b: Identity,
    /// JSON `Vec<OfferedItem>` each side puts up.
    pub a_items: String,
    pub b_items: String,
    pub a_money: i64,
    pub b_money: i64,
    pub a_locked: bool,
    pub b_locked: bool,
    pub a_accepted: bool,
    pub b_accepted: bool,
}

#[spacetimedb::table(accessor = trade_request, public)]
#[derive(Clone)]
pub struct TradeRequest {
    #[primary_key]
    #[auto_inc]
    pub request_id: u64,
    pub from: Identity,
    #[index(btree)]
    pub to: Identity,
    pub expires_us: i64,
}

/// A bag item to trade: `quantity` of a stack, or the whole item.
#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct TradeSlot {
    pub page: u8,
    pub index: u16,
    pub quantity: u32,
}

/// What an offer shows the other side: the bag slot and the item with the offered quantity.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct OfferedItem {
    pub page: u8,
    pub index: u16,
    pub item: Item,
}

pub fn trade_of(ctx: &ReducerContext, identity: Identity) -> Option<Trade> {
    ctx.db.trade().a().filter(identity).next().or_else(|| ctx.db.trade().b().filter(identity).next())
}

fn near(ctx: &ReducerContext, a: &Player, b: &Player) -> bool {
    let t = now_us(ctx);
    a.online
        && b.online
        && a.zone_id == b.zone_id
        && match (a.entity_id.and_then(|id| position(ctx, id, t)), b.entity_id.and_then(|id| position(ctx, id, t))) {
            (Some(x), Some(y)) => distance(x, y) <= TRADE_RANGE_CM,
            _ => false,
        }
}

fn find_player(ctx: &ReducerContext, identity: Identity) -> Result<Player, String> {
    ctx.db.player().identity().find(identity).ok_or_else(|| "no such player".into())
}

pub fn expire_requests(ctx: &ReducerContext, t: i64) {
    let old: Vec<u64> = ctx.db.trade_request().iter().filter(|r| r.expires_us <= t).map(|r| r.request_id).collect();
    for id in old {
        ctx.db.trade_request().request_id().delete(id);
    }
}

/// Ask the player with this entity to trade.
#[spacetimedb::reducer]
pub fn trade_ask(ctx: &ReducerContext, target_entity_id: u64) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let target = ctx.db.player().iter().find(|q| q.entity_id == Some(target_entity_id)).ok_or("that isn't a player")?;
    if target.identity == p.identity {
        return Err("you can't trade with yourself".into());
    }
    if !near(ctx, &p, &target) {
        return Err(format!("{} is too far away", target.name));
    }
    if trade_of(ctx, p.identity).is_some() {
        return Err("you are already trading".into());
    }
    if trade_of(ctx, target.identity).is_some() {
        return Err(format!("{} is busy trading", target.name));
    }
    for old in ctx.db.trade_request().to().filter(target.identity).filter(|r| r.from == p.identity) {
        ctx.db.trade_request().request_id().delete(old.request_id);
    }
    let t = now_us(ctx);
    ctx.db.trade_request().insert(TradeRequest { request_id: 0, from: p.identity, to: target.identity, expires_us: t + REQUEST_EXPIRE_US });
    items::notify(ctx, p.identity, format!("Asked {} to trade", target.name));
    Ok(())
}

/// Accept or decline a trade request sent to us.
#[spacetimedb::reducer]
pub fn trade_answer(ctx: &ReducerContext, request_id: u64, accept: bool) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let request = ctx.db.trade_request().request_id().find(request_id).filter(|r| r.to == p.identity).ok_or("that request is gone")?;
    ctx.db.trade_request().request_id().delete(request_id);
    let from = find_player(ctx, request.from)?;
    if !accept {
        items::notify(ctx, from.identity, format!("{} doesn't want to trade", p.name));
        return Ok(());
    }
    if trade_of(ctx, p.identity).is_some() || trade_of(ctx, from.identity).is_some() {
        return Err(format!("{} is busy trading", from.name));
    }
    if !near(ctx, &p, &from) {
        return Err(format!("{} is too far away", from.name));
    }
    for r in ctx.db.trade_request().iter().filter(|r| [p.identity, from.identity].contains(&r.to) || [p.identity, from.identity].contains(&r.from)) {
        ctx.db.trade_request().request_id().delete(r.request_id);
    }
    ctx.db.trade().insert(Trade {
        trade_id: 0,
        a: from.identity,
        b: p.identity,
        a_items: "[]".into(),
        b_items: "[]".into(),
        a_money: 0,
        b_money: 0,
        a_locked: false,
        b_locked: false,
        a_accepted: false,
        b_accepted: false,
    });
    Ok(())
}

/// Check an offer against a bag and turn it into what the other side sees.
fn check_offer(inventory: &Inventory, slots: &[TradeSlot], money: i64) -> Result<Vec<OfferedItem>, String> {
    if slots.len() > MAX_TRADE_ITEMS {
        return Err(format!("you can trade at most {MAX_TRADE_ITEMS} items at once"));
    }
    if money < 0 || money > inventory.money.0 {
        return Err("you don't have that much Zuly".into());
    }
    let mut offered = Vec::new();
    for (i, s) in slots.iter().enumerate() {
        if slots[..i].iter().any(|o| o.page == s.page && o.index == s.index) {
            return Err("that item is already on the table".into());
        }
        let item = inventory.get_item(items::inventory_slot(s.page, s.index)?).ok_or("that item is gone")?;
        let mut item = item.clone();
        if let Item::Stackable(stack) = &mut item {
            if s.quantity == 0 || s.quantity > stack.quantity {
                return Err("you don't have that many".into());
            }
            stack.quantity = s.quantity;
        }
        offered.push(OfferedItem { page: s.page, index: s.index, item });
    }
    Ok(offered)
}

/// Put these bag items and Zuly on the table, replacing our earlier offer.
#[spacetimedb::reducer]
pub fn trade_offer(ctx: &ReducerContext, slots: Vec<TradeSlot>, money: i64) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let mut trade = trade_of(ctx, p.identity).ok_or("you aren't trading")?;
    let offered = serde_json::to_string(&check_offer(&p.inventory(), &slots, money)?).unwrap_or_default();
    if trade.a == p.identity {
        trade.a_items = offered;
        trade.a_money = money;
    } else {
        trade.b_items = offered;
        trade.b_money = money;
    }
    trade.a_locked = false;
    trade.b_locked = false;
    trade.a_accepted = false;
    trade.b_accepted = false;
    ctx.db.trade().trade_id().update(trade);
    Ok(())
}

/// Lock our offer (or unlock it to change it). Trade needs both sides locked.
#[spacetimedb::reducer]
pub fn trade_lock(ctx: &ReducerContext, locked: bool) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let mut trade = trade_of(ctx, p.identity).ok_or("you aren't trading")?;
    if trade.a == p.identity {
        trade.a_locked = locked;
    } else {
        trade.b_locked = locked;
    }
    if !locked {
        trade.a_accepted = false;
        trade.b_accepted = false;
    }
    ctx.db.trade().trade_id().update(trade);
    Ok(())
}

/// Agree to the locked offers; when both sides have, the items and Zuly change hands.
#[spacetimedb::reducer]
pub fn trade_accept(ctx: &ReducerContext) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let mut trade = trade_of(ctx, p.identity).ok_or("you aren't trading")?;
    if !trade.a_locked || !trade.b_locked {
        return Err("both offers must be locked first".into());
    }
    if trade.a == p.identity {
        trade.a_accepted = true;
    } else {
        trade.b_accepted = true;
    }
    if !(trade.a_accepted && trade.b_accepted) {
        ctx.db.trade().trade_id().update(trade);
        return Ok(());
    }
    let result = exchange(ctx, &trade);
    ctx.db.trade().trade_id().delete(trade.trade_id);
    let names = (find_player(ctx, trade.a).map(|p| p.name).unwrap_or_default(), find_player(ctx, trade.b).map(|p| p.name).unwrap_or_default());
    match result {
        Ok(()) => {
            items::notify(ctx, trade.a, format!("Traded with {}", names.1));
            items::notify(ctx, trade.b, format!("Traded with {}", names.0));
        }
        Err(e) => {
            items::notify(ctx, trade.a, format!("The trade failed: {e}"));
            items::notify(ctx, trade.b, format!("The trade failed: {e}"));
        }
    }
    Ok(())
}

/// Take one side's offer out of its bag, checking it is all still there.
fn take_offer(inventory: &mut Inventory, offer: &[OfferedItem], money: i64, name: &str) -> Result<Vec<Item>, String> {
    let mut taken = Vec::new();
    for o in offer {
        let slot: ItemSlot = items::inventory_slot(o.page, o.index)?;
        let still_there = match (inventory.get_item(slot), &o.item) {
            (Some(Item::Stackable(have)), Item::Stackable(want)) => have.item == want.item && have.quantity >= want.quantity,
            (Some(have), want) => have == want,
            (None, _) => false,
        };
        if !still_there {
            return Err(format!("{name} no longer has what they offered"));
        }
        taken.push(inventory.try_take_quantity(slot, o.item.get_quantity()).ok_or_else(|| format!("{name} no longer has what they offered"))?);
    }
    inventory.try_take_money(Money(money)).map_err(|_| format!("{name} no longer has that much Zuly"))?;
    Ok(taken)
}

fn give(inventory: &mut Inventory, items: Vec<Item>, money: i64, name: &str) -> Result<(), String> {
    for item in items {
        inventory.try_add_item(item).map_err(|_| format!("{name}'s bag is full"))?;
    }
    inventory.try_add_money(Money(money)).map_err(|_| format!("{name} can't carry that much Zuly"))?;
    Ok(())
}

fn exchange(ctx: &ReducerContext, trade: &Trade) -> Result<(), String> {
    let (mut a, mut b) = (find_player(ctx, trade.a)?, find_player(ctx, trade.b)?);
    if !near(ctx, &a, &b) {
        return Err("you are too far apart".into());
    }
    let a_offer: Vec<OfferedItem> = serde_json::from_str(&trade.a_items).unwrap_or_default();
    let b_offer: Vec<OfferedItem> = serde_json::from_str(&trade.b_items).unwrap_or_default();
    let (mut a_bag, mut b_bag) = (a.inventory(), b.inventory());
    let from_a = take_offer(&mut a_bag, &a_offer, trade.a_money, &a.name)?;
    let from_b = take_offer(&mut b_bag, &b_offer, trade.b_money, &b.name)?;
    give(&mut a_bag, from_b, trade.b_money, &a.name)?;
    give(&mut b_bag, from_a, trade.a_money, &b.name)?;
    a.set_inventory(&a_bag);
    b.set_inventory(&b_bag);
    ctx.db.player().identity().update(a);
    ctx.db.player().identity().update(b);
    Ok(())
}

/// Walk away from the table.
#[spacetimedb::reducer]
pub fn trade_cancel(ctx: &ReducerContext) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    cancel_for(ctx, p.identity);
    Ok(())
}

/// End any trade this player is in, telling the other side.
pub fn cancel_for(ctx: &ReducerContext, identity: Identity) {
    let Some(trade) = trade_of(ctx, identity) else { return };
    ctx.db.trade().trade_id().delete(trade.trade_id);
    let other = if trade.a == identity { trade.b } else { trade.a };
    let name = ctx.db.player().identity().find(identity).map_or_else(String::new, |p| p.name);
    items::notify(ctx, other, format!("{name} stopped trading"));
}
