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
  that have no IFO file.
- rose-game-common: `AbilityValues` and its parts serialize with serde.
- rose-game-irose: random numbers come from `rng.rs`, a seedable generator (the module
  seeds it each tick), instead of `thread_rng`; `levelup_require_xp` and
  `basic_stat_increase_cost` are public.

## Our own crate

rose-game-data is not from upstream. It builds every database from a virtual filesystem
(`load_game_data`, ported from rose-offline-server's `irose/data/mod.rs`) and reads the
starting characters from INIT_AVATAR.STB.
