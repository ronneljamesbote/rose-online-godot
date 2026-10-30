---
kind: npc
id: 1070
name: '[Lift Keeper] Gus'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2111-the-warden-below|The Warden Below]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5210
  y: 5538
source:
  data: godot/data/npcs/1070-lift-keeper-gus.tres; dialog godot/data/dialogs/em21-016-lift-keeper-gus.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Lift Keeper] Gus

Stands at the sealed vault under the dig in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** This is as deep as the lift goes. Say the word and I'll take you up.
  - **You:** Take me back up.  → runs quest step `2111-04` ([[quests/2111-the-warden-below|The Warden Below]])
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
