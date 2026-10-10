---
kind: rule
id: carts
name: Carts and Castle Gear
status: in-game
part_slots: "Body (frame or core), Engine, Legs (wheels or legs), Arms"
needed_to_drive: "Body, Engine, Legs"
full_fuel: 1000
fuel_tick_s: 10
fuel_used: "engine's fuel rate on getting on, on each attack swing and every 10 seconds"
broken_engine_or_legs_speed: "200 (2 m/s)"
passenger_seats: 1
ride_offer_range_m: 6
ride_offer_lapses_s: 30
passenger_arrives_after_warp_m: 1
attack_range_bonus_m: 1.2
fuel_items:
  - { item: "[[items/consumable/293-engine-fuel-s|Engine Fuel (S)]]", adds: "250 (25%)" }
  - { item: "[[items/consumable/294-engine-fuel-m|Engine Fuel (M)]]", adds: "500 (50%)" }
  - { item: "[[items/consumable/295-engine-fuel-l|Engine Fuel (L)]]", adds: "1000 (100%)" }
example_parts:
  - { part: "[[items/vehicle/101-first-engine|First Engine]]", kind: "cart engine", speed: 100, fuel_rate: 0 }
  - { part: "[[items/vehicle/131-fornef-engine|Fornef Engine]]", kind: "castle gear engine", speed: 100, fuel_rate: 3 }
  - { part: "[[items/vehicle/201-wooden-wheels|Wooden Wheels]]", kind: "cart legs", speed: 110, fuel_rate: "" }
  - { part: "[[items/vehicle/231-stand-leg|Stand Leg]]", kind: "castle gear legs", speed: 40, fuel_rate: "" }
  - { part: "[[items/vehicle/331-punch-arms|Punch Arms]]", kind: "castle gear arms", speed: "", fuel_rate: "" }
  - { part: "[[items/vehicle/303-back-seat|Back Seat]]", kind: "cart arms, second seat", speed: "", fuel_rate: "" }
vehicle_skills:
  - { skill: "[[skills/17-drive-cart|Drive Cart]]", does: "get on or off" }
  - { skill: "[[skills/25-ride-request|Ride Request]]", does: "offer a player a ride" }
  - { skill: "[[skills/3501-castle-gear-skill|Castle Gear Skill]] (3501, 3502)", does: "castle gear only, costs Fuel 100 / 200" }
  - { skill: "[[skills/3511-castle-gear-skill|Castle Gear Skill]] (3511)", does: "castle gear only, costs Fuel 80 and MP 100" }
source:
  code:
    - module/src/vehicle.rs (drive_toggle, get_off, use_fuel, tick, refuel, skill_refusal, equip_part, unequip_vehicle_part, ride_offer_to, ride_answer, ride_leave, check_guest)
    - module/src/character.rs (player_stats)
    - module/src/lib.rs (move_to, attack loop, recovery tick, kill)
    - module/src/world.rs (teleport)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_vehicle_move_speed, calculate_vehicle_attack_power, calculate_vehicle_attack_speed, calculate_vehicle_attack_range, calculate_vehicle_hit, calculate_vehicle_critical, calculate_vehicle_avoid, calculate_defence)
  data: LIST_PAT.STB (columns 2 part, 16 type, 32 fuel rate, 33 speed, 35 range, 36 attack, 37 attack speed, 39 seat); LIST_USEITEM.STB column 20 (fuel added); LIST_ZONE.STB column 30 (vehicle ban)
---
# Carts and Castle Gear

A **cart** is a fast vehicle for travelling. **Castle gear** is a fighting machine. Both are
built from parts that you put in the vehicle tab of your inventory:

- **Body**: a cart frame or a castle gear core. It decides whether this is a cart or castle
  gear.
- **Engine**: holds the fuel and sets the speed together with the legs.
- **Legs**: cart wheels or castle gear legs.
- **Arms** (optional): a weapon, or an add-on like the [[items/vehicle/303-back-seat|Back Seat]].

You need a body, an engine and legs to drive. Parts have requirements like other gear
(level, and for some parts a skill). A cart body only takes cart parts and castle gear only
castle gear parts; putting on a new body of the other kind sends the old parts to your bag.
You can't change parts while driving, and a broken part (life 0) can't be put on, except an
engine with no fuel.

## Getting on and off

Use [[skills/17-drive-cart|Drive Cart]] (or the Drive button) to get on, and again to get
off. You can't get on while dead, with a shop open, while riding as a passenger, or while a
status stops you from acting. Getting on stops you moving, attacking and casting, makes you
stand, and **ends your buffs**. So does getting off.

You get off by yourself when you die, warp or leave the game.

A zone can forbid carts or castle gear (LIST_ZONE column 30), but no zone in the iROSE data
does.

## While driving

The vehicle's stats replace some of yours:

| Stat | While driving |
| --- | --- |
| Move speed | engine speed x legs speed / 10, plus speed bonuses of the parts |
| Attack | 3 x level + Concentration + arms attack + attack bonuses of the parts |
| Attack speed | 1500 / (arms attack speed + 5) |
| Attack range | arms range + 1.2 metres |
| Hit | (Concentration + 10) x 0.8 + level x 0.5 + arms quality x 1.2 (0 with no working arms) |
| Critical | Sense x 0.8 + level x 0.3 |
| Avoid | (Dexterity + 10) x 0.8 + level x 0.5 |
| Defence | your normal defence, plus the grade bonus of each vehicle part |

Concentration, Sense and Dexterity here are your own points, without gear or passive
skill bonuses. Passive skills still add to hit, critical and avoid. A broken arms part
gives no attack, attack speed or hit. If the engine or the legs are broken (an engine with
no fuel counts as broken), speed is 200 (2 m/s).

Also while driving:

- HP and MP don't recover.
- You can't sit or open a shop. Attacks don't use arrows or bullets.
- Your weapon doesn't wear; the Arms part does, and hits wear the Body, Legs or Arms. See
  [[rules/durability|Durability]].
- Most attack skills can't be used (instant attacks, weapon and bullet buffs, shots, area
  skills, self-damage skills). Buffs and other skills still work.
- Vehicle skills (table above) need the right body: castle gear skills need castle gear.

**Worked example.** A [[items/vehicle/101-first-engine|First Engine]] (speed 100) on
[[items/vehicle/201-wooden-wheels|Wooden Wheels]] (speed 110) drives at 100 x 110 / 10 = 1100,
11 metres a second.

A level 70 character with 60 Concentration in castle gear with
[[items/vehicle/331-punch-arms|Punch Arms]] (attack 350, attack speed 9, range 3 m, quality
40) has attack 210 + 60 + 350 = 620, attack speed 1500 / 14 = 107, range 4.2 m and hit
70 x 0.8 + 35 + 48 = 139.

## Fuel

The engine's life is its fuel, from 0 to 1000 (shown as 0-100%). The engine's fuel rate is
used up:

- when you get on,
- on each attack swing,
- every 10 seconds while driving.

Skills that cost Fuel take it from the engine too.

At 0 fuel the vehicle stops: you can't move or attack until you refuel ("Out of fuel").
Engine Fuel items add 10 x their value, up to 1000 (table above). You can refuel while
driving, but not when the tank is full.

All cart engines in the iROSE data have a fuel rate of 0, so **carts never run out of
fuel**. Castle gear engines use fuel.

**Worked example.** A [[items/vehicle/131-fornef-engine|Fornef Engine]] uses 3 fuel. A full
tank lasts 333 ticks of 10 seconds (55 minutes) of driving without fighting; each attack
swing takes another 3.

## Passengers

A cart with a second seat (the [[items/vehicle/303-back-seat|Back Seat]] in its Arms slot)
can carry one passenger.

1. While driving, use [[skills/25-ride-request|Ride Request]] on a player (or pick "Offer a
   ride"). They must be in the same zone, within 6 metres, alive, not driving or riding, and
   not running a shop.
2. They accept or decline. An unanswered offer lapses after 30 seconds. A new offer replaces
   the old one. They can't accept while trading.
3. The passenger sits behind the driver and moves with the vehicle. A passenger can't move,
   fight, use skills, sit, use most items or open a shop. They can get off at any time.

The passenger gets off when the driver gets off, dies or leaves. When the driver warps,
the passenger comes along and arrives 1 metre to the side, on foot.

> Open question: the parts' Defence and other stat bonuses (for example a frame's Defence +26) are read into the vehicle's bonuses, but defence while driving only uses your normal gear plus the parts' grade bonus, so those bonuses are not applied.
> Open question: engines list a maximum fuel (LIST_PAT column 31, e.g. 1600 for the First Engine), but every engine's tank holds 1000.
