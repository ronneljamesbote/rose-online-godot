---
kind: npc
id: 1074
name: '[Caravan Smith] Garro'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
quests:
- '[[quests/2116-the-broker-s-trial|The Broker''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5189
  y: 5070
source:
  data: godot/data/npcs/1074-caravan-smith-garro.tres; dialog godot/data/dialogs/em21-017-caravan-smith-garro.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Smith] Garro

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Every wagon needs a blade or two on board. Have a look, they're all sharp.
  - **You:** Harlan asks what you'd pay for pumpkin seeds.  *(if quest step `2116-04` passes)*  → runs quest step `2116-04` ([[quests/2116-the-broker-s-trial|The Broker's Trial]])
    - **NPC:** Seeds? I'm a smith, not a cook. Eight Zuly, and only because I like Harlan.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
