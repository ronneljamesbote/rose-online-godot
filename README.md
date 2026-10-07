# rose-stdb: ROSE on SpacetimeDB (phase 1 prototype)

Server-side prototype from the plan of action, phase 1 and 2. Tested 2026-10-06 against SpacetimeDB 2.10.2 standalone.

- `module/`: the SpacetimeDB module (Rust, wasm32). Tables: player, entity, motion, combat, stats, monster_ai, zone_info, monster_spawn, npc_data, damage_event (event), tick_stats. Reducers: move_to, attack, stop, set_loadout, set_name, plus admin import_npcs, import_zone, set_aggro_range, reset_monsters, place_player. Timers: combat_tick (100 ms), spawn_tick (1 s).
- `crates/`: rose-offline's file readers, data and game-rule crates, vendored so they can be changed here (upstream commit in `crates/UPSTREAM.md`).
- `import/`: reads a 129_129en install with the vendored readers and writes `npcs.json` / `zone.json` (reducer arguments). `attack_timing` prints a weapon's attack animation length and hit frame.
- `bot/`: headless Rust SDK client. `rose-stdb-bot <ws uri> ranged|melee` fights the nearest monster and checks animation cancelling; `rose-stdb-bot <ws uri> load <n> <secs>` runs n random bots.
- `data/zone1/`: imported Zant data.
- `godot/`: the Godot 4 client, step 1 (Zant and an animated character from the original data files, no networking yet). See `godot/README.md`.

## Run

```
# server (spacetimedb-standalone needs an ES256 key pair, PKCS8 private key)
spacetimedb-standalone start --listen-addr 127.0.0.1:3000 --data-dir ./stdb-data --jwt-pub-key-path id_ecdsa.pub --jwt-priv-key-path id_ecdsa.p8
spacetime server add --url http://127.0.0.1:3000 --default local && spacetime login --server-issued-login local

cd module && cargo build --release --target wasm32-unknown-unknown
spacetime publish --server local --bin-path target/wasm32-unknown-unknown/release/rose_stdb_module.wasm -y rose

cd .. && cargo run --release -p rose-stdb-import -- /path/to/iRose_129_129/data.idx 1 data/zone1 && cd import
spacetime call --server local rose import_npcs "$(jq -c '.[0]' ../data/zone1/npcs.json)"
spacetime call --server local rose import_zone "$(jq -c '.[0]' ../data/zone1/zone.json)" "$(jq -c '.[1]' ../data/zone1/zone.json)"

cd ../bot && cargo run --release -- ws://127.0.0.1:3000 ranged
```

## Combat model
- Attacking needs the attacker to stand still; clicking to move drops the target (attack-while-moving was removed on 2026-10-07).
- A swing has a wind-up and a hit: it starts when the cooldown allows, and damage lands at the attack animation's hit frame
  (read from the ZMO frame events: Short Sword 566 of 1233 ms, Short Bow 900 of 1566 ms, monsters per NPC), scaled by attack speed.
- Animation cancelling: moving, stopping or switching target before the hit frame cancels the swing with no damage and refunds
  the cooldown. After the hit frame, the rest of the animation is skipped and you move at once; the cooldown still applies, so
  cancelling doesn't raise attack speed. The client patch lets Move/Stop interrupt the attack animation to match.

## Simplifications
- Player is a fixed test character (level 10, 236 HP) with the stats the client calculates for it; Short Sword 2.7 m reach, Short Bow 22.2 m, run speed 450.5.
- Damage uses rose-offline's physical PvE formula only (no magic, no PvP). Monster attack speed uses the NPC attack animation length.
- Monsters fight back when hit; proactive aggro is off by default (`set_aggro_range` per monster type), because ROSE's AI scripts aren't ported.
- No collision or walkability: move_to only checks zone bounds.

## Results (2026-10-06, local server, 4-core cloud container)

| Check | Result | Plan target |
| --- | --- | --- |
| move_to to motion update, 1 player | 2.2 ms | under 100 ms localhost |
| same, with 200 bots running | 5 to 28 ms | under 100 ms localhost |
| Swing timing (ranged, 870 ms interval, before the hit-frame change) | gaps 793 to 910 ms | within one 100 ms tick |
| Worst gap between 100 ms combat ticks, 200 bots | 156 ms | (tick under 30 ms of 100 ms) |
| Server memory | 108 MB idle, 130 MB at 50 bots, 247 MB at 200 bots | flat over 30 min (not run yet) |
| Server CPU at 200 bots | about 70% of one core | |
| Animation cancelling (bot, 2026-10-07) | moving mid-swing: target cleared, swing cancelled, no damage, cooldown refunded; moving right after a hit: applied in 10 ms, cooldown kept | all 6 checks pass |
| Real client (rose-offline-client, patch in `client/`) | logs in, spawns in Zant; sword chases and plays its swing animation; clicking away cancels the swing at once | works |
| Not measured yet | bandwidth per player, 30-minute memory run | |

## Client demo

`client/` holds the rose-offline-client patch. Screenshots and video are in `media/` (software rendering in a
headless container, so the frame rate is low). `clip7` and `clip8` show the current combat; the older kiting clips
(`fight-*`, `clip2`, `05-shooting-while-running.png`) show the removed attack-while-moving behaviour.
The `place_player` admin reducer moves a character, for example next to monsters:
`spacetime call --server local rose place_player '"Tester"' 536400 539320`.
