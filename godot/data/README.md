# Game data

The game's own data, made in the Godot editor: one resource (`.tres`) per thing, edited in
the Inspector. It replaces the iROSE tables one kind at a time
(plan: `TODO/own-game-data.md` in the project files). So far: **items**, **monsters**, **NPCs**, **drop tables** and **shop tabs**.

| Folder | Class | What |
| --- | --- | --- |
| `items/face/` | `RoseItemFace` | Masks, glasses |
| `items/head/` | `RoseItemHead` | Helmets, hats |
| `items/body/` | `RoseItemBody` | Armour, clothes |
| `items/hands/` | `RoseItemHands` | Gloves, gauntlets |
| `items/feet/` | `RoseItemFeet` | Boots, shoes (item 0: the run speed barefoot) |
| `items/back/` | `RoseItemBack` | Back armour, bags, wings, ammunition boxes |
| `items/jewellery/` | `RoseItemJewellery` | Rings, necklaces, earrings |
| `items/weapon/` | `RoseItemWeapon` | Weapons (item 0: fighting unarmed) |
| `items/subweapon/` | `RoseItemSubWeapon` | Shields, support tools |
| `items/consumable/` | `RoseItemConsumable` | Potions, food, scrolls, skill books, repair tools |
| `items/gem/` | `RoseItemGem` | Gems and jewels for sockets |
| `items/material/` | `RoseItemMaterial` | Crafting materials, arrows, bullets, shells |
| `items/quest/` | `RoseItemQuest` | Quest items |
| `items/vehicle/` | `RoseItemVehicle` | Cart and castle gear parts |
| `items/grades/` | `RoseItemGrade` | Refine grades 0 to 9: what each adds, the glow colour |
| `monsters/` | `RoseMonster` | Monsters (and summons, pets): stats, rewards, drop table |
| `npcs/` | `RoseNpc` | Town NPCs (the ones a zone places as NPCs): look, shop tabs |
| `drop_tables/` | `RoseDropTable` | What a monster (or a zone) drops |
| `shop_tabs/` | `RoseShopTab` | One tab of an NPC shop: up to 48 items |

Monsters and NPCs share one id space (an NPC and a monster never have the same id), because
saved quests and the zone markers store just the number. Links between resources are plain
resource fields: a monster's `drop_table`, an NPC's `shop_tab_1` to `shop_tab_4`, the items
of a drop table or shop tab, and the monsters of a zone's spawn markers (`RoseSpawnSpot`)
and the NPC of an NPC marker (`RoseNpcSpot`). Drag a resource from the FileSystem dock onto
the field to set it.

A drop table has 30 `slots` and 4 groups of 5 items. A slot is an item, or a number 1 to 4
that means "one of group N". When a monster drops something the game picks a slot (see
`wiki/rules/drops.md` for how). Spawn and NPC markers also store the plain ids
(`basic_monster_ids`, `npc_id`), so a marker whose monster has no resource keeps its id.

Files are named `<id>-<name>.tres` (`weapon/2-short-sword.tres`, `monsters/101-woopie.tres`).
The name is only for finding them: the game goes by the `id` field and by the class, so a
file can be renamed or moved to any folder of its kind. Hover a field in the Inspector for what it does.

## Changing something

1. Open the resource, change the field in the Inspector, save.
2. Export, so the server and the game get it:

   ```sh
   godot --headless --path godot -- --export-data          # writes ../overrides/data/
   godot --headless --path godot -- --export-data=check    # fails when it is out of date
   ```

3. Commit the resource and `overrides/data/` together, with the wiki page of the item,
   monster or NPC (the wiki is the spec, see `AGENTS.md`).

A new item (or monster, NPC, drop table, shop tab) is a new resource of the right class
with an `id` no other one of that class has (for example duplicate an item in the FileSystem dock and change its id). Two items of
one class with the same id stop the export with an error naming both files. Ids are what
saved characters, shops, drop lists and quests store, so don't change the id of an item
players may have.

The export writes `overrides/data/items/<class>.toml` and `overrides/data/monsters.toml`,
`npcs.toml`, `drop-tables.toml`, `shop-tabs.toml` (layout in
`crates/rose-data-export/src/items.rs` and `npcs.rs`). A link is written as the linked
resource's kind and id (`"weapon/3"`, `drop_table = 147`). The server's upload sends it (as
JSON) in place of the iROSE item, NPC (`LIST_NPC`), drop (`ITEM_DROP`) and shop
(`LIST_SELL`) tables, and the game reads it from the repository (or from `overrides/data/`
next to `ROSE.exe` in the Windows build). Both build scripts refuse to build when it is out
of date. To list what differs from iROSE:
`cargo run --release -p rose-stdb-import -- compare /path/to/data.idx`.

## Where it came from

`--convert-data` wrote these files once from the iROSE tables (run headless, with the
client data):

```sh
godot --headless --path godot -- --convert-data --data-idx=/path/to/data.idx
```

It converts only the kinds that have no resources here yet, because the others are the
source now; `--convert-data=force` writes them all again from iROSE (keeping each file's
uid) and loses every change made since. Converted and exported, every value of all 4,172
items and the 10 grades, the 545 monsters, 117 NPCs, 265 drop tables and 83 shop tabs was
the same as in iROSE (`rose-stdb-import compare`), except one shop place: tab 98 listed
`material/158`, an item that doesn't exist (the game skipped it already), so it was left out.

Each field is a value the game reads, by the name the game's code gives it. The converter
left out the columns nothing in the game reads: the Korean name (column 0 of each table),
the artists' model notes (columns 1 and 2) and a few unused columns (weapons 44, bodies 33
and 34, hands 33, heads and feet 34, vehicle parts 34, 51 and 52). Units are iROSE's
(ranges in centimetres, cooldowns in seconds). Some consumable fields came from one iROSE
column (its "data value"), so a potion also shows that number as a skill id or fuel; the
game reads only the field the item's class uses.

For monsters and NPCs the converter left out the Korean name and model notes (`LIST_NPC`
columns 0, 1, 36 and 37) and the columns the game doesn't read. The iROSE table used some
columns for different things in monsters and NPCs; each class has its own named fields for
them (a monster's `drop_table` is the column an NPC uses for its minimap icon). Drop table
rows that no monster or zone used were left out, and each drop table is named after the
monsters that use it (or "zone ..." for a zone's own table). `ITEM_DROP` column 30 was never
read by the game, so it isn't in the drop tables.

Numbers that point at art or sound (`icon_index`, `field_model_index`, the sound and effect
ids) are still iROSE numbers; later steps of the plan turn them into links to the Godot
icons, models, sounds and effects.
