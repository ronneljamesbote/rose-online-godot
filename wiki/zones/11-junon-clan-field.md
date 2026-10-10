---
kind: zone
id: 11
name: Junon Clan Field
status: in-game
planet: Junon
pvp: yes, except your clan
monster_levels: 55–105
map:
  image: maps/11-junon-clan-field.jpg
  width: 448
  height: 448
  left: 4640
  top: 5760
  right: 5760
  bottom: 4640
monster_spawns:
- monsters: '[[monsters/218-golem-guardian|Golem Guardian]] ×1'
  x: 5084.3
  y: 5378.7
  radius: 13
  max_alive: 1
  respawn_seconds: 563
- monsters: '[[monsters/218-golem-guardian|Golem Guardian]] ×1'
  x: 5184.6
  y: 5367.4
  radius: 15
  max_alive: 1
  respawn_seconds: 522
- monsters: '[[monsters/215-aqua-guardian|Aqua Guardian]] ×1'
  x: 5345.8
  y: 5334
  radius: 15
  max_alive: 1
  respawn_seconds: 578
- monsters: '[[monsters/215-aqua-guardian|Aqua Guardian]] ×1'
  x: 5432.6
  y: 5355.9
  radius: 19
  max_alive: 1
  respawn_seconds: 757
- monsters: '[[monsters/217-krawfy-guardian|Krawfy Guardian]] ×1'
  x: 4993
  y: 5140.5
  radius: 19
  max_alive: 1
  respawn_seconds: 762
- monsters: '[[monsters/219-goblin-guardian|Goblin Guardian]] ×1'
  x: 5193.5
  y: 5197.6
  radius: 35
  max_alive: 1
  respawn_seconds: 410
- monsters: '[[monsters/219-goblin-guardian|Goblin Guardian]] ×1'
  x: 5147.7
  y: 5175.4
  radius: 16
  max_alive: 1
  respawn_seconds: 910
- monsters: '[[monsters/219-goblin-guardian|Goblin Guardian]] ×1'
  x: 5224.6
  y: 5248.1
  radius: 14
  max_alive: 1
  respawn_seconds: 1209
- monsters: '[[monsters/215-aqua-guardian|Aqua Guardian]] ×1'
  x: 5425.5
  y: 5194.1
  radius: 15
  max_alive: 1
  respawn_seconds: 490
- monsters: '[[monsters/217-krawfy-guardian|Krawfy Guardian]] ×1'
  x: 5075.3
  y: 5049
  radius: 16
  max_alive: 1
  respawn_seconds: 599
- monsters: '[[monsters/216-grunter-guardian|Grunter Guardian]] ×1'
  x: 5270.2
  y: 5058.9
  radius: 15
  max_alive: 1
  respawn_seconds: 799
- monsters: '[[monsters/216-grunter-guardian|Grunter Guardian]] ×1'
  x: 5176.4
  y: 5004.4
  radius: 15
  max_alive: 1
  respawn_seconds: 593
- monsters: '[[monsters/216-grunter-guardian|Grunter Guardian]] ×1'
  x: 5414.3
  y: 5047.7
  radius: 13
  max_alive: 1
  respawn_seconds: 456
npcs:
- npc: '[[npcs/1085-akram-minister-nel|Akram Minister Nel]]'
  x: 4978.5
  y: 5478.6
- npc: '[[npcs/1085-akram-minister-nel|Akram Minister Nel]]'
  x: 5455.6
  y: 5479
- npc: '[[npcs/1085-akram-minister-nel|Akram Minister Nel]]'
  x: 4953.4
  y: 4992.4
- npc: '[[npcs/1085-akram-minister-nel|Akram Minister Nel]]'
  x: 5521.9
  y: 5020.7
zone_drops:
- '[[items/material/74-blue-powder|Blue Powder]]'
- '[[items/material/75-pink-powder|Pink Powder]]'
- '[[items/material/76-orange-powder|Orange Powder]]'
- '[[items/consumable/181-clan-point-1|Clan Point (+1)]]'
- '[[items/consumable/182-clan-point-2|Clan Point (+2)]]'
- '[[items/material/77-red-powder|Red Powder]]'
- '[[items/material/78-golden-powder|Golden Powder]]'
- '[[items/consumable/183-clan-point-3|Clan Point (+3)]]'
- '[[items/material/162-blue-crystal|Blue Crystal]]'
- '[[items/material/164-white-crystal|White Crystal]]'
- '[[items/material/161-green-crystal|Green Crystal]]'
- '[[items/material/163-red-crystal|Red Crystal]]'
source:
  data: LIST_ZONE.STB row 11, the zone's IFO files, ITEM_DROP.STB row 11
  code:
  - module/src/world.rs
  - crates/rose-data-irose/src/zone_database.rs
---
# Junon Clan Field

Positions are in metres. Each spawn point keeps up to "max alive" monsters; the
reinforcements show up once a point has been refilled many times (see
[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see
[[rules/drops|Drops]]).
