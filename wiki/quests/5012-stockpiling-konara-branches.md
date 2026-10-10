---
kind: quest
id: 5012
name: Stockpiling Konara Branches
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
monsters:
- '[[monsters/52-elder-smouly|Elder Smouly]]'
- '[[monsters/53-old-smouly|Old Smouly]]'
- '[[monsters/181-small-clown|Small Clown]]'
- '[[monsters/182-clown|Clown]]'
- '[[monsters/183-fighter-clown|Fighter Clown]]'
- '[[monsters/185-ranger-clown|Ranger Clown]]'
- '[[monsters/186-hunter-clown|Hunter Clown]]'
steps: 10
source:
  data: LIST_QUEST.STB row 5012; QSD triggers 5012-01, 5012-02, 5012-31, 5012-32, 5012-33, 5012-34, 5012-35, 5012-36, 5012-37, 5012-38
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Konara Branches

More material shortages have occurred in Junon Polis as its population has risen. If you bring Konara Branches to Eva, she will surely be pleased. You can find Konara Branches by hunting Clown type monsters.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5012-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- your Level ≥ 41
- your Level ≤ 50

Then:

- you get the quest [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]

### `5012-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you carry ≥ 1 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- Zuly, base 89 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5012-31`

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–9
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-32`

Happens by killing [[monsters/52-elder-smouly|Elder Smouly]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–14
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-33`

Happens by killing [[monsters/53-old-smouly|Old Smouly]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–19
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-34`

Happens by killing [[monsters/181-small-clown|Small Clown]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–19
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-35`

Happens by killing [[monsters/182-clown|Clown]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–21
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-36`

Happens by killing [[monsters/183-fighter-clown|Fighter Clown]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–23
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-37`

Happens by killing [[monsters/185-ranger-clown|Ranger Clown]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–25
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9

### `5012-38`

Happens by killing [[monsters/186-hunter-clown|Hunter Clown]].

Checks:

- you have [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- a random roll 0–99 lands in 0–25
- your Level ≥ 41
- your Level ≤ 50
- you carry < 84 × [[items/quest/27-konara-branch|Konara Branch]]

Then:

- works on [[quests/5012-stockpiling-konara-branches|Stockpiling Konara Branches]]
- you get 1 × [[items/quest/27-konara-branch|Konara Branch]]
- add 1 to quest variable 9
