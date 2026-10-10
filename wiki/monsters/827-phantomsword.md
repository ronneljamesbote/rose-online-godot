---
kind: monster
id: 827
name: PhantomSword
status: in-game
level: 60
hp: 1704
attack: 213
hit: 179
defence: 159
resistance: 65
avoid: 68
attack_speed: 106
attack_range: 2.5
damage: physical
walk_speed: 164
run_speed: 760
xp: 99
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 827, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
