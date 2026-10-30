---
kind: npc
id: 1080
name: '[Caravan Sergeant] Brakka'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2113-the-sergeant-s-trial|The Sergeant''s Trial]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5220
  y: 5056
source:
  data: godot/data/npcs/1080-caravan-sergeant-brakka.tres; dialog godot/data/dialogs/em21-023-caravan-sergeant-brakka.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Sergeant] Brakka

Stands at the Sandwind Caravan Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Stand up straight. The caravan's guards are trained by me, and I don't train cowards.
  - **You:** About "The Sergeant's Trial"  *(if you don't have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]; you don't have [[quests/2114-the-sage-s-trial|The Sage's Trial]]; you don't have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]; you don't have [[quests/2116-the-broker-s-trial|The Broker's Trial]]; quest step `2113-01` passes)*
    - **NPC:** You want to be a Soldier? Then show me you can stand in front of something and not run. The Elder Pumpkins in the Withered Fields hit hard and don't give up. Bring me 8 of their thorned vines. Come back alive and I'll make you a Soldier, here and now.
      - **You:** I'll do it.  → runs quest step `2113-01` ([[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]])
        - **NPC:** North of the camp, past the road. The fields with the dead crops. Don't come back with fewer than eight.
      - **You:** Not now.
  - **You:** About "The Sergeant's Trial"  *(if you have [[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]]; quest step `2113-02` doesn't pass)*
    - **NPC:** Eight vines. If you're hurt, rest, then go again. That's the job.
  - **You:** "The Sergeant's Trial": it's done.  *(if quest step `2113-02` passes)*
    - **NPC:** Eight. You stood your ground, and that's all a Soldier really is. From today you're one of us. Every Soldier of the caravan gets a blade from me. Which kind?
      - **You:** I'll take the Sandwind Blade.  *(if quest step `2113-02` passes)*  → runs quest step `2113-02` ([[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]])
      - **You:** I'll take the Sandwind Greatsword.  *(if quest step `2113-21` passes)*  → runs quest step `2113-21` ([[quests/2113-the-sergeant-s-trial|The Sergeant's Trial]])
        - **NPC:** Use it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
