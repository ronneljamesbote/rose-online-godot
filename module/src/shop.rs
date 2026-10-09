//! Personal shops (the Vending basic action), as iROSE's private store (rose-next's
//! Recv_cli_P_STORE_OPEN, _LIST_REQ, _BUY_REQ and _CLOSE). rose-offline only lists and buys
//! from debug shops, so the rules come from rose-next.
//!
//! A player sits down with a title and up to 30 items from their bag, each with a price per
//! unit. Others see the title over them, browse the list and buy; the Zuly goes straight to
//! the owner. The shop owner can't move, fight, use skills or trade while the shop is open.
//! Standing up, dying, leaving or changing zone closes it. Shops can't open in PvP zones.
//! Items that can't be traded (item STB column 3) can't be sold. Every purchase re-checks
//! that the owner still has the item and that the buyer can pay and carry it.
//! iROSE's buy list (asking for items to buy from others) isn't here yet.

use rose_data::Item;
use rose_game_common::components::Money;
use rose_game_data::GameData;
use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

use crate::{distance, entity, game_data::game, items, my_player, now_us, player, position, pvp, sitting, skills, trade, Player};

/// MAX_P_STORE_ITEM_SLOT - 1.
pub const MAX_STORE_ITEMS: usize = 30;
/// MAX_USER_TITLE.
const MAX_TITLE_CHARS: usize = 50;
/// iROSE has no range check; this keeps buyers to those who can see the shop.
const BUY_RANGE_CM: f32 = 2000.0;
/// Prices are a u32 per unit in iROSE.
const MAX_PRICE: i64 = u32::MAX as i64;

#[spacetimedb::table(accessor = personal_store, public)]
#[derive(Clone)]
pub struct PersonalStore {
    #[primary_key]
    pub entity_id: u64,
    #[unique]
    pub owner: Identity,
    pub title: String,
    pub opened_at_us: i64,
}

/// One item for sale: the bag slot it comes from, the item as listed (JSON, with the
/// quantity for sale) and the price for each one.
#[spacetimedb::table(accessor = store_item, public)]
#[derive(Clone)]
pub struct StoreItem {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub store_entity: u64,
    pub page: u8,
    pub index: u16,
    pub item: String,
    pub price: i64,
}

/// A bag item to sell: `quantity` of a stack (ignored for equipment) at `price` each.
#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct StoreListing {
    pub page: u8,
    pub index: u16,
    pub quantity: u32,
    pub price: i64,
}

pub fn is_open(ctx: &ReducerContext, entity_id: u64) -> bool {
    ctx.db.personal_store().entity_id().find(entity_id).is_some()
}

/// Close the shop on this entity, if it has one.
pub fn close(ctx: &ReducerContext, entity_id: u64) {
    if ctx.db.personal_store().entity_id().find(entity_id).is_none() {
        return;
    }
    let rows: Vec<u64> = ctx.db.store_item().store_entity().filter(entity_id).map(|r| r.id).collect();
    for id in rows {
        ctx.db.store_item().id().delete(id);
    }
    ctx.db.personal_store().entity_id().delete(entity_id);
}

/// Whether players may trade this item with each other (not ITEM_DONT_DROP_EXCHANGE).
pub fn exchangeable(game: &GameData, item: &Item) -> bool {
    game.items.get_base_item(item.get_item_reference()).is_none_or(|data| data.trade_restriction & 0x02 == 0)
}

fn item_name(game: &GameData, item: &Item) -> String {
    game.items.get_base_item(item.get_item_reference()).map_or_else(|| "item".into(), |d| d.name.to_string())
}

/// Sit down and open a shop.
#[spacetimedb::reducer]
pub fn store_open(ctx: &ReducerContext, title: String, listings: Vec<StoreListing>) -> Result<(), String> {
    let game = game(ctx)?;
    let (p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    if let Some(reason) = skills::disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    if skills::is_casting(ctx, id) {
        return Err("you are using a skill".into());
    }
    if crate::vehicle::is_driving(ctx, id) {
        return Err("get off first".into());
    }
    if pvp::zone_pvp(&game, p.zone_id) != 0 {
        return Err("shops can't open in a PvP zone".into());
    }
    if trade::trade_of(ctx, p.identity).is_some() {
        return Err("you are trading".into());
    }
    let title: String = title.chars().filter(|c| !c.is_control()).collect();
    let title = title.trim();
    let title = if title.is_empty() { format!("{}'s shop", p.name) } else { title.to_string() };
    if title.chars().count() > MAX_TITLE_CHARS {
        return Err(format!("the title can be at most {MAX_TITLE_CHARS} characters"));
    }
    if listings.is_empty() {
        return Err("put something up for sale first".into());
    }
    if listings.len() > MAX_STORE_ITEMS {
        return Err(format!("a shop holds at most {MAX_STORE_ITEMS} items"));
    }
    let inventory = p.inventory();
    let mut rows = Vec::new();
    for (i, l) in listings.iter().enumerate() {
        if listings[..i].iter().any(|o| o.page == l.page && o.index == l.index) {
            return Err("that item is already for sale".into());
        }
        if l.price <= 0 || l.price > MAX_PRICE {
            return Err("pick a price for each item".into());
        }
        let mut item = inventory.get_item(items::inventory_slot(l.page, l.index)?).ok_or("that item is gone")?.clone();
        if !exchangeable(&game, &item) {
            return Err(format!("{} can't be sold to other players", item_name(&game, &item)));
        }
        if let Item::Stackable(stack) = &mut item {
            if l.quantity == 0 || l.quantity > stack.quantity {
                return Err("you don't have that many".into());
            }
            stack.quantity = l.quantity;
        }
        rows.push((l, serde_json::to_string(&item).unwrap_or_default()));
    }

    // Opening stops whatever we were doing and sits us down.
    let t = now_us(ctx);
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, t);
    skills::cancel_cast(ctx, id);
    close(ctx, id);
    if ctx.db.sitting().entity_id().find(id).is_none() {
        ctx.db.sitting().insert(crate::Sitting { entity_id: id, since_us: t });
    }
    ctx.db.personal_store().insert(PersonalStore { entity_id: id, owner: p.identity, title, opened_at_us: t });
    for (l, item) in rows {
        ctx.db.store_item().insert(StoreItem { id: 0, store_entity: id, page: l.page, index: l.index, item, price: l.price });
    }
    Ok(())
}

/// Close our shop (and stand up).
#[spacetimedb::reducer]
pub fn store_close(ctx: &ReducerContext) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    crate::stand_up(ctx, id);
    Ok(())
}

/// Buy `quantity` of one item from the shop on `store_entity`.
#[spacetimedb::reducer]
pub fn store_buy(ctx: &ReducerContext, store_entity: u64, store_item_id: u64, quantity: u32) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut buyer, buyer_id) = my_player(ctx)?;
    let store = ctx.db.personal_store().entity_id().find(store_entity).ok_or("that shop has closed")?;
    if store.owner == buyer.identity {
        return Err("you can't buy from your own shop".into());
    }
    if !crate::is_alive(ctx, buyer_id) {
        return Err("dead".into());
    }
    if trade::trade_of(ctx, buyer.identity).is_some() {
        return Err("you are trading".into());
    }
    let mut seller: Player = ctx.db.player().identity().find(store.owner).ok_or("that shop has closed")?;
    let t = now_us(ctx);
    let near = match (position(ctx, buyer_id, t), position(ctx, store_entity, t)) {
        (Some(a), Some(b)) => {
            ctx.db.entity().entity_id().find(store_entity).is_some_and(|e| e.zone_id == buyer.zone_id) && distance(a, b) <= BUY_RANGE_CM
        }
        _ => false,
    };
    if !near {
        return Err("that shop is too far away".into());
    }
    let mut row = ctx
        .db
        .store_item()
        .id()
        .find(store_item_id)
        .filter(|r| r.store_entity == store_entity)
        .ok_or("that item is sold out")?;
    let mut listed: Item = serde_json::from_str(&row.item).map_err(|_| "that item is sold out")?;
    let quantity = match &listed {
        Item::Stackable(s) => {
            if quantity == 0 {
                return Err("buy at least one".into());
            }
            quantity.min(s.quantity)
        }
        _ => 1,
    };
    let total = row.price.checked_mul(quantity as i64).ok_or("too expensive")?;
    let name = item_name(&game, &listed);

    // Work on copies of both bags and only save them if everything fits.
    let slot = items::inventory_slot(row.page, row.index)?;
    let mut seller_bag = seller.inventory();
    let still_there = match (seller_bag.get_item(slot), &listed) {
        (Some(Item::Stackable(have)), Item::Stackable(want)) => have.item == want.item && have.quantity >= quantity,
        (Some(have), want) => have == want,
        (None, _) => false,
    };
    if !still_there {
        ctx.db.store_item().id().delete(row.id);
        return Err("that item is sold out".into());
    }
    let mut buyer_bag = buyer.inventory();
    buyer_bag.try_take_money(Money(total)).map_err(|_| "you don't have enough Zuly")?;
    let bought = seller_bag.try_take_quantity(slot, quantity).ok_or("that item is sold out")?;
    buyer_bag.try_add_item(bought).map_err(|_| "your bag is full")?;
    seller_bag.try_add_money(Money(total)).map_err(|_| format!("{} can't carry more Zuly", seller.name))?;
    seller.set_inventory(&seller_bag);
    buyer.set_inventory(&buyer_bag);
    ctx.db.player().identity().update(seller.clone());
    ctx.db.player().identity().update(buyer.clone());

    // What is left for sale.
    let left = match &mut listed {
        Item::Stackable(s) => {
            s.quantity -= quantity;
            s.quantity
        }
        _ => 0,
    };
    if left == 0 {
        ctx.db.store_item().id().delete(row.id);
    } else {
        row.item = serde_json::to_string(&listed).unwrap_or_default();
        ctx.db.store_item().id().update(row);
    }
    let what = if quantity > 1 { format!("{quantity} {name}") } else { name };
    items::notify(ctx, buyer.identity, format!("Bought {what} from {} for {total} Zuly", seller.name));
    items::notify(ctx, seller.identity, format!("{} bought {what} for {total} Zuly", buyer.name));
    if ctx.db.store_item().store_entity().filter(store_entity).next().is_none() {
        items::notify(ctx, seller.identity, "Your shop sold out");
        crate::stand_up(ctx, store_entity);
    }
    Ok(())
}
