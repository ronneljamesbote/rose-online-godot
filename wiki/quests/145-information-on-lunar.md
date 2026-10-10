---
kind: quest
id: 145
name: Information on Lunar
status: in-game
given_by:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
- '[[npcs/1117-soldier-winters|Soldier Winters]]'
steps: 3
source:
  data: LIST_QUEST.STB row 145; QSD triggers 144-01, 144-02, 145-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Information on Lunar

Lutis started babbling about Luna, but maybe you should visit that planet someday. Now you need to speak to Winters, a Guard in Junon Polis, to continue your search for information on the Golden Dagger.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `144-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]

Then:

- works on [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- experience: 20000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/material/153-blue-hearts|Blue Hearts]] (item count: 1)
- the quest becomes [[quests/145-information-on-lunar|Information on Lunar]] (progress kept)
- set episode variable 0 to 44

### `144-02`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- episode variable 0 = 44

Then:

- you get the quest [[quests/145-information-on-lunar|Information on Lunar]]
- works on [[quests/145-information-on-lunar|Information on Lunar]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `145-01`

Happens by talking to [[npcs/1117-soldier-winters|Soldier Winters]].

Checks:

- you have [[quests/145-information-on-lunar|Information on Lunar]]

Then:

- works on [[quests/145-information-on-lunar|Information on Lunar]]
- experience: 20000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- set episode variable 0 to 45
