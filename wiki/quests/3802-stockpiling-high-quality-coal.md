---
kind: quest
id: 3802
name: Stockpiling High Quality Coal
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3802; QSD triggers 3802-01, 3802-02, 3802-03, 3802-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stockpiling High Quality Coal

It's expected that the price for coal will rise since the Moldies have stopped mining for coal. You should chase after those Moldies and try to get 10 High Quality Coal as quickly as you can!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3802-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5

Then:

- you get the quest [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]

### `3802-02`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- your Faction = 5
- your Level ≤ 50
- you carry ≥ 10 × [[items/quest/311-high-quality-coal|High Quality Coal]]

Then:

- works on [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- 10 × [[items/quest/311-high-quality-coal|High Quality Coal]] is taken
- add 3 to your UnionPoint5
- experience, base 100 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3802-03` is tried instead.

### `3802-03`

Checks:

- you have [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- your Faction = 5
- your Level > 50
- you carry ≥ 10 × [[items/quest/311-high-quality-coal|High Quality Coal]]

Then:

- works on [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- 10 × [[items/quest/311-high-quality-coal|High Quality Coal]] is taken
- add 1 to your UnionPoint5
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3802-31`

Checks:

- you have [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- a random roll 0–99 lands in 0–30
- you carry < 10 × [[items/quest/311-high-quality-coal|High Quality Coal]]

Then:

- works on [[quests/3802-stockpiling-high-quality-coal|Stockpiling High Quality Coal]]
- you get 1 × [[items/quest/311-high-quality-coal|High Quality Coal]]

If the checks fail, step `5017-33` is tried instead.
