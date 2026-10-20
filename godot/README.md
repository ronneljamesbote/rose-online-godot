# Godot client

The game client, in Godot 4 with a Rust GDExtension (`rust/`, crate `rose-gd`, built with
gdext 0.5.5) that reads the original ROSE data files through the vendored rose-offline
crates (`../crates/`). It plays online on the SpacetimeDB server (`../module`) and signs in through the account
website (`../web`). The game it shows (controls, windows, combat, skills, NPCs, zones) is
described in `../wiki/`, starting with `../wiki/rules/controls.md`. Zones load either from
their Godot scenes in `zones/` (Birth Island, Adventurer's Plain and Valley of Luxem Tower
so far) or straight from the client's map files, and models, textures and animations
either from the converted Godot resources in `assets/` or from the client files.

## Build and run

```sh
# from the repository root
cargo build --release -p rose-gd
mkdir -p godot/bin && cp target/release/librose_gd.so godot/bin/    # rose_gd.dll on Windows
godot --path godot -- --data-idx=/path/to/iRose_129_129/data.idx
```

Optionally convert the client's models and textures into Godot resources first (see
`assets/README.md`); the game then loads those and only falls back to the client files for
the rest:

```sh
godot --headless --path godot --import
godot --headless --path godot -- --import-rose --data-idx=/path/to/iRose_129_129/data.idx
```

Godot 4.7 or newer. `ROSE_DATA_IDX` works instead of `--data-idx`. The start screen offers
Sign in (online) or Play offline. In game: left-click to move, S to stop, right-drag to
orbit the camera, mouse wheel to zoom. Online, left-click a monster to attack it and Space
attacks the nearest one, C opens the character window, I the inventory, K the skills, Q the quests, 1-8 use the hotbar, Z picks up the nearest item, X sits or stands, Enter chats; offline, Space swings the sword.

For the GM commands (on a dev server, see `../server/README.md`), build the extension with
`cargo build --release -p rose-gd --features dev`. A run from the editor or a source
checkout counts as the dev client: the start screen suggests the dev server
(`ws://127.0.0.1:3100`) and offers Connect without account, and the version line at the top
left reads the commit from git (`scripts/build_info.gd`).

The start screen's server address may end in a database name, `ws://HOST:3000/NAME`
(default `rose`). Without `--data-idx` the game looks for the last `data.idx` picked, then
`iRose_129_129/data.idx`, `data/data.idx` or `data.idx` next to the executable, and
otherwise asks for it once.

### Windows build

`godot/tools/build-windows.sh` builds `dist/ROSE-windows.zip` from Linux
(`godot/tools/build-windows.ps1` does the same on Windows): it
cross-compiles `rose_gd.dll` with mingw-w64 (`rustup target add x86_64-pc-windows-gnu`,
`apt install gcc-mingw-w64-x86-64`), exports `ROSE.exe` with Godot's Windows release
template (only `windows_release_x86_64.exe` from the editor's export templates is needed;
the script fetches it when it's missing) with two packs beside it, `ROSE.pck` (scripts,
interface and zone scenes, about 5 MB) and `assets.pck` (the converted ROSE assets, about
320 MB, from the "ROSE assets" export preset; the game opens it at start), and zips them with the DLLs (ours and Terrain3D's) and
`tools/README-windows.txt`. It needs `ROSE_DATA_IDX` (the client's `data.idx`; the PowerShell
script also takes `-DataIdx`): it first
runs `--import-rose`, because `assets.pck` carries the converted models, textures and
animations (see `assets/README.md`), then checks that
`overrides/motions.toml` and `overrides/zones/` match the animations and zone scenes
(`--export-motions=check`, `--export-zones=check`); when they don't, it stops without
building. The DLL only imports Windows system DLLs. Under
Wine 9 the build loads the data, builds Zant and plays online headless (Wine's `dinput8`
crashes Godot here, so the test ran with `WINEDLLOVERRIDES=dinput8=d`); it has not been
run on a real Windows PC yet.

Three switches (`-Dev`, `-Folder` and `-GameOnly` in the PowerShell script):

- `--dev` builds the **dev client**, `ROSE-windows-dev` (folder `ROSE-dev`): the
  "Windows Desktop Dev" export preset (feature tag `dev`, the `gm/` chat included), the
  debug template `windows_debug_x86_64.exe` with a console window (`ROSE.console.exe`) and
  `rose_gd.dll` built with `--features dev`. The production build checks that neither
  `gm_commands` (the `gm/` scripts) is in the pck nor `gm_command` in the DLL, and stops if
  one is.
- `--folder` writes a folder, `dist/ROSE-windows[-dev]/`, instead of a zip, for testing. A
  rebuild replaces only the exe, packs, DLLs and readme, so the `themes` folder, the ROSE
  client data and anything else next to `ROSE.exe` stay.
- `--game-only` leaves `assets.pck` out: `ROSE-windows[-dev]-update.zip` (about 40 MB, the
  exe, `ROSE.pck` and the DLLs) to unzip over a full install, or with `--folder` the
  folder's `assets.pck` stays. Use the full build when the converted assets changed (an
  import change, `IMPORT_VERSION`, or anything else in `assets/rose/`): a newer `ROSE.pck`
  wins over an older `assets.pck` for files in both, but can't add missing assets.

Both write the commit into the export (`build_version.txt`, git-ignored) for the version
line.

Other options (after `--`):

- `--server=ws://HOST:3000` connects straight away, with `--name=NAME` on a server without
  accounts. With accounts add `--email=EMAIL --website=http://HOST:3001` and the password in
  `ROSE_PASSWORD` (or `--password=`); `--create=NAME,female,FACE,HAIR` fills in the
  character creation screen and `--create-after=SECONDS` presses Create. `--weapon=bow`
  equips the bow and arrows from the bag once the character meets the bow's requirements.
- `--open=inventory,character,skills,quests` opens those windows at the start (for screenshots).
- `--profile=NAME` picks the identity file (`user://identity-NAME-HOST-PORT.token`, one per
  server because each server signs its own), so two clients on one PC are two players.
- `--offline` skips the start screen.
- `--net-demo[=NAME]` runs a scripted online test (a route, fights, shops, talks, parties,
  trades and more), `--demo` a scripted offline run, swing and cancel, and
  `--effect=PATH,...` plays effects in a loop. They are for tests and recordings, live in
  `demos/` and are left out of the Windows build; `demos/README.md` lists them and their
  options.
- `--net-log` prints every player's position once a second, `--quit-after=SECONDS` quits.
  The two-client test: `--headless ... --name=Alice --profile=alice --net-demo` in one
  process and `... --name=Bob --profile=bob --weapon=bow --net-demo=line --net-log` in
  another.
- `--time=morning|day|evening|night|TICKS` fixes the start time (one tick is 10 s).
- `--zone-files` builds zones from the client's map files even when they have scenes in
  `zones/`; `--import-zone=ID[,force]` writes a zone's scene and `--merge-zone-blocks=ID|all`
  moves the `block_X_Y.tscn` files of zones converted before 2026-10-18 into it (see
  `assets/README.md`, which also covers the zone tools in the editor).
- `--free-camera=x,y,z,yaw,pitch` uses a fixed camera with the Bevy zone viewer's convention.
- `--screenshot=PATH` saves one frame and quits.

## Layout

- `rust/src/zone.rs` builds the zone (port of `zone_loader.rs`), plus terrain height
  queries and the zone lighting at a given time. `rust/src/zone_scene.rs` writes the zone
  scenes in `zones/` and reads them back (heights, collision, warps, sounds) when a zone
  loads from its scene; the ground is a Terrain3D node (`addons/terrain_3d`) drawn with
  `shaders/rose_terrain3d.gdshader`. `addons/rose_zone_tools/` is the editor plugin for
  zone scenes (Go to, ROSE coordinates, block outlines, Drop to ground); it is not in the
  Windows build.
- `rust/src/character.rs` builds the skeleton, parts and animations (port of
  `model_loader.rs` and `zmo_asset_loader.rs`).
- `rust/src/material.rs`, `mesh.rs`, `texture.rs` convert ZSC materials, ZMS meshes and DDS
  textures. ROSE DDS files often declare a full mip chain but store only one or two levels;
  those are cut to level 0 and their mipmaps are generated after loading.
- `rust/src/assets.rs` is the import command (`--import-rose`, class `RoseImport`) and the
  lookup that `mesh.rs` and `texture.rs` ask first: `assets/ours/` (committed) wins over
  `assets/rose/` (generated, git-ignored), and the client data comes last. The player
  characters (skeleton scenes, part scenes, motions with hit/step/effect markers and one
  motion library per weapon type) are imported by `character.rs` (`import_avatars`), and
  `RoseCharacter` builds players from them when they are there.
- `rust/src/character.rs` also holds `RoseNpc`, the monster and NPC model builder (port of
  `spawn_npc_model`).
- `rust/src/net.rs` is the `RoseNet` node: the SpacetimeDB connection (SDK 2.10.2, bindings
  in `module_bindings/` generated from `../module`). It handles messages in `_process`, and
  GDScript polls `get_entities()` for positions on the motion paths at the current server
  time. The server's clock is estimated from the fastest motion update seen, so a PC whose
  clock is off still shows the right positions.
- `rust/src/audio.rs` and `rust/src/effect.rs` play the sounds and the EFT/PTL effects
  (ports of rose-offline-client's sound and effect systems), at the frame events in the ZMO
  motions.
- `shaders/` holds the ports of the WGSL shaders. Textures are sampled raw and all maths runs
  in gamma space like the original, then converted to linear at the end, so colours match.
- `scripts/` holds the GDScript: the scene setup (`main.gd`), the offline player
  (`player.gd`), the online world (`online.gd`, `net_entity.gd`), the start screen
  (`connect_panel.gd`, `account_login.gd`), character creation (`character_create.gd`), the
  loading screen, the camera, sounds and effects (`fx.gd`) and one script per game window
  (`*_window.gd`, `hotbar.gd`).
- `scripts/ui/` is the UI framework: draggable, scalable windows and HUD widgets, the
  options window, the cursor and the version line. `ui/themes/` holds the four themes as
  TOML (Arumic is the default); the Windows build also reads a `themes` folder next to
  `ROSE.exe`.
- `zones/` holds the zone scenes, one folder per zone, edited in Godot. Their spawns, NPCs
  and warps are exported to `../overrides/zones/` for the server.
- `assets/` holds the converted models, textures and animations (see `assets/README.md`).
- `gm/` is the GM command chat for dev servers, left out of the production export.
- `tools/` holds the Windows build scripts and the readme that goes in the zip.
- `demos/` holds the scripted runs for tests and recordings, one file per demo (see
  `demos/README.md`). The Windows export leaves the folder out (`exclude_filter` in
  `export_presets.cfg`), so ROSE.exe can't run them.

Coordinates follow the Bevy client: ROSE (x, y, z) in centimetres becomes Godot
(x, z, -y) in metres. Godot treats clockwise triangles as front faces, so every index
buffer is flipped.

## Links

- How the game plays: `../wiki/`. Which wiki pages to update after a client change:
  `../.claude/rules/wiki-sync.md`.
- Converting assets and editing zones: `assets/README.md`.
- Test runs: `demos/README.md`.
- The server it connects to: `../server/README.md` (Docker, dev server), `../module/README.md`.
