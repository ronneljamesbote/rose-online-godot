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
  a name and a weapon (Short Sword or Short Bow). The client signs in with an identity token
  kept in Godot's user folder, so the same PC gets the same character back. Every player in
  the zone shows up with a name tag and runs along the server's motion paths. Your own
  character starts running the moment you click and hands over to the server's path when
  it arrives.

Not yet: monsters and fighting online (the server has them, the client does not show them
yet), collision with objects (characters follow the terrain height only), effects and
particles, animated zone objects, sound, most UI.

## Results (2026-10-07, headless, lavapipe software Vulkan)

| Check | Result |
|---|---|
| Zant vs rose-offline-client, same camera and time of day | Same layout; mean colour per region within 1-3 of 255 (`media/godot-01-zant-vs-bevy.png`) |
| Zone load (release build) | 0.6-0.7 s for the zone, 20-35 ms for the data tables |
| Draw calls at the zone viewer camera | 279 (539 objects, 173k triangles) |
| Character | Runs on the terrain, swings, cancels (`media/godot-clip1-run-swing-cancel.mp4`) |
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
orbit the camera, mouse wheel to zoom; offline, Space swings the sword.

Other options (after `--`):

- `--server=ws://HOST:3000` connects straight away, with `--name=NAME` and
  `--weapon=sword|bow`.
- `--profile=NAME` picks the identity file (`user://identity-NAME.token`), so two clients on
  one PC are two players.
- `--offline` skips the start screen.
- `--net-demo[=square|line]` walks a scripted route online, `--net-log` prints every
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
