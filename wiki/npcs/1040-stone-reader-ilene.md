---
kind: npc
id: 1040
name: '[Stone-reader] Ilene'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/2204-the-humming-stones|The Humming Stones]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5708
  y: 5396
source:
  data: godot/data/npcs/1040-stone-reader-ilene.tres; dialog godot/data/dialogs/em22-010-stone-reader-ilene.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Stone-reader] Ilene

Stands among the standing stones on Stonewatch Meadow, west of Arua's Landing, in [[zones/22-adventurer-s-plain|Adventurer's Plain]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Hush. Can you hear them? The stones hum when the wind is from the sea.
  - **You:** About "The Humming Stones"  *(if you don't have [[quests/2204-the-humming-stones|The Humming Stones]]; quest step `2204-01` passes)*
    - **NPC:** The JellyNuts gather round the standing stones, and the little cores inside them hum the same note as the stones. Bring me eight Jelly Cores and I'll find out why.
      - **You:** I'll do it.  → runs quest step `2204-01` ([[quests/2204-the-humming-stones|The Humming Stones]])
        - **NPC:** The JellyNuts stay close to the stones, up here on the meadow.
      - **You:** Not now.
  - **You:** About "The Humming Stones"  *(if you have [[quests/2204-the-humming-stones|The Humming Stones]]; quest step `2204-02` doesn't pass)*
    - **NPC:** Eight Jelly Cores, from the JellyNuts round the standing stones.
  - **You:** "The Humming Stones": it's done.  *(if quest step `2204-02` passes)*  → runs quest step `2204-02` ([[quests/2204-the-humming-stones|The Humming Stones]])
    - **NPC:** Eight, and every one humming. The stones are calling the jellies here, but something else is sickening the plain. Old Myad on the pier has been asking for help. Go and see him.
  - **You:** Who raised these stones?
    - **NPC:** Nobody remembers. The rune stones are older than Arua's Landing. I read what I can of them, and write down the rest.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
