# wiki-gen

Writes the first draft of the wiki's table pages from the iROSE 129 client data:

| Folder | One page per |
| --- | --- |
| `wiki/skills/` | skill, with a row per level (cost, power, range, cooldown, effects, learn points) |
| `wiki/monsters/` | monster (anything that spawns in a zone, is summoned or spawned by a quest): stats, drops, spawn points |
| `wiki/npcs/` | town NPC: where it stands, its shop, the quests it is part of and its whole dialog |
| `wiki/quests/` | quest: its text, who gives it, every step (quest trigger) with its checks and rewards |
| `wiki/items/<type>/` | item: stats, requirements, recipe, who drops or sells it, quests that give or need it |
| `wiki/zones/` | zone: the minimap with monster spawns, NPCs and warp gates drawn on it, and the zone's drop table |

plus `wiki/assets/icons` (the item, skill and status icon sheets and `icons.json`, which
says where each icon sits) and `wiki/assets/maps` (zone minimaps).

**This tool is for the first draft only.** Once the pages are in the repository they are
the source of truth and are edited by hand; running it again would overwrite those edits.
It only reads the game data; it changes nothing in the game.

## Running it

```sh
cargo run -p wiki-gen --release -- <ROSE client>/data.idx wiki /tmp/wiki-dds
python3 tools/wiki-gen/convert-images.py /tmp/wiki-dds wiki   # needs Pillow
node web/scripts/check-wiki.mjs
```

The third argument is a scratch folder: the icon sheets and minimaps are DDS textures,
which the generator copies there together with `convert.json`, and `convert-images.py`
turns them into PNG (icons) and JPEG (maps).

## How it reads things

- Stats come from the same loaders the server uses (`crates/rose-game-data`), so a page
  shows what the game loads.
- NPC dialogs: the CON file's menus are walked from menu 0 the way the game does
  (`godot/rust/src/conversation.rs`). The dialog's Lua script is read, not run: for each
  script function the generator lists the calls it makes with their literal arguments
  (`QF_checkQuestCondition("5032-02")`, `QF_findQuest(909)`, `GF_openStore`, ...) and the
  comparisons around them, and writes them in words. Menus whose check always answers 0
  are left out (the game never shows them).
- A quest's steps are the quest triggers that select, add or change to that quest, plus
  the triggers chained after them.
- Zone maps: the minimap image starts one map block before LIST_ZONE.STB's minimap start
  block, in both directions. With that, the warp gates land on the arrows drawn on the maps.
- Drops: a monster's own drop table row and the zone's row in ITEM_DROP.STB. "Slots of 30"
  counts the row's columns that can give the item (a group column counts 1/5 per item).
