---
kind: monster
id: 286
name: Master Goblin
status: in-game
level: 100
hp: 20400
hp_per_level: 204
attack: 480
hit: 303
defence: 394
resistance: 291
avoid: 127
attack_speed: 105
attack_range: 2.5
damage: physical
walk_speed: 310
run_speed: 730
xp: 545
drop_item_rate: 76
drop_money_rate: 15
zones: 1
drops:
- item: '[[items/material/191-animal-leg-bone|Animal Leg Bone]]'
  slots_of_30: 6
- item: '[[items/material/192-animal-backbone|Animal Backbone]]'
  slots_of_30: 3
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/173-stamina-100|Stamina (+100)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 3
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 2
- item: '[[items/jewellery/271-socket-earring|Socket Earring]]'
  slots_of_30: 1
- item: '[[items/weapon/209-rider-bow|Rider Bow]]'
  slots_of_30: 0.4
- item: '[[items/weapon/339-blizzard-wand|Blizzard Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/341-windstorm-wand|Windstorm Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/308-golden-staff|Golden Staff]]'
  slots_of_30: 0.4
- item: '[[items/weapon/41-pony-hammer|Pony Hammer]]'
  slots_of_30: 0.4
- item: '[[items/head/64-violet-beret|Violet Beret]]'
  slots_of_30: 0.4
- item: '[[items/weapon/139-golden-axe|Golden Axe]]'
  slots_of_30: 0.4
- item: '[[items/weapon/268-cross-launcher|Cross Launcher]]'
  slots_of_30: 0.4
- item: '[[items/weapon/340-recovery-wand|Recovery Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/438-blood-sword-axe|Blood Sword & Axe]]'
  slots_of_30: 0.4
- item: '[[items/hands/34-chrome-gauntlets|Chrome Gauntlets]]'
  slots_of_30: 0.4
- item: '[[items/feet/64-violet-boots|Violet Boots]]'
  slots_of_30: 0.4
- item: '[[items/body/94-black-pirate-armor|Black Pirate Armor]]'
  slots_of_30: 0.4
- item: '[[items/head/124-semiya-goggles|Semiya Goggles]]'
  slots_of_30: 0.4
- item: '[[items/back/247-buffalo-backshield|Buffalo Backshield]]'
  slots_of_30: 0.4
- item: '[[items/weapon/831-assassin-katar|Assassin Katar]]'
  slots_of_30: 0.6
- item: '[[items/weapon/564-dark-bow-gun|Dark Bow Gun]]'
  slots_of_30: 0.6
- item: '[[items/weapon/512-katana|Katana]]'
  slots_of_30: 0.6
- item: '[[items/body/532-mariner-s-look|Mariner’s Look]]'
  slots_of_30: 0.6
- item: '[[items/feet/632-merchant-boots|Merchant Boots]]'
  slots_of_30: 0.6
spawns:
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5238.1
  y: 5499.1
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5690
  y: 5451.4
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5371.3
  y: 5273.8
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5353.4
  y: 5247
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5261.2
  y: 4999.4
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5282.8
  y: 5009.5
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5695.9
  y: 5087.6
  count: 1
  group: reinforcements
- zone: '[[zones/33-goblin-cave-b3|Goblin Cave (B3)]]'
  x: 5515.1
  y: 4941.7
  count: 1
  group: reinforcements
source:
  data: LIST_NPC.STB row 286, ITEM_DROP.STB row 240
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Master Goblin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
