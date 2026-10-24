---
kind: rule
id: how-this-wiki-works
name: How this wiki works
status: in-game
source:
  code:
    - AGENTS.md (the rules developers follow)
    - wiki/README.md (page format)
    - godot/data/ (the game data made in the Godot editor: items, monsters, NPCs, drop tables, shops, skills, status effects, job classes, zone settings, warp gates, recipes, new characters)
    - godot/zones/ (the zones edited in the Godot editor)
    - tools/wiki-gen (wrote the first draft of the table pages)
---
# How this wiki works

This wiki is two things at once: the guide players read, and the **specification of the
game**. What a page says is what the game does. When the game should change, the page
changes first, and the game is then made to match it.

## Where the numbers come from

Every number in the game comes from one of three places:

| Place | What it holds | Example |
| --- | --- | --- |
| Our game data (`godot/data/`, `godot/zones/`) | Things made in the Godot editor, one file each, converted once from iROSE: items, monsters, town NPCs, drop tables, shop tabs, skill levels, status effects, job classes, zone settings, warp gates, crafting recipes, new characters and sounds so far, and the zones edited as scenes | Short Sword attack 10, in `godot/data/items/weapon/2-short-sword.tres`; Woopie level 16, in `godot/data/monsters/101-woopie.tres`; a zone's PvP state, in `godot/data/zones/` |
| The iROSE 129 game data | The original data not converted yet: quests, dialogs, monster AI, effects, the maps of zones not edited in Godot yet | A quest's reward |
| The game code | Rules and formulas, and features iROSE didn't have | How attack turns into damage |

The original data is never edited. Our own data is changed in the Godot editor and
exported (`export/`). Data not converted yet can't be changed as data until it is (the plan
converts it one kind at a time). Both the server and the game client read the same files,
so they always agree. A rule or formula change goes into the code instead.

A page's `source` row names the data (our file, or the iROSE table and row) and the code
behind it.

## Changing the game

1. Open the page and click **Edit on GitHub**.
2. Change what the game should do: a number in the table, a sentence, a formula.
3. Save it as a pull request and hand the link to Claude (or a developer), who changes the
   game in the same pull request and adds screenshots of it working.
4. Review and merge: the page and the game change together.

Some examples:

- *"Short Sword should hit harder"*: edit the attack on the item page. That becomes a
  change to the sword's file in `godot/data/items/`.
- *"Jelly Bean should drop Jelly Bean Gel more often"*: edit the drops on the monster
  page. That becomes a change to its drop table in `godot/data/drop_tables/`.
- *"Heal should cost less MP"*: edit the cost on the skill page. That becomes a change to
  each level's file in the skill's folder in `godot/data/skills/`.
- *"Critical hits should do double damage"*: edit the formula on [[rules/combat|Combat]].
  That is a code change.
- *"We want clans"*: change the status of the clan page under
  [[backlog|Not in game yet]] to `in-game`. That asks for the feature to be built.

## Reading a page

- **The table at the top** holds the page's numbers. Rows that are lists (drops, skill
  levels, spawns) show as their own tables.
- **Links** go both ways: every page lists the pages that link to it.
- **Changed from iROSE** says where our game works differently from the original.
- **Open question** marks a place where the page and the game are known to disagree, or
  where a fact hasn't been checked yet. Those get settled, not guessed.
- **Status**: `in-game` (works as written), `changed-from-irose` (works as written, which
  differs from iROSE), `not-in-game-yet` (the original feature, which we don't have yet).

## Units

Distances are in metres and times in seconds unless a page says otherwise. Positions are
metres from the zone's corner, the same numbers the zone maps use. Speeds of monsters and
characters are in centimetres per second, as in the game data.

## How the pages were made

The skill, monster, NPC, quest, item and zone pages were first written by a program that
read the iROSE 129 game data, and the rules pages were written from the game code. From
now on the pages are the source: they are edited by hand and never regenerated.
