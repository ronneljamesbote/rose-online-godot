---
kind: quest
id: 3002
name: Gorthein's Hidden Talent
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3002; QSD triggers 3002-01, 3002-02, 3002-03, 3002-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Gorthein's Hidden Talent

Gorthein has always wished to make his own healing potion, but he's always failed. Now, he's running out of materials to continue his experiments. Help Gorthein by supplying him with 10 Porkie Spines.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3002-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1

Then:

- you get the quest [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]

### `3002-02`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- you carry ≥ 10 × [[items/quest/301-porkie-spine|Porkie Spine]]
- your Faction = 1
- your Level ≤ 50

Then:

- works on [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- 10 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- experience, base 100 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- add 3 to your UnionPoint1
- the quest ends (removed from your list)

If the checks fail, step `3002-03` is tried instead.

### `3002-03`

Checks:

- you have [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- you carry ≥ 10 × [[items/quest/301-porkie-spine|Porkie Spine]]
- your Faction = 1
- your Level > 50

Then:

- works on [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- 10 × [[items/quest/301-porkie-spine|Porkie Spine]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to your UnionPoint1
- the quest ends (removed from your list)

### `3002-31`

Checks:

- you have [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- a random roll 0–99 lands in 0–40
- you carry < 10 × [[items/quest/301-porkie-spine|Porkie Spine]]

Then:

- works on [[quests/3002-gorthein-s-hidden-talent|Gorthein's Hidden Talent]]
- you get 1 × [[items/quest/301-porkie-spine|Porkie Spine]]

If the checks fail, step `3401-31` is tried instead.
