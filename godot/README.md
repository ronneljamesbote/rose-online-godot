# Godot client

A Godot 4 client that reads the original ROSE data files at load time through a Rust
GDExtension (`rust/`, crate `rose-gd`, built with gdext 0.5.5). It reuses the vendored
rose-offline crates for every file format and data table.

What works now:

- Canyon City of Zant from `data.idx`: 16 terrain blocks with two-layer tile blending and
  lightmaps, 834 zone objects (1088 mesh parts) with their lightmaps, water planes, the sky dome.
- The day/night cycle, ported from rose-offline-client's `zone_time_system.rs`.
- A male character built from the ZMD skeleton and ZSC parts (face, hair, body, arms, feet,
  Short Sword), with every action motion converted from ZMO for the equipped weapon type.
- Click-to-move on the terrain, a three-swing sword combo, and animation cancelling: moving
  during a swing ends it at once. Hit times come from the ZMO frame events and match the
  server's `attack_hit_ms` (566 / 533 / 600 ms for the sword swings).

- Online play on the SpacetimeDB server (`../module`): a start screen asks for the server,
  a name. The client signs in with an identity token
  kept in Godot's user folder, so the same PC gets the same character back. Every player in
  the zone shows up with a name tag and runs along the server's motion paths. Your own
  character starts running the moment you click and hands over to the server's path when
  it arrives.

- Monsters from the server, built from `LIST_NPC.CHR` and `PART_NPC.ZSC` with their
  motions (stop, walk, run, attack, hit, die) and the NPC scale from `LIST_NPC.STB`.
- Fighting online: click a monster (or press Space for the nearest one) to attack it; the
  server runs to it and swings. Each swing plays at the speed that puts its hit frame on the
  server's hit time. Damage numbers rise over whoever is hit ("Miss" for 0, yellow for
  criticals, red when it is you), idle monsters flinch, and a killed monster plays its death
  before it disappears. Moving cancels a swing at once. The HUD shows your HP and your
  target's HP; monster names show within 15 m or when targeted.

- Collision with zone objects, as in the Bevy client's `collision_system.rs`: parts with a
  ZSC collision shape become two static trimesh bodies, walls (layer 1) and walkable floors
  (layer 2). Your own character casts a 0.4 m sphere ahead at 1.2 m height while it runs;
  on a hit it stops and tells the server (`move_collision`), which stops the move there for
  everyone. Every character stands on the floor object under its feet (bridges, stairs) if
  that is higher than the terrain, within a 1.35 m step. Clicks can land on floors, and the
  camera pulls in when a wall is between it and you.

- Characters with levels: the client loads the zone the server says your character is in
  (new characters start on Birth Island) and builds every player from the equipment the
  server stores. Kills give experience ("+N XP" over you, "Level up!" on a new level), the
  bar at the bottom shows progress to the next level, and the HUD shows level, HP and MP.
  C opens the character window: level, stat and skill points, the six basic stats with a +
  button each (spending stat points at rose-offline's cost), and the ability values the
  server calculated from them.

- Items: monsters drop items and Zuly with their ground models (LIST_FIELDITEM.ZSC) and
  names; click one, or press Z for the nearest, to walk over and pick it up. I opens the
  inventory window: equipped items and ammo, the four bag pages with the original icons
  (ITEM1.TSI) and tooltips, and money. Right-click or double-click equips or uses an item,
  or takes off an equipped one; drag moves items between slots; Drop (or Delete) drops the
  selected one. Messages such as "Picked up Banana" or "Needs Level 10" show bottom left.

- Warp gates: the client finds each zone's warp gate objects (event objects of the zone's
  ZSC) and asks the server to warp you when you walk into one; the server sends you to the
  gate's target zone and event position (WARP.STB), and the client loads that zone.

- Town NPCs and stores: the server places every NPC from the zone files, and the client
  shows them with their names in green. Click one to walk over and open its store: tabs of
  items with icons and prices from rose-offline's formulas. Right-click buys one,
  Shift+right-click ten; while the store is open, right-clicking a bag item sells one
  (Shift sells the stack) and its tooltip shows what the store pays. There is no path
  finding yet, so a store also opens when a wall stops you within 12 m of the NPC.

- Skills: K opens the skill window (basic, active, passive and clan pages with the original
  icons and tooltips, and Level up for skill points). Skill books from the stores teach
  skills; scrolls cast theirs (return scrolls warp you home). Drag skills or items onto the
  hotbar at the bottom and use them with 1-8 or a right-click. Attack skills go on the
  current target, buffs on yourself; the character plays the skill's casting and action
  motions, and buffs and debuffs show as icons at the top. There are no skill effects
  (particles) yet, and summons, resurrection and emotes are not in yet.
- Conversations and quests: clicking a town NPC opens its conversation (the original CON
  dialog scripts, run by a Lua 4 VM ported from rose-offline-client) instead of going
  straight to the store; pick answers with the mouse or 1-9. Quest conditions are checked
  on the client and the server runs the trigger, so quests, rewards and quest items come
  from the server. Q opens the quest log: each quest's description, time left, quest
  items and an Abandon button. Clans, event objects and the bank are not in yet.

Not yet: monsters walking around objects (the server has no zone geometry), effects and
particles, animated zone objects, sound, most UI.

## Results (2026-10-07, headless, lavapipe software Vulkan)

| Check | Result |
|---|---|
| Zant vs rose-offline-client, same camera and time of day | Same layout; mean colour per region within 1-3 of 255 (`media/godot-01-zant-vs-bevy.png`) |
| Zone load (release build) | 0.6-0.7 s for the zone, 20-35 ms for the data tables |
| Draw calls at the zone viewer camera | 279 (539 objects, 173k triangles) |
| Character | Runs on the terrain, swings, cancels (`media/godot-clip1-run-swing-cancel.mp4`) |
| Two Godot clients fighting monsters | 182 monsters around the Woopie field; both players fight, hits and kills match the server's damage events (`media/godot-05-monsters-fight.png`, `media/godot-clip3-monsters-fight.mp4`) |
| Collision | Running 45 m out from the Zant start in 12 directions: 9 runs stopped at buildings, fences and trees, and the server agreed each time (`media/godot-06-wall-stop.png`, `media/godot-clip4-walls.mp4`) |
| Two Godot clients on one server | Each sees the other run its route; positions agree to the centimetre once a move ends (`media/godot-03-two-clients.png`, `media/godot-clip2-two-clients.mp4`) |

The Bevy client's load time was not measured for comparison.

## Build and run

```sh
# from the rose-stdb root
cargo build --release -p rose-gd
mkdir -p godot/bin && cp target/release/librose_gd.so godot/bin/    # rose_gd.dll on Windows
godot --path godot -- --data-idx=/path/to/iRose_129_129/data.idx
```

Godot 4.6 or newer. `ROSE_DATA_IDX` works instead of `--data-idx`. The start screen offers
Connect (online) or Play offline. In game: left-click to move, S to stop, right-drag to
orbit the camera, mouse wheel to zoom. Online, left-click a monster to attack it and Space
attacks the nearest one, C opens the character window, I the inventory, K the skills, Q the quests, 1-8 use the hotbar, Z picks up the nearest item; offline, Space swings the sword.

The start screen's server address may end in a database name, `ws://HOST:3000/NAME`
(default `rose`). Without `--data-idx` the game looks for the last `data.idx` picked, then
`iRose_129_129/data.idx`, `data/data.idx` or `data.idx` next to the executable, and
otherwise asks for it once.

### Windows build

`godot/tools/build-windows.sh` builds `dist/ROSE-windows.zip` from Linux: it
cross-compiles `rose_gd.dll` with mingw-w64 (`rustup target add x86_64-pc-windows-gnu`,
`apt install gcc-mingw-w64-x86-64`), exports `ROSE.exe` with Godot's Windows release
template (only `windows_release_x86_64.exe` from the 4.7.2 export templates is needed) and
zips both with `tools/README-windows.txt`. The DLL only imports Windows system DLLs. Under
Wine 9 the build loads the data, builds Zant and plays online headless (Wine's `dinput8`
crashes Godot here, so the test ran with `WINEDLLOVERRIDES=dinput8=d`); it has not been
run on a real Windows PC yet.

Other options (after `--`):

- `--server=ws://HOST:3000` connects straight away, with `--name=NAME`. `--weapon=bow`
  equips the bow and arrows from the bag once the character meets the bow's requirements.
- `--open=inventory,character,skills,quests` opens those windows at the start (for screenshots).
- `--profile=NAME` picks the identity file (`user://identity-NAME.token`), so two clients on
  one PC are two players.
- `--offline` skips the start screen.
- `--net-demo[=square|line|fight|warp|shop|skills|talk|walls]` walks a scripted route, fights the nearest
  monsters, walks through warp gates, buys and sells at the nearest store (`--buy=TEXT` picks
  what to buy), uses the first two active skills, talks to the nearest NPC matching `--npc=NAME` and picks `--answers=1,2,...` (`q` prints the quest log, `talk` opens the conversation again), or runs out in `--wall-directions` directions for `--wall-reach` metres, `--net-log` prints every
  player's position once a second, `--quit-after=SECONDS` quits. The two-client test:
  `--headless ... --name=Alice --profile=alice --net-demo` in one process and
  `... --name=Bob --profile=bob --weapon=bow --net-demo=line --net-log` in another.

- `--time=morning|day|evening|night|TICKS` fixes the start time (one tick is 10 s).
- `--free-camera=x,y,z,yaw,pitch` uses a fixed camera with the Bevy zone viewer's convention.
- `--screenshot=PATH` saves one frame and quits.
- `--demo` plays a scripted run, swing, cancel and combo. With
  `--write-movie out.avi --fixed-fps 30` it records the clip.

## Layout

- `rust/src/zone.rs` builds the zone (port of `zone_loader.rs`), plus terrain height
  queries and the zone lighting at a given time.
- `rust/src/character.rs` builds the skeleton, parts and animations (port of
  `model_loader.rs` and `zmo_asset_loader.rs`).
- `rust/src/material.rs`, `mesh.rs`, `texture.rs` convert ZSC materials, ZMS meshes and DDS
  textures. ROSE DDS files often declare a full mip chain but store only one or two levels;
  those are cut to level 0 and their mipmaps are generated after loading.
- `shaders/` holds the ports of the WGSL shaders. Textures are sampled raw and all maths runs
  in gamma space like the original, then converted to linear at the end, so colours match.
- `rust/src/character.rs` also holds `RoseNpc`, the monster and NPC model builder (port of
  `spawn_npc_model`).
- `rust/src/net.rs` is the `RoseNet` node: the SpacetimeDB connection (SDK 2.10.2, bindings
  in `module_bindings/` generated from `../module`). It handles messages in `_process`, and
  GDScript polls `get_entities()` for positions on the motion paths at the current server
  time. The server's clock is estimated from the fastest motion update seen, so a PC whose
  clock is off still shows the right positions.
- `scripts/` holds the GDScript scene setup, the offline player (`player.gd`), the online
  world (`online.gd`, `net_entity.gd`), the start screen and the camera.

Coordinates follow the Bevy client: ROSE (x, y, z) in centimetres becomes Godot
(x, z, -y) in metres. Godot treats clockwise triangles as front faces, so every index
buffer is flipped.
