//! Carts and castle gear (iROSE's PAT items), following rose-next's SetCMD_TOGGLE,
//! Dec_EngineLife and Check_PerFRAME, with rose-offline's vehicle stats.
//!
//! Parts go in four slots: body, engine, legs (wheels) and arms (accessory or weapon).
//! Driving needs a body, an engine and legs, and is a toggle (the Drive Cart action).
//! Getting on or off ends your buffs. While driving, the vehicle's stats replace yours
//! (speed from the engine and legs, attack from the arms), HP and MP don't recover, and
//! you can't sit, open a shop or use most skills; vehicle skills need the right body.
//! The engine's life is its fuel: getting on, each attack and every 10 seconds use the
//! engine's fuel rate. With no fuel left the vehicle stops and can't move or attack until
//! refuelled with Engine Fuel. Zones can forbid carts or castle gear (LIST_ZONE column 30).
//! Dying or leaving gets you off. Passengers (a second seat) aren't here yet.

use rose_data::{EquipmentItem, Item, ItemType, SkillType, VehiclePartIndex, VehicleType};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::{character, game_data::game, items, my_player, now_us, player, Player};

/// GameStaticConfig::FUEL_DECREASE_TIME.
const FUEL_TICK_US: i64 = 10_000_000;
/// An engine's life (fuel) when full.
pub const MAX_FUEL: u16 = 1000;

#[spacetimedb::table(accessor = driving, public)]
#[derive(Clone)]
pub struct Driving {
    #[primary_key]
    pub entity_id: u64,
    pub since_us: i64,
    pub next_fuel_us: i64,
}

pub fn is_driving(ctx: &ReducerContext, entity_id: u64) -> bool {
    ctx.db.driving().entity_id().find(entity_id).is_some()
}

pub fn engine_life(p: &Player) -> i32 {
    p.equipment().get_vehicle_item(VehiclePartIndex::Engine).map_or(0, |e| e.life as i32)
}

/// Use `amount` of fuel (the engine's own rate when None). Returns the fuel left.
pub fn use_fuel(ctx: &ReducerContext, game: &GameData, p: &mut Player, amount: Option<i32>) -> i32 {
    let mut equipment = p.equipment();
    let Some(engine) = equipment.get_vehicle_item_mut(VehiclePartIndex::Engine) else { return 0 };
    let rate = amount.unwrap_or_else(|| game.items.get_vehicle_item(engine.item.item_number).map_or(0, |v| v.fuel_use_rate as i32));
    if rate <= 0 {
        return engine.life as i32;
    }
    engine.life = (engine.life as i32 - rate).max(0) as u16;
    let left = engine.life as i32;
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    if left == 0 {
        if let Some(id) = p.entity_id {
            crate::stop_motion(ctx, id);
            crate::cancel_attack(ctx, id, now_us(ctx));
        }
        items::notify(ctx, p.identity, "Out of fuel");
    }
    left
}

fn vehicle_type(game: &GameData, p: &Player) -> Option<VehicleType> {
    let body = p.equipment().get_vehicle_item(VehiclePartIndex::Body)?.item.item_number;
    game.items.get_vehicle_item(body).map(|v| v.vehicle_type)
}

/// Get on or off.
#[spacetimedb::reducer]
pub fn drive_toggle(ctx: &ReducerContext) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if is_driving(ctx, id) {
        get_off(ctx, &game, id);
        return Ok(());
    }
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    if let Some(reason) = crate::skills::disabled_reason(ctx, id) {
        return Err(reason.into());
    }
    if crate::shop::is_open(ctx, id) {
        return Err("close your shop first".into());
    }
    let equipment = p.equipment();
    let kind = vehicle_type(&game, &p).ok_or("put a body part on first")?;
    for (part, name) in [(VehiclePartIndex::Engine, "an engine"), (VehiclePartIndex::Leg, "legs or wheels")] {
        if equipment.get_vehicle_item(part).is_none() {
            return Err(format!("it needs {name}"));
        }
    }
    let flags = rose_data::ZoneId::new(p.zone_id).and_then(|z| game.zones.get_zone(z)).map_or(0, |z| z.vehicle_use_flags);
    let bit = match kind {
        VehicleType::Cart => 1,
        VehicleType::CastleGear => 2,
    };
    if flags & bit != 0 {
        return Err(format!("{} can't be used here", if bit == 1 { "Carts" } else { "Castle gear" }));
    }
    let t = now_us(ctx);
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, t);
    crate::skills::cancel_cast(ctx, id);
    crate::stand_up(ctx, id);
    crate::skills::clear_good(ctx, &game, id);
    ctx.db.driving().insert(Driving { entity_id: id, since_us: t, next_fuel_us: t + FUEL_TICK_US });
    use_fuel(ctx, &game, &mut p, None);
    let p = ctx.db.player().identity().find(p.identity).unwrap_or(p);
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Get off (also on death, logout and zone changes).
pub fn get_off(ctx: &ReducerContext, game: &GameData, id: u64) {
    if ctx.db.driving().entity_id().find(id).is_none() {
        return;
    }
    ctx.db.driving().entity_id().delete(id);
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, now_us(ctx));
    crate::skills::clear_good(ctx, game, id);
    if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) {
        character::refresh_player(ctx, game, &p, false);
    }
}

/// Drop the driving row of an entity that is going away.
pub fn forget(ctx: &ReducerContext, id: u64) {
    ctx.db.driving().entity_id().delete(id);
}

/// Every 10 seconds of driving uses fuel.
pub fn tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let due: Vec<Driving> = ctx.db.driving().iter().filter(|d| d.next_fuel_us <= t).collect();
    for mut d in due {
        let Some(mut p) = ctx.db.player().iter().find(|p| p.entity_id == Some(d.entity_id)) else {
            ctx.db.driving().entity_id().delete(d.entity_id);
            continue;
        };
        d.next_fuel_us += FUEL_TICK_US;
        ctx.db.driving().entity_id().update(d);
        use_fuel(ctx, game, &mut p, None);
    }
}

/// Engine Fuel: adds its value x10 to the engine's life, up to full (use_item_system).
pub fn refuel(p: &mut Player, add_fuel: i32) -> Result<(), String> {
    let mut equipment = p.equipment();
    let engine = equipment.get_vehicle_item_mut(VehiclePartIndex::Engine).ok_or("there is no engine to fill")?;
    if engine.life >= MAX_FUEL {
        return Err("the tank is full".into());
    }
    engine.life = (engine.life as i32 + add_fuel * 10).clamp(0, MAX_FUEL as i32) as u16;
    p.set_equipment(&equipment);
    Ok(())
}

/// What a skill needs while driving (rose-next's Do_SelfSKILL/Do_TargetSKILL checks and
/// CUserDATA::Skill_CheckNeedEQUIP for vehicle bodies). None if it may be used.
pub fn skill_refusal(ctx: &ReducerContext, game: &GameData, p: &Player, id: u64, skill: &rose_data::SkillData) -> Option<String> {
    let body_class = p
        .equipment()
        .get_vehicle_item(VehiclePartIndex::Body)
        .and_then(|b| game.items.get_base_item(b.item))
        .map(|d| d.class);
    let needs_vehicle = skill
        .required_equipment_class
        .iter()
        .any(|c| matches!(c, rose_data::ItemClass::CartBody | rose_data::ItemClass::CastleGearBody));
    let name = if skill.name.is_empty() { "That skill" } else { &skill.name };
    if !is_driving(ctx, id) {
        return needs_vehicle.then(|| format!("{name} is used while driving"));
    }
    if matches!(
        skill.skill_type,
        SkillType::Immediate
            | SkillType::EnforceWeapon
            | SkillType::EnforceBullet
            | SkillType::FireBullet
            | SkillType::AreaTarget
            | SkillType::SelfDamage
    ) && !needs_vehicle
    {
        return Some("you can't use that while driving".into());
    }
    if needs_vehicle && !skill.required_equipment_class.iter().any(|&c| Some(c) == body_class) {
        return Some(format!("{name} needs another vehicle"));
    }
    None
}

/// Put a vehicle part from the bag into its slot (equip_vehicle_from_inventory).
pub fn equip_part(ctx: &ReducerContext, game: &GameData, p: &mut Player, page: u8, index: u16) -> Result<(), String> {
    if p.entity_id.is_some_and(|id| is_driving(ctx, id)) {
        return Err("get off first".into());
    }
    let slot = items::inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let Some(Item::Equipment(part)) = inventory.get_item(slot).cloned() else { return Err("that can't be equipped".into()) };
    if part.item.item_type != ItemType::Vehicle {
        return Err("that isn't a vehicle part".into());
    }
    let data = game.items.get_vehicle_item(part.item.item_number).ok_or("unknown item")?;
    if part.life == 0 && data.vehicle_part != VehiclePartIndex::Engine {
        return Err("it is broken".into());
    }
    items::check_requirements(ctx, game, p, &data.item_data)?;
    // Some parts need a skill (LIST_PAT.STB), at least at the given level.
    if let Some((skill_id, level)) = data.equip_skill_requirement {
        let known = p.skill_list().find_skill_level(&game.skills, skill_id).map_or(0, |(_, _, l)| l as i32);
        if known < level.max(1) {
            let name = game.skills.get_skill(skill_id).map_or_else(|| "a skill".to_string(), |s| s.name.to_string());
            return Err(format!("that part needs {name}"));
        }
    }
    // A cart body takes cart parts, castle gear takes castle gear parts.
    let body_type = vehicle_type(game, p);
    if data.vehicle_part != VehiclePartIndex::Body && body_type.is_some_and(|t| t != data.vehicle_type) {
        return Err("that part is for another kind of vehicle".into());
    }
    let mut equipment = p.equipment();
    let inventory_slot = inventory.get_item_slot_mut(slot).unwrap();
    let Some(Item::Equipment(part)) = inventory_slot.take() else { unreachable!() };
    *inventory_slot = equipment.get_vehicle_slot_mut(data.vehicle_part).replace(part).map(Item::Equipment);
    // Parts of the other kind come off with a new body.
    if data.vehicle_part == VehiclePartIndex::Body {
        for other in [VehiclePartIndex::Engine, VehiclePartIndex::Leg, VehiclePartIndex::Arms] {
            let mismatched = equipment
                .get_vehicle_item(other)
                .and_then(|e| game.items.get_vehicle_item(e.item.item_number))
                .is_some_and(|v| v.vehicle_type != data.vehicle_type);
            if mismatched {
                let item = equipment.get_vehicle_slot_mut(other).take().unwrap();
                if let Err(item) = inventory.try_add_equipment_item(item) {
                    *equipment.get_vehicle_slot_mut(other) = Some(item);
                }
            }
        }
    }
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    Ok(())
}

/// Take off a vehicle part: 0 body, 1 engine, 2 legs, 3 arms.
#[spacetimedb::reducer]
pub fn unequip_vehicle_part(ctx: &ReducerContext, part: u8) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if is_driving(ctx, id) {
        return Err("get off first".into());
    }
    let index = [VehiclePartIndex::Body, VehiclePartIndex::Engine, VehiclePartIndex::Leg, VehiclePartIndex::Arms]
        .get(part as usize)
        .copied()
        .ok_or("no such part")?;
    let mut equipment = p.equipment();
    let mut inventory = p.inventory();
    let item: EquipmentItem = equipment.get_vehicle_slot_mut(index).take().ok_or("nothing there")?;
    inventory.try_add_equipment_item(item).map_err(|_| "your inventory is full")?;
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    Ok(())
}
