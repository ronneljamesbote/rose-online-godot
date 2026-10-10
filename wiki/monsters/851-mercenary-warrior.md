---
kind: monster
id: 851
name: Mercenary Warrior
status: in-game
level: 55
hp: 1789
attack: 173
hit: 159
defence: 173
resistance: 93
avoid: 63
attack_speed: 100
attack_range: 2.8
damage: physical
walk_speed: 225
run_speed: 500
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/2351-hire-warrior|Hire Warrior]]'
source:
  data: LIST_NPC.STB row 851, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Mercenary Warrior

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
