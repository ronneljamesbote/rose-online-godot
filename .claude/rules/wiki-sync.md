---
paths:
  - "module/**"
  - "crates/**"
  - "godot/**"
  - "overrides/**"
---

# Keep the wiki in step with the game

You are changing game code. `wiki/` is the source of truth for how the game works, so a
change in behaviour (a number, a formula, a rule, a new feature, a removed feature) must
update the pages below in the same commit or pull request. Pure refactors, build fixes and
log changes need no wiki change.

| Code | Wiki pages |
| --- | --- |
| `module/src/lib.rs` (combat tick, attacks, hit frames, animation cancelling), `crates/rose-game-irose/src/` (damage, hit, crit, speeds, ability values, XP, level-up, stat costs) | `wiki/rules/combat.md`, `wiki/rules/stats.md`, `wiki/rules/experience.md` |
| `module/src/skills.rs`, skill rows in `overrides/` | `wiki/skills/`, `wiki/rules/skills.md` |
| `module/src/monster_brain.rs`, `module/src/world.rs` (spawns), drop tables | `wiki/monsters/`, `wiki/zones/`, `wiki/rules/drops.md` |
| `module/src/npcs.rs`, `module/src/npc_ai.rs`, `module/src/shop.rs` | `wiki/npcs/`, `wiki/rules/shops.md` |
| `module/src/quests.rs`, `crates/rose-quest/` | `wiki/quests/` |
| `module/src/items.rs`, `module/src/durability.rs`, `module/src/craft.rs`, `module/src/vehicle.rs` | `wiki/items/`, `wiki/rules/durability.md`, `wiki/rules/crafting.md`, `wiki/rules/carts.md` |
| `module/src/death.rs`, `stamina.rs`, `party.rs`, `pvp.rs`, `trade.rs`, `friends.rs`, `chat.rs`, `bank.rs` | the matching `wiki/rules/<topic>.md` |
| `godot/` (controls, windows, what the player sees) | `wiki/rules/controls.md` and the page of the system shown |
| `overrides/game-data.toml` | the page of every item, monster, skill or NPC whose value changed |

How to update a page:

- Change the value in the page's front matter (the data block) and any sentence that
  repeats it. Keep `source:` pointing at the right file and function.
- If our game now differs from iROSE, say so under "Changed from iROSE".
- A feature that goes away moves to `wiki/backlog/` or gets `status: not-in-game-yet`.
- Check with `node web/scripts/check-wiki.mjs` that every page still parses and links.
