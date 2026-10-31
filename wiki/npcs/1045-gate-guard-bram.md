---
kind: npc
id: 1045
name: '[Gate Guard] Bram'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5188
  y: 5124
source:
  data: godot/data/npcs/1045-gate-guard-bram.tres; dialog godot/data/dialogs/em22-015-gate-guard-bram.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Gate Guard] Bram

Guards the palisade at the crossing to the Goblin Cave gate in the south-west of [[zones/22-adventurer-s-plain|Adventurer's Plain]], and warns Visitors that the cave is for level 45 and up.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Halt, Visitor! That gate leads down into the Goblin Cave. The goblins there would eat a newcomer alive.
  - **You:** How strong are they?
    - **NPC:** Level 45 and up, and they come in packs. Come back when you've made a name for yourself far from here. The road to Luxem is north, up on Gatehill.
  - **You:** Can I go in anyway?
    - **NPC:** I can't stop you. I can only tell you that nobody your size has walked back out.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
