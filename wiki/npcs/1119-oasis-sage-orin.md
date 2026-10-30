---
kind: npc
id: 1119
name: '[Oasis Sage] Orin'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2114-the-sage-s-trial|The Sage''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5191
  y: 5038
source:
  data: godot/data/npcs/1119-oasis-sage-orin.tres; dialog godot/data/dialogs/em21-024-oasis-sage-orin.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Oasis Sage] Orin

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** The water here remembers the tower. Sit, if you want. Listen.
  - **You:** About "The Sage's Trial"  *(if you don't have [[quests/2114-the-sage-s-trial|The Sage's Trial]]; you don't have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]; you don't have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]; you don't have [[quests/2116-the-broker-s-trial|The Broker's Trial]]; quest step `2114-01` passes)*
    - **NPC:** Close your eyes. Can you feel it? The valley is full of mana right now, more than it should be, and the Pomics soak it up like sponges. A Muse must learn to find mana and to take it. Bring me 6 handfuls of the dust that hums. Then I will teach you to use what you found.
      - **You:** I'll do it.  → runs quest step `2114-01` ([[quests/2114-the-sage-s-trial|The Sage's Trial]])
        - **NPC:** The Pomics gather at the Fallen Watch on the Caravan Road, and by the oasis. Listen for the humming.
      - **You:** Not now.
  - **You:** About "The Sage's Trial"  *(if you have [[quests/2114-the-sage-s-trial|The Sage's Trial]]; quest step `2114-02` doesn't pass)*
    - **NPC:** Six handfuls. Listen for the hum; not every Pomic carries it.
  - **You:** "The Sage's Trial": it's done.  *(if quest step `2114-02` passes)*
    - **NPC:** Hear how it sings in your hands? You found it. That is the first lesson, and the hardest. From today you are a Muse. Let me give you something to carry the mana with. A staff, or a wand?
      - **You:** I'll take the Oasis Staff.  *(if quest step `2114-02` passes)*  → runs quest step `2114-02` ([[quests/2114-the-sage-s-trial|The Sage's Trial]])
      - **You:** I'll take the Oasis Wand.  *(if quest step `2114-21` passes)*  → runs quest step `2114-21` ([[quests/2114-the-sage-s-trial|The Sage's Trial]])
        - **NPC:** Use it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
