---
kind: quest
id: 3003
name: Necessity of a New Shrine (1)
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
monsters:
- '[[monsters/51-smouly|Smouly]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3003; QSD triggers 3003-01, 3003-02, 3003-03, 3003-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Necessity of a New Shrine (1)

Help the effort to construct a new shrine for the Junon Order by hunting Smoulies and gathering 12 Thick Pulpwood.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3003-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1

Then:

- you get the quest [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]

### `3003-02`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- you carry ≥ 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]
- your Faction = 1
- your Level ≤ 50

Then:

- works on [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]] is taken
- add 4 to your UnionPoint1
- experience, base 100 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3003-03` is tried instead.

### `3003-03`

Checks:

- you have [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- you carry ≥ 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]
- your Faction = 1
- your Level > 50

Then:

- works on [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]] is taken
- add 1 to your UnionPoint1
- experience, base 6000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3003-31`

Happens by killing [[monsters/51-smouly|Smouly]].

Checks:

- you have [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- a random roll 0–99 lands in 0–30
- you carry < 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

Then:

- works on [[quests/3003-necessity-of-a-new-shrine-1|Necessity of a New Shrine (1)]]
- you get 1 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

If the checks fail, step `3605-31` is tried instead.
