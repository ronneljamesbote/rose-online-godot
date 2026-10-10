---
kind: quest
id: 5013
name: Stockpiling Oak Branches
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
monsters:
- '[[monsters/154-doonga-fighter|Doonga Fighter]]'
- '[[monsters/156-doonga-hunter|Doonga Hunter]]'
steps: 6
source:
  data: LIST_QUEST.STB row 5013; QSD triggers 5013-01, 5013-02, 5013-31, 5013-32, 5013-33, 5013-34
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Oak Branches

More material shortages have occurred in Junon Polis as its population has risen. If you bring Oak Branches to Eva, she will surely be pleased. You can find Oak Branches by hunting Doonga type monsters.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5013-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- your Level ≥ 51
- your Level ≤ 60

Then:

- you get the quest [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]

### `5013-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- you carry ≥ 1 × [[items/quest/28-oak-branch|Oak Branch]]

Then:

- works on [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- money: 116 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5013-31`

Checks:

- you have [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- a random roll 0–99 lands in 0–23
- your Level ≥ 51
- your Level ≤ 60
- you carry < 91 × [[items/quest/28-oak-branch|Oak Branch]]

Then:

- works on [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- you get 1 × [[items/quest/28-oak-branch|Oak Branch]]
- add 1 to quest variable 9

### `5013-32`

Checks:

- you have [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- a random roll 0–99 lands in 0–23
- your Level ≥ 51
- your Level ≤ 60
- you carry < 91 × [[items/quest/28-oak-branch|Oak Branch]]

Then:

- works on [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- you get 1 × [[items/quest/28-oak-branch|Oak Branch]]
- add 1 to quest variable 9

### `5013-33`

Happens by killing [[monsters/154-doonga-fighter|Doonga Fighter]].

Checks:

- you have [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- a random roll 0–99 lands in 0–23
- your Level ≥ 51
- your Level ≤ 60
- you carry < 91 × [[items/quest/28-oak-branch|Oak Branch]]

Then:

- works on [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- you get 1 × [[items/quest/28-oak-branch|Oak Branch]]
- add 1 to quest variable 9

### `5013-34`

Happens by killing [[monsters/156-doonga-hunter|Doonga Hunter]].

Checks:

- you have [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- a random roll 0–99 lands in 0–23
- your Level ≥ 51
- your Level ≤ 60
- you carry < 91 × [[items/quest/28-oak-branch|Oak Branch]]

Then:

- works on [[quests/5013-stockpiling-oak-branches|Stockpiling Oak Branches]]
- you get 1 × [[items/quest/28-oak-branch|Oak Branch]]
- add 1 to quest variable 9
