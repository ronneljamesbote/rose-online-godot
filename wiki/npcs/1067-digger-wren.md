---
kind: npc
id: 1067
name: '[Digger] Wren'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2109-lost-diggers|Lost Diggers]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5374
  y: 5452
source:
  data: godot/data/npcs/1067-digger-wren.tres; dialog godot/data/dialogs/em21-013-digger-wren.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Digger] Wren

Stands at the shattered monument in the sunken ruins in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Don't touch the monument, please. It's been standing for a thousand years.
  - **You:** Wren? Pell sent me to find you.  *(if quest step `2109-04` passes)*  → runs quest step `2109-04` ([[quests/2109-lost-diggers|Lost Diggers]])
    - **NPC:** I couldn't leave. Look at this monument: a tower, a beam of light, and something chained under the ground. I'll be back before dark.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
