---
kind: monster
id: 859
name: Mercenary Warrior
status: in-game
level: 63
hp: 2033
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
  data: LIST_NPC.STB row 859, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
