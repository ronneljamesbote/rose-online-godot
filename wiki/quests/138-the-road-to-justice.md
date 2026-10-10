---
kind: quest
id: 138
name: The Road to Justice
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
npcs:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 138; QSD triggers 137-01, 137-02, 138-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Road to Justice

Eva will be using the Owl Eye to watch Shroon from now on. You should return to Harin so you can tell her everything that happened.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `137-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/137-the-road-to-justice|The Road to Justice]]

Then:

- works on [[quests/137-the-road-to-justice|The Road to Justice]]
- experience: 10000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/138-the-road-to-justice|The Road to Justice]]
- set episode variable 0 to 37

### `137-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 37

Then:

- you get the quest [[quests/138-the-road-to-justice|The Road to Justice]]

### `138-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/138-the-road-to-justice|The Road to Justice]]

Then:

- works on [[quests/138-the-road-to-justice|The Road to Justice]]
- money: 20000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/material/151-black-hearts|Black Hearts]] (item count: 1)
- you get [[items/material/152-green-hearts|Green Hearts]] (item count: 1)
- the quest becomes [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]
- set episode variable 0 to 38
