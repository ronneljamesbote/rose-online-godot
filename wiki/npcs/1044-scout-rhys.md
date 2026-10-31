---
kind: npc
id: 1044
name: '[Scout] Rhys'
status: in-game
zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
quests:
- '[[quests/2207-tracks-to-the-ford|Tracks to the Ford]]'
- '[[quests/2208-poisoned-seed|Poisoned Seed]]'
locations:
- zone: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
  x: 5208
  y: 5196
source:
  data: godot/data/npcs/1044-scout-rhys.tres; dialog godot/data/dialogs/em22-014-scout-rhys.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Scout] Rhys

Watches the strangers' abandoned camp at Bramble Ford, in the south-west of [[zones/22-adventurer-s-plain|Adventurer's Plain]], near the Jelly King's hollow.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Quiet. They might come back. Whoever camped here left in a hurry, and left this place poisoned.
  - **You:** Corvel sent me to find you.  *(if quest step `2207-02` passes)*  → runs quest step `2207-02` ([[quests/2207-tracks-to-the-ford|Tracks to the Ford]])
    - **NPC:** Corvel sent you? Good. Look at this camp: cold fire, torn tents, and seed sacks gone black. Somebody poisoned that seed and fed it to the plain.
  - **You:** About "Poisoned Seed"  *(if you don't have [[quests/2208-poisoned-seed|Poisoned Seed]]; quest step `2208-01` passes)*
    - **NPC:** The Choropies round here dragged the sacks off. Bring me six of them back so Corvel can see what we're dealing with.
      - **You:** I'll do it.  → runs quest step `2208-01` ([[quests/2208-poisoned-seed|Poisoned Seed]])
        - **NPC:** The Spotted and Mother Choropies round the camp have them. They're tougher than they look.
      - **You:** Not now.
  - **You:** About "Poisoned Seed"  *(if you have [[quests/2208-poisoned-seed|Poisoned Seed]]; quest step `2208-02` doesn't pass)*
    - **NPC:** Six Rotten Seed Sacks, from the Choropies round the camp.
  - **You:** "Poisoned Seed": it's done.  *(if quest step `2208-02` passes)*  → runs quest step `2208-02` ([[quests/2208-poisoned-seed|Poisoned Seed]])
    - **NPC:** Six. All poisoned, and all from the same red-sealed sacks. Go back to Corvel and tell him. And watch the hollow to the west.
  - **You:** What is in the hollow?
    - **NPC:** The Jelly King. It gorged itself on the seed and it's twice the size it was. Don't go in alone.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Adventurer's Plain redesign, using an existing iROSE model.
