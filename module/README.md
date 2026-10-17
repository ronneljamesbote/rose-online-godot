# Server module

The SpacetimeDB module (Rust, built for wasm32) that is the game server. It holds the world
in tables (players, monsters, motions, items, quests, parties, ...) and runs every game rule
in reducers and timers: movement, combat, skills, drops, quests, NPC stores, crafting,
trading, chat and the rest. How each rule plays is described in `../wiki/`; this file is
about the code.

No game data ships with the module. The host uploads the client files the game databases
are built from (items, monsters, skills, quests, zones, drop tables, with
`../overrides/` applied on top), and the module builds the databases from them once per
instance (`game_data.rs`).

## Build and publish

The Docker image does all of this (see `../server/README.md`); by hand:

```sh
cd module
cargo build --release --target wasm32-unknown-unknown            # add --features dev for a dev server
spacetime publish --server local --bin-path target/wasm32-unknown-unknown/release/rose_stdb_module.wasm -y rose
```

Then upload the game data with the import tool (`../import/`, see `../server/README.md`'s
"How it works"), logged in as the identity that published the module.

The module has its own Cargo workspace (it builds for wasm32), so it is excluded from the
root one. SpacetimeDB 2.10.2.

- **`dev` feature**: a dev server. Adds the GM chat commands (`src/gm.rs`, one `gm`
  reducer; see `../wiki/rules/gm-commands.md`) and god mode. Production builds leave it out
  entirely. The `server_info` view reports the mode and the git commit the module was built
  from (`build.rs`).
- **Admin**: the identity that published the module is the admin (`init`). Admin-only
  reducers include the game data upload (`begin_game_data_upload`, `upload_game_files`,
  `finish_game_data_upload`), accounts (`set_auth_issuer`, `assign_character`), world and
  store rates (`set_world_rates`, `set_price_rates`, `set_aggro_range`, `reset_monsters`) and
  test helpers (`place_player`, `warp_player`, `admin_teleport_to_npc`, `set_hp_mp`,
  `give_xp`, `give_money`, `give_item`, `give_skill`, `set_job`, `set_basic_stat`,
  `set_stamina`, `admin_set_item_life`, `admin_quest_trigger`, `admin_give_quest`,
  `admin_clear_quests`, `admin_set_job_var`, `set_npc_variable`). Call them with
  `spacetime call --server local rose REDUCER ARGS...`.
- **Timers**: `combat_tick` every 100 ms (who sees whom, swings and hits, skill casts,
  summons), `spawn_tick` every second (spawn points, regen, status effects, monster and
  NPC AI, and expiring drops, chat and requests).

## Layout

Each file in `src/` starts with a comment saying what it does and which iROSE or
rose-offline code it follows.

| File | What it holds |
| --- | --- |
| `lib.rs` | Tables for players, entities and motions; moving, attacking, the combat tick, hit frames and animation cancelling; admin helpers |
| `game_data.rs` | The game data upload and the databases built from it |
| `account.rs` | Sign-in through the website's tokens, character assignment |
| `character.rs`, `ability.rs` | Character creation, ability values, experience, levels and stat points |
| `area.rs` | Who sees whom: the sector each entity is in, for the clients' subscriptions |
| `world.rs`, `monster_brain.rs` | Zones, spawn points and monsters; monster AI from the AIP scripts |
| `npcs.rs`, `npc_ai.rs` | Town NPCs and their stores; NPC AIP scripts |
| `items.rs`, `durability.rs`, `craft.rs`, `vehicle.rs` | Drops, inventory and equipment; wear and repair; crafting, refining, disassembly and gems; carts and castle gear |
| `skills.rs`, `stamina.rs` | Skills, casting, buffs and debuffs; Stamina |
| `quests.rs` | Quest triggers (QSD), run on the server |
| `bank.rs`, `party.rs`, `trade.rs`, `shop.rs`, `friends.rs`, `chat.rs` | Storage, parties, trading, personal shops, the friends list, chat |
| `death.rs`, `pvp.rs` | Dying and reviving; PvP zones |
| `gm.rs` | GM commands (`dev` feature only) |

The game rules shared with the client (damage, ability values, drop tables) live in
`../crates/`, mostly `rose-game-irose`.

## Links

- After changing a table or reducer, regenerate the Rust bindings the client
  (`../godot/rust/src/module_bindings`) and the bot (`../bot/src/module_bindings`) use, with
  `spacetime generate --lang rust --module-path . --out-dir <that folder>`.
- Which wiki pages describe which file: `../.claude/rules/wiki-sync.md`.
- Running it in Docker, the dev server and HTTPS: `../server/README.md`.
