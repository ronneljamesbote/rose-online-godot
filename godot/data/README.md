# Game data

The game's own data, made in the Godot editor: one resource (`.tres`) per thing, edited in
the Inspector. It replaces the iROSE tables one kind at a time
(plan: `TODO/own-game-data.md` in the project files). So far: **items**.

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

Files are named `<id>-<name>.tres` (`weapon/2-short-sword.tres`). The name is only for
finding them: the game goes by the `id` field and by the class, so a file can be renamed or
moved to any folder under `items/`. Hover a field in the Inspector for what it does.

## Changing something

1. Open the resource, change the field in the Inspector, save.
2. Export, so the server and the game get it:

   ```sh
   godot --headless --path godot -- --export-data          # writes ../overrides/data/
   godot --headless --path godot -- --export-data=check    # fails when it is out of date
   ```

3. Commit the resource and `overrides/data/` together, with the wiki page of the item (the
   wiki is the spec, see `AGENTS.md`).

A new item is a new resource of the right class with an `id` no other item of that class
has (for example duplicate an item in the FileSystem dock and change its id). Two items of
one class with the same id stop the export with an error naming both files. Ids are what
saved characters, shops, drop lists and quests store, so don't change the id of an item
players may have.

The export writes `overrides/data/items/<class>.toml` (layout in
`crates/rose-data-export/src/items.rs`). The server's upload sends it (as JSON) in place of
the iROSE item tables, and the game reads it from the repository (or from `overrides/data/`
next to `ROSE.exe` in the Windows build). Both build scripts refuse to build when it is out
of date. To list what differs from iROSE:
`cargo run --release -p rose-stdb-import -- compare /path/to/data.idx`.

## Where it came from

`--convert-data` wrote these files once from the iROSE tables (run headless, with the
client data):

```sh
godot --headless --path godot -- --convert-data --data-idx=/path/to/data.idx
```

It refuses to run while item resources are here, because they are the source now;
`--convert-data=force` writes them again from iROSE (keeping each file's uid) and loses
every change made since. Converted and exported, every value of all 4,172 items and the 10
grades was the same as in iROSE (`rose-stdb-import compare`).

Each field is a value the game reads, by the name the game's code gives it. The converter
left out the columns nothing in the game reads: the Korean name (column 0 of each table),
the artists' model notes (columns 1 and 2) and a few unused columns (weapons 44, bodies 33
and 34, hands 33, heads and feet 34, vehicle parts 34, 51 and 52). Units are iROSE's
(ranges in centimetres, cooldowns in seconds). Some consumable fields came from one iROSE
column (its "data value"), so a potion also shows that number as a skill id or fuel; the
game reads only the field the item's class uses.

Numbers that point at art or sound (`icon_index`, `field_model_index`, the sound and effect
ids) are still iROSE numbers; later steps of the plan turn them into links to the Godot
icons, models, sounds and effects.
