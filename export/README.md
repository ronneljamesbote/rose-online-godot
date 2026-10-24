# What the Godot editor exports

Everything here is written by one command from what is made in the Godot editor, and read by
the server and the game. None of it is edited by hand: change the source in Godot, then run

```sh
godot --headless --path godot -- --export-data          # writes export/
godot --headless --path godot -- --export-data=check    # fails when something is out of date
```

(headless, with the ROSE client data: `--data-idx=/path/to/data.idx` or `ROSE_DATA_IDX`, after
`--import-rose`). Commit the changed files together with what they were exported from. Both
Windows build scripts run the check and refuse to build when it fails.

| Here | Exported from | What the server and the game do with it |
| --- | --- | --- |
| `data/` | the resources in `godot/data/` (see `godot/data/README.md`) | read in place of the iROSE tables |
| `zones/<id>-<name>.toml` | the zone scenes in `godot/zones/` | the server places spawns, NPCs and named spots from it |
| `motions.toml` | the hit markers of the Godot animations | the server times hits from it |

## data/

The game data: the items (`data/items/<class>.toml`, all item tables and the refine
grades), the monsters (`data/monsters.toml`), the town NPCs (`data/npcs.toml`), the drop
tables (`data/drop-tables.toml`), the shop tabs (`data/shop-tabs.toml`), the skills
(`data/skills.toml`), the status effects (`data/status-effects.toml`), the job classes
(`data/job-classes.toml`), the zones' settings (`data/zones.toml`: name, map file, start and
revive spots, music, sky, minimap, time of day, PvP, planet, carts), the warp gates
(`data/warp-gates.toml`), the crafting and refining recipes (`data/craft-recipes.toml`),
what a new character starts with (`data/new-characters.toml`), the sounds
(`data/sounds.toml`: each sound's audio file, `data/hit-sounds.toml`, `data/footsteps.toml`)
the effects (`data/effect-files.toml`: each effect's scene, `data/attack-effects.toml`:
how attacks look), the skies (`data/skies.toml`: each sky's mesh, day and night picture and
light at each time of day) and the interface (`data/interface.toml`: the loading and
character creation pictures, each mouse cursor and the pixel it clicks at) and the quests
(`data/quests.toml`: each quest's name, text and time limit; `data/quest-triggers.toml`:
every step with its conditions, rewards and next step).

The server's upload sends it in place of the iROSE tables it replaces (the item tables,
LIST_NPC.STB, ITEM_DROP.STB, LIST_SELL.STB, LIST_SKILL.STB, LIST_STATUS.STB, LIST_CLASS.STB,
LIST_ZONE.STB, WARP.STB, LIST_PRODUCT.STB, INIT_AVATAR.STB, LIST_QUEST.STB and the QSD files; as JSON, which the module
reads faster; `ROSE_DATA` names another folder; the sounds, effects, skies and interface stay
out, the server plays and shows none), and the game reads it from here (from
`export/data/` next to `ROSE.exe` in the Windows build, or where `ROSE_DATA` points). Without
it both fall back to the iROSE tables (FILE_SOUND.STB, LIST_HITSOUND.STB and
LIST_STEPSOUND.STB for the sounds, FILE_EFFECT.STB and LIST_EFFECT.STB for the effects,
LIST_SKY.STB and the client's own loading picture and cursor files for the skies and
interface).

Icons and models are Godot paths (`icon = "res://assets/rose/3ddata/control/res/item1/919.tres"`,
`field_model`, `model`, or `model_male` and `model_female` for hats, bodies, gloves and shoes, and
a monster's or NPC's `model`), which the game loads from its `assets/`; empty means none. The
server only keeps them, except that a monster's or NPC's motions follow its model's
`LIST_NPC.CHR` entry, so a model change can change its attack timing.

Every number the game reads from these tables is a field of a resource now, so changing the
game data means changing a resource (or code, for a rule). There is no file of changes laid on
top of the iROSE tables any more (`game-data.toml` before 2026-10-23): the original client
files stay as they are, and the wiki keeps their old values under "Changed from iROSE".

## zones/

`zones/<id>-<name>.toml` holds what the server needs from a zone that is edited as Godot
scenes (`godot/zones/`, see `godot/assets/README.md`): its monster spawn points, town NPCs
and named spots (start, revive and warp targets). Canyon City of Zant
(`zones/1-canyon-city-of-zant.toml`), Birth Island (`zones/20-birth-island.toml`), Valley of
Luxem Tower (`zones/21-valley-of-luxem-tower.toml`), Adventurer's Plain
(`zones/22-adventurer-s-plain.toml`) and Breezy Hills (`zones/23-breezy-hills.toml`) have one
so far. The server uses the file in place of the zone's IFO and ZON data (the upload sends
it, `ROSE_ZONES` names another folder). Positions are metres: `x` east, `y` north, `z` up. A
zone without a file keeps its client data. The upload stops with an error naming the zone
when a file names a monster or NPC that doesn't exist. A zone scene needs settings
(`godot/data/zones/`) with its number, or the export stops.

## motions.toml

The length and hit frames of every player, monster and NPC motion, from the hit markers of
the Godot animations (`godot/assets/README.md`): move a `hit` marker in the Godot editor, then
export. The server times every swing, monster attack and skill hit from it (the upload sends
it with the game files, `ROSE_MOTIONS` names another file); the client plays the markers
themselves. The server reads no ZMO file: a motion missing from the file has no animation
there, as when its ZMO file is missing, and a motion in the file that no player, monster or
NPC plays stops the upload. `rose-stdb-import compare` checks every length and hit frame
against the ZMO files.

## After a change

Rebuild the server image (`docker compose up -d --build`; it uploads the game data again when
a file here changed) and ship the game with the new `export/data/` (the Windows build carries
it next to `ROSE.exe`). Both sides must have the same files, or the client shows one number
while the server uses another. `ROSE_EXPORT` names another export folder for the upload
(`server/upload-game-data.sh`).

To list what the export changes from iROSE:
`cargo run --release -p rose-stdb-import -- compare /path/to/data.idx`.
