---
kind: monster
id: 824
name: PhantomSword
status: in-game
level: 57
hp: 1622
attack: 201
hit: 173
defence: 150
resistance: 62
avoid: 65
attack_speed: 103
attack_range: 2.5
damage: physical
walk_speed: 152
run_speed: 730
xp: 83
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 824, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
