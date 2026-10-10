---
kind: quest
id: 3005
name: Necessity of a New Shrine (2)
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
monsters:
- '[[monsters/136-grunter-warrior|Grunter Warrior]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3005; QSD triggers 3005-01, 3005-02, 3005-03, 3005-04, 3005-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Necessity of a New Shrine (2)

Help the effort to construct a new shrine for the Junon Order by hunting Grunter Warriors and gathering 18 Iron Fragments.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3005-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC's variable 0 = 7
- the NPC's variable 6 < 10

Then:

- you get the quest [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- add 1 to the NPC's variable 6
- then runs step `3005-02`

### `3005-02`

Checks:

- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC's variable 0 = 7
- the NPC's variable 6 ≥ 10

Then:

- set the NPC's variable 0 to 8
- set the NPC's variable 6 to 0

### `3005-03`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- your Faction = 1
- your Level ≤ 70
- you carry ≥ 18 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- 18 × [[items/quest/22-iron-fragment|Iron Fragment]] is taken
- add 8 to your UnionPoint1
- experience: 120 (XP, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

If the checks fail, step `3005-04` is tried instead.

### `3005-04`

Checks:

- you have [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- your Faction = 1
- your Level > 70
- you carry ≥ 18 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- 18 × [[items/quest/22-iron-fragment|Iron Fragment]] is taken
- add 3 to your UnionPoint1
- experience: 12000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

### `3005-31`

Happens by killing [[monsters/136-grunter-warrior|Grunter Warrior]].

Checks:

- you have [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- a random roll 0–99 lands in 0–30
- you carry < 18 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3005-necessity-of-a-new-shrine-2|Necessity of a New Shrine (2)]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]

If the checks fail, step `3805-31` is tried instead.
