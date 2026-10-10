---
kind: monster
id: 826
name: PhantomSword
status: in-game
level: 59
hp: 98943
hp_per_level: 1677
attack: 209
hit: 177
defence: 156
resistance: 64
avoid: 67
attack_speed: 105
attack_range: 2.5
damage: physical
walk_speed: 160
run_speed: 750
xp: 94
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1171-phantom-sword|Phantom Sword]]'
source:
  data: LIST_NPC.STB row 826, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# PhantomSword

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
