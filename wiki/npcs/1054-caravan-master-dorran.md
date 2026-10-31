---
kind: npc
id: 1054
name: '[Caravan Master] Dorran'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2210-over-the-ridge|Over the Ridge]]'
- '[[quests/2101-dust-on-the-road|Dust on the Road]]'
- '[[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]'
- '[[quests/2105-stolen-supplies|Stolen Supplies]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5192
  y: 5062
source:
  data: godot/data/npcs/1054-caravan-master-dorran.tres; dialog godot/data/dialogs/em21-004-caravan-master-dorran.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Master] Dorran

Stands at the Sandwind Caravan Camp (Hub 1, by the Sandgate) in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Welcome to the Sandwind Caravan Camp. We trade between the Adventurer's Plain and Zant, when the road lets us.
  - **You:** About "Dust on the Road"  *(if you don't have [[quests/2101-dust-on-the-road|Dust on the Road]]; quest step `2101-01` passes)*
    - **NPC:** Another traveller through the Sandgate! The road to Zant is longer than it looks, and the valley has turned strange. Before you go on, see Mira on the rock over there. She has the best eyes in the camp.
      - **You:** I'll do it.  → runs quest step `2101-01` ([[quests/2101-dust-on-the-road|Dust on the Road]])
        - **NPC:** Good. Mira will tell you what she has seen from up there.
      - **You:** Not now.
  - **You:** About "The Wagon That Never Came"  *(if you don't have [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]; quest step `2104-01` passes)*
    - **NPC:** Oskar left the oasis with a full wagon three days ago and never arrived. The river used to run past the oasis, and the wagons followed it. Now it's a dry wadi. Find him, please.
      - **You:** I'll do it.  → runs quest step `2104-01` ([[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]])
        - **NPC:** Follow the dry riverbed south-east from the oasis. Bring me news.
      - **You:** Not now.
  - **You:** About "The Wagon That Never Came"  *(if you have [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]; quest step `2104-02` doesn't pass)*
    - **NPC:** The wadi starts at the oasis and runs south-east. Oskar's wagon has a red cover.
  - **You:** "The Wagon That Never Came": it's done.  *(if quest step `2104-02` passes)*  → runs quest step `2104-02` ([[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]])
    - **NPC:** Raiders? Cowboy Pumpkins, taking crates? And the river dry when he crossed... I'll send a cart for Oskar. Thank you for finding him.
  - **You:** About "Stolen Supplies"  *(if you don't have [[quests/2105-stolen-supplies|Stolen Supplies]]; quest step `2105-01` passes)*
    - **NPC:** Those crates are half our stock. The Cowboy Pumpkins carried them up to Raider's Perch, the plateau in the Redrock Canyons. Bring back six and I'll make it worth your while.
      - **You:** I'll do it.  → runs quest step `2105-01` ([[quests/2105-stolen-supplies|Stolen Supplies]])
        - **NPC:** Take the pass south of the wreck. The ramp up to the perch is steep, and they won't be alone.
      - **You:** Not now.
  - **You:** About "Stolen Supplies"  *(if you have [[quests/2105-stolen-supplies|Stolen Supplies]]; quest step `2105-02` doesn't pass)*
    - **NPC:** Six crates. The Cowboy Pumpkins carry them around their camps.
  - **You:** "Stolen Supplies": it's done.  *(if quest step `2105-02` passes)*
    - **NPC:** Six crates, and the seals unbroken. Take a pair of gloves from our stock. Pick the ones that suit how you fight.
      - **You:** I'll take the Luxem Guard Gloves.  *(if quest step `2105-02` passes)*  → runs quest step `2105-02` ([[quests/2105-stolen-supplies|Stolen Supplies]])
      - **You:** I'll take the Luxem Acolyte Gloves.  *(if quest step `2105-21` passes)*  → runs quest step `2105-21` ([[quests/2105-stolen-supplies|Stolen Supplies]])
      - **You:** I'll take the Luxem Stalker Gloves.  *(if quest step `2105-22` passes)*  → runs quest step `2105-22` ([[quests/2105-stolen-supplies|Stolen Supplies]])
      - **You:** I'll take the Luxem Broker Gloves.  *(if quest step `2105-23` passes)*  → runs quest step `2105-23` ([[quests/2105-stolen-supplies|Stolen Supplies]])
        - **NPC:** Wear it well, <NAME>.
      - **You:** Let me think about it.
  - **You:** "Over the Ridge": Corvel sent me with this emblem.  *(if quest step `2210-02` passes)*  → runs quest step `2210-02` ([[quests/2210-over-the-ridge|Over the Ridge]])
    - **NPC:** From Riverford? Corvel's seal, and... this emblem. The same patterns we found on the raiders' crates. You did right to bring it here. Rest at the camp, then come and talk to me about the road.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.

The Adventurer's Plain redesign added the "Over the Ridge" answer: [[quests/2210-over-the-ridge|Over the Ridge]] ends here, where the plain's story hands over to this valley's.
