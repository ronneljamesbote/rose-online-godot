# Game assets

Models, textures, sounds and effects as Godot resources, in two folders with the same layout:

- `rose/` is made from the ROSE client data by the import command and is not committed.
- `ours/` holds edited and new art and is committed. A file here wins over the one with the
  same path in `rose/`.

A ROSE file's place in either folder is its client path in lower case with a Godot
extension: `3DDATA/AVATAR/MALE/BODY/BODY1_00.ZMS` is `3ddata/avatar/male/body/body1_00.mesh`
and `3DDATA/AVATAR/MALE/BODY/BODY1_00.DDS` is `3ddata/avatar/male/body/body1_00.rtex`. In
`ours/` a mesh may also be a `.res`, and a texture a `.png` or `.dds` (Godot imports those
when the project is opened in the editor).

## Making `rose/`

After building the extension (see `../README.md`), from the repository root:

```sh
godot --headless --path godot --import                     # once, so Godot knows the extension
godot --headless --path godot -- --import-rose --data-idx=/path/to/iRose_129_129/data.idx
```

It converts every ZMS mesh and DDS texture in `data.idx` except the interface
(`3DDATA/CONTROL`), which still loads from the client data apart from the icons (see
"Icons and ground models" below); every effect (the 351 EFT files)
into an effect scene (`3DDATA/EFFECT/LEVELUP_01.EFT` is `3ddata/effect/levelup_01.scn`, a
`RoseEffect` with a node per part that links the imported meshes, textures and particle
systems, its motions as Godot `Animation`s) and every PTL particle system into a `.particles.res`
(a `RoseParticleSystem`), which the effect resources (`../data/README.md`) and the zones'
effect spots link (one EFT, `RAINBOW01.EFT`, can't be read and is left out); and the 315 WAV
and OGG files of the client's `Sound/` folder (loose files next to `data.idx`): a sound
becomes Godot's own `AudioStreamWAV` or `AudioStreamOggVorbis` in a `.res`
(`Sound/attack/Sword_001.wav` is `sound/attack/sword_001.res`), which the sound resources
(`../data/README.md`) and the zones' music link. In `ours/` a sound may also be a `.wav` or
`.ogg`, and an effect a `.tscn`. Meshes keep
their bone weights. A texture becomes a `.rtex`: the client's DDS file as it is, in a
compressed file, which `scripts/rose_texture_loader.gd` opens as an `ImageTexture` in the
game and in the editor (the inspector and the FileSystem thumbnails show it like any
texture). Most ROSE textures declare mipmaps they don't store; for those the DDS is cut to
the level it has and the loader makes the mipmaps from the decompressed image, the same
way the game treats the client files, so they look exactly like the client data. An
uncompressed DDS is kept as lossless WebP instead when that is smaller and decodes to the
same pixels. `rose/` is about 413 MB (88 MB of it sounds, 8 MB effects) and the import takes about 35 seconds. (Earlier
imports saved every mip level as WebP, 840 MB in 5 minutes, or re-compressed to S3TC,
400 MB but with blue and grey marks along the UV seams of skin and blocky skies.) Running
it again only converts what is missing; `--import-rose=force` redoes everything. It also
starts `rose/` afresh after an update that changes the converted output (`IMPORT_VERSION`
in `rust/src/assets.rs`).

### The editor leaves `rose/` out

The import puts an empty `.gdignore` in `rose/`, so the Godot editor doesn't list the
converted files. In its tree view the editor's FileSystem dock makes a thumbnail of every
file in the project each time it opens, even in folders nobody expands, and for the
16,500 converted files that means loading each one: more than six minutes and 2 GB the
first time and after every import, until the editor stops responding. Without them it opens
in about 15 seconds.

Everything links the converted files by path, so zone scenes, the game data resources, the
game and `--export-data` load them as before. What you lose is browsing and dragging them
in the FileSystem dock and their thumbnails. To link one in the Inspector, use the
property's **Load**: its file dialog still shows the folder. The build scripts
take the `.gdignore` away while they export, because Godot's export would leave the folder
out too, and put it back at the end. Close the editor during a build, or it rescans the folder.

### Troubleshooting the import

- **It ran the old import.** The import is code in the extension (`bin/rose_gd.dll`, or
  `librose_gd.so`), so pulling a change to `rust/src/` does nothing until the extension is
  rebuilt. Rebuild it first (`cargo build --release -p rose-gd` and copy it into `bin/`, or
  run `tools/build-windows.ps1`, which builds it and then runs the import). The size on the
  last line tells which import ran: `assets/rose is` about 413 MB is the current one (about
405 MB before the effects were imported, 320 MB before the sounds); about
  840 MB (after about 5 minutes) or about 400 MB are older ones, and the zone scenes then
  show grey ground and untextured objects.
- **It looks stuck on Windows.** When `godot` is the editor exe rather than the
  `_console.exe`, PowerShell doesn't wait for it: the prompt comes back first and Godot's
  output prints after it. The import is done once `assets/rose is ... MB` appears; press
  Enter to get the prompt back. Run the console exe, or `build-windows.ps1`, to have
  PowerShell wait.
- **`DDS width '2' is not divisible by 4` warnings** come from a few tiny ROSE textures.
  Godot loads them anyway; they are harmless.

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

The server times hits from `export/motions.toml` (it reads no ZMO file), so after moving a
`hit` marker (or changing an animation's length) export it and commit it with the animation:

```sh
godot --headless --path godot -- --export-data          # writes ../export/ (motions.toml too)
godot --headless --path godot -- --export-data=check    # fails when it is out of date
```

The Windows build (`tools/build-windows.sh`) runs the check and refuses to build while the
file is out of date. An edited animation or scene goes in `ours/` under the same path (for example
`ours/3ddata/motion/avatar/onehand_attack01_m1.anim` or `ours/3ddata/npc/list_npc/1.tscn`),
since `rose/` is remade by the import. Monsters often share an animation file, so moving a
marker in one moves it for all of them, as in iROSE.

## Icons and ground models

The game data (`../data/README.md`) links each item's, skill's and status effect's icon and
each model by path, so the import makes those files too:

| File | What it is |
| --- | --- |
| `3ddata/control/res/item1.rtex`, `skillicon.rtex`, `stateicon.rtex` | The icon sheets (the TSI files' textures). |
| `3ddata/control/res/item1/919.tres` and so on | One `AtlasTexture` per icon, a region of its sheet: 2,366 item icons, 338 skill icons, 168 status effect icons. It is a few hundred bytes and loads only its sheet, so the Inspector shows its thumbnail without slowing the editor. |
| `3ddata/item/list_fielditem/12.tscn` and so on | One scene per item model lying on the ground (`LIST_FIELDITEM.ZSC`, 441 of them). |

New art goes in `ours/` under any path, for example `ours/items/my_sword.png` for an icon (any
texture works, a whole picture or an `AtlasTexture` of a sheet) or `ours/items/my_sword.tscn`
for a model built like the imported part scenes (each `MeshInstance3D` with `skinned` or a
`bone`). Link it from the item, monster or skill (`../data/README.md`). A model scene of our
own has no gem point yet (weapons draw a socketed gem at the point the ZSC list names) and a
monster takes its motions (and the server its hit timing) from its model's `LIST_NPC.CHR`
entry, so a new monster reuses an imported model, or an edited copy under the same path in
`ours/` (`ours/3ddata/npc/list_npc/12.tscn`); a model elsewhere has no motions yet.

## Zones

Zone objects and ground textures are converted by the same import:

| File | What it is |
| --- | --- |
| `3ddata/junon/list_deco_jg/12.tscn`, `list_cnst_jg/1.tscn` and so on (and the event objects a zone uses) | One scene per building, decoration or event object (`RoseZoneObject`): its mesh parts with their collision and its effects. Placed copies get their lightmap from the block. |
| `terrain_textures/3ddata/terrain/tiles/junon/jg/t001_01.res` and so on | The ground textures of the zones that have scenes, as Terrain3D takes them (256 x 256 RGBA with mipmaps), named after the ROSE tile file, so zones using the same tile share one. |

The zones themselves are scenes in `res://zones/`, one folder per zone (`1_canyon_city_of_zant/`, `20_birth_island/`, `21_valley_of_luxem_tower/`, `22_adventurer_s_plain/`, `23_breezy_hills/`),
committed to the repository. They are written once from the client's map files:

```sh
godot --headless --path godot -- --import-zone=20 --data-idx=/path/to/data.idx
godot --headless --path godot -- --import-zone=20,force --data-idx=...   # replace edited scenes
```

| File | What it is |
| --- | --- |
| `zone.tscn` | The whole zone. Its root is a `RoseZoneScene` (the zone number in the Inspector) with one `RoseBlock` per 160 m block (`Block_32_34`: its `block_x` and `block_y`, then `Water`, `Buildings`, `Decorations`, `EventObjects`, `Warps`, `Effects`, `Sounds`, `Spawns` and `Npcs`), the ground (`Terrain`, a Terrain3D node with a `RoseTerrainLook` child holding the block lightmaps) and the zone's events. Spawns and NPCs are marker nodes that show the monster or NPC in the editor. |
| `terrain/`, `terrain_assets.tres` | The Terrain3D heights, tile textures and holes (1280 m regions, 2.5 m between vertices like ROSE). |

Zones converted before 2026-10-18 kept each block in its own `block_X_Y.tscn`. On a branch
that still has those, merge them into `zone.tscn` once (after `--import-rose`; it changes no
node, name or position, so `--export-data=check` still passes):

```sh
godot --headless --path godot -- --merge-zone-blocks=24      # or =all
```

Open `zone.tscn` in the editor to change the zone: move, add or delete objects and markers
in its blocks, and select `Terrain` to sculpt, paint ground textures or cut holes with
the Terrain3D tools. Objects and markers sit at their ROSE world positions (blocks start
about 5 km from Godot's origin) and belong in the block they stand in; a block shows a
warning in the Scene panel when something stands outside it. The ROSE Zone Tools plugin
(`addons/rose_zone_tools/`) helps there:

- **Go to** (3D toolbar): the start spot, revive spots, any block, NPC (by name), spawn or
  warp gate, a search over all of them, or a ROSE x, y (`/tp 20 5300 5300` works too).
  Opening a zone with the camera far away puts it over the start spot, and selecting a
  block in the Scene panel (or **F** with a block selected) frames the block.
- The bottom left of the 3D view shows the point under the mouse in ROSE coordinates, the
  metres `/tp` and the server use (x east, y north), its block and the `/tp` line.
- **Zone view**: block outlines and names on the ground (on by default), the 10 m ground
  tiles that painting follows, and Redraw after sculpting.
- **Drop to ground** puts the selected models and markers on the ground under them (undo
  with Ctrl+Z).
- Terrain3D's toolbar hides the tools our ground shader ignores (Autoshader, Paint color,
  Wetness, Navigation, Instancer); the eye button at its bottom, or Zone view > Show all
  terrain tools, brings them back. Painting uses the overlay texture of each 10 m ROSE tile; the textures
in the Terrain3D list are the zone's tile textures in order. Terrain3D holds 32 textures; a
zone that uses more (Valley of Luxem Tower uses 39, Adventurer's Plain 40) puts the 32 used on most tiles in the
Terrain3D list and the rest in `RoseTerrainLook`'s `extra_textures`, marked by two control
bits Terrain3D doesn't use. Those extra textures can't be picked for painting, and painting
over a tile that shows one turns it into the Terrain3D texture with the same place in the
list, so repaint such tiles with the texture you want. The game loads the scenes
when the zone has a folder here and its converted assets exist, and builds the zone from
the client's map files otherwise (or with `--zone-files`), so both always work.

The server doesn't read the scenes: after moving or changing spawn, NPC or event markers,
export them to `export/zones/<id>-<name>.toml` (`20-birth-island.toml`) with `--export-data` (see
`../../export/README.md`) and commit it with the scenes. A zone scene needs the zone's
settings (`godot/data/zones/`, the `settings` field of its root shows them), or the export
stops. Warp gates stay client-side, as before: the server sends a player through a gate by
its warp number, and the gate's `RoseWarpGate` (`godot/data/warp_gates/`, the `warp_gate`
field of the placed gate) says where it leads.

The game falls back to the client data for anything not in either folder, so it runs the
same with or without `rose/`.

## In the Windows build

`tools/build-windows.sh` runs the import and exports two packs next to `ROSE.exe`:
`ROSE.pck` with the scripts, the interface, `ours/` and the zone scenes (about 5 MB), and
`assets.pck` with all of `rose/` (about 320 MB, the "ROSE assets" export preset). Godot
opens `ROSE.pck` by itself, as it is named after the exe; the extension opens
`assets.pck` at start (`load_assets_pack` in `rust/src/assets.rs`), and without it the game
loads models, textures and zones from the client data. `--game-only` builds an update
without `assets.pck`. Players still need the client data (`data.idx`) for the game tables,
effects, the interface and the zones without scenes.
