---
kind: npc
id: 1068
name: '[Digger] Fenn'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2109-lost-diggers|Lost Diggers]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5426
  y: 5488
source:
  data: godot/data/npcs/1068-digger-fenn.tres; dialog godot/data/dialogs/em21-014-digger-fenn.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Digger] Fenn

Stands at the Processional Way, the road up to the tower in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Are they gone? The Pomic Fighters?
  - **You:** Fenn? Are you hurt?  *(if quest step `2109-05` passes)*  → runs quest step `2109-05` ([[quests/2109-lost-diggers|Lost Diggers]])
    - **NPC:** Just my pride. The Pomic Fighters chased me up the road and I've been hiding behind these pillars. Tell Pell the road is the old pilgrims' way to the tower.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
