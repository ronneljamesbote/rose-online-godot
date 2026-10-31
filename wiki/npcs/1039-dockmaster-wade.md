---
kind: npc
id: 1039
name: '[Dockmaster] Wade'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]'
- '[[quests/2203-bones-on-the-beach|Bones on the Beach]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5840
  y: 5302
source:
  data: godot/data/npcs/1039-dockmaster-wade.tres; dialog godot/data/dialogs/em22-009-dockmaster-wade.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Dockmaster] Wade

Stands on the north jetty of Arua's Landing, by the ship, in [[zones/22-adventurer-s-plain|Adventurer's Plain]]. Looks after the jetties and sends new Visitors out along the beach.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Mind the planks, they're slick. I'm Wade, I keep the jetties of Arua's Landing in one piece. Mostly.
  - **You:** About "Jelly on the Jetty"  *(if you don't have [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]; quest step `2202-01` passes)*
    - **NPC:** The Mini-Jelly Beans on the beach leave a goo that sets hard as tar. Bring me six blobs of Jelly Goo and I can seal these planks before the tide gets under them.
      - **You:** I'll do it.  → runs quest step `2202-01` ([[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]])
        - **NPC:** Mini-Jelly Beans are all along the sand, north and south of the village. Easy work for a new arrival.
      - **You:** Not now.
  - **You:** About "Jelly on the Jetty"  *(if you have [[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]]; quest step `2202-02` doesn't pass)*
    - **NPC:** Six blobs of Jelly Goo. The Mini-Jelly Beans on the beach drop it.
  - **You:** "Jelly on the Jetty": it's done.  *(if quest step `2202-02` passes)*  → runs quest step `2202-02` ([[quests/2202-jelly-on-the-jetty|Jelly on the Jetty]])
    - **NPC:** That'll do nicely. Here, some grapes from the ship's stores. Don't tell the captain.
  - **You:** About "Bones on the Beach"  *(if you don't have [[quests/2203-bones-on-the-beach|Bones on the Beach]]; quest step `2203-01` passes)*
    - **NPC:** Every Visitor should walk the whole shore once. Up north there are the bones of an old whale, and Old Tam, who combs the beach there. Bring me something from him so I know you went.
      - **You:** I'll do it.  → runs quest step `2203-01` ([[quests/2203-bones-on-the-beach|Bones on the Beach]])
        - **NPC:** Follow the sand north past the standing stones' hill. You can't miss the bones.
      - **You:** Not now.
  - **You:** About "Bones on the Beach"  *(if you have [[quests/2203-bones-on-the-beach|Bones on the Beach]]; quest step `2203-02` doesn't pass)*
    - **NPC:** Old Tam is at the whale bones, up the beach to the north.
  - **You:** "Bones on the Beach": it's done.  *(if quest step `2203-02` passes)*  → runs quest step `2203-02` ([[quests/2203-bones-on-the-beach|Bones on the Beach]])
    - **NPC:** A carved tooth! Tam only gives those to people he likes. Take these gloves, Riverford made, better than anything Ronk has for Visitors.
  - **You:** What is there to see on the plain?
    - **NPC:** East is the sea, west is everything else. The road runs west over two rivers to Riverford Post, then up to the gates of the Valley of Luxem Tower. The further you go from the sea, the meaner the monsters.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
