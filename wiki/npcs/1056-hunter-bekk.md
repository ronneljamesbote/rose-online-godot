---
kind: npc
id: 1056
name: '[Hunter] Bekk'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5212
  y: 5047
source:
  data: godot/data/npcs/1056-hunter-bekk.tres; dialog godot/data/dialogs/em21-006-hunter-bekk.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Hunter] Bekk

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Watch your step out there. The road isn't as safe as it was.
  - **You:** About "Pumpkins on the Road"  *(if you don't have [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]; quest step `2102-01` passes)*
    - **NPC:** The Pumpkins have moved out of the fields and onto the road. Wagons can't pass when they block it. Thin them out and bring me 10 of their stalks, so I know it's done.
      - **You:** I'll do it.  → runs quest step `2102-01` ([[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]])
        - **NPC:** Pumpkins and Big Pumpkins, both. The patches are all around the flats and along the road.
      - **You:** Not now.
  - **You:** About "Pumpkins on the Road"  *(if you have [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]; quest step `2102-02` doesn't pass)*
    - **NPC:** Ten stalks. The Pumpkins won't move off the road by themselves.
  - **You:** "Pumpkins on the Road": it's done.  *(if quest step `2102-02` passes)*  → runs quest step `2102-02` ([[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]])
    - **NPC:** Ten. That will keep the road open for a few days. Here, you earned this.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
