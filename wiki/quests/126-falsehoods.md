---
kind: quest
id: 126
name: Falsehoods
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
npcs:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
steps: 3
source:
  data: LIST_QUEST.STB row 126; QSD triggers 125-01, 125-02, 126-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Falsehoods

Harin says that Shroon is still continuing to research dark magic. She asks you to deliver a letter to Bellia, who might be able to deal with Shroon.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `125-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/125-falsehoods|Falsehoods]]

Then:

- works on [[quests/125-falsehoods|Falsehoods]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]
- experience, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/126-falsehoods|Falsehoods]] (progress kept)
- set episode variable 0 to 25

### `125-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 25

Then:

- you get the quest [[quests/126-falsehoods|Falsehoods]]
- works on [[quests/126-falsehoods|Falsehoods]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]

### `126-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/126-falsehoods|Falsehoods]]

Then:

- works on [[quests/126-falsehoods|Falsehoods]]
- 1 × [[items/quest/615-small-letter|Small Letter]] is taken
- Zuly, base 5000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/127-the-owl-eye|The Owl Eye]] (progress kept)
- set episode variable 0 to 26
