---
kind: quest
id: 105
name: Healing Hands
status: in-game
given_by:
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
monsters:
- '[[monsters/21-pumpkin|Pumpkin]]'
steps: 5
source:
  data: LIST_QUEST.STB row 105; QSD triggers 104-01, 104-02, 105-01, 105-31, 106-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Healing Hands

Karitte said that the epidemic is actually a curse. An antidote needs to be created, so you'll have to collect 10 Pumpkin Seeds for Karitte.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `104-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/104-mysterious-emblem|Mysterious Emblem]]

Then:

- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- the quest becomes [[quests/105-healing-hands|Healing Hands]] (progress kept)
- set episode variable 0 to 3

### `104-02`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- episode variable 0 = 3

Then:

- you get the quest [[quests/105-healing-hands|Healing Hands]]

### `105-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/105-healing-hands|Healing Hands]]
- you carry ≥ 10 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/105-healing-hands|Healing Hands]]
- 10 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]] is taken
- the quest ends (removed from your list)
- then runs step `105-02`

### `105-31`

Happens by killing [[monsters/21-pumpkin|Pumpkin]].

Checks:

- you have [[quests/105-healing-hands|Healing Hands]]
- you carry < 10 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/105-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

### `106-02`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- the quest timer ≤ 0

Then:

- works on [[quests/106-healing-hands|Healing Hands]]
- the quest becomes [[quests/105-healing-hands|Healing Hands]]
