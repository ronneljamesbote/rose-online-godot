---
kind: quest
id: 855
name: Living as a True Soldier
status: in-game
given_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
monsters:
- '[[monsters/81-beetle|Beetle]]'
- '[[monsters/82-beetle-fighter|Beetle Fighter]]'
steps: 4
source:
  data: LIST_QUEST.STB row 855; QSD triggers 855-01, 855-02, 855-31, 855-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Living as a True Soldier

For the sake of becoming a true Soldier, Raffle has suggested that you complete 3 training exercises. First, you need to defeat 40 Beetles.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `855-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- your Job = 1
- your Level ≥ 20
- job variable 0 = 1

Then:

- you get the quest [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]

### `855-02`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 ≥ 40

Then:

- works on [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- the quest becomes [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]

### `855-31`

Happens by killing [[monsters/81-beetle|Beetle]].

Checks:

- you have [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 < 40
- you carry < 40 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- add 1 to quest variable 0
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1005-31` is tried instead.

### `855-32`

Happens by killing [[monsters/82-beetle-fighter|Beetle Fighter]].

Checks:

- you have [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 < 40
- you carry < 40 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/855-living-as-a-true-soldier|Living as a True Soldier]]
- add 1 to quest variable 0
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1005-32` is tried instead.
