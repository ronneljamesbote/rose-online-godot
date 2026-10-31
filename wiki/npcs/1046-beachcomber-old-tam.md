---
kind: npc
id: 1046
name: '[Beachcomber] Old Tam'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/2203-bones-on-the-beach|Bones on the Beach]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5784
  y: 5452
source:
  data: godot/data/npcs/1046-beachcomber-old-tam.tres; dialog godot/data/dialogs/em22-016-beachcomber-old-tam.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Beachcomber] Old Tam

Combs the northern beach by the old whale bones in [[zones/22-adventurer-s-plain|Adventurer's Plain]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Bones, shells, bottles. The sea gives back everything in the end, if you wait long enough.
  - **You:** Wade sent me to find the whale bones.  *(if quest step `2203-11` passes)*  → runs quest step `2203-11` ([[quests/2203-bones-on-the-beach|Bones on the Beach]])
    - **NPC:** Wade sent you? Ha. Here, take this tooth. I carved a gull on it. Tell him the old whale is still here, and so am I.
  - **You:** What happened to the whale?
    - **NPC:** Washed up before the village had a jetty. The Mini-Jelly Beans like the shade under its ribs. Mind them, they bite more than they look.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
