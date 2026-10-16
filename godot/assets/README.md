# Game assets

Models and textures as Godot resources, in two folders with the same layout:

- `rose/` is made from the ROSE client data by the import command and is not committed.
- `ours/` holds edited and new art and is committed. A file here wins over the one with the
  same path in `rose/`.

A ROSE file's place in either folder is its client path in lower case with a Godot
extension: `3DDATA/AVATAR/MALE/BODY/BODY1_00.ZMS` is `3ddata/avatar/male/body/body1_00.mesh`
and `3DDATA/AVATAR/MALE/BODY/BODY1_00.DDS` is `3ddata/avatar/male/body/body1_00.res`. In
`ours/` a mesh may also be a `.res`, and a texture a `.png` or `.dds` (Godot imports those
when the project is opened in the editor).

## Making `rose/`

After building the extension (see `../README.md`), from the repository root:

```sh
godot --headless --path godot --import                     # once, so Godot knows the extension
godot --headless --path godot -- --import-rose --data-idx=/path/to/iRose_129_129/data.idx
```

It converts every ZMS mesh and DDS texture in `data.idx` except effects (`3DDATA/EFFECT`)
and the interface (`3DDATA/CONTROL`), which still load from the client data. Meshes keep
their bone weights; textures get the same mipmap fixes the game applies at run time and are
saved S3TC-compressed (DXT1, or DXT5 with alpha, as most ROSE textures already are), so
`rose/` is about 400 MB and looks the same as the client data. Running it again only converts what is
missing; `--import-rose=force` redoes everything. It also redoes everything after an update
that changes the converted output (`IMPORT_VERSION` in `rust/src/assets.rs`).

## Characters, monsters, NPCs and carts

The import also builds the characters from the converted meshes:

| File | What it is |
| --- | --- |
| `3ddata/avatar/male.tscn`, `female.tscn` | The skeleton (`Skeleton3D`, bones `b0`... and dummy bones `d0`...) and an `AnimationPlayer` with the no-weapon motions. |
| `3ddata/motion/avatar/*.anim` | Every player, cart and castle gear motion. Hit frames, footsteps and effect frames are markers on the timeline. |
| `3ddata/motion/npc/**/*.anim` | Every monster and NPC motion, with the same markers. |
| `3ddata/avatar/male_motions/weapon_N.tres` (and `female_motions`) | One animation library per weapon motion type (the column of `TYPE_MOTION.STB`): stop1, run, attack, attack2, attack3, hit, die... |
| `3ddata/avatar/list_mbody/12.tscn` and so on | One scene per part in each part list (faces, hair, caps, bodies, arms, feet; `3ddata/weapon/list_weapon` and `list_subwpn` for weapons and shields), named by the item's model number. Each mesh instance says whether it is `skinned` or which `bone` it hangs on. |
| `3ddata/npc/list_npc/12.tscn` and so on | One scene per monster or NPC model (the `LIST_NPC.CHR` entry, the same number as the monster or NPC): its skeleton, its parts and an `AnimationPlayer` with stop, move, attack, hit, die and run. Weapons and the size come from the monster data when it spawns. |
| `3ddata/pat/cart/cart01.tscn`, `3ddata/pat/castlegear/castlegear02/castlegear02.tscn` | The cart and castle gear skeletons with the driver's `Seat` and, on carts, the passenger's `BackSeat`. |
| `3ddata/pat/list_pat/1.tscn` and so on | One scene per cart and castle gear part (body, engine, wheels, arms), like the player parts. |

Markers are named `kind` + number + `:` + the ROSE event id, for example `hit1:21`. `hit`
markers are when a swing or skill lands (event ids 10, 20-28, 56, 57, 66, 67, the same ones
the server's `attack_hit_ms` uses), `step` markers play footsteps and `effect` markers the
other sounds and effects. Moving a marker in the Animation panel moves that event in game;
the id after the colon must stay.

The server times hits from `overrides/motions.toml`, so after moving a `hit` marker (or
changing an animation's length) export it and commit it with the animation:

```sh
godot --headless --path godot -- --export-motions          # writes ../overrides/motions.toml
godot --headless --path godot -- --export-motions=check    # fails when it is out of date
```

The Windows build (`tools/build-windows.sh`) runs the check and refuses to build while the
file is out of date. An edited animation or scene goes in `ours/` under the same path (for example
`ours/3ddata/motion/avatar/onehand_attack01_m1.anim` or `ours/3ddata/npc/list_npc/1.tscn`),
since `rose/` is remade by the import. Monsters often share an animation file, so moving a
marker in one moves it for all of them, as in iROSE.

## Zones

Zone objects and ground textures are converted by the same import:

| File | What it is |
| --- | --- |
| `3ddata/junon/list_deco_jg/12.tscn`, `list_cnst_jg/1.tscn` and so on (and the event objects a zone uses) | One scene per building, decoration or event object (`RoseZoneObject`): its mesh parts with their collision and its effects. Placed copies get their lightmap from the block. |
| `3ddata/maps/junon/jg07/terrain_textures/12.res` and so on | The ground textures of each zone that has scenes, by tile texture number. |

The zones themselves are scenes in `res://zones/`, one folder per zone (`20_birth_island/`, `22_adventurer_s_plain/`),
committed to the repository. They are written once from the client's map files:

```sh
godot --headless --path godot -- --import-zone=20 --data-idx=/path/to/data.idx
godot --headless --path godot -- --import-zone=20,force --data-idx=...   # replace edited scenes
```

| File | What it is |
| --- | --- |
| `zone.tscn` | The zone: every block, the ground (`Terrain`, a Terrain3D node with a `RoseTerrainLook` child holding the block lightmaps) and the zone's events. |
| `block_32_34.tscn` and so on | One 160 m block: `Water`, `Buildings`, `Decorations`, `EventObjects`, `Warps`, `Effects`, `Sounds`, `Spawns` and `Npcs`. Spawns and NPCs are marker nodes that show the monster or NPC in the editor. |
| `terrain/`, `terrain_assets.tres` | The Terrain3D heights, tile textures and holes (1280 m regions, 2.5 m between vertices like ROSE). |

Open `zone.tscn` in the editor to change the zone: move, add or delete objects and markers
in the block scenes, and select `Terrain` to sculpt, paint ground textures or cut holes with
the Terrain3D tools. Painting uses the overlay texture of each 10 m ROSE tile; the textures
in the Terrain3D list are the zone's tile textures in order. Terrain3D holds 32 textures; a
zone that uses more (Adventurer's Plain uses 40) puts the 32 used on most tiles in the
Terrain3D list and the rest in `RoseTerrainLook`'s `extra_textures`, marked by two control
bits Terrain3D doesn't use. Those extra textures can't be picked for painting, and painting
over a tile that shows one turns it into the Terrain3D texture with the same place in the
list, so repaint such tiles with the texture you want. The game loads the scenes
when the zone has a folder here and its converted assets exist, and builds the zone from
the client's map files otherwise (or with `--zone-files`), so both always work.

The server doesn't read the scenes: after moving or changing spawn, NPC or event markers,
export them to `overrides/zones/<id>-<name>.toml` (`20-birth-island.toml`) with `--export-zones` (see
`../../overrides/README.md`) and commit it with the scenes. Warp gates stay client-side, as
before: the server sends a player through a gate by its warp number.

The game falls back to the client data for anything not in either folder, so it runs the
same with or without `rose/`.

## In the Windows build

`tools/build-windows.sh` runs the import, so the export puts all of `rose/`, `ours/` and the
zone scenes into `ROSE.pck`, next to `ROSE.exe` in the zip (about 400 MB; the zip was 44 MB
before). Players still need the client data (`data.idx`) for the game tables, effects, the
interface and the zones without scenes.
