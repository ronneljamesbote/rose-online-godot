---
kind: zone
id: 5
name: Junon Cartel
status: in-game
planet: Junon
pvp: yes, except your party
map:
  image: maps/5-junon-cartel.jpg
  width: 320
  height: 384
  left: 4800
  top: 5760
  right: 5600
  bottom: 4800
npcs:
- npc: '[[npcs/1086-akram-minister-rodath|Akram Minister Rodath]]'
  x: 5155.1
  y: 5279.9
- npc: '[[npcs/1087-akram-minister-mel|Akram Minister Mel]]'
  x: 5247.1
  y: 5280
source:
  data: LIST_ZONE.STB row 5, the zone's IFO files, ITEM_DROP.STB row 5
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Junon Cartel

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
