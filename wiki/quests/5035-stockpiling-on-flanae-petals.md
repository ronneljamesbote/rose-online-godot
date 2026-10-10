---
kind: quest
id: 5035
name: Stockpiling on Flanae Petals
status: in-game
given_by:
- '[[npcs/1038-village-chief-gray|Village Chief Gray]]'
steps: 5
source:
  data: LIST_QUEST.STB row 5035; QSD triggers 5035-01, 5035-02, 5035-03, 5035-31, 5035-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling on Flanae Petals

With the endless influx of Visitors, the Adventurer's Plains is suffering from a supply shortage. Collect some Flanae Petals for the tormented Gray and provide him with some relief.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5035-01`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks: none.

Then:

- you get the quest [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]

### `5035-02`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- you carry ≥ 1 × [[items/quest/806-flanae-petal|Flanae Petal]]

Then:

- works on [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- money: 35 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `5035-01`

### `5035-03`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- you carry ≥ 1 × [[items/quest/806-flanae-petal|Flanae Petal]]

Then:

- works on [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- money: 35 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5035-31`

Checks:

- you have [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- a random roll 0–99 lands in 0–75
- you carry < 32 × [[items/quest/806-flanae-petal|Flanae Petal]]
- your Level ≤ 15

Then:

- works on [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- you get 1 × [[items/quest/806-flanae-petal|Flanae Petal]]
- add 1 to quest variable 9

### `5035-32`

Checks:

- you have [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- a random roll 0–99 lands in 0–75
- you carry < 32 × [[items/quest/806-flanae-petal|Flanae Petal]]
- your Level ≤ 15

Then:

- works on [[quests/5035-stockpiling-on-flanae-petals|Stockpiling on Flanae Petals]]
- you get 1 × [[items/quest/806-flanae-petal|Flanae Petal]]
- add 1 to quest variable 9
