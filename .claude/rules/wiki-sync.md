---
paths:
  - "module/**"
  - "crates/**"
  - "godot/**"
  - "export/**"
---

# Keep the wiki in step with the game

You are changing game code. `wiki/` is the source of truth for how the game works, so a
change in behaviour (a number, a formula, a rule, a new feature, a removed feature) must
update the pages below in the same commit or pull request. Pure refactors, build fixes and
log changes need no wiki change.

| Code | Wiki pages |
| --- | --- |
| `module/src/lib.rs` (combat tick, attacks, hit frames, animation cancelling), `crates/rose-game-irose/src/` (damage, hit, crit, speeds, ability values, XP, level-up, stat costs) | `wiki/rules/combat.md`, `wiki/rules/stats.md`, `wiki/rules/experience.md` |
| `module/src/skills.rs` | `wiki/skills/`, `wiki/rules/skills.md` |
| `module/src/monster_brain.rs`, `module/src/world.rs` (spawns), drop tables | `wiki/monsters/`, `wiki/zones/`, `wiki/rules/drops.md` |
| `module/src/npcs.rs`, `module/src/npc_ai.rs`, `module/src/shop.rs` | `wiki/npcs/`, `wiki/rules/shops.md` |
| `module/src/quests.rs`, `crates/rose-quest/` | `wiki/quests/` |
| `module/src/items.rs`, `module/src/durability.rs`, `module/src/craft.rs`, `module/src/vehicle.rs` | `wiki/items/`, `wiki/rules/durability.md`, `wiki/rules/crafting.md`, `wiki/rules/carts.md` |
| `module/src/death.rs`, `stamina.rs`, `party.rs`, `pvp.rs`, `trade.rs`, `friends.rs`, `chat.rs`, `bank.rs` | the matching `wiki/rules/<topic>.md` |
| `module/src/gm.rs`, `godot/gm/` (GM commands, dev servers only) | `wiki/rules/gm-commands.md` |
| `godot/` (controls, windows, what the player sees) | `wiki/rules/controls.md` and the page of the system shown |
| `godot/data/items/` (item resources), `export/data/items/` (exported from them) | the `wiki/items/` page of every item whose value changed; for refine grades `wiki/rules/refining.md` |
| `godot/data/monsters/`, `godot/data/drop_tables/`, `export/data/monsters.toml`, `export/data/drop-tables.toml` | the `wiki/monsters/` page of every monster whose value changed (a drop table: every monster using it, and `wiki/zones/` for a zone's own table); `wiki/rules/drops.md` for drop rules |
| `godot/data/npcs/`, `godot/data/shop_tabs/`, `export/data/npcs.toml`, `export/data/shop-tabs.toml` | the `wiki/npcs/` page of every NPC whose value or shop changed; `wiki/rules/shops.md` |
| `godot/data/skills/`, `godot/data/status_effects/`, `godot/data/job_classes/`, `export/data/skills.toml`, `export/data/status-effects.toml`, `export/data/job-classes.toml` | the `wiki/skills/` page of every skill whose value changed (a status effect or job class: every skill using it); `wiki/rules/skills.md`, `wiki/rules/status-effects.md` for rules |
| `godot/data/zones/`, `godot/data/warp_gates/`, `export/data/zones.toml`, `export/data/warp-gates.toml` | the `wiki/zones/` page of that zone (name, start and revive spots, PvP, planet, carts, warps); `wiki/rules/zones-and-warps.md`, `wiki/rules/pvp.md`, `wiki/rules/carts.md`, `wiki/rules/death.md` for the rules that read them |
| `godot/data/craft_recipes/`, `export/data/craft-recipes.toml` | `wiki/rules/crafting.md` (crafting recipes) or `wiki/rules/refining.md` (recipes 1-20), and the `wiki/items/` page of every item made with the recipe |
| `godot/data/new_characters/`, `export/data/new-characters.toml` | `wiki/rules/stats.md` ("New characters") |
| `godot/zones/` spawn, NPC and event markers, `export/zones/` (exported from them) | the `wiki/zones/` page of that zone (spawn points, NPCs) and the pages of monsters and NPCs it places |
| `export/motions.toml` (exported from the hit markers of the Godot player, monster and NPC animations) | `wiki/rules/combat.md` (animation lengths and hit frames), the page of every monster whose attack moved and of every skill whose hit moved |

How to update a page:

- Change the value in the page's front matter (the data block) and any sentence that
  repeats it. Keep `source:` pointing at the right file and function.
- If our game now differs from iROSE, say so under "Changed from iROSE".
- A feature that goes away moves to `wiki/backlog/` or gets `status: not-in-game-yet`.
- Check with `node web/scripts/check-wiki.mjs` that every page still parses and links.
