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
