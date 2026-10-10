---
kind: rule
id: durability
name: Durability and Repair
status: in-game
max_life: 1000
life_shown_as: "life / 10, as 0-100%"
durability_range: "0-100 (refining can raise it to 120)"
life_lost_per_wear: 1
weapon_wear_roll: "1-710, wears when roll >= durability + 600"
armour_slot_roll: "1-400, only 1-100 picks a slot (25%)"
armour_wear_roll: "1-120, wears when roll >= durability + 10 - damage / 10"
npc_repair_range_m: 15
weapon_wear_chance:
  - { durability: 35, chance: "76 in 710 (10.7%)" }
  - { durability: 50, chance: "61 in 710 (8.6%)" }
  - { durability: 100, chance: "11 in 710 (1.5%)" }
  - { durability: "111 or more", chance: "never" }
armour_slot_no_shield:
  - { slot: Body, rolls: "65-100", share: "36%" }
  - { slot: Feet, rolls: "47-64", share: "18%" }
  - { slot: Back, rolls: "16-31", share: "16%" }
  - { slot: Hands, rolls: "32-46", share: "15%" }
  - { slot: Head, rolls: "8-15", share: "8%" }
  - { slot: Face, rolls: "1-7", share: "7%" }
armour_slot_with_shield:
  - { slot: Shield, rolls: "61-100", share: "40%" }
  - { slot: Body, rolls: "31-60", share: "30%" }
  - { slot: Feet, rolls: "23-30", share: "8%" }
  - { slot: Back, rolls: "10-16", share: "7%" }
  - { slot: Head, rolls: "17-22", share: "6%" }
  - { slot: Face, rolls: "1-5", share: "5%" }
  - { slot: Hands, rolls: "6-9", share: "4%" }
vehicle_slot_when_driving:
  - { part: Body, rolls: "51-100", share: "50%" }
  - { part: Legs, rolls: "21-50", share: "30%" }
  - { part: Arms, rolls: "1-20", share: "20%" }
repair_tools:
  - { item: "[[items/consumable/291-repair-hammer|Repair Hammer]]", lowers_durability: "may (see formula)" }
  - { item: "[[items/consumable/951-perfect-repair-hammer|Perfect Repair Hammer]]", lowers_durability: never }
  - { item: "Perfect Repair Hammer (consumable 292, no page)", lowers_durability: never }
repair_npcs:
  - { npc: "[[npcs/1008-weapon-seller-raffle|Raffle]]", zone: "[[zones/1-canyon-city-of-zant|Zant]]" }
  - { npc: "[[npcs/1034-smith-ronk|Ronk]]", zone: "[[zones/22-adventurer-s-plain|Adventurer's Plain]]" }
  - { npc: "[[npcs/1093-weapon-merchant-crune|Crune]]", zone: "[[zones/2-city-of-junon-polis|Junon Polis]]" }
  - { npc: "[[npcs/1181-smith-pavrick|Pavrick]]", zone: "[[zones/51-magic-city-of-the-eucar|Eucar]]" }
  - { npc: "[[npcs/1223-smith-nel-eldora|Nel Eldora]]", zone: "[[zones/61-refuge-xita|Refuge Xita]]" }
  - { npc: "[[npcs/1161-clan-base-camp-manager-kushard|Kushard]]", zone: "[[zones/15-zone-15|Clan hall]]" }
source:
  code:
    - module/src/durability.rs (wear, weapon_used, hit_taken, repair_at_npc, repair_with_item, npc_price, check_repairable)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_decrease_weapon_life, calculate_decrease_armour_life, calculate_repair_from_npc_price)
    - module/src/skills.rs (weapon_wear)
    - module/src/lib.rs (deal_damage)
    - module/src/items.rs (equip_item)
    - module/src/shop.rs (store_sell)
  data: LIST_WEAPON.STB and the other equipment STBs column 29 (durability), column 5 (price); LIST_USEITEM.STB column 20 (perfect hammer)
---
# Durability and Repair

Your gear wears out as you fight. Every piece of equipment has two numbers:

- **Life** is its condition right now. It is shown as 0-100% (the game counts it 0-1000).
  It goes down while you fight.
- **Durability** (0-100) is how tough the item is. The higher it is, the less often life
  goes down. Durability never drops by itself; only a cheap repair hammer can lower it.

When life reaches 0 the item is **broken**. It stays where it is, but gives no stats at all
(no attack, no defence, no bonus options or gem) until you repair it. You get the message
"Your *item* broke". A broken item can't be put on, and you can't sell it to another
player's shop.

## What wears down

Each time something wears, it loses 1 life (0.1%). A new item at 1000 life needs 1000
wears to break.

### Your weapon

Every basic attack swing, and every damaging skill (except pure magic spells), rolls a
number from 1 to 710. The weapon loses 1 life when the roll is at least durability + 600.
While driving a cart or castle gear, the **Arms** part wears instead of your weapon.

```math
P(\text{weapon wears}) = \frac{111 - \text{durability}}{710}
```

**Worked example.** A [[items/weapon/2-short-sword|Short Sword]] has durability 35. It wears
on a roll of 635 to 710: 76 rolls out of 710, so 10.7% of swings. At durability 100 only
11 rolls out of 710 wear it (1.5%). An item refined above durability 110 never wears from
attacking.

### Your armour

Every hit that does damage to you (from a monster or a player) first rolls 1 to 400. Only
1 to 100 picks a slot (a 25% chance); the slot is chosen from that same roll, see the
tables above. If you carry a working shield, it takes the most hits. While driving, the hit
picks the Body, Legs or Arms part of the vehicle instead.

If nothing is worn in the chosen slot, or that item is already broken, nothing happens.
Otherwise a second roll from 1 to 120 decides; big hits get through more often:

```math
\text{wears when } \text{roll}_{1..120} \ge \text{durability} + 10 - \left\lfloor \frac{\text{damage}}{10} \right\rfloor
```

**Worked example.** Without a shield, a 50 damage hit lands. The slot roll picks Body 9% of
the time (25% x 36%). Body armour with durability 40 then wears on a roll of 45 or more
(40 + 10 - 5), which is 76 of 120 (63.3%). So that hit wears the body armour 5.7% of the
time.

### Vehicle engines

Engines don't wear this way. An engine's life is its fuel, see [[rules/carts|Carts]].

## Repairing

Repair always brings life back to 1000 (100%). It can't repair:

- an item with durability 0,
- an engine (refuel it instead),
- an item that is already at full life.

You can repair worn gear, vehicle parts or items in your bag.

### At an NPC (costs Zuly)

Smiths and weapon sellers with a repair line in their dialog (see the table above) repair
for Zuly. You must be in the same zone and within 15 metres of the NPC. The fee is at
least 1 Zuly:

```math
\text{fee} = \left\lfloor \frac{\text{price} + 1000}{400000} \times (\text{durability} + 10) \times (1100 - \text{life}) \right\rfloor
```

where *price* is the item's shop price and *life* is 0-1000.

**Worked example.** A [[items/weapon/2-short-sword|Short Sword]] (price 420, durability 35)
at 40% life (400): 1420 / 400000 x 45 x 700 = 111.825, so 111 Zuly.

### With a hammer (free, uses one hammer)

Use a hammer from your bag on the item. It works anywhere.

- The [[items/consumable/951-perfect-repair-hammer|Perfect Repair Hammer]] never lowers
  durability.
- The [[items/consumable/291-repair-hammer|Repair Hammer]] may lower durability. The more
  worn the item, the bigger the loss. Each division drops the remainder:

```math
\text{durability lost} = \left\lfloor \left\lfloor \frac{(1400 - \text{life}) \times (\text{roll}_{0..99} + 11)}{\text{durability} + 40} \right\rfloor \Big/ 400 \right\rfloor
```

It never takes more durability than the item has.

**Worked example.** A broken Short Sword (life 0, durability 35) and a roll of 50:
1400 x 61 = 85400, / 75 = 1138, / 400 = 2. The sword keeps 33 durability. The loss for
this sword ranges from 0 (roll 0) to 5 (roll 99).

How a hammer counts as "perfect": its LIST_USEITEM column 20 is not 0.

> Open question: the server repairs at any NPC within 15 metres; only the client limits repair to NPCs whose dialog offers it.
