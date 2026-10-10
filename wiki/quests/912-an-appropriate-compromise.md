---
kind: quest
id: 912
name: An Appropriate Compromise
status: in-game
npcs:
- '[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]]'
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 4
source:
  data: LIST_QUEST.STB row 912; QSD triggers 911-01, 912-01, 912-02, 912-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Appropriate Compromise

Shroon was very pleased to have this new place and is willing to leave Anima Lake for good. Now, you should go back to Francis and let him know of this.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `912-01`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]
- you get [[items/weapon/304-mage-s-rod|Mage's Rod]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 3 to job variable 0
- the quest ends (removed from your list)

### `912-02`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]
- you get [[items/weapon/334-elven-wand|Elven Wand]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 3 to job variable 0
- the quest ends (removed from your list)

### `912-03`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/912-an-appropriate-compromise|An Appropriate Compromise]]
- you get [[items/jewellery/83-textual-necklace|Textual Necklace]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/jewellery/153-textual-earring|Textual Earring]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 3 to job variable 0
- the quest ends (removed from your list)
