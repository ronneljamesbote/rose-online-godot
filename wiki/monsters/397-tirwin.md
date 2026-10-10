---
kind: monster
id: 397
name: Tirwin
status: in-game
level: 124
hp: 3596
hp_per_level: 29
attack: 449
hit: 286
defence: 299
resistance: 475
avoid: 192
attack_speed: 105
attack_range: 6
damage: magic
walk_speed: 250
run_speed: 700
xp: 64
drop_item_rate: 52
drop_money_rate: 30
zones: 1
drops:
- item: '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
  slots_of_30: 1
- item: '[[items/consumable/152-hp-point-100|HP Point (+100)]]'
  slots_of_30: 1
- item: '[[items/consumable/153-hp-point-200|HP Point (+200)]]'
  slots_of_30: 1
- item: '[[items/material/79-transparent-powder|Transparent Powder]]'
  slots_of_30: 3
- item: '[[items/material/80-rainbow-powder|Rainbow Powder]]'
  slots_of_30: 3
- item: '[[items/material/65-high-ether|High Ether]]'
  slots_of_30: 2
- item: '[[items/material/81-lisent-fe|Lisent (Fe)]]'
  slots_of_30: 3
- item: '[[items/material/78-golden-powder|Golden Powder]]'
  slots_of_30: 1
- item: '[[items/material/66-iricer|Iricer]]'
  slots_of_30: 1
- item: '[[items/material/82-lisent-cu|Lisent (Cu)]]'
  slots_of_30: 2
- item: '[[items/material/67-hime|Hime]]'
  slots_of_30: 1
- item: '[[items/gem/301-garnet-1|Garnet 1]]'
  slots_of_30: 1
- item: '[[items/gem/311-ruby-1|Ruby 1]]'
  slots_of_30: 1
- item: '[[items/gem/321-sapphire-1|Sapphire 1]]'
  slots_of_30: 1
- item: '[[items/gem/331-topaz-1|Topaz 1]]'
  slots_of_30: 1
- item: '[[items/gem/341-emerald-1|Emerald 1]]'
  slots_of_30: 1
- item: '[[items/gem/351-peridot-1|Peridot 1]]'
  slots_of_30: 1
spawns:
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5319.5
  y: 5352.5
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5317.9
  y: 5123.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5545.1
  y: 5129.7
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5545.1
  y: 5129.7
  count: 1
  group: reinforcements
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5540.7
  y: 5127.1
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5540.7
  y: 5127.1
  count: 1
  group: basic
- zone: '[[zones/59-luna-clan-field|Luna Clan Field]]'
  x: 5320.6
  y: 4904
  count: 1
  group: basic
source:
  data: LIST_NPC.STB row 397, ITEM_DROP.STB row 282
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Tirwin

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
