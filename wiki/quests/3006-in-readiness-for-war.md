---
kind: quest
id: 3006
name: In Readiness for War
status: in-game
given_by:
- '[[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]]'
monsters:
- '[[monsters/151-doonga|Doonga]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3006; QSD triggers 3006-01, 3006-02, 3006-03, 3006-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# In Readiness for War

As antagonism between soldiers of opposing Factions increases, it seems that war will soon break out. The Junon Order is said to win every battle, so long as every member possesses a Silver Cross. Go and bring back 15 Silver Fragments so that the Junon Order can make more Silver Crosses.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3006-01`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- your Faction = 1
- your Level ≥ 51

Then:

- you get the quest [[quests/3006-in-readiness-for-war|In Readiness for War]]

### `3006-02`

Happens by talking to [[npcs/1109-elder-of-junon-order-gorthein|Elder of Junon Order Gorthein]].

Checks:

- you have [[quests/3006-in-readiness-for-war|In Readiness for War]]
- your Faction = 1
- your Level ≤ 60
- you carry ≥ 15 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/3006-in-readiness-for-war|In Readiness for War]]
- 15 × [[items/quest/23-silver-fragment|Silver Fragment]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 6 to your UnionPoint1
- the quest ends (removed from your list)

If the checks fail, step `3006-03` is tried instead.

### `3006-03`

Checks:

- you have [[quests/3006-in-readiness-for-war|In Readiness for War]]
- your Faction = 1
- your Level > 60
- you carry ≥ 15 × [[items/quest/23-silver-fragment|Silver Fragment]]

Then:

- works on [[quests/3006-in-readiness-for-war|In Readiness for War]]
- 15 × [[items/quest/23-silver-fragment|Silver Fragment]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to your UnionPoint1
- the quest ends (removed from your list)

### `3006-31`

Happens by killing [[monsters/151-doonga|Doonga]].

Checks:

- you have [[quests/3006-in-readiness-for-war|In Readiness for War]]
- a random roll 0–99 lands in 0–20
- you carry < 15 × [[items/quest/23-silver-fragment|Silver Fragment]]
- your Faction = 1

Then:

- works on [[quests/3006-in-readiness-for-war|In Readiness for War]]
- you get 1 × [[items/quest/23-silver-fragment|Silver Fragment]]

If the checks fail, step `3606-31` is tried instead.
