---
kind: zone
id: 9
name: Zone 9
status: in-game
planet: Junon
pvp: yes, except your party
map:
  image: maps/9-zone-9.jpg
  width: 320
  height: 448
  left: 4800
  top: 5600
  right: 5600
  bottom: 4480
npcs:
- npc: '[[npcs/1113-referee-leum|Referee Leum]]'
  x: 5200
  y: 5390
- npc: '[[npcs/1114-referee-pirre|Referee Pirre]]'
  x: 5201.6
  y: 4765.7
source:
  data: LIST_ZONE.STB row 9, the zone's IFO files, ITEM_DROP.STB row 9
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Zone 9

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
