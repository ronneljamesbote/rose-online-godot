---
kind: zone
id: 8
name: Lion's Plains
status: in-game
planet: Junon
pvp: yes, except your party
map:
  image: maps/8-lion-s-plains.jpg
  width: 384
  height: 512
  left: 4800
  top: 5600
  right: 5760
  bottom: 4320
source:
  data: LIST_ZONE.STB row 8, the zone's IFO files, ITEM_DROP.STB row 8
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Lion's Plains

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
