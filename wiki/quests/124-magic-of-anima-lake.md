---
kind: quest
id: 124
name: Magic of Anima Lake
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
npcs:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 3
source:
  data: LIST_QUEST.STB row 124; QSD triggers 123-01, 123-02, 124-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Magic of Anima Lake

You must confront Shroon, possibly the mastermind of this entire plot, who is in the Shrine in Anima Lake which is North of Junon Polis.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `123-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/123-resurrection|Resurrection]]
- you carry = 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- you carry = 1 × [[items/quest/612-small-necklace|Small Necklace]]

Then:

- works on [[quests/123-resurrection|Resurrection]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- 1 × [[items/quest/612-small-necklace|Small Necklace]] is taken
- experience, base 6000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/352-junon-polis-return-scroll|Junon Polis Return Scroll]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/head/314-joker-jester|Joker Jester]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/body/214-islamic-dress|Islamic Dress]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/hands/214-gloves-of-iguje|Gloves of Iguje]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/feet/214-land-walkers|Land Walkers]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/124-magic-of-anima-lake|Magic of Anima Lake]] (progress kept)
- set episode variable 0 to 20

### `123-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 20

Then:

- you get the quest [[quests/124-magic-of-anima-lake|Magic of Anima Lake]]

### `124-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/124-magic-of-anima-lake|Magic of Anima Lake]]

Then:

- works on [[quests/124-magic-of-anima-lake|Magic of Anima Lake]]
- experience, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/125-falsehoods|Falsehoods]]
- set episode variable 0 to 24
