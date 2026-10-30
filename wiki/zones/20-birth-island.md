---
kind: zone
id: 20
name: Birth Island
status: changed-from-irose
planet: Junon
pvp: no
map:
  image: maps/20-birth-island.jpg
  width: 448
  height: 448
  left: 4800
  top: 5600
  right: 5920
  bottom: 4480
zone_drops:
- '[[items/material/31-twig|Twig]]'
- '[[items/consumable/101-apple|Apple]]'
- '[[items/consumable/102-banana|Banana]]'
- '[[items/material/32-maple-wood|Maple Wood]]'
- '[[items/consumable/57-vital-jam-2|Vital Jam (+2)]]'
- '[[items/material/33-aromatic-wood|Aromatic Wood]]'
- '[[items/consumable/120-milk|Milk]]'
- '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
source:
  data: the zone settings godot/data/zones/20-birth-island.tres, export/zones/20-birth-island.toml (exported from the scenes in godot/zones/20_birth_island), drop table godot/data/drop_tables/20-zone-birth-island.tres
  code:
  - module/src/world.rs
  - crates/rose-game-data/src/lib.rs (apply_zone_layouts)
  - crates/rose-data-irose/src/zone_database.rs
---
# Birth Island

**Birth Island is for testing only.** New characters start in
[[zones/22-adventurer-s-plain|Adventurer's Plain]] (see [[rules/accounts|Accounts]]), no warp
gate or quest leads here, and the island has no monsters and no NPCs. A GM on the dev server
reaches it with `/tp 20` (see [[rules/gm-commands|GM commands]]) and can place monsters with
`/spawn`. Its start and revive spots still work, so a character that dies here gets up on the
island; `/tp 22` leaves it.

The zone's drop table is kept: a monster a GM spawns here can drop this zone's zone drops.

Birth Island is edited in Godot: its start and revive spots are markers in the zone's
scenes, and the server reads them from `export/zones/20-birth-island.toml`, which is
exported from those scenes.

## Changed from iROSE

In iROSE every new character starts here, among 81 spawn points of Jelly Beans, Choropies
and ButterFlies (levels 1 to 6), with ten Fairies of Arua (NPC 1030) who give the Hunting
Practice quests and send you to Adventurer's Plain when you are ready. In our game the spawn
points and the fairies are gone (removed from the zone's scene on 2026-10-30) and new
characters start in Adventurer's Plain with what the fairy handed over on leaving: the
[[skills/19-party|Party]] and [[skills/21-vending|Vending]] skills and the
[[quests/801-junon-planet-identification|Junon Planet Identification]] quest with its
identification papers.
