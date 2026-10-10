---
kind: quest
id: 139
name: Punwell in Windmill Village
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
npcs:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
steps: 3
source:
  data: LIST_QUEST.STB row 139; QSD triggers 138-01, 138-02, 139-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Punwell in Windmill Village

Harin heard everything about Shroon, and has now asked you to help Punwell in the Windmill Village.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `138-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 38

Then:

- you get the quest [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]

### `139-01`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]

Then:

- works on [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]
- experience: 10000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- set episode variable 0 to 39
