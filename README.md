# rose-stdb: ROSE on SpacetimeDB (phase 1 prototype)

Server-side prototype from the plan of action, phase 1 and 2. Tested 2026-10-06 against SpacetimeDB 2.10.2 standalone.

- `module/`: the SpacetimeDB module (Rust, wasm32). It builds rose-offline's game databases (items, monsters, skills, quests, zones, drop tables) from client files the host uploads into its `game_file` table, and runs the game rules from the vendored crates on them: damage, experience and levels, stat points, HP/MP recovery, monster spawn points, monster drops (ITEM_DROP.STB), the inventory, equipment with its requirements, ammo, potions, warp gates, town NPCs and their stores, skills (skill books, skill points, casting, buffs and debuffs, scrolls), quests (every QSD condition and reward that doesn't need clans; monster death triggers) and town NPC scripts (AIP idle events that switch quests on and off by time of day and make NPCs speak), the bank (120 storage slots per account, no storage fees yet) and parties of up to five (invites, leader, experience shared equally or by level with 10% extra per member nearby, drops to the picker with Zuly split or in turn), and item crafting with the craft skills (LIST_PRODUCT.STB recipes; the step rolls, durability and bonus options follow the iROSE game server's formulas, since rose-offline has no crafting). Main tables: player, entity, motion, combat, stats, monster_ai, zone_info, monster_spawn, npc, bank, party, party_member, party_invitation, ground_item, regen, skill_cast, skill_cooldown, status_effect, damage_event, xp_event and notice (events), game_data_status. Reducers: move_to, attack, stop, add_basic_stat, pickup_item, equip_item, unequip_item, unequip_ammo, use_item, drop_item, drop_money, move_item, use_warp_gate, npc_store_transaction, cast_skill, level_up_skill, set_hotbar_slot, quest_trigger, abandon_quest, bank_deposit, bank_withdraw, bank_move, party_invite, party_accept, party_decline, party_leave, party_kick, party_set_leader, party_set_rules, craft_item, set_name, plus admin reducers for the upload, world rates and store price rates, set_aggro_range, reset_monsters, place_player, warp_player, give_xp, give_money, give_skill, set_job, set_basic_stat, admin_quest_trigger, admin_give_quest, set_npc_variable and give_item. Timers: combat_tick (100 ms), spawn_tick (1 s).
- `crates/`: rose-offline's file readers, data and game-rule crates, vendored so they can be changed here (upstream commit in `crates/UPSTREAM.md`).
- `import/`: `rose-stdb-import pack <data.idx> <dir>` collects the client files the databases read (about 2,500 files, 33 MB for 129_129en) into upload batches; `server/upload-game-data.sh` sends them. `list` and `time` show what is read and how long building takes.
- `bot/`: headless Rust SDK client. `rose-stdb-bot <ws uri> ranged|melee` fights the nearest monster and checks animation cancelling; `rose-stdb-bot <ws uri> load <n> <secs>` runs n random bots.
- `godot/`: the Godot 4 client (Zant and characters from the original data files, online play on the SpacetimeDB server with players and monsters, click-to-attack). See `godot/README.md`.

## Run

```
# server (spacetimedb-standalone needs an ES256 key pair, PKCS8 private key)
spacetimedb-standalone start --listen-addr 127.0.0.1:3000 --data-dir ./stdb-data --jwt-pub-key-path id_ecdsa.pub --jwt-priv-key-path id_ecdsa.p8
spacetime server add --url http://127.0.0.1:3000 --default local && spacetime login --server-issued-login local

cd module && cargo build --release --target wasm32-unknown-unknown
spacetime publish --server local --bin-path target/wasm32-unknown-unknown/release/rose_stdb_module.wasm -y rose

# The module has no game data until the host uploads it from a ROSE client folder. The
# script needs spacetimedb-cli logged in as the identity that published the module.
ROSE_IMPORT=target/release/rose-stdb-import sh server/upload-game-data.sh /path/to/iRose_129_129/data.idx

cd ../bot && cargo run --release -- ws://127.0.0.1:3000 ranged
```

### Hosting with Docker

`docker compose up -d` in this folder builds and runs the whole server (SpacetimeDB plus the
module) on any OS with Docker and uploads the game data from the ROSE client folder named
by `ROSE_CLIENT`; see `server/README.md`.

## Combat model
- Attacking needs the attacker to stand still; clicking to move drops the target (attack-while-moving was removed on 2026-10-07).
- A swing has a wind-up and a hit: it starts when the cooldown allows, and damage lands at the attack animation's hit frame
  (read from the ZMO frame events: Short Sword 566 of 1233 ms, Short Bow 900 of 1566 ms, monsters per NPC), scaled by attack speed.
- Animation cancelling: moving, stopping or switching target before the hit frame cancels the swing with no damage and refunds
  the cooldown. After the hit frame, the rest of the animation is skipped and you move at once; the cooldown still applies, so
  cancelling doesn't raise attack speed. The client patch lets Move/Stop interrupt the attack animation to match.

## Simplifications
- New characters start at level 1 on Birth Island (zone 20) with the INIT_AVATAR stats and clothes, plus a Short Sword in hand and a Short Bow with 999 arrows in the bag (the bow needs level 10 and DEX 29). Their ability values come from rose-offline's calculator, so they change with level, stats and equipment.
- Damage uses rose-offline's damage formula for weapon attacks (no skills yet, no PvP). Bows, guns and launchers use one ammo per hit.
- Drops follow rose-offline: they belong to the killer for 60 s and vanish after 120 s. Weight limits, item durability loss, stamina and scrolls that cast skills are not in yet. Experience follows rose-offline's experience_points_system, at its default world rate of 300%.
- Monsters fight back when hit; proactive aggro is off by default (`set_aggro_range` per monster type), because ROSE's AI scripts aren't ported.
- No zone geometry on the server: move_to only checks zone bounds. The Godot client finds walls itself and calls `move_collision(x, y)`; the server stops the move there if the point is on the current path and not more than 3 m ahead of the server's own position (rose-next's CANTMOVE check). Monsters still walk through objects.

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
