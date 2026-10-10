---
kind: monster
id: 858
name: Mercenary Warrior
status: in-game
level: 62
hp: 2002
attack: 198
hit: 173
defence: 203
resistance: 104
avoid: 70
attack_speed: 114
attack_range: 2.8
damage: physical
walk_speed: 267
run_speed: 584
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: LIST_NPC.STB row 858, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
