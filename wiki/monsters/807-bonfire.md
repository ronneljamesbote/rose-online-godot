---
kind: monster
id: 807
name: Bonfire
status: in-game
level: 36
hp: 690
attack: 10
hit: 10
defence: 80
resistance: 47
avoid: 50
attack_speed: 98
attack_range: 0
damage: physical
walk_speed: 0
run_speed: 0
xp: 0
drop_item_rate: 100
drop_money_rate: 0
summoned_by:
- '[[skills/1161-bonfire|Bonfire]]'
source:
  data: LIST_NPC.STB row 807, ITEM_DROP.STB row 3
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Bonfire

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].
