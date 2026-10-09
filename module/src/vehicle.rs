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
//! Dying or leaving gets you off.
//!
//! Passengers (rose-next's Recv_cli_CART_RIDE): a driver whose vehicle has a second seat (an
//! arms part with a chair, LIST_PAT ability type 1) can offer a nearby player a ride. They
//! accept or decline (an unanswered offer lapses after 30 seconds). The passenger sits on the
//! back seat, goes where the vehicle goes and can't move, fight, use skills or open a shop;
//! they can get off at any time. Both get off when the driver gets off, dies, leaves or
//! changes zone; a driver who warps takes the passenger along, who then gets off.

use rose_data::{EquipmentItem, Item, ItemType, SkillType, VehiclePartIndex, VehicleType};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::{character, entity, motion, game_data::game, items, my_player, now_us, player, EntityKind, Player};

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
    if let Some(reason) = crate::skills::disabled_reason(ctx, id).or(passenger_refusal(ctx, id)) {
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
    drop_passenger(ctx, id);
    ctx.db.driving().entity_id().delete(id);
    crate::stop_motion(ctx, id);
    crate::cancel_attack(ctx, id, now_us(ctx));
    crate::skills::clear_good(ctx, game, id);
    if let Some(p) = ctx.db.player().iter().find(|p| p.entity_id == Some(id)) {
        character::refresh_player(ctx, game, &p, false);
    }
}

/// Drop the driving and riding rows of an entity that is going away.
pub fn forget(ctx: &ReducerContext, id: u64) {
    drop_passenger(ctx, id);
    leave_seat(ctx, id);
    let invites: Vec<u64> = ctx.db.ride_offer().iter().filter(|o| o.driver == id || o.guest == id).map(|o| o.id).collect();
    for invite in invites {
        ctx.db.ride_offer().id().delete(invite);
    }
    ctx.db.driving().entity_id().delete(id);
}

/// Every 10 seconds of driving uses fuel.
pub fn tick(ctx: &ReducerContext, game: &GameData, t: i64) {
    let lapsed: Vec<u64> = ctx.db.ride_offer().iter().filter(|o| o.expires_us <= t).map(|o| o.id).collect();
    for id in lapsed {
        ctx.db.ride_offer().id().delete(id);
    }
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

// ---------------------------------------------------------------- passengers

/// An offer from a driver to take a player along (CART_RIDE_REQ).
const RIDE_OFFER_US: i64 = 30_000_000;
/// How close the passenger must be. rose-next uses Ride Request's scope (skill 25), which is
/// empty in this client's data; this is our own choice.
const RIDE_RANGE_CM: f32 = 600.0;

/// A passenger on the back seat of `driver`'s vehicle.
#[spacetimedb::table(accessor = passenger, public)]
#[derive(Clone)]
pub struct Passenger {
    #[primary_key]
    pub guest: u64,
    #[unique]
    pub driver: u64,
    pub since_us: i64,
}

#[spacetimedb::table(accessor = ride_offer, public)]
#[derive(Clone)]
pub struct RideOffer {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub driver: u64,
    pub guest: u64,
    pub expires_us: i64,
}

pub fn is_passenger(ctx: &ReducerContext, id: u64) -> bool {
    ctx.db.passenger().guest().find(id).is_some()
}

/// Why a passenger can't do something, or None for everyone else.
pub fn passenger_refusal(ctx: &ReducerContext, id: u64) -> Option<&'static str> {
    is_passenger(ctx, id).then_some("you are riding as a passenger (get off first)")
}

fn has_seat(game: &GameData, p: &Player) -> bool {
    p.equipment()
        .get_vehicle_item(VehiclePartIndex::Arms)
        .and_then(|a| game.items.get_vehicle_item(a.item.item_number))
        .is_some_and(|v| v.has_seat)
}

fn player_of(ctx: &ReducerContext, id: u64) -> Option<Player> {
    ctx.db.player().iter().find(|p| p.entity_id == Some(id))
}

/// The passenger goes where the driver goes: same path, same speed.
pub fn sync_passenger(ctx: &ReducerContext, driver: u64) {
    let Some(ride) = ctx.db.passenger().driver().find(driver) else { return };
    let Some(m) = ctx.db.motion().entity_id().find(driver) else { return };
    let row = crate::Motion { entity_id: ride.guest, chase_target: None, ..m };
    if ctx.db.motion().entity_id().find(ride.guest).is_some() {
        ctx.db.motion().entity_id().update(row);
    } else {
        ctx.db.motion().insert(row);
    }
}

/// The driver's passenger gets off where they are.
fn drop_passenger(ctx: &ReducerContext, driver: u64) {
    if let Some(ride) = ctx.db.passenger().driver().find(driver) {
        leave_seat(ctx, ride.guest);
    }
}

/// Get off the back seat (nothing if not riding).
pub fn leave_seat(ctx: &ReducerContext, guest: u64) {
    let Some(ride) = ctx.db.passenger().guest().find(guest) else { return };
    ctx.db.passenger().guest().delete(guest);
    crate::stop_motion(ctx, guest);
    if let Some(g) = player_of(ctx, guest) {
        if let Some(d) = player_of(ctx, ride.driver) {
            items::notify(ctx, d.identity, format!("{} got off", g.name));
        }
    }
}

/// The driver's passenger, if any (for warps: they come along).
pub fn passenger_of(ctx: &ReducerContext, driver: u64) -> Option<u64> {
    ctx.db.passenger().driver().find(driver).map(|r| r.guest)
}

/// Offer the player on `guest_entity` a ride (the Ride Request action).
#[spacetimedb::reducer]
pub fn ride_offer_to(ctx: &ReducerContext, guest_entity: u64) -> Result<(), String> {
    let game = game(ctx)?;
    let (p, id) = my_player(ctx)?;
    if !is_driving(ctx, id) {
        return Err("get on your vehicle first".into());
    }
    if !has_seat(&game, &p) {
        return Err("your vehicle needs a second seat (an add-on chair)".into());
    }
    if ctx.db.passenger().driver().find(id).is_some() {
        return Err("someone is already riding with you".into());
    }
    if guest_entity == id {
        return Err("pick another player".into());
    }
    let target = ctx.db.entity().entity_id().find(guest_entity).filter(|e| e.kind == EntityKind::Player).ok_or("pick another player")?;
    let guest = player_of(ctx, guest_entity).ok_or("they aren't here")?;
    check_guest(ctx, &p, id, &guest, guest_entity, target.zone_id)?;
    let t = now_us(ctx);
    let old: Vec<u64> = ctx.db.ride_offer().iter().filter(|o| o.driver == id).map(|o| o.id).collect();
    for o in old {
        ctx.db.ride_offer().id().delete(o);
    }
    ctx.db.ride_offer().insert(RideOffer { id: 0, driver: id, guest: guest_entity, expires_us: t + RIDE_OFFER_US });
    items::notify(ctx, p.identity, format!("You offered {} a ride", guest.name));
    Ok(())
}

fn check_guest(ctx: &ReducerContext, driver: &Player, driver_id: u64, guest: &Player, guest_id: u64, guest_zone: u16) -> Result<(), String> {
    let t = now_us(ctx);
    let near = match (crate::position(ctx, driver_id, t), crate::position(ctx, guest_id, t)) {
        (Some(a), Some(b)) => guest_zone == driver.zone_id && crate::distance(a, b) <= RIDE_RANGE_CM,
        _ => false,
    };
    if !near {
        return Err(format!("{} is too far away", guest.name));
    }
    if is_driving(ctx, guest_id) || is_passenger(ctx, guest_id) {
        return Err(format!("{} is already riding", guest.name));
    }
    if !crate::is_alive(ctx, guest_id) {
        return Err(format!("{} is down", guest.name));
    }
    if crate::shop::is_open(ctx, guest_id) {
        return Err(format!("{} has a shop open", guest.name));
    }
    Ok(())
}

/// Accept or decline a ride offer made to us.
#[spacetimedb::reducer]
pub fn ride_answer(ctx: &ReducerContext, offer_id: u64, accept: bool) -> Result<(), String> {
    let game = game(ctx)?;
    let (me, id) = my_player(ctx)?;
    let offer = ctx.db.ride_offer().id().find(offer_id).filter(|o| o.guest == id).ok_or("that offer has lapsed")?;
    ctx.db.ride_offer().id().delete(offer_id);
    let driver = player_of(ctx, offer.driver).ok_or("they have left")?;
    if !accept {
        items::notify(ctx, driver.identity, format!("{} doesn't want a ride", me.name));
        return Ok(());
    }
    if !is_driving(ctx, offer.driver) || !has_seat(&game, &driver) {
        return Err(format!("{} isn't driving any more", driver.name));
    }
    if ctx.db.passenger().driver().find(offer.driver).is_some() {
        return Err("someone else got on first".into());
    }
    check_guest(ctx, &driver, offer.driver, &me, id, me.zone_id)?;
    if crate::trade::trade_of(ctx, me.identity).is_some() {
        return Err("you are trading".into());
    }
    let t = now_us(ctx);
    crate::stand_up(ctx, id);
    crate::cancel_attack(ctx, id, t);
    crate::skills::cancel_cast(ctx, id);
    ctx.db.passenger().insert(Passenger { guest: id, driver: offer.driver, since_us: t });
    sync_passenger(ctx, offer.driver);
    items::notify(ctx, driver.identity, format!("{} got on", me.name));
    Ok(())
}

/// Get off someone's vehicle.
#[spacetimedb::reducer]
pub fn ride_leave(ctx: &ReducerContext) -> Result<(), String> {
    let (_, id) = my_player(ctx)?;
    if !is_passenger(ctx, id) {
        return Err("you aren't riding with anyone".into());
    }
    leave_seat(ctx, id);
    Ok(())
}
