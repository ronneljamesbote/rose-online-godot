---
kind: quest
id: 5008
name: Stockpiling Silver Fragments
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
monsters:
- '[[monsters/84-turtle|Turtle]]'
- '[[monsters/85-turtle-guard|Turtle Guard]]'
- '[[monsters/91-honey-rackie|Honey Rackie]]'
- '[[monsters/92-rackie-hooligan|Rackie Hooligan]]'
- '[[monsters/96-hunter-rackie|Hunter Rackie]]'
steps: 8
source:
  data: LIST_QUEST.STB row 5008; QSD triggers 5008-01, 5008-02, 5008-31, 5008-32, 5008-33, 5008-34, 5008-35, 5008-36
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Silver Fragments

Lena has asked you to gather Silver Fragments to help alleviate the dwindling supply problem in Zant. You can hunt Rackie and Turtle type monsters to obtain Silver Fragments.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5008-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- your Level ≥ 23
- your Level ≤ 30

Then:

- you get the quest [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]

### `5008-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you carry ≥ 1 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- money: 63 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5008-31`

Happens by killing [[monsters/91-honey-rackie|Honey Rackie]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–10
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9

### `5008-32`

Happens by killing [[monsters/92-rackie-hooligan|Rackie Hooligan]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–13
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9

### `5008-33`

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–15
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9

### `5008-34`

Happens by killing [[monsters/96-hunter-rackie|Hunter Rackie]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–17
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9

### `5008-35`

Happens by killing [[monsters/84-turtle|Turtle]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–15
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9

### `5008-36`

Happens by killing [[monsters/85-turtle-guard|Turtle Guard]].

Checks:

- you have [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- a random roll 0–99 lands in 0–20
- your Level ≤ 30
- you carry < 59 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/5008-stockpiling-silver-fragments|Stockpiling Silver Fragments]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]
- add 1 to quest variable 9
