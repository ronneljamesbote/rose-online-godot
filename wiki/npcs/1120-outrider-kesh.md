---
kind: npc
id: 1120
name: '[Outrider] Kesh'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2115-the-outrider-s-trial|The Outrider''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5207
  y: 5038
source:
  data: godot/data/npcs/1120-outrider-kesh.tres; dialog godot/data/dialogs/em21-025-outrider-kesh.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Outrider] Kesh

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Looking for the outriders? You found one. The fast one.
  - **You:** About "The Outrider's Trial"  *(if you don't have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]; you don't have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]; you don't have [[quests/2114-the-sage-s-trial|The Sage's Trial]]; you don't have [[quests/2116-the-broker-s-trial|The Broker's Trial]]; quest step `2115-01` passes)*
    - **NPC:** A Hawker hits first and is gone before the other side knows. Honeybees are the fastest things in this valley. Bring me 10 of their wings, in one piece. Do it and you'll ride out with me as a Hawker.
      - **You:** I'll do it.  → runs quest step `2115-01` ([[quests/2115-the-outrider-s-trial|The Outrider's Trial]])
        - **NPC:** The bees swarm around the dead trees with hives, on the flats and by the road. Be quicker than them.
      - **You:** Not now.
  - **You:** About "The Outrider's Trial"  *(if you have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]; quest step `2115-02` doesn't pass)*
    - **NPC:** Ten wings, whole ones. Torn wings don't count.
  - **You:** "The Outrider's Trial": it's done.  *(if quest step `2115-02` passes)*
    - **NPC:** Ten, and barely a scratch on you. You're quick. From today you're a Hawker, and you ride with the outriders. Take a weapon from me: a bow to hit them from far away, or a katar to hit them up close?
      - **You:** I'll take the Outrider Bow.  *(if quest step `2115-02` passes)*  → runs quest step `2115-02` ([[quests/2115-the-outrider-s-trial|The Outrider's Trial]])
      - **You:** I'll take the Outrider Katar.  *(if quest step `2115-21` passes)*  → runs quest step `2115-21` ([[quests/2115-the-outrider-s-trial|The Outrider's Trial]])
        - **NPC:** Use it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
