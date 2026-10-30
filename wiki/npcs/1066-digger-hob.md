---
kind: npc
id: 1066
name: '[Digger] Hob'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2109-lost-diggers|Lost Diggers]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5268
  y: 5470
source:
  data: godot/data/npcs/1066-digger-hob.tres; dialog godot/data/dialogs/em21-012-digger-hob.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Digger] Hob

Stands at the pyramid dig in the sunken ruins in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** So much carving, so little light.
  - **You:** Hob? Pell is worried about you.  *(if quest step `2109-03` passes)*  → runs quest step `2109-03` ([[quests/2109-lost-diggers|Lost Diggers]])
    - **NPC:** Worried? I've found a doorway under the pyramid! Tell Pell I'm fine. Tell her the stone over the door shows the tower.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
