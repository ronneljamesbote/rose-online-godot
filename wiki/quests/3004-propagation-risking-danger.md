---
kind: quest
id: 3004
name: Propagation Risking Danger
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
npcs:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3004; QSD triggers 3004-01, 3004-02, 3004-03, 3004-04, 3004-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Propagation Risking Danger

Whenever a new Junon Scripture is released, Gorthein sends a copy to Kay. Now, you must travel to the Forest of Wisdom so that you can deliver the Junon Scripture to Kay.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3004-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 10
- the NPC is within 0 m

Then:

- you get the quest [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- works on [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- you get 1 × [[items/quest/303-junon-scripture|Junon Scripture]]
- set quest variable 0 to 10
- add 1 to the NPC's variable 2
- then runs step `3004-02`

### `3004-02`

Checks:

- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC's variable 0 = 1
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2
- set the NPC's variable 2 to 0

### `3004-03`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- you carry = 1 × [[items/quest/303-junon-scripture|Junon Scripture]]
- quest variable 0 = 10
- the NPC [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]
- the NPC is within 0 m

Then:

- works on [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- 1 × [[items/quest/303-junon-scripture|Junon Scripture]] is taken
- add 10 to quest variable 0

### `3004-04`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- quest variable 0 = 20
- your Faction = 1
- your Level ≤ 50
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC is within 0 m

Then:

- works on [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- add 5 to your UnionPoint1
- experience, base 80 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3004-05` is tried instead.

### `3004-05`

Checks:

- you have [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- quest variable 0 = 20
- your Faction = 1
- your Level > 50
- the NPC [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]
- the NPC is within 0 m

Then:

- works on [[quests/3004-propagation-risking-danger|Propagation Risking Danger]]
- add 2 to your UnionPoint1
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
