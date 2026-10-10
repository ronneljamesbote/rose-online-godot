---
kind: quest
id: 5011
name: Stockpiling Maple Branches
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
steps: 7
source:
  data: LIST_QUEST.STB row 5011; QSD triggers 5011-01, 5011-02, 5011-31, 5011-32, 5011-33, 5011-34, 5011-35
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Maple Branches

More material shortages have occurred in Junon Polis as its population has risen. If you bring Maple Branches to Eva, she will surely be pleased. You can find Maple Branches by hunting Porkie type monsters.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5011-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- your Level ≥ 31
- your Level ≤ 40

Then:

- you get the quest [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]

### `5011-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you carry ≥ 1 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- money: 75 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5011-31`

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- a random roll 0–99 lands in 0–19
- your Level ≥ 31
- your Level ≤ 40
- you carry < 97 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you get 1 × [[items/quest/26-maple-branch|Maple Branch]]
- add 1 to quest variable 9

If the checks fail, step `5017-31` is tried instead.

### `5011-32`

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- a random roll 0–99 lands in 0–22
- your Level ≥ 31
- your Level ≤ 40
- you carry < 97 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you get 1 × [[items/quest/26-maple-branch|Maple Branch]]
- add 1 to quest variable 9

### `5011-33`

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- a random roll 0–99 lands in 0–25
- your Level ≥ 31
- your Level ≤ 40
- you carry < 97 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you get 1 × [[items/quest/26-maple-branch|Maple Branch]]
- add 1 to quest variable 9

### `5011-34`

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- a random roll 0–99 lands in 0–9
- your Level ≥ 31
- your Level ≤ 40
- you carry < 97 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you get 1 × [[items/quest/26-maple-branch|Maple Branch]]
- add 1 to quest variable 9

### `5011-35`

Checks:

- you have [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- a random roll 0–99 lands in 0–14
- your Level ≥ 31
- your Level ≤ 40
- you carry < 97 × [[items/quest/26-maple-branch|Maple Branch]]

Then:

- works on [[quests/5011-stockpiling-maple-branches|Stockpiling Maple Branches]]
- you get 1 × [[items/quest/26-maple-branch|Maple Branch]]
- add 1 to quest variable 9
