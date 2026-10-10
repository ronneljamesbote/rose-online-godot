---
kind: quest
id: 1983
name: The Search for the Prince
status: in-game
npcs:
- '[[npcs/1116-soldier-odelo|Soldier Odelo]]'
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1983; QSD triggers 1982-01, 1983-01, 1983-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Search for the Prince

Odelo said that the only trace of the Prince is a Golden Dagger kept by Gallahad in Kenji's Beach.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1982-01`

Happens by talking to [[npcs/1116-soldier-odelo|Soldier Odelo]].

Checks:

- you have [[quests/1982-the-prince-of-akram|The Prince of Akram]]
- quest switch 128 is on
- quest switch 129 is off

Then:

- works on [[quests/1982-the-prince-of-akram|The Prince of Akram]]
- experience: 1000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
- quest switch 129 on

### `1983-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
- quest switch 129 is on
- quest switch 130 is off

Then:

- works on [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
- experience: 1000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 130 on

### `1983-02`

Checks:

- quest switch 129 is on
- quest switch 130 is off

Then:

- you get the quest [[quests/1983-the-search-for-the-prince|The Search for the Prince]]
