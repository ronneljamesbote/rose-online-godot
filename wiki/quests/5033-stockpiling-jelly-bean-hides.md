---
kind: quest
id: 5033
name: Stockpiling Jelly Bean Hides
status: in-game
given_by:
- '[[npcs/1038-village-chief-gray|Village Chief Gray]]'
monsters:
- '[[monsters/1-mini-jelly-bean|Mini-Jelly Bean]]'
- '[[monsters/3-jellynut|JellyNut]]'
steps: 6
source:
  data: LIST_QUEST.STB row 5033; QSD triggers 5033-01, 5033-02, 5033-03, 5033-31, 5033-32, 5033-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Jelly Bean Hides

With the endless influx of Visitors, the Adventurer's Plains is suffering from a supply shortage. Collect some Jelly Bean Hides for the overtaxed Gray and provide him with some relief.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5033-01`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks: none.

Then:

- you get the quest [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]

### `5033-02`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- you carry ≥ 1 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]

Then:

- works on [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- money: 20 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `5033-01`

### `5033-03`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- you carry ≥ 1 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]

Then:

- works on [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- money: 20 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5033-31`

Checks:

- you have [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- a random roll 0–99 lands in 0–75
- you carry < 26 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]
- your Level ≤ 15

Then:

- works on [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- you get 1 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]
- add 1 to quest variable 9

### `5033-32`

Happens by killing [[monsters/1-mini-jelly-bean|Mini-Jelly Bean]].

Checks:

- you have [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- a random roll 0–99 lands in 0–75
- you carry < 26 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]
- your Level ≤ 15

Then:

- works on [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- you get 1 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]
- add 1 to quest variable 9

If the checks fail, step `5051-33` is tried instead.

### `5033-33`

Happens by killing [[monsters/3-jellynut|JellyNut]].

Checks:

- you have [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- a random roll 0–99 lands in 0–75
- your Level ≤ 15

Then:

- works on [[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]
- you get 1 × [[items/quest/804-jelly-bean-hide|Jelly Bean Hide]]
- add 1 to quest variable 9
