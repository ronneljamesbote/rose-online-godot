---
kind: quest
id: 911
name: An Appropriate Compromise
status: in-game
npcs:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
monsters:
- '[[monsters/132-porkie|Porkie]]'
- '[[monsters/133-porkie-hooligan|Porkie Hooligan]]'
steps: 4
source:
  data: LIST_QUEST.STB row 911; QSD triggers 910-01, 911-01, 911-31, 911-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Appropriate Compromise

Shroon has agreed with Francis' suggestion. To help prepare this new place for the Ikaness, you must kill 20 Porkies and 5 Porkie Hooligans.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `910-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/910-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/910-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/911-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)

### `911-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- you carry ≥ 20 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- you carry ≥ 5 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- 20 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- 5 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- the quest becomes [[quests/912-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)

### `911-31`

Happens by killing [[monsters/132-porkie|Porkie]].

Checks:

- you have [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- you carry < 20 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `3002-31` is tried instead.

### `911-32`

Happens by killing [[monsters/133-porkie-hooligan|Porkie Hooligan]].

Checks:

- you have [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- you carry < 5 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/911-an-appropriate-compromise|An Appropriate Compromise]]
- you get 1 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `3404-31` is tried instead.
