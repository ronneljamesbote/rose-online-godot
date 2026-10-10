---
kind: quest
id: 3001
name: Shannon's Medical Treatment
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
npcs:
- '[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3001; QSD triggers 3001-01, 3001-02, 3001-03, 3001-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Shannon's Medical Treatment

Shannon has been saving the lives of Visitors for a long time in the Valley of Luxem Tower. Gorthein wants you to help by bringing medical supplies to Shannon.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3001-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC is within 0 m

Then:

- you get the quest [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- works on [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- you get 10 × [[items/quest/304-healing-medicine|Healing Medicine]]
- set quest variable 0 to 10

### `3001-02`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- you carry = 10 × [[items/quest/304-healing-medicine|Healing Medicine]]
- the NPC [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]
- the NPC is within 0 m

Then:

- 10 × [[items/quest/304-healing-medicine|Healing Medicine]] is taken
- add 10 to quest variable 0

### `3001-03`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- your Faction = 1
- your Level ≤ 50
- quest variable 0 = 20
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC is within 0 m

Then:

- works on [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- experience: 80 (XP, scaled by your level, see [[rules/quests|Quests]])
- add 1 to your UnionPoint1
- the quest ends (removed from your list)

If the checks fail, step `3001-04` is tried instead.

### `3001-04`

Checks:

- you have [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- your Faction = 1
- your Level > 50
- quest variable 0 = 20
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC is within 0 m

Then:

- works on [[quests/3001-shannon-s-medical-treatment|Shannon's Medical Treatment]]
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- add 1 to your UnionPoint1
- the quest ends (removed from your list)
