---
kind: quest
id: 5007
name: Stockpiling Iron Fragments
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
monsters:
- '[[monsters/44-old-flanae|Old Flanae]]'
- '[[monsters/45-big-flanae|Big Flanae]]'
steps: 7
source:
  data: LIST_QUEST.STB row 5007; QSD triggers 5007-01, 5007-02, 5007-31, 5007-32, 5007-33, 5007-34, 5007-35
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Iron Fragments

Lena has asked you to gather Iron Fragments to help alleviate the dwindling supply problem in Zant. You can hunt Red Flanae, Old Flanae, Big Flanae and Beetles to obtain Iron Fragments.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5007-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- your Level ≥ 16
- your Level ≤ 24

Then:

- you get the quest [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]

### `5007-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you carry ≥ 1 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- money: 54 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5007-31`

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- a random roll 0–99 lands in 0–10
- your Level ≤ 24

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]
- add 1 to quest variable 9

### `5007-32`

Happens by killing [[monsters/44-old-flanae|Old Flanae]].

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- a random roll 0–99 lands in 0–13
- your Level ≤ 24
- you carry < 53 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]
- add 1 to quest variable 9

### `5007-33`

Happens by killing [[monsters/45-big-flanae|Big Flanae]].

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- a random roll 0–99 lands in 0–17
- your Level ≤ 24
- you carry < 53 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]
- add 1 to quest variable 9

### `5007-34`

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- a random roll 0–99 lands in 0–21
- your Level ≤ 24
- you carry < 53 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]
- add 1 to quest variable 9

### `5007-35`

Checks:

- you have [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- a random roll 0–99 lands in 0–25
- your Level ≤ 24
- you carry < 53 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/5007-stockpiling-iron-fragments|Stockpiling Iron Fragments]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]
- add 1 to quest variable 9
