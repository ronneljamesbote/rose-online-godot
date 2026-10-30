---
kind: npc
id: 1078
name: '[Caravan Tutor] Ilsa'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5181
  y: 5053
source:
  data: godot/data/npcs/1078-caravan-tutor-ilsa.tres; dialog godot/data/dialogs/em21-021-caravan-tutor-ilsa.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Tutor] Ilsa

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Fighting is more than swinging a blade. I teach caravan guards their craft. Once you have a job, I can teach you too.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
