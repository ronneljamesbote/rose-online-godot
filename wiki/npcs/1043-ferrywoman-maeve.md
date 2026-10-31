---
kind: npc
id: 1043
name: '[Ferrywoman] Maeve'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
services: shop, save point
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5488
  y: 5356
source:
  data: godot/data/npcs/1043-ferrywoman-maeve.tres; dialog godot/data/dialogs/em22-013-ferrywoman-maeve.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Ferrywoman] Maeve

Runs the ferry at Riverford Post in [[zones/22-adventurer-s-plain|Adventurer's Plain]]. Sells food, medicines and return scrolls (see [[rules/shops|Shops]]) and can make the post your save point.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Ferry's free for the warden's friends, and the shop is open for everybody. Food, potions, anything for the road.
  - **You:** Show me what you sell.  → opens the shop
  - **You:** I'd like to make Riverford Post my save point.  → makes this your save point
    - **NPC:** There. If you fall on the plain, you'll get back up here at the post.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
