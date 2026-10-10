---
kind: rule
id: bank
name: Bank
status: changed-from-irose
pages: 4
slots_per_page: 30
total_slots: 120
premium_slots: 0
fee_zuly: 0
max_stack: 999
bank_range_m: 15
stores_zuly: false
storage_npcs:
  - { npc: "[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]", zone: "[[zones/1-canyon-city-of-zant|Canyon City of Zant]]" }
  - { npc: "[[npcs/1089-manager-of-ferrell-arothel|Manager of Ferrell Arothel]]", zone: "[[zones/2-city-of-junon-polis|City of Junon Polis]]" }
  - { npc: "[[npcs/1161-clan-base-camp-manager-kushard|Clan Base Camp Manager Kushard]]", zone: "[[zones/15-zone-15|Zone 15]]" }
  - { npc: "[[npcs/1180-ferrell-guild-banker-andre|Ferrell Guild Banker Andre]]", zone: "[[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]" }
  - { npc: "[[npcs/1222-storage-keeper-dustin-leta|Storage Keeper Dustin Leta]]", zone: "[[zones/61-refuge-xita|Refuge Xita]]" }
source:
  code:
    - module/src/bank.rs (bank_deposit, bank_withdraw, bank_move, try_add, BANK_SLOTS, BANK_RANGE_CM)
    - godot/scripts/bank_window.gd
    - godot/scripts/inventory_window.gd (right-click to store)
    - godot/rust/src/conversation.rs (GF_openBank)
  data: "NPC dialog files (the storage answer)"
---
# Bank

Your storage keeps items you don't want to carry. It belongs to your account, so it is the
same wherever you open it.

## Opening it

Talk to a storage NPC and pick the storage answer in its dialog (on NPC pages it shows as
`→ opens storage`, see [[rules/npc-dialogs|NPC dialogs]]). These NPCs offer it:

| NPC | Zone |
| --- | --- |
| [[npcs/1004-ferrell-guild-staff-crow\|Ferrell Guild Staff Crow]] | [[zones/1-canyon-city-of-zant\|Canyon City of Zant]] |
| [[npcs/1089-manager-of-ferrell-arothel\|Manager of Ferrell Arothel]] | [[zones/2-city-of-junon-polis\|City of Junon Polis]] |
| [[npcs/1161-clan-base-camp-manager-kushard\|Clan Base Camp Manager Kushard]] | [[zones/15-zone-15\|Zone 15]] |
| [[npcs/1180-ferrell-guild-banker-andre\|Ferrell Guild Banker Andre]] | [[zones/51-magic-city-of-the-eucar\|Magic City of the Eucar]] |
| [[npcs/1222-storage-keeper-dustin-leta\|Storage Keeper Dustin Leta]] | [[zones/61-refuge-xita\|Refuge Xita]] |

The storage window opens next to your inventory. You must stay within **15 m** of the
NPC: the window closes when you walk further away, and the server refuses to move items
from further away. Esc or Close shuts it.

## Slots

- **4 pages of 30 slots**, 120 in all, for everyone.
- Stackable items (potions, materials, gems) join a matching stack, up to 999 in a stack;
  what doesn't fit goes into the first empty slot. Equipment takes one slot each.
- When there is no room the item stays in your bag ("your bank is full").

## Moving items

- **Store**: while storage is open, right-click a bag item to store the whole stack,
  Shift + right-click to store one, or drag it onto the storage window.
- **Take back**: right-click a storage item to take the whole stack, Shift + right-click
  to take one, or drag it to your bag. A full bag refuses ("your inventory is full").
- Drag inside the storage window to move or swap items, or to join two stacks.

Storing and taking items is free. Zuly can't be stored.

## Changed from iROSE

- In iROSE the last 30 slots (page 4) were for premium accounts only; here everyone gets
  all four pages.

> Open question: iROSE charged a small Zuly fee for storing items and let you keep Zuly in storage. Our storage is free and holds no Zuly. Is that what we want?
