---
kind: monster
id: 853
name: Mercenary Warrior
status: in-game
level: 57
hp: 105393
hp_per_level: 1849
attack: 180
hit: 163
defence: 181
resistance: 96
avoid: 65
attack_speed: 104
attack_range: 2.8
damage: physical
walk_speed: 237
run_speed: 524
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: godot/data/monsters/853-mercenary-warrior.tres, drop table godot/data/drop_tables/3-bonfire-butterfly-phantomsword-and-others.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
