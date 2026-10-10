---
kind: quest
id: 140
name: The Zeppastone Wind Gem
status: in-game
given_by:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
steps: 4
source:
  data: LIST_QUEST.STB row 140; QSD triggers 139-01, 139-02, 140-01, 140-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Zeppastone Wind Gem

Punwell says that the Zeppastone Wind Gem is necessary for the prosperity of the Windmill Village. All you know is that the Zeppastone should be in a dark, enclosed place.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `139-01`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]

Then:

- works on [[quests/139-punwell-in-windmill-village|Punwell in Windmill Village]]
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- set episode variable 0 to 39

### `139-02`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- episode variable 0 = 39

Then:

- you get the quest [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]

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

### `140-31`

Checks:

- you have [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- a random roll 0–99 lands in 0–70
- you carry < 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]

Then:

- works on [[quests/140-the-zeppastone-wind-gem|The Zeppastone Wind Gem]]
- you get 1 × [[items/quest/509-zeppastone-wind-gem|Zeppastone Wind Gem]]
