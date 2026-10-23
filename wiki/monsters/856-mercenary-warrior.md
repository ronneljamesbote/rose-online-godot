---
kind: monster
id: 856
name: Mercenary Warrior
status: in-game
level: 60
hp: 116400
hp_per_level: 1940
attack: 191
hit: 169
defence: 194
resistance: 101
avoid: 68
attack_speed: 110
attack_range: 2.8
damage: physical
walk_speed: 255
run_speed: 560
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: godot/data/monsters/856-mercenary-warrior.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
