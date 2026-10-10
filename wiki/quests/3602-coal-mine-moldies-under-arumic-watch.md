---
kind: quest
id: 3602
name: Coal Mine Moldies under Arumic Watch
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
monsters:
- '[[monsters/112-coal-mine-moldie|Coal Mine Moldie]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3602; QSD triggers 3602-01, 3602-02, 3602-03, 3602-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Coal Mine Moldies under Arumic Watch

The Arumics have started research on the strange behavior of Moldies in the Coal Mines. Gather 12 Moldie Eyelashes as specimens for study.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3602-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4

Then:

- you get the quest [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]

### `3602-02`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- your Faction = 4
- your Level ≤ 50
- you carry ≥ 12 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]]

Then:

- works on [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- 12 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]] is taken
- add 3 to your UnionPoint4
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3602-03` is tried instead.

### `3602-03`

Checks:

- you have [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- your Faction = 4
- your Level > 50
- you carry ≥ 12 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]]

Then:

- works on [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- 12 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]] is taken
- add 1 to your UnionPoint4
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3602-31`

Happens by killing [[monsters/112-coal-mine-moldie|Coal Mine Moldie]].

Checks:

- you have [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- a random roll 0–99 lands in 0–30
- you carry < 12 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]]

Then:

- works on [[quests/3602-coal-mine-moldies-under-arumic-watch|Coal Mine Moldies under Arumic Watch]]
- you get 1 × [[items/quest/308-moldie-eyelash|Moldie Eyelash]]

If the checks fail, step `3802-31` is tried instead.
