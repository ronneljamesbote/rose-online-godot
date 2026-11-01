---
kind: npc
id: 1017
name: '[Caravan Master] Odell'
status: in-game
zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
quests:
- '[[quests/2412-the-wind-road|The Wind Road]]'
locations:
- zone: '[[zones/1-canyon-city-of-zant|Canyon City of Zant]]'
  x: 5507.0
  y: 5182.0
source:
  data: godot/data/npcs/1017-caravan-master-odell.tres; dialog godot/data/dialogs/em01-017-caravan-master-odell.tres
  code:
  - module/src/npcs.rs
  - godot/rust/src/conversation.rs
---
# [Caravan Master] Odell

Runs the Wind Road Caravan Camp by Zant's east gate, where the road to [[zones/23-breezy-hills|Breezy Hills]] starts. Lena's last lesson, [[quests/2412-the-wind-road|The Wind Road]], ends with him.

## Dialog

The NPC says the **last** message in a list whose condition passes; the lines under
it are your choices. See [[rules/npc-dialogs|NPC dialogs]].

- **NPC:** The Wind Road starts right here, at this gate. Caravans leave for Breezy Hills every morning, if the Rackies let them.
  - **You:** "The Wind Road": Lena sent me.  *(if quest step `2412-02` passes ([[quests/2412-the-wind-road|The Wind Road]]))* → runs quest step `2412-02` ([[quests/2412-the-wind-road|The Wind Road]])
    - **NPC:** Lena's pupil? Then you know your way around Zant. The Wind Road runs east through this gate into Breezy Hills. Keep a Zant Return Scroll in your bag, and good luck out there.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** What's in Breezy Hills?
    - **NPC:** Windy hills, farms and a big lake. The Woopies and Honeybees out there are tougher than the ones by Zant, and the Rackies rob our wagons further on. Go carefully, and come back to Zant to rest and trade.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** What is this camp?
    - **NPC:** The Wind Road Caravan Camp. Every caravan to Breezy Hills and back stops here: we load, mend the wagons and wait for the road to be safe.
      - **You:** Something else.
        - *(back to the start)*
      - **You:** Goodbye.
  - **You:** Goodbye.

After an answer the NPC offers "Something else." (back to the first list) and "Goodbye."

## Changed from iROSE

A new NPC of the Zant city hub redesign, using an existing iROSE model.
