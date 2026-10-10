---
kind: zone
id: 3
name: Dolphin Island
status: in-game
planet: Junon
pvp: no
map:
  image: maps/3-dolphin-island.jpg
  width: 512
  height: 448
  left: 4800
  top: 5760
  right: 6080
  bottom: 4640
npcs:
- npc: '[[npcs/1513-ferrell-guild-staff-rooen|Ferrell Guild Staff Rooen]]'
  x: 5104.5
  y: 5019.7
source:
  data: LIST_ZONE.STB row 3, the zone's IFO files, ITEM_DROP.STB row 3
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Dolphin Island

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
