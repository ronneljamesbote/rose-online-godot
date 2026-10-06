# rose-stdb: ROSE on SpacetimeDB (phase 1 prototype)

Server-side prototype from the plan of action, phase 1 and 2. Tested 2026-10-06 against SpacetimeDB 2.10.2 standalone.

- `module/`: the SpacetimeDB module (Rust, wasm32). Tables: player, entity, motion, combat, stats, monster_ai, zone_info, monster_spawn, npc_data, damage_event (event), tick_stats. Reducers: move_to, attack, stop, set_loadout, set_name, plus admin import_npcs, import_zone, set_aggro_range, reset_monsters. Timers: combat_tick (100 ms), spawn_tick (1 s).
- `crates/`: rose-offline's file readers, data and game-rule crates, vendored so they can be changed here (upstream commit in `crates/UPSTREAM.md`).
- `import/`: reads a 129_129en install with the vendored readers and writes `npcs.json` / `zone.json` (reducer arguments).
- `bot/`: headless Rust SDK client. `rose-stdb-bot <ws uri> ranged|melee` fights the nearest monster and kites it; `rose-stdb-bot <ws uri> load <n> <secs>` runs n random bots.
- `data/zone1/`: imported Zant data.

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

## Simplifications
- Player is a fixed test character (about a level 10 soldier, 300 HP); melee 1.2 m range, ranged loadout 10 m.
- Damage uses rose-offline's physical PvE formula only (no magic, no PvP). Monster attack speed uses the NPC attack animation length.
- Monsters fight back when hit; proactive aggro is off by default (`set_aggro_range` per monster type), because ROSE's AI scripts aren't ported.
- No collision or walkability: move_to only checks zone bounds.

## Results (2026-10-06, local server, 4-core cloud container)

| Check | Result | Plan target |
| --- | --- | --- |
| move_to to motion update, 1 player | 2.2 ms | under 100 ms localhost |
| same, with 200 bots running | 5 to 28 ms | under 100 ms localhost |
| Swing timing (ranged, 870 ms interval) | gaps 793 to 910 ms | within one 100 ms tick |
| Worst gap between 100 ms combat ticks, 200 bots | 156 ms | (tick under 30 ms of 100 ms) |
| Server memory | 108 MB idle, 130 MB at 50 bots, 247 MB at 200 bots | flat over 30 min (not run yet) |
| Server CPU at 200 bots | about 70% of one core | |
| Attack while moving | 5 of 5 hits after kiting started landed while the server saw the player walking | works |
| Not measured yet | bandwidth per player, 30-minute memory run, feel in a real client | |
