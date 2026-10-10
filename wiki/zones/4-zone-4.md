---
kind: zone
id: 4
name: Zone 4
status: in-game
planet: Junon
pvp: no
map:
  image: maps/4-zone-4.jpg
  width: 384
  height: 384
  left: 4800
  top: 5600
  right: 5760
  bottom: 4640
source:
  data: LIST_ZONE.STB row 4, the zone's IFO files, ITEM_DROP.STB row 4
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Zone 4

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
