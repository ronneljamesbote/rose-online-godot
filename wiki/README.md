# The game wiki

Everything a player needs to know about the game, and the source of truth for developing
it: what a page says is what the game does. The website shows these files at `/wiki`.

## How to change the game through the wiki

1. Open the page on the website and click **Edit on GitHub** (or open the file here).
2. Change it in GitHub's editor and choose **Create a new branch and start a pull request**.
3. Paste the pull request link to Claude and ask to implement it. Claude adds the code or
   data change to the same pull request (skill `implement-wiki-change`) with screenshots.
4. Review and merge. The page and the game change together.

Small fixes that don't change the game (typos, clearer wording) need no implementation:
say so in the pull request.

## Folders

| Folder | One page per |
| --- | --- |
| `rules/` | game system: combat, stats, experience, death, party, PvP, stamina, durability, crafting ... |
| `skills/` | skill (all its levels on one page) |
| `monsters/` | monster |
| `npcs/` | town NPC |
| `quests/` | quest |
| `items/<type>/` | item, by type (`weapon`, `body`, `consumable` ...) |
| `zones/` | zone (map, monsters, NPCs, warps) |
| `backlog/` | feature that is not in the game yet |
| `assets/` | zone maps and icon sheets the pages use |

Each folder has an `index.md` that is its list page.

File names are `<id>-<name>.md`, for example `monsters/1-jelly-bean.md`. The id is the
internal ID the game uses (the row in the iROSE data), so it never changes even if the
name does.

## Page format

A page is Markdown with a data block (YAML front matter) at the top. Numbers belong in the
data block; the text explains them. The website shows the data block as the page's tables.

```markdown
---
kind: monster            # rule, skill, monster, npc, quest, item, zone, backlog, list
id: 1                    # internal ID (items: "<type>/<number>")
name: Jelly Bean
status: in-game          # in-game, changed-from-irose, not-in-game-yet
level: 1
hp: 37
drops:
  - { item: "[[items/etc/1-jelly-bean-gel|Jelly Bean Gel]]", chance: "5%" }
source:
  data: LIST_NPC.STB row 1
  code: module/src/monster_brain.rs
---
A small, slow monster ...

## Changed from iROSE
...
```

- **Links**: `[[path|label]]` links to another page (`path` is relative to `wiki/`, without
  `.md`). They work in the text and in data values. Every page lists the pages that link
  to it, so links work both ways.
- **Lists of rows** in the data block (drops, skill levels, spawns) show as tables.
- **Formulas** go in a fenced `math` block in TeX, with a worked example under it.
- **Changed from iROSE**: a section saying how our game differs from the original iROSE.
- **Open question**: a note where the page and the game are known to disagree or a fact
  is not checked yet. Start the line with `> Open question:`.
- **source**: the data file and row the values came from, and the code that uses them, so
  a change knows where to go. Data values changed through the wiki are recorded in
  `overrides/game-data.toml`.

## Status values

- `in-game`: works as described, same as iROSE.
- `changed-from-irose`: works as described, which differs from iROSE (see the section).
- `not-in-game-yet`: the original iROSE behaviour, which our game doesn't have yet. Changing
  it to `in-game` in a pull request asks for the feature to be built.

## Checking pages

`node web/scripts/check-wiki.mjs` checks that every page's data block parses, that ids
match file names and that every `[[link]]` points to a page.

Most table pages (skills, monsters, NPCs, quests, items, zones) were first written by
`tools/wiki-gen` from the iROSE 129_129 data. Since then the pages are the source: edit
them here, don't regenerate them.
