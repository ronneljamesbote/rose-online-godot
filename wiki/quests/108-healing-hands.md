---
kind: quest
id: 108
name: Healing Hands
status: in-game
given_by:
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
npcs:
- '[[npcs/1014-guide-lena|Guide Lena]]'
steps: 4
source:
  data: LIST_QUEST.STB row 108; QSD triggers 107-06, 107-07, 107-09, 108-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Healing Hands

Myad said that Zant Village seems to be cursed too. He's asked you to give the Antidote Recipe to Lena.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `107-06`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- episode variable 0 = 4

Then:

- you get the quest [[quests/108-healing-hands|Healing Hands]]
- works on [[quests/108-healing-hands|Healing Hands]]
- experience: 200 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]

### `107-07`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- episode variable 0 = 5

Then:

- you get the quest [[quests/108-healing-hands|Healing Hands]]
- works on [[quests/108-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]

### `107-09`

Checks: none.

Then:

- you get the quest [[quests/108-healing-hands|Healing Hands]]
- works on [[quests/108-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]

### `108-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/108-healing-hands|Healing Hands]]

Then:

- works on [[quests/108-healing-hands|Healing Hands]]
- experience: 300 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/109-sacrificed-soul|Sacrificed Soul]]
- set episode variable 0 to 6
