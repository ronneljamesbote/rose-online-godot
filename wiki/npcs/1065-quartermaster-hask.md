---
kind: npc
id: 1065
name: '[Quartermaster] Hask'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5453
  y: 5300
source:
  data: godot/data/npcs/1065-quartermaster-hask.tres; dialog godot/data/dialogs/em21-011-quartermaster-hask.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Quartermaster] Hask

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Supplies for the expedition. If you're working for Ardeth, I'll sell you some.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
