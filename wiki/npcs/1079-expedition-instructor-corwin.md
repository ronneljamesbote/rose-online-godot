---
kind: npc
id: 1079
name: '[Expedition Instructor] Corwin'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5478
  y: 5295
source:
  data: godot/data/npcs/1079-expedition-instructor-corwin.tres; dialog godot/data/dialogs/em21-022-expedition-instructor-corwin.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Expedition Instructor] Corwin

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** The ruins test everyone. I keep the expedition's fighters sharp, and I'll teach any trained adventurer who asks.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
