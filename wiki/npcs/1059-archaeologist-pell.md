---
kind: npc
id: 1059
name: '[Archaeologist] Pell'
status: in-game
zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
quests:
- '[[quests/2109-lost-diggers|Lost Diggers]]'
- '[[quests/2110-words-in-stone|Words in Stone]]'
locations:
- zone: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
  x: 5473
  y: 5304
source:
  data: godot/data/npcs/1059-archaeologist-pell.tres; dialog godot/data/dialogs/em21-009-archaeologist-pell.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Archaeologist] Pell

Stands at the Arumic Expedition Camp in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** Every stone in these ruins has something written on it. Most of it is complaints about the weather.
  - **You:** About "Lost Diggers"  *(if you don't have [[quests/2109-lost-diggers|Lost Diggers]]; quest step `2109-01` passes)*
    - **NPC:** Hob, Wren and Fenn went into the ruins with their notes and haven't come back. Hob was at the pyramid, Wren at the shattered monument and Fenn on the road up to the tower. Find them for me.
      - **You:** I'll do it.  → runs quest step `2109-01` ([[quests/2109-lost-diggers|Lost Diggers]])
        - **NPC:** The pyramid first, it's the closest. Then the monument, east of it, and the Processional Way.
      - **You:** Not now.
  - **You:** About "Lost Diggers"  *(if you have [[quests/2109-lost-diggers|Lost Diggers]]; quest step `2109-02` doesn't pass)*
    - **NPC:** Hob at the pyramid, Wren at the shattered monument, Fenn on the Processional Way.
  - **You:** "Lost Diggers": it's done.  *(if quest step `2109-02` passes)*  → runs quest step `2109-02` ([[quests/2109-lost-diggers|Lost Diggers]])
    - **NPC:** All three alive, and with their notes. The carvings they copied all say the same thing: the tower is a lens. It holds something down under the valley.
  - **You:** About "Words in Stone"  *(if you don't have [[quests/2110-words-in-stone|Words in Stone]]; quest step `2110-01` passes)*
    - **NPC:** The Pomic Soldiers carry stones from the ruin walls in their bodies. Some of those stones are carved. Bring me four inscribed fragments and I can read the rest of the story.
      - **You:** I'll do it.  → runs quest step `2110-01` ([[quests/2110-words-in-stone|Words in Stone]])
        - **NPC:** The Pomic Soldiers stand guard at the pyramid and the ruin gates.
      - **You:** Not now.
  - **You:** About "Words in Stone"  *(if you have [[quests/2110-words-in-stone|Words in Stone]]; quest step `2110-02` doesn't pass)*
    - **NPC:** Four fragments. They are heavy, but don't drop them.
  - **You:** "Words in Stone": it's done.  *(if quest step `2110-02` passes)*  → runs quest step `2110-02` ([[quests/2110-words-in-stone|Words in Stone]])
    - **NPC:** Here it is. 'The lens burns, the warden sleeps. When the lens dims, the warden wakes.' The tower's crystal is dimming, and there is a vault under the dig. Tell Ardeth.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Valley of Luxem Tower redesign, using an existing iROSE model.
