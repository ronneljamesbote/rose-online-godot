---
kind: monster
id: 254
name: Stone Golem
status: in-game
level: 74
hp: 37
attack: 254
hit: 186
defence: 279
resistance: 170
avoid: 61
attack_speed: 90
attack_range: 1.9
damage: physical
walk_speed: 200
run_speed: 450
xp: 50
drop_item_rate: 0
drop_money_rate: 0
quests:
- '[[quests/135-the-scheme|The Scheme]]'
source:
  data: LIST_NPC.STB row 254, ITEM_DROP.STB row 0
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Stone Golem

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
