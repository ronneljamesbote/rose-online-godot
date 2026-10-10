---
kind: monster
id: 863
name: Mercenary Hunter
status: in-game
level: 66
hp: 119130
hp_per_level: 1805
attack: 215
hit: 204
defence: 174
resistance: 82
avoid: 98
attack_speed: 104
attack_range: 16.5
damage: physical
walk_speed: 240
run_speed: 530
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: LIST_NPC.STB row 863, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
