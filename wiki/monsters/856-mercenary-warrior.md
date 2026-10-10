---
kind: monster
id: 856
name: Mercenary Warrior
status: in-game
level: 60
hp: 1940
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
  data: LIST_NPC.STB row 856, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
