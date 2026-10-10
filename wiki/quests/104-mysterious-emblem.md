---
kind: quest
id: 104
name: Mysterious Emblem
status: in-game
given_by:
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
npcs:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
steps: 3
source:
  data: LIST_QUEST.STB row 104; QSD triggers 103-01, 103-02, 104-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Mysterious Emblem

Just like Seyon said, those suspicious strangers were also seen by Myad. Myad told you to meet Karitte at the Tower of Luxem Valley.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `103-01`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]], talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/103-mysterious-emblem|Mysterious Emblem]]
- you carry = 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/103-mysterious-emblem|Mysterious Emblem]]
- experience: 200 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/104-mysterious-emblem|Mysterious Emblem]] (progress kept)
- set episode variable 0 to 2

### `103-02`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- episode variable 0 = 2

Then:

- you get the quest [[quests/104-mysterious-emblem|Mysterious Emblem]]
- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

### `104-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/104-mysterious-emblem|Mysterious Emblem]]

Then:

- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- the quest becomes [[quests/105-healing-hands|Healing Hands]] (progress kept)
- set episode variable 0 to 3
