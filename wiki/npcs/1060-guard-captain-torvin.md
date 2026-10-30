---
kind: npc
id: 1060
name: '[Guard Captain] Torvin'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2108-hornets-in-the-dig|Hornets in the Dig]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5465
  y: 5291
source:
  data: godot/data/npcs/1060-guard-captain-torvin.tres; dialog godot/data/dialogs/em21-010-guard-captain-torvin.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Guard Captain] Torvin

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Stay inside the camp if you can't fight. The ruins are no place for sightseeing.
  - **You:** About "Hornets in the Dig"  *(if you don't have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]; quest step `2108-01` passes)*
    - **NPC:** My diggers won't go near the pyramid. Hornets nest in the old walls and drive them off. I can't spare a guard. Clear them out and bring me 8 wings as proof.
      - **You:** I'll do it.  → runs quest step `2108-01` ([[quests/2108-hornets-in-the-dig|Hornets in the Dig]])
        - **NPC:** Cross the upper bridge and head for the pyramid. The nests are in the high walls.
      - **You:** Not now.
  - **You:** About "Hornets in the Dig"  *(if you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]; quest step `2108-02` doesn't pass)*
    - **NPC:** Eight wings. Needle Hornets or Hornets, I don't mind which.
  - **You:** "Hornets in the Dig": it's done.  *(if quest step `2108-02` passes)*  → runs quest step `2108-02` ([[quests/2108-hornets-in-the-dig|Hornets in the Dig]])
    - **NPC:** Good work. My diggers can go back. Here, this beret will keep the sun off your head.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
