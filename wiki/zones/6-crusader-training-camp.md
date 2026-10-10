---
kind: zone
id: 6
name: Crusader Training Camp
status: in-game
planet: Junon
pvp: yes, except your party
map:
  image: maps/6-crusader-training-camp.jpg
  width: 320
  height: 384
  left: 4800
  top: 5760
  right: 5600
  bottom: 4800
npcs:
- npc: '[[npcs/1084-akram-minister-gamp|Akram Minister Gamp]]'
  x: 5248.1
  y: 5279.2
source:
  data: LIST_ZONE.STB row 6, the zone's IFO files, ITEM_DROP.STB row 6
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Crusader Training Camp

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
