---
kind: monster
id: 821
name: PhantomSword
status: in-game
level: 54
hp: 1542
attack: 190
hit: 166
defence: 141
resistance: 58
avoid: 62
attack_speed: 100
attack_range: 2.5
damage: physical
walk_speed: 140
run_speed: 700
xp: 71
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 821, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
