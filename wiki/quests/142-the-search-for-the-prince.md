---
kind: quest
id: 142
name: The Search for the Prince
status: in-game
given_by:
- '[[npcs/1116-soldier-odelo|Soldier Odelo]]'
npcs:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 3
source:
  data: LIST_QUEST.STB row 142; QSD triggers 141-01, 141-02, 142-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Search for the Prince

Odelo said that the only trace of the Prince is a Golden Dagger kept by Gallahad in Kenji's Beach.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `141-01`

Happens by talking to [[npcs/1116-soldier-odelo|Soldier Odelo]].

Checks:

- you have [[quests/141-the-prince-of-akram|The Prince of Akram]]

Then:

- works on [[quests/141-the-prince-of-akram|The Prince of Akram]]
- you get [[items/consumable/11-vital-water-m|Vital Water (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/30-spiritual-water-m|Spiritual Water (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/142-the-search-for-the-prince|The Search for the Prince]]
- set episode variable 0 to 41

### `141-02`

Happens by talking to [[npcs/1116-soldier-odelo|Soldier Odelo]].

Checks:

- episode variable 0 = 41

Then:

- you get the quest [[quests/142-the-search-for-the-prince|The Search for the Prince]]

### `142-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/142-the-search-for-the-prince|The Search for the Prince]]

Then:

- works on [[quests/142-the-search-for-the-prince|The Search for the Prince]]
- experience, base 20000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- set episode variable 0 to 42
