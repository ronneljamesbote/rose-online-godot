---
kind: monster
id: 156
name: Doonga Hunter
status: in-game
level: 62
hp: 1488
hp_per_level: 24
attack: 237
hit: 167
defence: 167
resistance: 91
avoid: 101
attack_speed: 85
attack_range: 20
damage: physical
walk_speed: 230
run_speed: 510
xp: 41
drop_item_rate: 52
drop_money_rate: 18
zones: 1
drops:
- item: '[[items/material/187-black-animal-tail-fur|Black Animal Tail Fur]]'
  slots_of_30: 9
- item: '[[items/material/188-animal-tail-fur|Animal Tail Fur]]'
  slots_of_30: 8
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/10-vital-water-s|Vital Water (S)]]'
  slots_of_30: 1
- item: '[[items/jewellery/261-socket-necklace|Socket Necklace]]'
  slots_of_30: 1
- item: '[[items/consumable/312-damage-scroll-solo|Damage Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/236-sorden-gun|Sorden Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/406-bear-s-paw|Bear''s Paw]]'
  slots_of_30: 0.6
- item: '[[items/weapon/434-twin-ghost-bat|Twin Ghost Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/336-windphon-wand|Windphon Wand]]'
  slots_of_30: 0.2
- item: '[[items/weapon/207-iron-bow|Iron Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/135-orc-axe|Orc Axe]]'
  slots_of_30: 0.2
- item: '[[items/weapon/37-stone-hammer|Stone Hammer]]'
  slots_of_30: 0.2
- item: '[[items/weapon/306-wisp-staff|Wisp Staff]]'
  slots_of_30: 0.2
- item: '[[items/hands/33-trunket-gloves|Trunket Gloves]]'
  slots_of_30: 0.4
- item: '[[items/feet/63-purple-sandals-of-witch|Purple Sandals of Witch]]'
  slots_of_30: 0.4
- item: '[[items/head/93-ranger-hat|Ranger Hat]]'
  slots_of_30: 0.4
- item: '[[items/body/123-tamiya-vest|Tamiya Vest]]'
  slots_of_30: 0.4
- item: '[[items/subweapon/66-elven-anthology|Elven Anthology]]'
  slots_of_30: 0.4
quests:
- '[[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]'
spawns:
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5279.8
  y: 5364.7
  count: 2
  group: reinforcements
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5285.1
  y: 5294.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5323.3
  y: 5321.3
  count: 1
  group: basic
- zone: '[[zones/27-kenji-beach|Kenji Beach]]'
  x: 5829.1
  y: 5361.8
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 156, ITEM_DROP.STB row 175
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Doonga Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
