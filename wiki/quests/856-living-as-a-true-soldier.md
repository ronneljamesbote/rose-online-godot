---
kind: quest
id: 856
name: Living as a True Soldier
status: in-game
npcs:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
steps: 4
source:
  data: LIST_QUEST.STB row 856; QSD triggers 855-02, 856-01, 856-31, 856-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Living as a True Soldier

For the second exercise, you must defeat 15 Dalpings and 15 Ranger Dalpings.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `855-02`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 ≥ 40

Then:

- works on [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- the quest becomes [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]

### `856-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 ≥ 15
- quest variable 1 ≥ 15

Then:

- works on [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]

### `856-31`

Checks:

- you have [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you carry < 15 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- quest variable 0 < 15

Then:

- works on [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- add 1 to quest variable 0

### `856-32`

Checks:

- you have [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you carry < 15 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]
- quest variable 1 < 15

Then:

- works on [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you get 1 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]
- add 1 to quest variable 1
