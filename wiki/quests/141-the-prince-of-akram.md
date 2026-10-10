---
kind: quest
id: 141
name: The Prince of Akram
status: in-game
given_by:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
npcs:
- '[[npcs/1116-soldier-odelo|Soldier Odelo]]'
steps: 3
source:
  data: LIST_QUEST.STB row 141; QSD triggers 140-01, 140-02, 141-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Prince of Akram

After recovering the Zeppastone, you hear about the runaway Prince of Akram. You should try to talk to Odelo, a Guard in Junon Polis, to learn more.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `140-01`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- you carry = 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

Then:

- works on [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]] is taken
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/141-the-prince-of-akram|The Prince of Akram]] (progress kept)
- set episode variable 0 to 40

### `140-02`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- episode variable 0 = 40

Then:

- you get the quest [[quests/141-the-prince-of-akram|The Prince of Akram]]

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
