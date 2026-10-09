//! Item wear and repair (iROSE's Dec_WeaponLife, Dec_ItemLife, Recv_cli_REPAIR_FROM_NPC and
//! Recv_cli_USE_ITEM_TO_REPAIR, via rose-offline's calculators).
//!
//! An equipment item's life (0-1000) is its condition and its durability (0-100) how fast
//! that goes down. Every swing or damaging skill may take 1 life from the weapon (the arms
//! part when driving), every hit taken may take 1 from one armour piece (vehicle parts
//! when driving). At life 0 the item gives no stats until repaired. NPCs repair for Zuly;
//! a repair hammer repairs anywhere but may cost durability. Engines are refuelled, never
//! repaired, and an item at durability 0 can't be repaired.

use rose_data::{ItemClass, ItemType, VehiclePartIndex};
use rose_data::EquipmentItem;
use rose_game_common::components::{Equipment, InventoryPageType, ItemSlot, Money};
use rose_game_common::data::Damage;
use rose_game_data::GameData;
use spacetimedb::{rand::Rng, ReducerContext, Table};

use crate::{character, distance, items, my_player, now_us, npc, player, position, Player};

const MAX_LIFE: u16 = 1000;

fn player_of(ctx: &ReducerContext, id: u64) -> Option<Player> {
    ctx.db.player().iter().find(|p| p.entity_id == Some(id))
}

fn item_mut(equipment: &mut Equipment, slot: ItemSlot) -> Option<&mut EquipmentItem> {
    match slot {
        ItemSlot::Equipment(index) => equipment.get_equipment_item_mut(index),
        ItemSlot::Vehicle(index) => equipment.get_vehicle_item_mut(index),
        _ => None,
    }
}

/// Take one life from the item in this slot; at 0 it breaks and the stats are refreshed.
fn wear(ctx: &ReducerContext, game: &GameData, mut p: Player, slot: ItemSlot) {
    let mut equipment = p.equipment();
    let Some(item) = item_mut(&mut equipment, slot) else { return };
    if item.life == 0 {
        return;
    }
    item.life -= 1;
    let broke = item.life == 0;
    let name = game.items.get_base_item(item.item).map_or_else(String::new, |d| d.name.to_string());
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    if broke {
        character::refresh_player(ctx, game, &p, false);
        items::notify(ctx, p.identity, format!("Your {name} broke (repair it to use it again)"));
    }
}

/// The attacker swung or used a damaging skill.
pub fn weapon_used(ctx: &ReducerContext, game: &GameData, attacker: u64) {
    let Some(p) = player_of(ctx, attacker) else { return };
    let driving = crate::vehicle::is_driving(ctx, attacker);
    if let Some(slot) = game.ability_value_calculator.calculate_decrease_weapon_life(driving, &p.equipment()) {
        wear(ctx, game, p, slot);
    }
}

/// The defender took a hit of this size.
pub fn hit_taken(ctx: &ReducerContext, game: &GameData, defender: u64, amount: i32) {
    if amount <= 0 {
        return;
    }
    let Some(p) = player_of(ctx, defender) else { return };
    let driving = crate::vehicle::is_driving(ctx, defender);
    let damage = Damage { amount: amount as u32, is_critical: false, apply_hit_stun: false };
    if let Some(slot) = game.ability_value_calculator.calculate_decrease_armour_life(driving, &p.equipment(), &damage) {
        wear(ctx, game, p, slot);
    }
}

/// Where a repair target is: worn gear (equipment or vehicle slot), or the bag. Reducers
/// take it as (kind 0 equipped / 1 vehicle / 2 bag, slot or page, index in the page).
#[derive(Clone, Copy)]
enum RepairTarget {
    Equipped(u8),
    Vehicle(u8),
    Bag(u8, u16),
}

fn target_of(kind: u8, a: u8, b: u16) -> Result<RepairTarget, String> {
    match kind {
        0 => Ok(RepairTarget::Equipped(a)),
        1 => Ok(RepairTarget::Vehicle(a)),
        2 => Ok(RepairTarget::Bag(a, b)),
        _ => Err("no such slot".into()),
    }
}

fn slot_of(target: RepairTarget) -> Result<ItemSlot, String> {
    use rose_data::EquipmentIndex as E;
    Ok(match target {
        RepairTarget::Equipped(i) => ItemSlot::Equipment(
            [E::Face, E::Head, E::Body, E::Back, E::Hands, E::Feet, E::Weapon, E::SubWeapon, E::Necklace, E::Ring, E::Earring]
                .get(i as usize)
                .copied()
                .ok_or("no such slot")?,
        ),
        RepairTarget::Vehicle(i) => ItemSlot::Vehicle(
            [VehiclePartIndex::Body, VehiclePartIndex::Engine, VehiclePartIndex::Leg, VehiclePartIndex::Arms]
                .get(i as usize)
                .copied()
                .ok_or("no such slot")?,
        ),
        RepairTarget::Bag(page, index) => {
            let page = [InventoryPageType::Equipment, InventoryPageType::Consumables, InventoryPageType::Materials, InventoryPageType::Vehicles]
                .get(page as usize)
                .copied()
                .ok_or("no such page")?;
            ItemSlot::Inventory(page, index as usize)
        }
    })
}

/// Run `f` on the repair target (an equipment item in any slot), then save the player.
fn with_target<R>(
    p: &mut Player,
    target: RepairTarget,
    f: impl FnOnce(&mut EquipmentItem) -> Result<R, String>,
) -> Result<R, String> {
    let slot = slot_of(target)?;
    if let ItemSlot::Inventory(..) = slot {
        let mut inventory = p.inventory();
        let item = match inventory.get_item_mut(slot) {
            Some(rose_data::Item::Equipment(e)) => e,
            _ => return Err("that can't be repaired".into()),
        };
        let r = f(item)?;
        p.set_inventory(&inventory);
        Ok(r)
    } else {
        let mut equipment = p.equipment();
        let item = item_mut(&mut equipment, slot).ok_or("nothing there")?;
        let r = f(item)?;
        p.set_equipment(&equipment);
        Ok(r)
    }
}

fn check_repairable(game: &GameData, item: &EquipmentItem) -> Result<String, String> {
    let name = game.items.get_base_item(item.item).map_or_else(String::new, |d| d.name.to_string());
    if item.item.item_type == ItemType::Vehicle
        && game.items.get_vehicle_item(item.item.item_number).is_some_and(|v| v.vehicle_part == VehiclePartIndex::Engine)
    {
        return Err("engines are refuelled, not repaired".into());
    }
    if item.durability == 0 {
        return Err(format!("{name} has no durability left and can't be repaired"));
    }
    if item.life >= MAX_LIFE {
        return Err(format!("{name} doesn't need repairing"));
    }
    Ok(name)
}

/// What a repair at an NPC costs (Recv_cli_REPAIR_FROM_NPC).
pub fn npc_price(game: &GameData, item: &EquipmentItem) -> i64 {
    game.ability_value_calculator.calculate_repair_from_npc_price(item).0.max(1)
}

/// Repair at an NPC whose dialog offers it (GF_repair); costs Zuly.
#[spacetimedb::reducer]
pub fn repair_at_npc(ctx: &ReducerContext, npc_entity: u64, kind: u8, slot: u8, index: u16) -> Result<(), String> {
    let target = target_of(kind, slot, index)?;
    let game = crate::game_data::game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    let n = ctx.db.npc().entity_id().find(npc_entity).ok_or("no such NPC")?;
    let t = now_us(ctx);
    let (me, there) = (position(ctx, id, t).ok_or("no position")?, position(ctx, npc_entity, t).ok_or("no position")?);
    if n.zone_id != p.zone_id || distance(me, there) > crate::craft::NPC_RANGE_CM {
        return Err("you are too far away".into());
    }
    let (name, price) = with_target(&mut p, target, |item| {
        let name = check_repairable(&game, item)?;
        Ok((name, npc_price(&game, item)))
    })?;
    let mut inventory = p.inventory();
    if inventory.money.0 < price {
        return Err(format!("repairing {name} costs {price} Zuly"));
    }
    inventory.money = Money(inventory.money.0 - price);
    p.set_inventory(&inventory);
    with_target(&mut p, target, |item| {
        item.life = MAX_LIFE;
        Ok(())
    })?;
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    items::notify(ctx, p.identity, format!("Repaired {name} for {price} Zuly"));
    Ok(())
}

/// Repair with a hammer from the bag (Recv_cli_USE_ITEM_TO_REPAIR). Hammers whose data value
/// is 0 may lower the durability, more the more worn the item was; the others never do.
#[spacetimedb::reducer]
pub fn repair_with_item(ctx: &ReducerContext, tool_page: u8, tool_index: u16, kind: u8, slot: u8, index: u16) -> Result<(), String> {
    let target = target_of(kind, slot, index)?;
    let game = crate::game_data::game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let tool_slot = slot_of(RepairTarget::Bag(tool_page, tool_index))?;
    let inventory = p.inventory();
    let tool = match inventory.get_item(tool_slot) {
        Some(rose_data::Item::Stackable(s)) => s.item,
        _ => return Err("that isn't a repair tool".into()),
    };
    let data = game.items.get_consumable_item(tool.item_number).filter(|_| tool.item_type == ItemType::Consumable);
    let Some(data) = data.filter(|d| d.item_data.class == ItemClass::RepairTool) else {
        return Err("that isn't a repair tool".into());
    };
    let perfect = data.add_data_value != 0;
    let mut rng = ctx.rng();
    let (name, lost) = with_target(&mut p, target, |item| {
        let name = check_repairable(&game, item)?;
        let mut lost = 0;
        if !perfect {
            let dec = (1400 - item.life as i32) * (rng.gen_range(0..100) + 11) / (item.durability as i32 + 40) / 400;
            lost = (dec.max(0) as u8).min(item.durability);
            item.durability -= lost;
        }
        item.life = MAX_LIFE;
        Ok((name, lost))
    })?;
    let mut inventory = p.inventory();
    inventory.try_take_quantity(tool_slot, 1).ok_or("the tool is gone")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    let text = if lost > 0 { format!("Repaired {name} (durability -{lost})") } else { format!("Repaired {name}") };
    items::notify(ctx, p.identity, text);
    Ok(())
}

/// Debug: set the life of a player's item (target as in repair_at_npc), for testing wear.
#[spacetimedb::reducer]
pub fn admin_set_item_life(ctx: &ReducerContext, player_name: String, kind: u8, slot: u8, index: u16, life: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = crate::game_data::game(ctx)?;
    let target = target_of(kind, slot, index)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == player_name).ok_or("no such player")?;
    with_target(&mut p, target, |item| {
        item.life = life.min(MAX_LIFE);
        Ok(())
    })?;
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}
