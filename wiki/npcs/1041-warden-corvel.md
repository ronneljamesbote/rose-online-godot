---
kind: npc
id: 1041
name: '[Warden] Corvel'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/104-mysterious-emblem|Mysterious Emblem]]'
- '[[quests/2205-ford-watch|Ford Watch]]'
- '[[quests/2207-tracks-to-the-ford|Tracks to the Ford]]'
- '[[quests/2209-the-jelly-king|The Jelly King]]'
- '[[quests/2210-over-the-ridge|Over the Ridge]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5462
  y: 5370
source:
  data: godot/data/npcs/1041-warden-corvel.tres; dialog godot/data/dialogs/em22-011-warden-corvel.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Warden] Corvel

The warden of Riverford Post, the palisaded ferry stop where the road crosses the two rivers in [[zones/22-adventurer-s-plain|Adventurer's Plain]]. Most of the plain's story quests go through him.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Riverford Post holds the bridges, and the bridges hold the plain together. I'm Corvel, the warden here. What do you need?
  - **You:** Myad sent me with this emblem.  *(if quest step `104-21` passes)*  → runs quest step `104-21` ([[quests/104-mysterious-emblem|Mysterious Emblem]])
    - **NPC:** So this is the emblem Myad told me about. Red cloaks, poisoned mushrooms, and now this. Keep it safe for now: it will need to go to Luxem in the end. But first, my bridges.
  - **You:** About "Ford Watch"  *(if you don't have [[quests/2205-ford-watch|Ford Watch]]; quest step `2205-01` passes)*
    - **NPC:** Red Jelly Beans have taken the river banks by both bridges, and the farmers won't cross. Bring me eight Red Jelly Drops and the road is open again.
      - **You:** I'll do it.  → runs quest step `2205-01` ([[quests/2205-ford-watch|Ford Watch]])
        - **NPC:** They sit along the banks, right by the bridges. Watch your back on the planks.
      - **You:** Not now.
  - **You:** About "Ford Watch"  *(if you have [[quests/2205-ford-watch|Ford Watch]]; quest step `2205-02` doesn't pass)*
    - **NPC:** Eight Red Jelly Drops, from the Red Jelly Beans by the bridges.
  - **You:** "Ford Watch": it's done.  *(if quest step `2205-02` passes)*
    - **NPC:** The banks are clear. Pick a weapon from the armoury. Riverford made, any Visitor can use it.
      - **You:** I'll take the Riverford Sword.  *(if quest step `2205-02` passes)*  → runs quest step `2205-02` ([[quests/2205-ford-watch|Ford Watch]])
        - **NPC:** Use it well, <NAME>.
      - **You:** I'll take the Riverford Spear.  *(if quest step `2205-21` passes)*  → runs quest step `2205-21` ([[quests/2205-ford-watch|Ford Watch]])
        - **NPC:** Use it well, <NAME>.
      - **You:** I'll take the Riverford Bow and arrows.  *(if quest step `2205-22` passes)*  → runs quest step `2205-22` ([[quests/2205-ford-watch|Ford Watch]])
        - **NPC:** Use it well, <NAME>.
      - **You:** I'll take the Riverford Gun and bullets.  *(if quest step `2205-23` passes)*  → runs quest step `2205-23` ([[quests/2205-ford-watch|Ford Watch]])
        - **NPC:** Use it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** About "Tracks to the Ford"  *(if you don't have [[quests/2207-tracks-to-the-ford|Tracks to the Ford]]; quest step `2207-01` passes)*
    - **NPC:** Scout Rhys went south-west to Bramble Ford two days ago, after a fire was seen there. He hasn't sent word. Find him.
      - **You:** I'll do it.  → runs quest step `2207-01` ([[quests/2207-tracks-to-the-ford|Tracks to the Ford]])
        - **NPC:** Follow the road south from the post, past the river mouth. Rhys will be near the old camp.
      - **You:** Not now.
  - **You:** About "Tracks to the Ford"  *(if you have [[quests/2207-tracks-to-the-ford|Tracks to the Ford]])*
    - **NPC:** Rhys is at the strangers' camp in Bramble Ford, south-west of here.
  - **You:** About "The Jelly King"  *(if you don't have [[quests/2209-the-jelly-king|The Jelly King]]; quest step `2209-01` passes)*
    - **NPC:** Rhys says the Jelly King is back in its hollow at Bramble Ford, fat on the poisoned seed. Drive it out and bring me its crown. Take friends: it hits harder than anything else on the plain.
      - **You:** I'll do it.  → runs quest step `2209-01` ([[quests/2209-the-jelly-king|The Jelly King]])
        - **NPC:** The hollow is west of the strangers' camp. It comes back every few minutes, so don't linger.
      - **You:** Not now.
  - **You:** About "The Jelly King"  *(if you have [[quests/2209-the-jelly-king|The Jelly King]]; quest step `2209-02` doesn't pass)*
    - **NPC:** Bring me the Jelly King's Crown. Its hollow is west of the strangers' camp.
  - **You:** "The Jelly King": it's done.  *(if quest step `2209-02` passes)*  → runs quest step `2209-02` ([[quests/2209-the-jelly-king|The Jelly King]])
    - **NPC:** The crown! The plain owes you. Wear this coat, the last of the Riverford set, and take a return scroll for the road.
  - **You:** About "Over the Ridge"  *(if you don't have [[quests/2210-over-the-ridge|Over the Ridge]]; quest step `2210-01` passes)*
    - **NPC:** Red cloaks, poisoned seed, that emblem... This is bigger than Riverford. Take the emblem over the ridge to Dorran, the caravan master at the Sandwind Caravan Camp in the Valley of Luxem Tower. Either gate on Gatehill gets you there.
      - **You:** I'll do it.  → runs quest step `2210-01` ([[quests/2210-over-the-ridge|Over the Ridge]])
        - **NPC:** Up the road to Gatehill, then through either gate. Dorran's camp is right on the other side.
      - **You:** Not now.
  - **You:** About "Over the Ridge"  *(if you have [[quests/2210-over-the-ridge|Over the Ridge]])*
    - **NPC:** Dorran, at the Sandwind Caravan Camp, just past the Luxem gates.
  - **You:** What is Riverford Post?
    - **NPC:** A ferry stop that grew a palisade. Maeve runs the ferry and sells what travellers need, and Tibbs is staying with us since the Flanae took his farm.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
