---
kind: npc
id: 1019
name: '[Bridge Guard] Tamsin'
status: in-game
zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
locations:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5283.0
  y: 5428.0
source:
  data: godot/data/npcs/1019-bridge-guard-tamsin.tres; dialog godot/data/dialogs/em01-019-bridge-guard-tamsin.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Bridge Guard] Tamsin

Guards the bridge to the north gate and warns Visitors that the [[zones/24-el-verloon-desert|El Verloon Desert]] beyond it is no place for beginners.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Halt. The bridge leads to the El Verloon Desert. Are you sure you want to go that way?
  - **You:** What's in El Verloon?
    - **NPC:** Sand, heat, and monsters of every strength mixed together. Strong travellers go there; new ones come back on a stretcher, if at all. Try Breezy Hills to the east first.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** I'm just looking.
    - **NPC:** Look all you like. The river's pretty from here.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Zant city hub redesign, using an existing iROSE model.
