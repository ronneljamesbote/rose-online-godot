---
kind: npc
id: 1047
name: '[Herbalist] Linnea'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/102-dreadful-epidemic|Dreadful Epidemic]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5652
  y: 5072
source:
  data: godot/data/npcs/1047-herbalist-linnea.tres; dialog godot/data/dialogs/em22-017-herbalist-linnea.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Herbalist] Linnea

Studies the rotting mushroom ring in Whisperwood, south-west of Arua's Landing, in [[zones/22-adventurer-s-plain|Adventurer's Plain]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Careful where you step. The mushrooms of this ring were healthy a month ago. Now look at them.
  - **You:** I'm looking for clues about the epidemic.  *(if quest step `102-01` passes)*
    - **NPC:** Seyon sent you too? Then look here, at the edge of the ring. Something is half buried in the rot.
      - **You:** Dig it up with your hands.
        - **NPC:** You pull out an emblem decorated with strange patterns.
          - **You:** I'd better show this to Myad.  → runs quest step `102-01` ([[quests/102-dreadful-epidemic|Dreadful Epidemic]])
  - **You:** What made them rot?
    - **NPC:** People in red camped here, I saw their fire from the wood. They left, and the ring started to die. Whatever they carried, they took it south-west, towards the ford.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
