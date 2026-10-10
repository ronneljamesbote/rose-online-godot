---
kind: quest
id: 5006
name: Stockpiling Bronze Fragments
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
monsters:
- '[[monsters/71-needle-pomic|Needle Pomic]]'
- '[[monsters/73-red-pomic|Red Pomic]]'
- '[[monsters/74-pomic-soldier|Pomic Soldier]]'
- '[[monsters/75-pomic-fighter|Pomic Fighter]]'
steps: 7
source:
  data: LIST_QUEST.STB row 5006; QSD triggers 5006-01, 5006-02, 5006-31, 5006-32, 5006-33, 5006-34, 5006-35
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Bronze Fragments

Lena has asked you to gather Bronze Fragments to help alleviate the dwindling supply problem in Zant. You can hunt Pomic type monsters to obtain Bronze Fragments.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5006-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- your Level ≥ 10
- your Level ≤ 18

Then:

- you get the quest [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]

### `5006-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you carry ≥ 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- money: 45 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5006-31`

Happens by killing [[monsters/71-needle-pomic|Needle Pomic]].

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- a random roll 0–99 lands in 0–10
- your Level ≤ 18
- you carry < 48 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you get 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]
- add 1 to quest variable 0

### `5006-32`

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- a random roll 0–99 lands in 0–13
- your Level ≤ 18
- you carry < 48 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you get 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]
- add 1 to quest variable 9

### `5006-33`

Happens by killing [[monsters/73-red-pomic|Red Pomic]].

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- a random roll 0–99 lands in 0–17
- your Level ≤ 18
- you carry < 48 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you get 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]
- add 1 to quest variable 9

### `5006-34`

Happens by killing [[monsters/74-pomic-soldier|Pomic Soldier]].

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- a random roll 0–99 lands in 0–21
- your Level ≤ 18
- you carry < 48 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you get 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]
- add 1 to quest variable 9

### `5006-35`

Happens by killing [[monsters/75-pomic-fighter|Pomic Fighter]].

Checks:

- you have [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- a random roll 0–99 lands in 0–25
- your Level ≤ 18
- you carry < 48 × [[items/quest/21-bronze-fragment|Bronze Fragment]]

Then:

- works on [[quests/5006-stockpiling-bronze-fragments|Stockpiling Bronze Fragments]]
- you get 1 × [[items/quest/21-bronze-fragment|Bronze Fragment]]
- add 1 to quest variable 9
