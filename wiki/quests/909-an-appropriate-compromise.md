---
kind: quest
id: 909
name: An Appropriate Compromise
status: in-game
npcs:
- '[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]]'
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 2
source:
  data: LIST_QUEST.STB row 909; QSD triggers 908-02, 909-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Appropriate Compromise

Shroon said that he'd never give up Anima Lake. You have to talk to Francis, the head of the Junon Order in Zant.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `908-02`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/908-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/908-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/909-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)

### `909-01`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/909-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/909-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/910-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)
