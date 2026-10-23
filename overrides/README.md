# Game data overrides

`data/` is the game data made in the Godot editor, exported from the resources in
`godot/data/` (see `godot/data/README.md`): the items (`data/items/<class>.toml`, all item
tables and the refine grades), the monsters (`data/monsters.toml`), the town NPCs
(`data/npcs.toml`), the drop tables (`data/drop-tables.toml`), the shop tabs
(`data/shop-tabs.toml`), the skills (`data/skills.toml`), the status effects
(`data/status-effects.toml`) and the job classes (`data/job-classes.toml`). It isn't edited
by hand: change the resource, then run

```sh
godot --headless --path godot -- --export-data          # writes overrides/data/
godot --headless --path godot -- --export-data=check    # fails when it is out of date
```

The server's upload sends it in place of the iROSE tables it replaces (the item tables,
LIST_NPC.STB, ITEM_DROP.STB, LIST_SELL.STB, LIST_SKILL.STB, LIST_STATUS.STB and
LIST_CLASS.STB; as JSON, which the module
reads faster; `ROSE_DATA` names another folder), and the game reads it from here (from
`overrides/data/` next to `ROSE.exe` in the Windows build, or where `ROSE_DATA` points).
Without it both fall back to the iROSE tables.

`game-data.toml` holds every change we make to the iROSE game data that isn't made in Godot
yet (zone settings, crafting recipes, new characters, ...). The original client files stay
as they are; this file is laid on top of them. Items, monsters, NPCs, drop tables, shops,
skills, status effects and job classes are not in it any more: an entry for their tables
(`LIST_WEAPON.STB`, `LIST_GRADE.STB`, `LIST_NPC.STB`, `ITEM_DROP.STB`, `LIST_SELL.STB`,
`LIST_SKILL.STB`, `LIST_STATUS.STB`, `LIST_CLASS.STB` ...) stops the client and the upload
with an error naming the resources to edit instead.

- **The game client** reads it when it loads its data. In the Windows zip it sits at
  `overrides/game-data.toml` next to `ROSE.exe`. During development the client finds it
  in the repository (`../overrides/game-data.toml` from `godot/`), or wherever
  `ROSE_OVERRIDES` points.
- **The server** gets the changed files in its game data upload: `rose-stdb-import pack`
  (run by `server/upload-game-data.sh`) applies the file before it packs the tables. The
  Docker server keeps the file inside its image and uploads the game data again by itself
  when the file changed (it remembers the last uploaded version in `/data/overrides.sha256`).

`motions.toml` holds the length and hit frames of every player, monster and NPC motion, exported from the
hit markers of the Godot animations (`godot/assets/README.md`). It isn't edited by hand:
move a `hit` marker in the Godot editor, then run

```sh
godot --headless --path godot -- --export-motions          # writes overrides/motions.toml
godot --headless --path godot -- --export-motions=check    # fails when it is out of date
```

The server times every swing, monster attack and skill hit from it (the upload sends it with the game
files, `ROSE_MOTIONS` names another file); the client plays the markers themselves. A
motion missing from the file keeps the time in its ZMO file.

`zones/<id>-<name>.toml` holds what the server needs from a zone that is edited as Godot
scenes (`godot/zones/`, see `godot/assets/README.md`): its monster spawn points, town NPCs
and named spots (start, revive and warp targets). Canyon City of Zant
(`zones/1-canyon-city-of-zant.toml`), Birth Island (`zones/20-birth-island.toml`),
Valley of Luxem Tower (`zones/21-valley-of-luxem-tower.toml`), Adventurer's Plain
(`zones/22-adventurer-s-plain.toml`) and Breezy Hills (`zones/23-breezy-hills.toml`) have one
so far. It isn't edited
by hand either: move or change the markers in the Godot editor, then run

```sh
godot --headless --path godot -- --export-zones          # writes overrides/zones/*.toml
godot --headless --path godot -- --export-zones=check    # fails when one is out of date
```

The server uses the file in place of the zone's IFO and ZON data (the upload sends it,
`ROSE_ZONES` names another folder). Positions are metres: `x` east, `y` north, `z` up. A
zone without a file keeps its client data. The upload stops with an error naming the zone
when a file names a monster or NPC that doesn't exist.

So after changing any of these files: rebuild the server image (`docker compose up -d --build`) and
ship the file with the client. Both sides must have the same file, or the client shows one
number while the server uses another.

## Rules

- Every entry belongs to a wiki page. The page shows the new value, its `note` names the
  page, and the change is described under "Changed from iROSE" on that page.
- Only change what the wiki asks for. A value the code computes (a formula) is changed in
  code, not here.
- A mistake in the file (unknown key, a row or column outside the table) stops the client
  and the upload with an error naming the entry, so a typo can't slip through unnoticed.

## Format

```toml
[[stb]]
file = "3DDATA/STB/LIST_ZONE.STB"     # path inside the client data, any case
row = 22                              # the zone's row (its id)
column = 18                           # its PvP state
value = 0                             # a number or "text"
note = "No PvP here, wiki/zones/<the zone's page>.md"
```

`row` is the number the wiki shows as the thing's id.
`column` counts from 0 at the first column after the row name, the same way the data
loaders in `crates/rose-data-irose/src/` number them (`stb_column! { 8, get_health_points, ... }`).

## Useful columns

Items, monsters, NPCs, drop tables, shop tabs, skills, status effects and job classes are
Godot resources now (`godot/data/`), edited in the Inspector by field name.

Zones (`3DDATA/STB/LIST_ZONE.STB`, one row per zone):

| Column | Meaning |
| --- | --- |
| 2 / 3 | start / revive spot (the name of a spot in the zone) |
| 13 to 17 | day cycle length, then when morning, day, evening and night start |
| 18 | PvP state |
| 19 | planet |
| 28 / 29 | party XP values |

The full column lists are the `stb_column!` lines in `crates/rose-data-irose/src/`.
