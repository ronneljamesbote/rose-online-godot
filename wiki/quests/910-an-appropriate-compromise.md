---
kind: quest
id: 910
name: An Appropriate Compromise
status: in-game
npcs:
- '[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]]'
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 2
source:
  data: LIST_QUEST.STB row 910; QSD triggers 909-01, 910-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Appropriate Compromise

Francis has accepted the Ikaness' suggestion and will delay the temple construction for 3 years to find and build a new place for the Ikaness to live. You should tell Shroon about this.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `909-01`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/909-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/909-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/910-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)

### `910-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/910-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/910-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/911-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)
