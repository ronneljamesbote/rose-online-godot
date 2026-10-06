# rose-offline-client on SpacetimeDB

`rose-offline-client-stdb.patch` applies to rose-offline-client commit 9802251
(github.com/exjam/rose-offline-client). It adds a `spacetimedb` network version:

- `src/protocol/stdb/`: login and world steps are answered locally (one server, one channel,
  one fixed "Tester" character). The game step connects to the `rose` database and turns
  table changes into the client's normal ServerMessages: spawns, moves, attacks, damage, revive.
- Click to move calls `move_to`; clicking a monster calls `attack`. Moving keeps the attack
  target, so the server keeps shooting while you run.
- The SpacetimeDB token is saved in the client's data folder (`stdb_token`), so the character
  keeps its identity, position and HP between runs.
- `ROSE_STDB_RANGED=1` gives the character a Short Bow (21 m range) instead of a Short Sword.

## Build and run

```sh
git clone https://github.com/exjam/rose-offline-client && cd rose-offline-client
git checkout 9802251 && git apply ../rose-stdb/client/rose-offline-client-stdb.patch
cargo build --release
cd /path/to/iRose_129_129
ROSE_STDB_RANGED=1 /path/to/rose-offline-client --data-idx data.idx --network-version spacetimedb \
  --ip ws://127.0.0.1:3000 --auto-login --username test --password test \
  --server-id 0 --channel-id 1 --character-name Tester
```

Regenerate the bindings after changing the module:
`spacetime generate --lang rust --out-dir src/protocol/stdb/module_bindings --bin-path <module wasm>`.

## Known gaps
- The HUD shows the client's own max HP (236) while the server uses 300.
- Character creation, inventory, skills, chat and NPCs are not wired; only the messages above.
- No client-side collision is sent to the server, so you can walk up cliffs.
- On a case-sensitive file system the client needs `trose.exe` (lowercase) next to `TRose.exe`
  and an uppercase `3DDATA/CONTROL/XML` path for the UI files.
