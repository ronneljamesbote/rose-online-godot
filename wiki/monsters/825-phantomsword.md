---
kind: monster
id: 825
name: PhantomSword
status: in-game
level: 58
hp: 1649
attack: 205
hit: 175
defence: 153
resistance: 63
avoid: 66
attack_speed: 104
attack_range: 2.5
damage: physical
walk_speed: 156
run_speed: 740
xp: 89
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 825, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
