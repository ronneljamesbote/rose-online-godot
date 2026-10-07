//! Items: monster drops on the ground, picking them up, the inventory, equipping, ammo and
//! using consumables. Rules follow rose-offline's npc_ai_system (drops), pickup_item_system,
//! equipment_event_system, use_ammo and use_item_system.

use rand::Rng;
use rose_data::{
    AmmoIndex, EquipmentIndex, Item, ItemClass, ItemType, JobId, NpcId, StackError, StackableSlotBehaviour,
    StatusEffectType, ZoneId,
};
use rose_game_common::components::{
    AbilityValues, DroppedItem, Equipment, InventoryPageType, ItemSlot, Money, INVENTORY_PAGE_SIZE,
};
use rose_game_data::GameData;
use spacetimedb::{Identity, ReducerContext, Table};

use crate::{
    ability, character, combat, distance, game_data::game, my_player, now_us, player, position, stats,
    world_rates_row, Player,
};

/// A drop stays on the ground this long (rose-offline's ITEM_DROP_ENTITY_EXPIRE_TIME).
const DROP_EXPIRE_US: i64 = 120_000_000;
/// Only the killer may pick a drop up for this long (ITEM_DROP_OWNER_EXPIRE_TIME).
const DROP_OWNER_US: i64 = 60_000_000;
/// Drops land up to this far from the monster (ITEM_DROP_RADIUS), in cm.
const DROP_RADIUS_CM: f32 = 200.0;
/// How close a character must be to pick something up.
const PICKUP_RANGE_CM: f32 = 400.0;

#[spacetimedb::table(accessor = ground_item, public)]
#[derive(Clone)]
pub struct GroundItem {
    #[primary_key]
    #[auto_inc]
    pub drop_id: u64,
    #[index(btree)]
    pub zone_id: u16,
    pub x: f32,
    pub y: f32,
    /// rose-game-common's DroppedItem (an item or money) as JSON.
    pub item: String,
    pub owner: Option<Identity>,
    pub owner_until_us: i64,
    pub expires_at_us: i64,
}

/// A message for one player ("Picked up 3 Arrows", "Out of arrows").
#[spacetimedb::table(accessor = notice, public, event)]
pub struct Notice {
    pub identity: Identity,
    pub text: String,
}

/// HP or MP a potion is still giving back, per second (rose-offline's StatusEffectsRegen).
#[spacetimedb::table(accessor = regen)]
#[derive(Clone)]
pub struct Regen {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub entity_id: u64,
    pub mana: bool,
    pub total: i32,
    pub per_second: i32,
    pub applied: i32,
}

pub fn notify(ctx: &ReducerContext, identity: Identity, text: impl Into<String>) {
    ctx.db.notice().insert(Notice { identity, text: text.into() });
}

fn item_name(game: &GameData, item: &Item) -> String {
    let name = game.items.get_base_item(item.get_item_reference()).map_or("an item", |d| d.name);
    match item {
        Item::Stackable(s) if s.quantity > 1 => format!("{} {name}", s.quantity),
        _ => name.to_string(),
    }
}

fn page_type(page: u8) -> Result<InventoryPageType, String> {
    Ok(match page {
        0 => InventoryPageType::Equipment,
        1 => InventoryPageType::Consumables,
        2 => InventoryPageType::Materials,
        3 => InventoryPageType::Vehicles,
        _ => return Err("no such inventory page".into()),
    })
}

pub(crate) fn inventory_slot(page: u8, index: u16) -> Result<ItemSlot, String> {
    if index as usize >= INVENTORY_PAGE_SIZE {
        return Err("no such inventory slot".into());
    }
    Ok(ItemSlot::Inventory(page_type(page)?, index as usize))
}

fn equipment_index(index: u8) -> Result<EquipmentIndex, String> {
    use EquipmentIndex::*;
    [Face, Head, Body, Back, Hands, Feet, Weapon, SubWeapon, Necklace, Ring, Earring]
        .get(index as usize)
        .copied()
        .ok_or_else(|| "no such equipment slot".into())
}

pub fn ammo_index(index: u8) -> Result<AmmoIndex, String> {
    [AmmoIndex::Arrow, AmmoIndex::Bullet, AmmoIndex::Throw]
        .get(index as usize)
        .copied()
        .ok_or_else(|| "no such ammo slot".into())
}

/// The ammo a weapon shoots, if any (rose-offline's command_system).
pub fn weapon_ammo(game: &GameData, equipment: &Equipment) -> Option<AmmoIndex> {
    let weapon = equipment.get_equipment_item(EquipmentIndex::Weapon)?;
    match game.items.get_base_item(weapon.item)?.class {
        ItemClass::Bow | ItemClass::Crossbow => Some(AmmoIndex::Arrow),
        ItemClass::Gun | ItemClass::DualGuns => Some(AmmoIndex::Bullet),
        ItemClass::Launcher => Some(AmmoIndex::Throw),
        _ => None,
    }
}

fn ammo_for_class(class: ItemClass) -> Option<AmmoIndex> {
    match class {
        ItemClass::Arrow => Some(AmmoIndex::Arrow),
        ItemClass::Bullet => Some(AmmoIndex::Bullet),
        ItemClass::Shell => Some(AmmoIndex::Throw),
        _ => None,
    }
}

pub(crate) fn player_ability_values(ctx: &ReducerContext, game: &GameData, p: &Player) -> AbilityValues {
    p.entity_id
        .and_then(|id| ctx.db.stats().entity_id().find(id))
        .and_then(|s| serde_json::from_str(&s.ability_values).ok())
        .unwrap_or_else(|| character::ability_values(game, p))
}

// ---------------------------------------------------------------- drops

fn drop_on_ground(
    ctx: &ReducerContext,
    zone_id: u16,
    at: (f32, f32),
    item: &DroppedItem,
    owner: Option<Identity>,
) {
    let mut rng = ctx.rng();
    let t = now_us(ctx);
    ctx.db.ground_item().insert(GroundItem {
        drop_id: 0,
        zone_id,
        x: at.0 + rng.gen_range(-DROP_RADIUS_CM..=DROP_RADIUS_CM),
        y: at.1 + rng.gen_range(-DROP_RADIUS_CM..=DROP_RADIUS_CM),
        item: serde_json::to_string(item).unwrap_or_default(),
        owner,
        owner_until_us: if owner.is_some() { t + DROP_OWNER_US } else { 0 },
        expires_at_us: t + DROP_EXPIRE_US,
    });
}

/// A killed monster's drop from ITEM_DROP.STB, owned by its killer for a minute.
pub fn monster_drop(ctx: &ReducerContext, game: &GameData, npc_id: u16, zone_id: u16, at: (f32, f32), killer: u64, monster_level: i32) {
    let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(killer)) else { return };
    let (Some(npc), Some(zone)) = (NpcId::new(npc_id), ZoneId::new(zone_id)) else { return };
    let av = player_ability_values(ctx, game, &p);
    let rates = world_rates_row(ctx);
    if let Some(item) = game.drop_table.get_drop(
        rates.drop_rate,
        rates.drop_money_rate,
        npc,
        zone,
        p.level as i32 - monster_level,
        av.get_drop_rate(),
        av.get_charm(),
    ) {
        drop_on_ground(ctx, zone_id, at, &item, Some(p.identity));
    }
}

/// Remove drops nobody picked up in time.
pub fn expire_drops(ctx: &ReducerContext, t: i64) {
    let old: Vec<u64> =
        ctx.db.ground_item().iter().filter(|g| g.expires_at_us <= t).map(|g| g.drop_id).collect();
    for id in old {
        ctx.db.ground_item().drop_id().delete(id);
    }
}

#[spacetimedb::reducer]
pub fn pickup_item(ctx: &ReducerContext, drop_id: u64) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let ground = ctx.db.ground_item().drop_id().find(drop_id).ok_or("it is gone")?;
    let t = now_us(ctx);
    let me = position(ctx, id, t).ok_or("no position")?;
    if ground.zone_id != p.zone_id || distance(me, (ground.x, ground.y)) > PICKUP_RANGE_CM {
        return Err("too far away".into());
    }
    if ground.owner.is_some_and(|o| o != p.identity) && t < ground.owner_until_us {
        return Err("that belongs to someone else".into());
    }
    let dropped: DroppedItem = serde_json::from_str(&ground.item).map_err(|e| e.to_string())?;
    match dropped {
        DroppedItem::Money(money) => {
            let mut inventory = p.inventory();
            inventory.try_add_money(money).map_err(|_| "you can't carry more money")?;
            p.set_inventory(&inventory);
            ctx.db.player().identity().update(p.clone());
            notify(ctx, p.identity, format!("Picked up {} Zuly", money.0));
        }
        DroppedItem::Item(item) => {
            let automatic = item.get_item_type() == ItemType::Consumable
                && game
                    .items
                    .get_consumable_item(item.get_item_number())
                    .is_some_and(|d| d.item_data.class == ItemClass::AutomaticConsumption);
            let name = item_name(&game, &item);
            if automatic {
                apply_consumable(ctx, &game, &mut p, id, item.get_item_number());
            } else {
                let mut inventory = p.inventory();
                inventory.try_add_item(item).map_err(|_| "your inventory is full")?;
                p.set_inventory(&inventory);
                ctx.db.player().identity().update(p.clone());
            }
            notify(ctx, p.identity, format!("Picked up {name}"));
        }
    }
    ctx.db.ground_item().drop_id().delete(drop_id);
    Ok(())
}

/// Drop an inventory item (or `quantity` of a stack) on the ground.
#[spacetimedb::reducer]
pub fn drop_item(ctx: &ReducerContext, page: u8, index: u16, quantity: u32) -> Result<(), String> {
    let (mut p, id) = my_player(ctx)?;
    let slot = inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let held = inventory.get_item(slot).ok_or("nothing there")?;
    let quantity = quantity.clamp(1, held.get_quantity());
    let item = inventory.try_take_quantity(slot, quantity).ok_or("nothing there")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    let at = position(ctx, id, now_us(ctx)).ok_or("no position")?;
    drop_on_ground(ctx, p.zone_id, at, &DroppedItem::Item(item), None);
    Ok(())
}

#[spacetimedb::reducer]
pub fn drop_money(ctx: &ReducerContext, amount: i64) -> Result<(), String> {
    let (mut p, id) = my_player(ctx)?;
    if amount <= 0 {
        return Err("nothing to drop".into());
    }
    let mut inventory = p.inventory();
    let money = inventory.try_take_money(Money(amount)).map_err(|_| "not enough money")?;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    let at = position(ctx, id, now_us(ctx)).ok_or("no position")?;
    drop_on_ground(ctx, p.zone_id, at, &DroppedItem::Money(money), None);
    Ok(())
}

/// Swap two slots of one inventory page (or move an item into an empty one).
#[spacetimedb::reducer]
pub fn move_item(ctx: &ReducerContext, page: u8, from: u16, to: u16) -> Result<(), String> {
    let (mut p, _) = my_player(ctx)?;
    let (a, b) = (inventory_slot(page, from)?, inventory_slot(page, to)?);
    if a == b {
        return Ok(());
    }
    let mut inventory = p.inventory();
    let first = inventory.get_item_slot_mut(a).ok_or("no such slot")?.take();
    let second = std::mem::replace(inventory.get_item_slot_mut(b).ok_or("no such slot")?, first);
    *inventory.get_item_slot_mut(a).unwrap() = second;
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p);
    Ok(())
}

// ---------------------------------------------------------------- equipment

fn meets_requirements(ctx: &ReducerContext, game: &GameData, p: &Player, item: &rose_data::BaseItemData) -> Result<(), String> {
    if let Some(job_class) = item.equip_job_class_requirement.and_then(|id| game.job_class.get(id)) {
        if !job_class.jobs.is_empty() && !job_class.jobs.contains(&JobId::new(p.job)) {
            return Err(format!("only {} can use this", job_class.name));
        }
    }
    if !item.equip_union_requirement.is_empty() {
        return Err("needs a union membership".into());
    }
    let av = player_ability_values(ctx, game, p);
    let combat = p.entity_id.and_then(|id| ctx.db.combat().entity_id().find(id));
    for &(ability_type, required) in item.equip_ability_requirement.iter() {
        let value = ability::get_value(p, combat.as_ref(), &av, ability_type).unwrap_or(0);
        if value < required as i32 {
            return Err(format!("needs {ability_type:?} {required}"));
        }
    }
    Ok(())
}

fn jewellery_index(class: ItemClass) -> Option<EquipmentIndex> {
    match class {
        ItemClass::Ring => Some(EquipmentIndex::Ring),
        ItemClass::Necklace => Some(EquipmentIndex::Necklace),
        ItemClass::Earring => Some(EquipmentIndex::Earring),
        _ => None,
    }
}

/// Equip an item from the inventory: gear goes to its equipment slot, arrows, bullets and
/// shells to their ammo slot. Whatever was there goes back to the inventory.
#[spacetimedb::reducer]
pub fn equip_item(ctx: &ReducerContext, page: u8, index: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let slot = inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let mut equipment = p.equipment();
    let item = inventory.get_item(slot).ok_or("nothing there")?.clone();
    let data = game.items.get_base_item(item.get_item_reference()).ok_or("unknown item")?;

    match item {
        Item::Stackable(stack) => {
            let ammo = ammo_for_class(data.class).ok_or("that can't be equipped")?;
            let ammo_slot = equipment.get_ammo_slot_mut(ammo);
            let inventory_slot = inventory.get_item_slot_mut(slot).unwrap();
            match ammo_slot.can_stack_with(&stack) {
                Ok(_) => {
                    ammo_slot.try_stack_with(stack).map_err(|_| "can't stack")?;
                    *inventory_slot = None;
                }
                Err(StackError::PartialStack(partial)) => {
                    let Some(Item::Stackable(held)) = inventory_slot else { unreachable!() };
                    let part = held.try_take_subquantity(partial).ok_or("can't stack")?;
                    ammo_slot.try_stack_with(part).map_err(|_| "can't stack")?;
                }
                Err(_) => {
                    let previous = ammo_slot.replace(stack);
                    *inventory_slot = previous.map(Item::Stackable);
                }
            }
        }
        Item::Equipment(gear) => {
            if gear.life == 0 {
                return Err("it is broken".into());
            }
            let target = match gear.item.item_type {
                ItemType::Face => EquipmentIndex::Face,
                ItemType::Head => EquipmentIndex::Head,
                ItemType::Body => EquipmentIndex::Body,
                ItemType::Hands => EquipmentIndex::Hands,
                ItemType::Feet => EquipmentIndex::Feet,
                ItemType::Back => EquipmentIndex::Back,
                ItemType::Jewellery => jewellery_index(data.class).ok_or("that can't be equipped")?,
                ItemType::Weapon => EquipmentIndex::Weapon,
                ItemType::SubWeapon => EquipmentIndex::SubWeapon,
                _ => return Err("that can't be equipped".into()),
            };
            meets_requirements(ctx, &game, &p, data)?;
            // A two-handed weapon takes the off-hand item off; an off-hand item can't go
            // with a two-handed weapon.
            if data.class.is_two_handed_weapon() {
                if let Some(off_hand) = equipment.get_equipment_slot_mut(EquipmentIndex::SubWeapon).take() {
                    if let Err(off_hand) = inventory.try_add_equipment_item(off_hand) {
                        *equipment.get_equipment_slot_mut(EquipmentIndex::SubWeapon) = Some(off_hand);
                        return Err("no room in the inventory for the off-hand item".into());
                    }
                }
            } else if target == EquipmentIndex::SubWeapon {
                let two_handed = equipment
                    .get_equipment_item(EquipmentIndex::Weapon)
                    .and_then(|w| game.items.get_base_item(w.item))
                    .is_some_and(|w| w.class.is_two_handed_weapon());
                if two_handed {
                    return Err("your weapon needs both hands".into());
                }
            }
            let inventory_slot = inventory.get_item_slot_mut(slot).unwrap();
            let equipment_slot = equipment.get_equipment_slot_mut(target);
            let Some(Item::Equipment(gear)) = inventory_slot.take() else { unreachable!() };
            *inventory_slot = equipment_slot.replace(gear).map(Item::Equipment);
        }
    }
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Take off an equipped item: 0 face, 1 head, 2 body, 3 back, 4 hands, 5 feet, 6 weapon,
/// 7 off-hand, 8 necklace, 9 ring, 10 earring.
#[spacetimedb::reducer]
pub fn unequip_item(ctx: &ReducerContext, equipment_slot: u8) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let index = equipment_index(equipment_slot)?;
    let mut equipment = p.equipment();
    let mut inventory = p.inventory();
    let item = equipment.get_equipment_slot_mut(index).take().ok_or("nothing equipped there")?;
    inventory.try_add_equipment_item(item).map_err(|_| "your inventory is full")?;
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Take ammo out of its slot: 0 arrows, 1 bullets, 2 shells.
#[spacetimedb::reducer]
pub fn unequip_ammo(ctx: &ReducerContext, ammo_slot: u8) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let index = ammo_index(ammo_slot)?;
    let mut equipment = p.equipment();
    let mut inventory = p.inventory();
    let item = equipment.get_ammo_slot_mut(index).take().ok_or("no ammo there")?;
    inventory.try_add_stackable_item(item).map_err(|_| "your inventory is full")?;
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Use up `count` of a player's ammo. Returns false if there isn't enough.
pub fn use_ammo(ctx: &ReducerContext, p: &mut Player, ammo: AmmoIndex, count: u32) -> bool {
    let mut equipment = p.equipment();
    if equipment.get_ammo_item(ammo).map_or(true, |a| a.quantity < count) {
        return false;
    }
    equipment.get_ammo_slot_mut(ammo).try_take_quantity(count);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    true
}

pub fn has_ammo(p: &Player, ammo: AmmoIndex, count: u32) -> bool {
    p.equipment().get_ammo_item(ammo).is_some_and(|a| a.quantity >= count)
}

// ---------------------------------------------------------------- consumables

/// A consumable's effect: a potion's HP/MP over time, or a direct ability change.
fn apply_consumable(ctx: &ReducerContext, game: &GameData, p: &mut Player, entity_id: u64, item_number: usize) -> bool {
    let Some(data) = game.items.get_consumable_item(item_number) else { return false };
    if let Some((status_effect, total)) = data.apply_status_effect {
        let Some(base) = game.status_effects.get_status_effect(status_effect) else { return false };
        let mut applied = false;
        for (effect, per_second) in base
            .apply_status_effects
            .iter()
            .filter_map(|(id, value)| game.status_effects.get_status_effect(*id).map(|d| (d, *value)))
        {
            let mana = match effect.status_effect_type {
                StatusEffectType::IncreaseHp => false,
                StatusEffectType::IncreaseMp => true,
                _ => continue, // Buff scrolls come with skills.
            };
            if per_second <= 0 {
                continue;
            }
            // A new potion of the same kind replaces the one still working (StatusEffects::can_apply).
            let old: Vec<u64> = ctx
                .db
                .regen()
                .entity_id()
                .filter(entity_id)
                .filter(|r| r.mana == mana)
                .map(|r| r.id)
                .collect();
            for id in old {
                ctx.db.regen().id().delete(id);
            }
            ctx.db.regen().insert(Regen { id: 0, entity_id, mana, total, per_second, applied: 0 });
            applied = true;
        }
        applied
    } else if let Some((ability_type, value)) = data.add_ability {
        let mut combat = ctx.db.combat().entity_id().find(entity_id);
        let changed = ability::add_value(p, combat.as_mut(), ability_type, value);
        if let Some(c) = combat {
            ctx.db.combat().entity_id().update(c);
        }
        ctx.db.player().identity().update(p.clone());
        if changed {
            character::refresh_player(ctx, game, p, false);
        }
        changed
    } else {
        false
    }
}

/// Use a consumable from the inventory (potions, food). Skill books and scrolls come with skills.
#[spacetimedb::reducer]
pub fn use_item(ctx: &ReducerContext, page: u8, index: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let slot = inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let item = inventory.get_item(slot).ok_or("nothing there")?;
    if item.get_item_type() != ItemType::Consumable {
        return Err("that can't be used".into());
    }
    let data = game.items.get_consumable_item(item.get_item_number()).ok_or("unknown item")?;
    match data.item_data.class {
        ItemClass::MagicItem | ItemClass::SkillBook | ItemClass::EngineFuel | ItemClass::RepairTool | ItemClass::TimeCoupon => {
            return Err("that can't be used yet".into());
        }
        _ => {}
    }
    if let Some((ability_type, required)) = data.ability_requirement {
        let av = player_ability_values(ctx, &game, &p);
        let combat = ctx.db.combat().entity_id().find(id);
        let value = ability::get_value(&p, combat.as_ref(), &av, ability_type).unwrap_or(0);
        if value < required {
            return Err(format!("needs {ability_type:?} {required}"));
        }
    }
    let item_number = item.get_item_number();
    inventory.try_take_quantity(slot, 1).ok_or("nothing there")?;
    p.set_inventory(&inventory);
    if !apply_consumable(ctx, &game, &mut p, id, item_number) {
        return Err("it had no effect".into());
    }
    // apply_consumable saved the player when it changed abilities; save the inventory too.
    let mut saved = ctx.db.player().identity().find(p.identity).unwrap_or(p.clone());
    saved.set_inventory(&inventory);
    ctx.db.player().identity().update(saved);
    Ok(())
}

/// One second of potion regeneration (rose-offline's status_effect_system).
pub fn regen_tick(ctx: &ReducerContext) {
    for mut r in ctx.db.regen().iter() {
        let Some(mut c) = ctx.db.combat().entity_id().find(r.entity_id) else {
            ctx.db.regen().id().delete(r.id);
            continue;
        };
        if c.dead_until_us.is_some() || c.hp <= 0 {
            ctx.db.regen().id().delete(r.id);
            continue;
        }
        let step = r.per_second.min(r.total - r.applied);
        r.applied += step;
        if r.mana {
            c.mp = (c.mp + step).min(c.max_mp);
        } else {
            c.hp = (c.hp + step).min(c.max_hp);
        }
        ctx.db.combat().entity_id().update(c);
        if r.applied >= r.total {
            ctx.db.regen().id().delete(r.id);
        } else {
            ctx.db.regen().id().update(r);
        }
    }
}

pub fn clear_regen(ctx: &ReducerContext, entity_id: u64) {
    let ids: Vec<u64> = ctx.db.regen().entity_id().filter(entity_id).map(|r| r.id).collect();
    for id in ids {
        ctx.db.regen().id().delete(id);
    }
}
