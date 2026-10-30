---
kind: npc
id: 1077
name: '[Expedition Tailor] Yselle'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5448
  y: 5308
source:
  data: godot/data/npcs/1077-expedition-tailor-yselle.tres; dialog godot/data/dialogs/em21-020-expedition-tailor-yselle.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Expedition Tailor] Yselle

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Ruin work wears clothes out in a week. I mend them, and I sell new ones.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
