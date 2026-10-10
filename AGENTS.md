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
   `crates/`, `godot/` or `overrides/` updates the wiki pages that describe it, in the same
   pull request. `.claude/rules/wiki-sync.md` says which pages cover which code.
3. **Never let them disagree on main.** If a page and the game differ and you are not
   asked to fix it, add an "Open question" note on the page instead of guessing.

Page format, folders and how to edit: `wiki/README.md`.

## Where game values come from

- **iROSE data files** (the client's `data.idx` and VFS files): the base data. Never edited.
- **`overrides/game-data.toml`**: our changes to that data, one STB cell per entry. Server
  and client both apply it on top of the iROSE files.
- **Code**: rules and formulas (`crates/rose-game-irose`, `module/src`) and features iROSE
  doesn't have.

A data change from the wiki (a drop rate, an item stat, a price) goes into the override
file. A rule change goes into code.

## Repository rules

- The repository is public. Never commit secrets, keys, tokens, SpacetimeDB identity files,
  personal emails or local paths. Original client data files are not committed; the wiki
  pages and their maps and icons are the one exception.
- Every finished change comes with screenshots or a short video showing it working.
