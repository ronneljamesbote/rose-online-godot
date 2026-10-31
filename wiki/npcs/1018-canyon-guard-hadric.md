---
kind: npc
id: 1018
name: '[Canyon Guard] Hadric'
status: in-game
zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
locations:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5374.0
  y: 5339.0
source:
  data: godot/data/npcs/1018-canyon-guard-hadric.tres; dialog godot/data/dialogs/em01-018-canyon-guard-hadric.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Canyon Guard] Hadric

Keeps watch from the post at the foot of the north-east mesa, by the Training Grounds. He tells you where the roads out of Zant go, and warns you about the [[monsters/205-woopie-king|Woopie King]] in Woopie Hollow.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** From up here I can see every road out of Zant. Lost already?
  - **You:** Where do the roads go?
    - **NPC:** East, through the Wind Road Caravan Camp, is Breezy Hills: start there. South is the gate to the Valley of Luxem Tower, where you came from. North, over the bridge, is the El Verloon Desert, and that's no place for a beginner.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** What do you watch for?
    - **NPC:** Rackies on the Wind Road, sandstorms from El Verloon, and Woopies. A big Woopie that calls himself king sits in the hollow south-east of the city with his court. Leave him be unless you're ready for him.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Zant city hub redesign, using an existing iROSE model.
