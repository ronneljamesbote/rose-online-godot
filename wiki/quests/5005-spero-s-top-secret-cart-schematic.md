---
kind: quest
id: 5005
name: Spero's Top Secret Cart Schematic
status: in-game
given_by:
- '[[npcs/1011-eccentric-inventor-spero|Eccentric Inventor Spero]]'
npcs:
- '[[npcs/1061-ferrell-guild-staff-belz|Ferrell Guild Staff Belz]]'
steps: 2
source:
  data: LIST_QUEST.STB row 5005; QSD triggers 5005-01, 5005-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Spero's Top Secret Cart Schematic

Spero has asked you to deliver his Top Secret Cart Schematic to Belz, who is in the Windmill Village in the Breezy Hills.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5005-01`

Happens by talking to [[npcs/1011-eccentric-inventor-spero|Eccentric Inventor Spero]].

Checks:

- your Level ≤ 20
- quest switch 33 is off

Then:

- you get the quest [[quests/5005-spero-s-top-secret-cart-schematic|Spero's Top Secret Cart Schematic]]
- works on [[quests/5005-spero-s-top-secret-cart-schematic|Spero's Top Secret Cart Schematic]]
- you get 1 × [[items/quest/505-top-secret-cart-schematic|Top Secret Cart Schematic]]

### `5005-02`

Happens by talking to [[npcs/1061-ferrell-guild-staff-belz|Ferrell Guild Staff Belz]].

Checks:

- you have [[quests/5005-spero-s-top-secret-cart-schematic|Spero's Top Secret Cart Schematic]]
- you carry ≥ 1 × [[items/quest/505-top-secret-cart-schematic|Top Secret Cart Schematic]]

Then:

- works on [[quests/5005-spero-s-top-secret-cart-schematic|Spero's Top Secret Cart Schematic]]
- 1 × [[items/quest/505-top-secret-cart-schematic|Top Secret Cart Schematic]] is taken
- quest switch 33 on
- experience: 220 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
