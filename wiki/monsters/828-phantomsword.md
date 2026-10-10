---
kind: monster
id: 828
name: PhantomSword
status: in-game
level: 61
hp: 1731
attack: 217
hit: 181
defence: 162
resistance: 66
avoid: 69
attack_speed: 107
attack_range: 2.5
damage: physical
walk_speed: 168
run_speed: 770
xp: 105
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 828, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
