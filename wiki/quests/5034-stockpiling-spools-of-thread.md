---
kind: quest
id: 5034
name: Stockpiling Spools of Thread
status: in-game
given_by:
- '[[npcs/1038-village-chief-gray|Village Chief Gray]]'
steps: 5
source:
  data: LIST_QUEST.STB row 5034; QSD triggers 5034-01, 5034-02, 5034-03, 5034-31, 5034-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling Spools of Thread

With the endless influx of Visitors, the Adventurer's Plains is suffering from a supply shortage. Collect some Spools of Thread from Choropies for the worrisome Gray and provide him with some relief.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5034-01`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks: none.

Then:

- you get the quest [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]

### `5034-02`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- you carry ≥ 1 × [[items/quest/805-spool-of-thread|Spool of Thread]]

Then:

- works on [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- money: 27 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `5034-01`

### `5034-03`

Happens by talking to [[npcs/1038-village-chief-gray|Village Chief Gray]].

Checks:

- you have [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- you carry ≥ 1 × [[items/quest/805-spool-of-thread|Spool of Thread]]

Then:

- works on [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- money: 27 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5034-31`

Checks:

- you have [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- a random roll 0–99 lands in 0–75
- you carry < 29 × [[items/quest/805-spool-of-thread|Spool of Thread]]
- your Level ≤ 15

Then:

- works on [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- you get 1 × [[items/quest/805-spool-of-thread|Spool of Thread]]
- add 1 to quest variable 9

### `5034-32`

Checks:

- you have [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- a random roll 0–99 lands in 0–75
- you carry < 29 × [[items/quest/805-spool-of-thread|Spool of Thread]]
- your Level ≤ 15

Then:

- works on [[quests/5034-stockpiling-spools-of-thread|Stockpiling Spools of Thread]]
- you get 1 × [[items/quest/805-spool-of-thread|Spool of Thread]]
- add 1 to quest variable 9
