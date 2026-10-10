---
kind: monster
id: 401
name: Junon's Aqua
status: in-game
level: 47
hp: 9118
hp_per_level: 194
attack: 265
hit: 180
defence: 180
resistance: 116
avoid: 79
attack_speed: 110
attack_range: 3.5
damage: physical
walk_speed: 220
run_speed: 650
xp: 733
drop_item_rate: 85
drop_money_rate: 1
drops:
- item: '[[items/consumable/306-hp-scroll-solo|HP Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/consumable/10-vital-water-s|Vital Water (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/29-spiritual-water-s|Spiritual Water (S)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 2
- item: '[[items/material/162-blue-crystal|Blue Crystal]]'
  slots_of_30: 2
- item: '[[items/consumable/312-damage-scroll-solo|Damage Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/jewellery/13-talisman-ring|Talisman Ring]]'
  slots_of_30: 1
- item: '[[items/jewellery/91-glamour-necklace|Glamour Necklace]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 2
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 6
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 1
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 1
- item: '[[items/gem/311-ruby-1|Ruby 1]]'
  slots_of_30: 1
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 1
- item: '[[items/gem/331-topaz-1|Topaz 1]]'
  slots_of_30: 1
- item: '[[items/weapon/641-bardiche|Bardiche]]'
  slots_of_30: 1
- item: '[[items/weapon/502-saber|Saber]]'
  slots_of_30: 1
- item: '[[items/weapon/581-cutter-edge|Cutter Edge]]'
  slots_of_30: 1
- item: '[[items/weapon/672-white-wing-bow|White Wing Bow]]'
  slots_of_30: 1
- item: '[[items/weapon/762-mage-s-rod|Mage''s Rod]]'
  slots_of_30: 1
- item: '[[items/weapon/821-rake-hand|Rake Hand]]'
  slots_of_30: 1
quests:
- '[[quests/1980-mana-engine-begotten-devil-pest|Mana Engine Begotten: Devil Pest]]'
source:
  data: LIST_NPC.STB row 401, ITEM_DROP.STB row 301
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Junon's Aqua

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
