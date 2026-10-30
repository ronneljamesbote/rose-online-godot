---
kind: npc
id: 1057
name: '[Caravan Trader] Sella'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
services: shop
quests:
- '[[quests/2116-the-broker-s-trial|The Broker''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5207
  y: 5067
source:
  data: godot/data/npcs/1057-caravan-trader-sella.tres; dialog godot/data/dialogs/em21-007-caravan-trader-sella.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Trader] Sella

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]. Sells what the caravan and expedition shops sell (see [[rules/shops|Shops]]).

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Potions, arrows, a return scroll for when it all goes wrong. What do you need?
  - **You:** Harlan asks what you'd pay for pumpkin seeds.  *(if quest step `2116-03` passes)*  → runs quest step `2116-03` ([[quests/2116-the-broker-s-trial|The Broker's Trial]])
    - **NPC:** Pumpkin seeds? Twelve Zuly a sack, and that's generous. Tell Harlan he still owes me for the last ones.
  - **You:** Let me see your goods.  → opens the shop
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
