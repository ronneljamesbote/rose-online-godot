//! The bank (storage): 120 slots per account, opened at a town NPC whose dialog offers it.
//! Follows rose-offline's bank_system. In iROSE the last 30 slots were premium; here
//! everyone gets all four pages.

use rose_data::{Item, ItemSlotBehaviour};
use spacetimedb::{Identity, ReducerContext, Table};

use crate::{distance, items, my_player, now_us, npcs::npc, player, position};

pub const BANK_SLOTS: usize = 30 * 4;
/// The player must stand this close to the NPC that opened the bank.
const BANK_RANGE_CM: f32 = 1500.0;

#[spacetimedb::table(accessor = bank, public)]
#[derive(Clone)]
pub struct Bank {
    #[primary_key]
    pub identity: Identity,
    /// JSON `Vec<Option<Item>>` of BANK_SLOTS slots.
    pub slots: String,
}

fn load(ctx: &ReducerContext, identity: Identity) -> Vec<Option<Item>> {
    let mut slots: Vec<Option<Item>> = ctx
        .db
        .bank()
        .identity()
        .find(identity)
        .and_then(|b| serde_json::from_str(&b.slots).ok())
        .unwrap_or_default();
    slots.resize(BANK_SLOTS, None);
    slots
}

fn save(ctx: &ReducerContext, identity: Identity, slots: &[Option<Item>]) {
    let row = Bank { identity, slots: serde_json::to_string(slots).unwrap_or_default() };
    if ctx.db.bank().identity().find(identity).is_some() {
        ctx.db.bank().identity().update(row);
    } else {
        ctx.db.bank().insert(row);
    }
}

/// Whether `item` stacks onto `onto`.
fn stacks(onto: &Item, item: &Item) -> bool {
    match item {
        Item::Stackable(item) => onto.can_stack_with(item).is_ok(),
        Item::Equipment(_) => false,
    }
}

/// Stack onto a matching item, else the first empty slot. Gives the item back when full.
fn try_add(slots: &mut [Option<Item>], item: Item) -> Result<usize, Item> {
    if let Some(i) = slots.iter().position(|s| s.as_ref().is_some_and(|s| stacks(s, &item))) {
        if slots[i].as_mut().unwrap().try_stack_with_item(item.clone()).is_ok() {
            return Ok(i);
        }
    }
    let Some(i) = slots.iter().position(|s| s.is_none()) else { return Err(item) };
    slots[i] = Some(item);
    Ok(i)
}

fn check_near(ctx: &ReducerContext, npc_entity_id: u64, entity_id: u64, zone_id: u16) -> Result<(), String> {
    let n = ctx.db.npc().entity_id().find(npc_entity_id).ok_or("no such NPC")?;
    let t = now_us(ctx);
    let (me, there) = (position(ctx, entity_id, t).ok_or("no position")?, position(ctx, npc_entity_id, t).ok_or("no position")?);
    if n.zone_id != zone_id || distance(me, there) > BANK_RANGE_CM {
        return Err("the bank is too far away".into());
    }
    Ok(())
}

/// Put `quantity` of a bag item into the bank (the whole item if it doesn't stack).
#[spacetimedb::reducer]
pub fn bank_deposit(ctx: &ReducerContext, npc_entity_id: u64, page: u8, index: u16, quantity: u32) -> Result<(), String> {
    let (mut p, id) = my_player(ctx)?;
    check_near(ctx, npc_entity_id, id, p.zone_id)?;
    let slot = items::inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let held = inventory.get_item(slot).ok_or("nothing there")?.get_quantity();
    let item = inventory.try_take_quantity(slot, quantity.clamp(1, held)).ok_or("nothing there")?;
    let mut slots = load(ctx, p.identity);
    try_add(&mut slots, item).map_err(|_| "your bank is full")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    save(ctx, p.identity, &slots);
    Ok(())
}

/// Take `quantity` of a bank item back into the bag.
#[spacetimedb::reducer]
pub fn bank_withdraw(ctx: &ReducerContext, npc_entity_id: u64, slot: u16, quantity: u32) -> Result<(), String> {
    let (mut p, id) = my_player(ctx)?;
    check_near(ctx, npc_entity_id, id, p.zone_id)?;
    let mut slots = load(ctx, p.identity);
    let bank_slot = slots.get_mut(slot as usize).ok_or("no such bank slot")?;
    let held = bank_slot.as_ref().ok_or("nothing there")?.get_quantity();
    let item = bank_slot.try_take_quantity(quantity.clamp(1, held)).ok_or("nothing there")?;
    let mut inventory = p.inventory();
    inventory.try_add_item(item).map_err(|_| "your inventory is full")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    save(ctx, p.identity, &slots);
    Ok(())
}

/// Swap two bank slots (dragging inside the bank window).
#[spacetimedb::reducer]
pub fn bank_move(ctx: &ReducerContext, from: u16, to: u16) -> Result<(), String> {
    let (p, _) = my_player(ctx)?;
    let mut slots = load(ctx, p.identity);
    let (from, to) = (from as usize, to as usize);
    if from >= BANK_SLOTS || to >= BANK_SLOTS {
        return Err("no such bank slot".into());
    }
    match (slots[from].clone(), slots[to].as_mut()) {
        (Some(a), Some(b)) if stacks(b, &a) => {
            if b.try_stack_with_item(a).is_ok() {
                slots[from] = None;
            }
        }
        _ => slots.swap(from, to),
    }
    save(ctx, p.identity, &slots);
    Ok(())
}

