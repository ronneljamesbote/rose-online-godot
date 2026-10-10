---
kind: monster
id: 407
name: Junon's Doonga
status: in-game
level: 85
hp: 273
attack: 478
hit: 307
defence: 339
resistance: 214
avoid: 147
attack_speed: 120
attack_range: 3.5
damage: physical
walk_speed: 200
run_speed: 700
xp: 1317
drop_item_rate: 85
drop_money_rate: 1
drops:
- item: '[[items/consumable/312-damage-scroll-solo|Damage Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/consumable/11-vital-water-m|Vital Water (M)]]'
  slots_of_30: 1
- item: '[[items/consumable/30-spiritual-water-m|Spiritual Water (M)]]'
  slots_of_30: 1
- item: '[[items/material/163-red-crystal|Red Crystal]]'
  slots_of_30: 5
- item: '[[items/consumable/316-advanced-strength-scroll-solo|Advanced Strength Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/jewellery/162-charming-earring|Charming Earring]]'
  slots_of_30: 1
- item: '[[items/jewellery/163-talisman-earring|Talisman Earring]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 1
- item: '[[items/material/151-black-hearts|Black Hearts]]'
  slots_of_30: 3
- item: '[[items/material/152-green-hearts|Green Hearts]]'
  slots_of_30: 2
- item: '[[items/material/153-blue-hearts|Blue Hearts]]'
  slots_of_30: 1
- item: '[[items/material/154-pink-hearts|Pink Hearts]]'
  slots_of_30: 1
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 1
- item: '[[items/gem/332-topaz-2|Topaz 2]]'
  slots_of_30: 1
- item: '[[items/gem/352-peridot-2|Peridot 2]]'
  slots_of_30: 1
- item: '[[items/gem/311-ruby-1|Ruby 1]]'
  slots_of_30: 1
source:
  data: LIST_NPC.STB row 407, ITEM_DROP.STB row 307
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Junon's Doonga

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
