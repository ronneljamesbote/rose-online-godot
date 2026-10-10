---
kind: quest
id: 1982
name: The Prince of Akram
status: in-game
npcs:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
- '[[npcs/1116-soldier-odelo|Soldier Odelo]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1982; QSD triggers 1981-02, 1982-01, 1982-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Prince of Akram

After recovering the Zeppastone, you hear about the runaway Prince of Akram. You should try to talk to Odelo, a Guard in Junon Polis, to learn more.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1981-02`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- you carry = 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

Then:

- works on [[quests/1981-zeppastone-wind-gem|Zeppastone Wind Gem]]
- 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]] is taken
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]] (item count: 10)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 10)
- the quest becomes [[quests/1982-the-prince-of-akram|The Prince of Akram]]
- quest switch 128 on

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

### `1982-02`

Checks:

- quest switch 128 is on
- quest switch 129 is off

Then:

- you get the quest [[quests/1982-the-prince-of-akram|The Prince of Akram]]
