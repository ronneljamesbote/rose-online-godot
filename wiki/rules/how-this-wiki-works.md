---
kind: rule
id: how-this-wiki-works
name: How this wiki works
status: in-game
source:
  code:
    - AGENTS.md (the rules developers follow)
    - wiki/README.md (page format)
    - overrides/game-data.toml (our changes to the game data)
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
| The iROSE 129 game data | The original tables: item stats, monster stats, skill levels, drop tables, shops, spawns, quests, dialogs | Short Sword attack 10 |
| The override file (`overrides/game-data.toml`) | Our changes to those tables, one cell per entry | Short Sword attack 12 instead of 10 |
| The game code | Rules and formulas, and features iROSE didn't have | How attack turns into damage |

The original data is never edited. Every change we make to it is one line in the override
file, which both the server and the game client read, so they always agree. A rule or
formula change goes into the code instead.

A page's `source` row names the data (file and row) and the code behind it.

## Changing the game

1. Open the page and click **Edit on GitHub**.
2. Change what the game should do: a number in the table, a sentence, a formula.
3. Save it as a pull request and hand the link to Claude (or a developer), who changes the
   game in the same pull request and adds screenshots of it working.
4. Review and merge: the page and the game change together.

Some examples:

- *"Jelly Bean should drop Jelly Bean Gel more often"*: edit the drop row on the monster
  page. That becomes an override entry for the drop table.
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
