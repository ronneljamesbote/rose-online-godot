---
kind: quest
id: 3601
name: Unexpectedly Useful Porkie Spines
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3601; QSD triggers 3601-01, 3601-02, 3601-03, 3601-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Unexpectedly Useful Porkie Spines

Porkie Spines are often useful as a magic ingredient. So go out and beat some Porkie Hooligans and bring back 10 Porkie Spines.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3601-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4

Then:

- you get the quest [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]

### `3601-02`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- your Faction = 4
- your Level ≤ 50
- you carry ≥ 10 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- 10 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- add 1 to your UnionPoint4
- experience, base 80 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3601-03` is tried instead.

### `3601-03`

Checks:

- you have [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- your Faction = 4
- your Level > 50
- you carry ≥ 10 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- 10 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- add 1 to your UnionPoint4
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3601-31`

Checks:

- you have [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- a random roll 0–99 lands in 0–40
- you carry < 10 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3601-unexpectedly-useful-porkie-spines|Unexpectedly Useful Porkie Spines]]
- you get 1 × [[items/quest/301-porkie-spine|Porkie Spine]]

If the checks fail, step `5011-35` is tried instead.
