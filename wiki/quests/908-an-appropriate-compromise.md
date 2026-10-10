---
kind: quest
id: 908
name: An Appropriate Compromise
status: in-game
given_by:
- '[[npcs/1095-designer-lisa|Designer Lisa]]'
npcs:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 2
source:
  data: LIST_QUEST.STB row 908; QSD triggers 908-01, 908-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Appropriate Compromise

Lisa asked you if you can help settle a dispute between Francis and Shroon. First, you should meet Shroon at the temple near Lake Anima.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `908-01`

Happens by talking to [[npcs/1095-designer-lisa|Designer Lisa]].

Checks:

- your Level ≥ 30
- your Job = 2
- job variable 0 ≥ 3
- job variable 0 < 6

Then:

- you get the quest [[quests/908-an-appropriate-compromise|An Appropriate Compromise]]

### `908-02`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/908-an-appropriate-compromise|An Appropriate Compromise]]

Then:

- works on [[quests/908-an-appropriate-compromise|An Appropriate Compromise]]
- the quest becomes [[quests/909-an-appropriate-compromise|An Appropriate Compromise]] (progress kept)
