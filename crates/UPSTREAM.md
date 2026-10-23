# Vendored crates

Copied from [exjam/rose-offline](https://github.com/exjam/rose-offline) at commit
`4a648ba720278f69bb8ee6257564175c271e081c` (2026-03-25). These are our copies now: edit them freely.

| Crate | What it is |
| --- | --- |
| rose-file-readers | Readers for ROSE file formats (VFS, STB, ZON, HIM, TIL, IFO, ZMS, ZMO, ...) |
| rose-data | Engine-neutral game data types (items, NPCs, skills, zones, motions) |
| rose-data-irose | Loads rose-data from the iROSE 129_129en client files |
| rose-game-common | Shared game rules and components (ability values, damage traits) |
| rose-game-irose | iROSE formulas: ability values, damage, drop tables |

To compare with upstream later: clone rose-offline at the commit above and diff each crate folder.

## Our changes

- bevy is gone: the crates use glam 0.24 for math, and the bevy Component, Reflect and
  Deref derives were dropped (a small `deref_newtype!` macro stands in where Deref was used),
  so the rule crates build for the SpacetimeDB module (wasm32).
- rose-file-readers: `MemoryFilesystemDevice` (files held in memory, used by the module);
  the memory-mapped VFS readers are left out on wasm32; `ZmoFile::first_attack_frame_time`.
- rose-data / rose-data-irose: motions carry `first_attack_time` (the attack hit frame);
  NPC and character motions load without animation data; the zone loader skips map blocks
  that have no IFO file; zones carry their PvP state (`pvp_state`, LIST_ZONE.STB column 18).
- rose-game-common: `AbilityValues` and its parts serialize with serde.
- rose-game-irose: random numbers come from `rng.rs`, a seedable generator (the module
  seeds it each tick), instead of `thread_rng`; `levelup_require_xp`,
  `basic_stat_increase_cost`, `npc_store_buy_price` and `npc_store_sell_price` are public
  free functions (the client uses them for prices); the sell price no longer panics on a
  non-equipment item of an equipment type.

## Our own crate

rose-game-data is not from upstream. It builds every database from a virtual filesystem
(`load_game_data`, ported from rose-offline-server's `irose/data/mod.rs`), and holds the
world: the zone settings, warp gates, new characters (`CharacterCreator`) and crafting
recipes (`CraftRecipe`, shared with the client; recipes 1-20 are the refining ones), from
the export or else from LIST_ZONE.STB, WARP.STB, INIT_AVATAR.STB and LIST_PRODUCT.STB.

rose-quest is ours too: quest conditions and the quest variables (ported from
rose-offline-server's quest system), shared by the module, which runs triggers and their
rewards, and the client, which checks a trigger's conditions before asking the server to
run it. The Lua 4 VM in `godot/rust/src/lua4` is copied from rose-offline-client's
`scripting/lua4`, with only its module paths changed.

rose-data-export is ours: it reads the game data exported from the Godot resources
(`export/data/`, made in `godot/data/`) into the same rose-data databases rose-data-irose
fills from the iROSE tables (items, then monsters, NPCs, shop tabs and the drop table rows,
then skills, status effects and job classes; for these the three databases got `with_text`,
the status effects an `iter`, and the skill enums serde and `Enum`; then the world, for
which rose-data has `ZoneSettings`, a zone's LIST_ZONE.STB row as data, and the zone
database and list got `with_text`; rose-data-irose builds both from settings
(`get_zone_settings`, `get_zone_database_from_settings`, `get_zone_list_from_settings`)).
It also reads the zone layouts and hit markers exported from the Godot scenes and animations
(`export/zones/`, `export/motions.toml`), which a crate of their own read before. The STB
cell patching for the old `game-data.toml` (`patch_stb` in rose-file-readers) is gone with
that file.
For it, rose-data-irose's NPC loader is split (`get_npc_motions`, `get_npc_conversations`,
`get_npc_action_map` give the parts the export doesn't hold yet) and rose-game-irose's drop
table can be built from rows (`get_drop_table_from_rows`, `read_drop_rows`).
