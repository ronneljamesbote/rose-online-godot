---
kind: npc
id: 1124
name: '[Caravan Broker] Harlan'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2116-the-broker-s-trial|The Broker''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5184
  y: 5046
source:
  data: godot/data/npcs/1124-caravan-broker-harlan.tres; dialog godot/data/dialogs/em21-026-caravan-broker-harlan.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Broker] Harlan

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Everything has a price, friend. Even advice. Mine's cheap today.
  - **You:** About "The Broker's Trial"  *(if you don't have [[quests/2116-the-broker-s-trial|The Broker's Trial]]; you don't have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]; you don't have [[quests/2114-the-sage-s-trial|The Sage's Trial]]; you don't have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]; quest step `2116-01` passes)*
    - **NPC:** Anyone can swing a sword. A Dealer knows what everything is worth. The Big Pumpkins on the flats carry seeds that sell well in Zant. Bring me 6, and ask Sella and Garro what they'd pay for them. Learn the market, and I'll make you a Dealer.
      - **You:** I'll do it.  → runs quest step `2116-01` ([[quests/2116-the-broker-s-trial|The Broker's Trial]])
        - **NPC:** Big Pumpkins are all over the flats and the road. Sella sells supplies, Garro sells blades. Both are here in the camp.
      - **You:** Not now.
  - **You:** About "The Broker's Trial"  *(if you have [[quests/2116-the-broker-s-trial|The Broker's Trial]]; quest step `2116-02` doesn't pass)*
    - **NPC:** Six seeds, and the prices from Sella and Garro. A Dealer never sells without asking around.
  - **You:** "The Broker's Trial": it's done.  *(if quest step `2116-02` passes)*
    - **NPC:** Six seeds, and you know Sella pays more than Garro. You'll do well. From today you're a Dealer. Every Dealer needs something to protect the goods. A pistol, or a launcher?
      - **You:** I'll take the Broker's Pistol.  *(if quest step `2116-02` passes)*  → runs quest step `2116-02` ([[quests/2116-the-broker-s-trial|The Broker's Trial]])
      - **You:** I'll take the Broker's Launcher.  *(if quest step `2116-21` passes)*  → runs quest step `2116-21` ([[quests/2116-the-broker-s-trial|The Broker's Trial]])
        - **NPC:** Use it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
