---
kind: npc
id: 1058
name: '[Expedition Leader] Ardeth'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]]'
- '[[quests/2111-the-warden-below|The Warden Below]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5459
  y: 5314
source:
  data: godot/data/npcs/1058-expedition-leader-ardeth.tres; dialog godot/data/dialogs/em21-008-expedition-leader-ardeth.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Expedition Leader] Ardeth

Stands at the Arumic Expedition Camp on the East Terrace in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Welcome to the Arumic Expedition Camp. Mind the crates, some of them are older than Zant.
  - **You:** "A Shard for the Expedition": it's done.  *(if quest step `2107-02` passes)*  → runs quest step `2107-02` ([[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]])
    - **NPC:** A Pomic shard, soaked in mana. Shannon was right to send you. This is what we came for: the tower is failing, and the valley pays for it. My people could use someone like you.
  - **You:** About "The Warden Below"  *(if you don't have [[quests/2111-the-warden-below|The Warden Below]]; quest step `2111-01` passes)*
    - **NPC:** Pell's reading matches what we found. Under the dig there is a sealed vault, and something in it is waking. My crane can lower you inside. Defeat the Ruin Warden and bring me the Lens Core it guards. Take friends if you have them.
      - **You:** I'll do it.  → runs quest step `2111-01` ([[quests/2111-the-warden-below|The Warden Below]])
        - **NPC:** When you're ready, ask me to ride the crane down. Gus runs the lift in the vault; he'll bring you back up.
      - **You:** Not now.
  - **You:** About "The Warden Below"  *(if you have [[quests/2111-the-warden-below|The Warden Below]]; quest step `2111-02` doesn't pass)*
    - **NPC:** The Ruin Warden holds the Lens Core. Ask me when you want to go down.
  - **You:** "The Warden Below": it's done.  *(if quest step `2111-02` passes)*  → runs quest step `2111-02` ([[quests/2111-the-warden-below|The Warden Below]])
    - **NPC:** The Lens Core. Feel how warm it is. It belongs at the top of Luxem Tower. Take this, and then take the core to Lutis at the tower.
  - **You:** Lower me into the vault with the crane.  *(if quest step `2111-03` passes; quest switch 609 = 1)*  → runs quest step `2111-03` ([[quests/2111-the-warden-below|The Warden Below]])
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
