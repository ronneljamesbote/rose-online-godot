---
kind: zone
id: 59
name: Luna Clan Field
status: in-game
planet: Luna
pvp: yes, except your clan
monster_levels: 124–155
map:
  image: maps/59-luna-clan-field.jpg
  width: 384
  height: 384
  left: 4800
  top: 5600
  right: 5760
  bottom: 4640
monster_spawns:
- monsters: '[[monsters/387-elec-ghost|Elec Ghost]] ×1, [[monsters/477-tirwin|Tirwin]] ×1, [[monsters/497-tirwin|Tirwin]] ×1, [[monsters/487-tirwin|Tirwin]] ×1, [[monsters/397-tirwin|Tirwin]] ×1'
  reinforcements: '[[monsters/478-tirwin|Tirwin]] ×1, [[monsters/488-tirwin|Tirwin]] ×2'
  x: 5319.5
  y: 5352.5
  radius: 17
  max_alive: 4
  respawn_seconds: 33
- monsters: '[[monsters/396-astarot-king|Astarot King]] ×1, [[monsters/395-astarot|Astarot]] ×1, [[monsters/396-astarot-king|Astarot King]] ×1'
  x: 5318.8
  y: 5129.5
  radius: 18
  max_alive: 1
  respawn_seconds: 1130
- monsters: '[[monsters/478-tirwin|Tirwin]] ×1, [[monsters/488-tirwin|Tirwin]] ×1, [[monsters/498-tirwin|Tirwin]] ×1, [[monsters/487-tirwin|Tirwin]] ×1, [[monsters/397-tirwin|Tirwin]] ×1'
  reinforcements: '[[monsters/488-tirwin|Tirwin]] ×2, [[monsters/478-tirwin|Tirwin]] ×2'
  x: 5317.9
  y: 5123.7
  radius: 29
  max_alive: 12
  respawn_seconds: 50
- monsters: '[[monsters/387-elec-ghost|Elec Ghost]] ×1, [[monsters/388-elec-ghost|Elec Ghost]] ×1, [[monsters/388-elec-ghost|Elec Ghost]] ×1, [[monsters/388-elec-ghost|Elec Ghost]] ×1, [[monsters/388-elec-ghost|Elec Ghost]] ×1'
  x: 5324.1
  y: 5125.8
  radius: 27
  max_alive: 1
  respawn_seconds: 132
- monsters: '[[monsters/487-tirwin|Tirwin]] ×1, [[monsters/477-tirwin|Tirwin]] ×1, [[monsters/497-tirwin|Tirwin]] ×1, [[monsters/487-tirwin|Tirwin]] ×1, [[monsters/397-tirwin|Tirwin]] ×1'
  reinforcements: '[[monsters/387-elec-ghost|Elec Ghost]] ×2, [[monsters/397-tirwin|Tirwin]] ×1'
  x: 5545.1
  y: 5129.7
  radius: 17
  max_alive: 4
  respawn_seconds: 70
- monsters: '[[monsters/395-astarot|Astarot]] ×1, [[monsters/387-elec-ghost|Elec Ghost]] ×1, [[monsters/395-astarot|Astarot]] ×1'
  x: 5546.6
  y: 5126.5
  radius: 13
  max_alive: 1
  respawn_seconds: 1277
- monsters: '[[monsters/387-elec-ghost|Elec Ghost]] ×1, [[monsters/397-tirwin|Tirwin]] ×1, [[monsters/387-elec-ghost|Elec Ghost]] ×1, [[monsters/397-tirwin|Tirwin]] ×1, [[monsters/387-elec-ghost|Elec Ghost]] ×1'
  x: 5540.7
  y: 5127.1
  radius: 15
  max_alive: 1
  respawn_seconds: 149
- monsters: '[[monsters/487-tirwin|Tirwin]] ×1, [[monsters/477-tirwin|Tirwin]] ×1, [[monsters/488-tirwin|Tirwin]] ×1, [[monsters/397-tirwin|Tirwin]] ×1, [[monsters/497-tirwin|Tirwin]] ×1'
  reinforcements: '[[monsters/387-elec-ghost|Elec Ghost]] ×2, [[monsters/487-tirwin|Tirwin]] ×1'
  x: 5320.6
  y: 4904
  radius: 18
  max_alive: 6
  respawn_seconds: 39
warp_gates:
- to: '[[zones/54-crystal-snowfields|Crystal Snowfields]]'
  x: 5007.5
  y: 5126.4
zone_drops:
- '[[items/material/63-low-ether|Low Ether]]'
- '[[items/material/64-ether|Ether]]'
- '[[items/material/77-red-powder|Red Powder]]'
- '[[items/material/76-orange-powder|Orange Powder]]'
- '[[items/material/62-essence|Essence]]'
- '[[items/material/61-low-essence|Low Essence]]'
- '[[items/consumable/182-clan-point-2|Clan Point (+2)]]'
- '[[items/material/65-high-ether|High Ether]]'
- '[[items/material/78-golden-powder|Golden Powder]]'
- '[[items/consumable/183-clan-point-3|Clan Point (+3)]]'
- '[[items/material/66-iricer|Iricer]]'
- '[[items/material/79-transparent-powder|Transparent Powder]]'
- '[[items/consumable/184-clan-point-5|Clan Point (+5)]]'
- '[[items/consumable/185-clan-point-7|Clan Point (+7)]]'
- '[[items/material/161-green-crystal|Green Crystal]]'
- '[[items/consumable/186-clan-point-10|Clan Point (+10)]]'
- '[[items/material/162-blue-crystal|Blue Crystal]]'
- '[[items/material/163-red-crystal|Red Crystal]]'
- '[[items/material/164-white-crystal|White Crystal]]'
source:
  data: LIST_ZONE.STB row 59, the zone's IFO files, ITEM_DROP.STB row 59
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Luna Clan Field

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
