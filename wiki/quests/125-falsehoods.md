---
kind: quest
id: 125
name: Falsehoods
status: in-game
given_by:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
npcs:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 125; QSD triggers 124-01, 124-02, 125-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Falsehoods

Shroon has denied any knowledge about Alex. You better let Harin know about this.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `124-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/124-magic-of-anima-lake|Magic of Anima Lake]]

Then:

- works on [[quests/124-magic-of-anima-lake|Magic of Anima Lake]]
- experience: 2000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/125-falsehoods|Falsehoods]]
- set episode variable 0 to 24

### `124-02`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- episode variable 0 = 24

Then:

- you get the quest [[quests/125-falsehoods|Falsehoods]]

### `125-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/125-falsehoods|Falsehoods]]

Then:

- works on [[quests/125-falsehoods|Falsehoods]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]
- experience: 2000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/126-falsehoods|Falsehoods]] (progress kept)
- set episode variable 0 to 25
