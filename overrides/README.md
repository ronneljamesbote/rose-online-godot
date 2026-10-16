# Game data overrides

`game-data.toml` holds every change we make to the iROSE game data (item stats, monster
stats, skill numbers, drop tables, shop lists, ...). The original client files stay as
they are; this file is laid on top of them.

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
and named spots (start, revive and warp targets). Birth Island (`zones/20-birth-island.toml`)
and Adventurer's Plain (`zones/22-adventurer-s-plain.toml`) have one so far. It isn't edited
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
file = "3DDATA/STB/LIST_WEAPON.STB"   # path inside the client data, any case
row = 2                               # Short Sword
column = 35                           # attack power
value = 12                            # a number or "text"
note = "Short Sword hits a bit harder, wiki/items/weapon/2-short-sword.md"
```

`row` is the number the wiki shows as the thing's id (item, skill, monster number).
`column` counts from 0 at the first column after the row name, the same way the data
loaders in `crates/rose-data-irose/src/` number them (`stb_column! { 35, get_weapon_attack_power, ... }`).

## Useful columns

Items (`LIST_WEAPON`, `LIST_SUBWPN`, `LIST_CAP`, `LIST_BODY`, `LIST_ARMS`, `LIST_FOOT`,
`LIST_BACK`, `LIST_FACEITEM`, `LIST_JEWEL`, `LIST_USEITEM`, `LIST_JEMITEM`, `LIST_NATURAL`,
`LIST_QUESTITEM`, `LIST_PAT`, all in `3DDATA/STB/`):

| Column | Meaning |
| --- | --- |
| 5 | base price |
| 7 | weight |
| 8 | quality |
| 16 | job class needed |
| 19 / 20, 21 / 22 | needed stat type / value (two pairs) |
| 24 / 25, 27 / 28 | added stat type / value (two pairs) |
| 29 | durability |
| 31 | defence (armour) |
| 32 | magic resistance (armour) |
| 33 | weapons: attack range (cm); feet and back: move speed |
| 35 | weapons: attack power |
| 36 | weapons: attack speed |

Monsters and NPCs (`3DDATA/STB/LIST_NPC.STB`):

| Column | Meaning |
| --- | --- |
| 2 / 3 | walk / run speed |
| 7 | level |
| 8 | HP |
| 9 | attack |
| 10 | hit |
| 11 | defence |
| 12 | magic resistance |
| 13 | avoid |
| 14 | attack speed |
| 17 | XP |
| 18 | drop table row (`ITEM_DROP.STB`) |
| 19 / 20 | money drop rate / item drop rate |
| 26 | attack range (cm) |

Skills (`3DDATA/STB/LIST_SKILL.STB`, one row per skill level):

| Column | Meaning |
| --- | --- |
| 3 | skill points to learn |
| 6 | cast range (cm) |
| 9 | power |
| 13 | success rate |
| 14 | effect duration (ms) |
| 16 / 17, 18 / 19 | cost type / amount (two pairs) |
| 20 | cooldown (units of 5 ms) |
| 21 / 22, 24 / 25 | added stat type / value (two pairs; 23 and 26 are their rates) |

The full column lists are the `stb_column!` lines in `crates/rose-data-irose/src/`.
