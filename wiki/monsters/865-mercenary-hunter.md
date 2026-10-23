---
kind: monster
id: 865
name: Mercenary Hunter
status: in-game
level: 68
hp: 126548
hp_per_level: 1861
attack: 223
hit: 209
defence: 180
resistance: 85
avoid: 101
attack_speed: 108
attack_range: 17.5
damage: physical
walk_speed: 250
run_speed: 550
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2361-hire-hunter|Hire Hunter]]'
source:
  data: godot/data/monsters/865-mercenary-hunter.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Hunter

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
