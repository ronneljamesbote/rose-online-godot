---
kind: monster
id: 203
name: Jelly King
status: in-game
level: 13
hp: 637
hp_per_level: 49
attack: 44
hit: 84
defence: 57
resistance: 34
avoid: 21
attack_speed: 90
attack_range: 2.3
damage: physical
walk_speed: 200
run_speed: 360
xp: 43
drop_item_rate: 60
drop_money_rate: 1
zones: 1
drops:
- item: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  slots_of_30: 10
- item: '[[items/consumable/171-stamina-50|Stamina (+50)]]'
  slots_of_30: 1
- item: '[[items/material/161-green-crystal|Green Crystal]]'
  slots_of_30: 4
- item: '[[items/consumable/21-mana-vial-s|Mana Vial (S)]]'
  slots_of_30: 1
- item: '[[items/consumable/307-mp-scroll-solo|MP Scroll (Solo)]]'
  slots_of_30: 1
- item: '[[items/weapon/31-wooden-bat|Wooden Bat]]'
  slots_of_30: 0.2
- item: '[[items/weapon/3-rapier|Rapier]]'
  slots_of_30: 0.2
- item: '[[items/weapon/101-haedong-sword|Haedong Sword]]'
  slots_of_30: 0.2
- item: '[[items/weapon/202-short-bow|Short Bow]]'
  slots_of_30: 0.2
- item: '[[items/weapon/231-bubble-gun|Bubble Gun]]'
  slots_of_30: 0.2
- item: '[[items/weapon/201-toy-bow|Toy Bow]]'
  slots_of_30: 0.2
- item: '[[items/head/10-joker-jester|Joker Jester]]'
  slots_of_30: 0.2
- item: '[[items/body/10-islamic-dress|Islamic Dress]]'
  slots_of_30: 0.2
- item: '[[items/hands/10-gloves-of-iguje|Gloves of Iguje]]'
  slots_of_30: 0.2
- item: '[[items/feet/10-land-walkers|Land Walkers]]'
  slots_of_30: 0.2
- item: '[[items/weapon/301-baobab-rod|Baobab Rod]]'
  slots_of_30: 0.4
- item: '[[items/weapon/401-knuckle|Knuckle]]'
  slots_of_30: 0.4
- item: '[[items/weapon/32-monkey-wrench|Monkey Wrench]]'
  slots_of_30: 0.4
- item: '[[items/weapon/331-baobab-wand|Baobab Wand]]'
  slots_of_30: 0.4
- item: '[[items/weapon/162-javelin|Javelin]]'
  slots_of_30: 0.4
quests:
- '[[quests/2209-the-jelly-king|The Jelly King]]'
- '[[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]'
spawns:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5122
  y: 5272
  count: 1
  group: basic
source:
  data: godot/data/monsters/203-jelly-king.tres, drop table godot/data/drop_tables/198-jelly-king.tres
  code:
  - module/src/monster_brain.rs
  - crates/rose-game-irose/src/data/drop_table.rs
---
# Jelly King

Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.
HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.
How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];
how the XP value turns into experience is on [[rules/experience|Experience]].

## Changed from iROSE

The Adventurer's Plain redesign makes it the plain's one elite: a single Jelly King lives in its hollow at Bramble Ford (5122, 5272) and comes back 3 minutes after it falls (iROSE has it as a rare reinforcement at five spawn points). Killing it also runs step `2209-31` of [[quests/2209-the-jelly-king|The Jelly King]], which gives the Jelly King's Crown.
