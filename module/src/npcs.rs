//! Town NPCs from the zone files, and buying from and selling to their stores. Rules follow
//! rose-offline's startup_zones_system (NPC spawns) and npc_store_system.

use rose_data::{Item, NpcId};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

use crate::{
    distance, entity, game_data::game, items, motion, my_player, now_us, player, position, world_rates_row, Entity,
    EntityKind, Motion,
};

/// How close a character must be to trade with a store (NPC_STORE_TRANSACTION_MAX_DISTANCE).
const STORE_RANGE_CM: f32 = 6000.0;

/// A town NPC: the entity's standing direction, its conversation (quest script) and
/// whether it has a store.
#[spacetimedb::table(accessor = npc, public)]
#[derive(Clone)]
pub struct Npc {
    #[primary_key]
    pub entity_id: u64,
    #[index(btree)]
    pub zone_id: u16,
    pub npc_id: u16,
    /// Facing in degrees about the up axis.
    pub direction: f32,
    pub conversation: String,
    pub has_store: bool,
}

/// Buy `quantity` of the item at `index` of the NPC's store tab `tab` (gear is always 1).
#[derive(SpacetimeType, Clone, Debug)]
pub struct StoreBuy {
    pub tab: u8,
    pub index: u16,
    pub quantity: u32,
}

/// Sell `quantity` of the item in an inventory slot.
#[derive(SpacetimeType, Clone, Debug)]
pub struct StoreSell {
    pub page: u8,
    pub index: u16,
    pub quantity: u32,
}

/// Replace every town NPC with the ones the zone files place.
pub fn spawn_npcs(ctx: &ReducerContext, game: &GameData) {
    let old: Vec<u64> = ctx.db.entity().iter().filter(|e| e.kind == EntityKind::Npc).map(|e| e.entity_id).collect();
    for id in old {
        crate::despawn(ctx, id);
    }
    let t = now_us(ctx);
    for zone in game.zones.iter() {
        for spawn in zone.npcs.iter() {
            let Some(data) = game.npcs.get_npc(spawn.npc_id) else { continue };
            let entity = ctx.db.entity().insert(Entity {
                entity_id: 0,
                kind: EntityKind::Npc,
                npc_id: spawn.npc_id.get(),
                zone_id: zone.id.get(),
                name: data.name.to_string(),
            });
            let (x, y) = (spawn.position.x, spawn.position.y);
            ctx.db.motion().insert(Motion {
                entity_id: entity.entity_id,
                from_x: x,
                from_y: y,
                to_x: x,
                to_y: y,
                started_at_us: t,
                speed: 0.0,
                chase_target: None,
            });
            ctx.db.npc().insert(Npc {
                entity_id: entity.entity_id,
                zone_id: zone.id.get(),
                npc_id: spawn.npc_id.get(),
                direction: spawn.direction,
                conversation: spawn.conversation.get().to_string(),
                has_store: data.store_tabs.iter().any(|t| t.is_some()),
            });
        }
    }
}

/// Sell, then buy, in one go: either all of it happens or none (rose-offline's
/// npc_store_do_transaction). Prices use the character's store rates and the world rates.
#[spacetimedb::reducer]
pub fn npc_store_transaction(
    ctx: &ReducerContext,
    npc_entity_id: u64,
    buy: Vec<StoreBuy>,
    sell: Vec<StoreSell>,
) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let npc = ctx.db.npc().entity_id().find(npc_entity_id).ok_or("no such store")?;
    let data = NpcId::new(npc.npc_id).and_then(|n| game.npcs.get_npc(n)).ok_or("no such store")?;
    if data.store_union_number.is_some() {
        return Err("only union members can trade here".into());
    }
    let t = now_us(ctx);
    let (me, there) = (position(ctx, id, t).ok_or("no position")?, position(ctx, npc_entity_id, t).ok_or("no position")?);
    if npc.zone_id != p.zone_id || distance(me, there) > STORE_RANGE_CM {
        return Err("the store is too far away".into());
    }

    let av = items::player_ability_values(ctx, &game, &p);
    let rates = world_rates_row(ctx);
    let mut inventory = p.inventory();
    let mut sell_value = 0i64;
    let mut buy_cost = 0i64;

    for s in sell.iter() {
        let slot = items::inventory_slot(s.page, s.index)?;
        let held = inventory.get_item(slot).ok_or("nothing to sell there")?.get_quantity();
        let item = inventory.try_take_quantity(slot, s.quantity.clamp(1, held)).ok_or("nothing to sell there")?;
        let price = rose_game_irose::data::npc_store_sell_price(
            &game.items,
            &item,
            av.get_npc_store_sell_rate(),
            rates.world_price_rate,
            rates.item_price_rate,
            rates.town_price_rate,
        )
        .ok_or("the store won't buy that")?;
        sell_value += price.max(0) as i64 * item.get_quantity() as i64;
    }

    for b in buy.iter() {
        let tab = data.store_tabs.get(b.tab as usize).copied().flatten().ok_or("no such store tab")?;
        let reference = *game.npcs.get_store_tab(tab).and_then(|t| t.items.get(&b.index)).ok_or("no such item")?;
        let item_data = game.items.get_base_item(reference).ok_or("unknown item")?;
        let price = rose_game_irose::data::npc_store_buy_price(
            &game.items,
            reference,
            av.get_npc_store_buy_rate(),
            rates.item_price_rate,
            rates.town_price_rate,
        )
        .ok_or("that isn't for sale")?;
        let quantity = if reference.item_type.is_stackable_item() { b.quantity.clamp(1, 999) } else { 1 };
        let item = Item::from_item_data(item_data, quantity).ok_or("unknown item")?;
        inventory.try_add_item(item).map_err(|_| "your inventory is full")?;
        buy_cost += price.max(0) as i64 * quantity as i64;
    }

    inventory.try_add_money(rose_game_common::components::Money(sell_value)).map_err(|_| "you can't carry more money")?;
    inventory.try_take_money(rose_game_common::components::Money(buy_cost)).map_err(|_| "not enough Zuly")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    let text = match (buy_cost, sell_value) {
        (0, v) => format!("Sold for {v} Zuly"),
        (c, 0) => format!("Bought for {c} Zuly"),
        (c, v) => format!("Bought for {c} and sold for {v} Zuly"),
    };
    items::notify(ctx, p.identity, text);
    Ok(())
}
