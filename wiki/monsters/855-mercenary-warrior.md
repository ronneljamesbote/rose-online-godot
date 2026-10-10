---
kind: monster
id: 855
name: Mercenary Warrior
status: in-game
level: 59
hp: 112690
hp_per_level: 1910
attack: 188
hit: 167
defence: 190
resistance: 99
avoid: 67
attack_speed: 108
attack_range: 2.8
damage: physical
walk_speed: 249
run_speed: 548
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: LIST_NPC.STB row 855, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
