---
kind: monster
id: 859
name: Mercenary Warrior
status: in-game
level: 63
hp: 128079
hp_per_level: 2033
attack: 202
hit: 175
defence: 207
resistance: 106
avoid: 71
attack_speed: 116
attack_range: 2.8
damage: physical
walk_speed: 273
run_speed: 596
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: godot/data/monsters/859-mercenary-warrior.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
