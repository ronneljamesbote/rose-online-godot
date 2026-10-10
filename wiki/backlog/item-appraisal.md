---
kind: backlog
id: item-appraisal
name: Item appraisal
status: not-in-game-yet
summary: Dropped gear with a hidden option must be appraised by an NPC for Zulie before the option's stats work
npcs:
  - "[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]"
  - "[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]"
  - "[[npcs/1034-smith-ronk|Smith Ronk]]"
  - "[[npcs/1223-smith-nel-eldora|Smith Nel Eldora]]"
  - "[[npcs/1185-magic-goods-seller-pabel|Magic Goods Seller Pabel]]"
  - "[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]"
source:
  data: LIST_JEMITEM.STB rows 1-300 (item options, with a base price) and STR_ITEMPREFIX.STL (option names); LIST_STRING.STL strings 12, 13, 28, 430, 573; NPC dialogs that call GF_appraisal
  reference: iROSE 129_129en client data; the is_appraised flag in crates/rose-data/src/item.rs, crates/rose-game-irose/src/data/drop_table.rs and ability_values.rs; godot/rust/src/conversation.rs
---
# Item appraisal

## How it works in iROSE 129

Equipment can carry an **option**: a hidden bonus such as "Demon" or "Golden" (rows 1 to
300 of LIST_JEMITEM.STB, the same field that holds a socketed gem from row 301 up). Each
option adds one or two stats. An item also has an **appraised** flag:

- An item with an option that is **not appraised** shows "[Item Appraisal Required]"
  (string 430) in its tooltip instead of the option, and the option's stats do **not**
  count while it is worn (the stat code only adds an option when the item is appraised or
  has a socket).
- Appraising sets the flag for good. The option's name then shows as a prefix and its stats
  apply.
- Items with a socket, crafted items and quest rewards come appraised.

**Appraising** is an NPC service. The NPCs above have a dialog choice such as "I'd like to
have my item appraised"; it calls `GF_appraisal`, which puts the client in appraisal mode.
The player clicks an item in their inventory and the client asks "To appraise *item*, you
need to pay *cost* Zulie." (string 12), with the cost also shown as "Appraisal Cost" (573).
On yes the server takes the Zulie and marks the item appraised. "You do not have enough
money for appraisal." (13) and "You have failed to appraise the item." (28) are the
failures.

> Open question: the appraisal cost formula. It is computed from the item (the client shows
> it before asking the server), probably from the item's and the option's base prices; it
> is not in the data files and must be taken from the original client or server code
> (rose-next).

> Open question: whether iROSE 129 drops items with an option unappraised. rose-offline's
> drop code, which our game uses, sets `is_appraised = (option != 0)` for every drop, so
> every dropped option already works and nothing ever needs appraisal. If the original
> server dropped them unappraised, that line must change when this page is built.

## What our game does today

Drops come appraised (see above), so options work at once. The NPC choice that calls
`GF_appraisal` only shows "That service is not in the game yet". Selling an unappraised
item already ignores its option's price (`npc_store_sell_price`).

## Building it

- **Server**: an `appraise_item` reducer near an appraisal NPC (range check like the
  store), cost check, take Zulie, set `is_appraised`; drop items with options unappraised
  if that is confirmed.
- **Client**: appraisal mode after `GF_appraisal`, the cost question, and the "[Item
  Appraisal Required]" tooltip line.
- **Data**: none to change.
