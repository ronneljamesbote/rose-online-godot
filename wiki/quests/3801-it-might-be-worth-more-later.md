---
kind: quest
id: 3801
name: It Might be Worth More Later
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3801; QSD triggers 3801-01, 3801-02, 3801-03, 3801-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# It Might be Worth More Later

Since demand for Porkie Spines is on the rise, it's time to get them while they're still hot. Go hunt some Porkies and get 12 Porkie Spines.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3801-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5

Then:

- you get the quest [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]

### `3801-02`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- your Faction = 5
- your Level ≤ 50
- you carry ≥ 12 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- 12 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- add 1 to your UnionPoint5
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3801-03` is tried instead.

### `3801-03`

Checks:

- you have [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- your Faction = 5
- your Level > 50
- you carry ≥ 12 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- 12 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- add 1 to your UnionPoint5
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3801-31`

Checks:

- you have [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- a random roll 0–99 lands in 0–40
- you carry < 12 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3801-it-might-be-worth-more-later|It Might be Worth More Later]]
- you get 1 × [[items/quest/301-porkie-spine|Porkie Spine]]

If the checks fail, step `5011-34` is tried instead.
