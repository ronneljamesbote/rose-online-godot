# Working in this repository

ROSE Online on SpacetimeDB with a Godot 4 client. See `README.md` for the parts (`module/`
server, `godot/` client, `web/` website, `crates/` game rules and data readers).

## The wiki is the source of truth

`wiki/` describes the game as it runs: rules and formulas, skills, monsters, NPCs, quests,
items and zones. The website shows it at `/wiki`. Development follows the wiki, not the
other way round.

1. **A wiki change is a spec.** When a pull request edits `wiki/`, the game must be changed
   to match it, in that same pull request. Use the `implement-wiki-change` skill
   (`.claude/skills/implement-wiki-change/SKILL.md`).
2. **A game change updates the wiki.** Any change to game behaviour in `module/`,
   `crates/`, `godot/` or `export/` updates the wiki pages that describe it, in the same
   pull request. `.claude/rules/wiki-sync.md` says which pages cover which code.
3. **Never let them disagree on main.** If a page and the game differ and you are not
   asked to fix it, add an "Open question" note on the page instead of guessing.

Page format, folders and how to edit: `wiki/README.md`.

## Where game values come from

- **iROSE data files** (the client's `data.idx` and VFS files): the base data. Never edited.
- **Godot resources** (`godot/data/`, see its README): the game data made in the Godot
  editor, one `.tres` per thing, replacing the iROSE tables one kind at a time (items,
  monsters, town NPCs, drop tables, shop tabs, skills, status effects, job classes, zone
  settings, warp gates, crafting recipes, new characters, sounds, effects, quests, NPC
  dialogs, and the icons and models they link so far). `--export-data` writes
  them to `export/data/`, which server and client both load instead of those iROSE tables.
  There is no file of changes on top of the iROSE tables any more: a value not made in Godot
  yet keeps its iROSE value until its kind is converted.
- **Godot scenes and animations**: zones edited in Godot (`godot/zones/`, Canyon City of Zant,
  Birth Island, Valley of Luxem Tower, Adventurer's Plain and Breezy Hills so far) and the
  animations' hit markers. The same `--export-data` writes them to `export/zones/` and
  `export/motions.toml`, which the server reads, so a spawn or NPC change on such a zone is
  made in its scene and exported.
- **Code**: rules and formulas (`crates/rose-game-irose`, `module/src`) and features iROSE
  doesn't have.

A data change from the wiki goes into the thing's resource when it is made in Godot (an item
stat or price: `godot/data/items/`; a monster stat or drop rate: `godot/data/monsters/`; what
drops: `godot/data/drop_tables/`; what a shop sells: `godot/data/shop_tabs/`; a skill level's
number: `godot/data/skills/`; a status effect: `godot/data/status_effects/`; a zone's
settings: `godot/data/zones/`; a warp gate: `godot/data/warp_gates/`; a recipe:
`godot/data/craft_recipes/`; what a new character starts with: `godot/data/new_characters/`;
an effect: `godot/data/effect_files/` or `godot/data/attack_effects/`; a quest's text or time
limit: `godot/data/quests/`; a quest step's conditions and rewards: `godot/data/quest_triggers/`;
what an NPC says and what its answers do: `godot/data/dialogs/`; an icon or model: the
thing's `icon` or `model` field, new art in `godot/assets/ours/`; then `--export-data`). A rule change goes into code.

## Repository rules

- The repository is public. Never commit secrets, keys, tokens, SpacetimeDB identity files,
  personal emails or local paths. Original client data files are not committed; the wiki
  pages and their maps and icons are the one exception.
- Every finished change comes with screenshots or a short video showing it working. They go
  with the report or the pull request, not into the repository.
