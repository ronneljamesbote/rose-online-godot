---
kind: monster
id: 152
name: Elder Doonga
status: in-game
level: 58
hp: 1856
hp_per_level: 32
attack: 223
hit: 134
defence: 145
resistance: 118
avoid: 87
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 220
run_speed: 460
xp: 43
drop_item_rate: 52
drop_money_rate: 12
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 9
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
  slots_of_30: 1
- item: '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
  slots_of_30: 1
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 5
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/421-doonga|Doonga]]'
  slots_of_30: 1
- item: '[[items/consumable/301-purify-scroll-solo|Purify Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/head/123-tamiya-goggles|Tamiya Goggles]]'
  slots_of_30: 0.2
- item: '[[items/body/33-trunket-armor|Trunket Armor]]'
  slots_of_30: 0.2
- item: '[[items/weapon/405-wolf-s-paw|Wolf''s Paw]]'
  slots_of_30: 0.2
- item: '[[items/weapon/305-white-staff|White Staff]]'
  slots_of_30: 0.2
- item: '[[items/weapon/7-saber|Saber]]'
  slots_of_30: 0.2
- item: '[[items/weapon/106-bull-sword|Bull Sword]]'
  slots_of_30: 0.6
- item: '[[items/hands/33-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.2
- item: '[[items/feet/63-purple-sandals-of-witch|Purple Sandals of Witch]]'
  slots_of_30: 0.2
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.2
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.2
- item: '[[items/weapon/166-iron-spear|Iron Spear]]'
  slots_of_30: 0.4
- item: '[[items/weapon/62-cutter-bow-gun|Cutter Bow Gun]]'
  slots_of_30: 0.4
- item: '[[items/weapon/8-elven-sword|Elven Sword]]'
  slots_of_30: 0.4
- item: '[[items/back/208-street-bag|Street Bag]]'
  slots_of_30: 0.4
quests:
- '[[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5268
  y: 5470.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5102.7
  y: 5345.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5044
  y: 5334.4
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5044
  y: 5334.4
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5147.5
  y: 5366.5
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5146.9
  y: 5305.7
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5279.8
  y: 5364.7
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5198.9
  y: 5381.2
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5276.4
  y: 5420.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5285.1
  y: 5294.3
  count: 2
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5323.3
  y: 5321.3
  count: 2
  group: basic
source:
  data: LIST_NPC.STB row 152, ITEM_DROP.STB row 171
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Elder Doonga

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
