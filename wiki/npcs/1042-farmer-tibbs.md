---
kind: npc
id: 1042
name: '[Farmer] Tibbs'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/2206-tibbs-farm|Tibbs'' Farm]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5448
  y: 5356
source:
  data: godot/data/npcs/1042-farmer-tibbs.tres; dialog godot/data/dialogs/em22-012-farmer-tibbs.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Farmer] Tibbs

Stays at Riverford Post in [[zones/22-adventurer-s-plain|Adventurer's Plain]] since the Flanae took over his farm on the terraces west of the river.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Twenty years I farmed the terraces up the hill. Then the Flanae came up between the rows, and now they're the ones growing.
  - **You:** About "Tibbs' Farm"  *(if you don't have [[quests/2206-tibbs-farm|Tibbs' Farm]]; quest step `2206-01` passes)*
    - **NPC:** If somebody showed me those Flanae can be beaten, I'd go back. Bring me ten petals from the ones on my farm, west of the river.
      - **You:** I'll do it.  → runs quest step `2206-01` ([[quests/2206-tibbs-farm|Tibbs' Farm]])
        - **NPC:** Up the hill west of the post, where the fences are broken. That was my farm.
      - **You:** Not now.
  - **You:** About "Tibbs' Farm"  *(if you have [[quests/2206-tibbs-farm|Tibbs' Farm]]; quest step `2206-02` doesn't pass)*
    - **NPC:** Ten Farm Flanae Petals, from the Flanae on the terraces.
  - **You:** "Tibbs' Farm": it's done.  *(if quest step `2206-02` passes)*  → runs quest step `2206-02` ([[quests/2206-tibbs-farm|Tibbs' Farm]])
    - **NPC:** Ten! Maybe there's hope for the old place. Take these bananas, they're the last of my store.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
