---
kind: npc
id: 1069
name: '[Wagon Driver] Oskar'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5405
  y: 5144
source:
  data: godot/data/npcs/1069-wagon-driver-oskar.tres; dialog godot/data/dialogs/em21-015-wagon-driver-oskar.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Wagon Driver] Oskar

Stands at his wrecked wagon in the dry wadi in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Water... is there any water left?
  - **You:** Oskar? Dorran sent me.  *(if quest step `2104-03` passes)*  → runs quest step `2104-03` ([[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]])
    - **NPC:** Dorran... thank the stars. Cowboy Pumpkins jumped us in the wadi. They took every crate up to their perch in the red rocks. And the river, it was dry, all of it. It was full last month.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
