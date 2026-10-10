---
kind: monster
id: 822
name: PhantomSword
status: in-game
level: 55
hp: 1569
attack: 194
hit: 169
defence: 144
resistance: 59
avoid: 63
attack_speed: 101
attack_range: 2.5
damage: physical
walk_speed: 144
run_speed: 710
xp: 75
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 822, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
