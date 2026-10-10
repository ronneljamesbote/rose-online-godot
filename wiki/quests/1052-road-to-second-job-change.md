---
kind: quest
id: 1052
name: Road to Second Job Change
status: in-game
given_by:
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
npcs:
- '[[npcs/1095-designer-lisa|Designer Lisa]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1052; QSD triggers 1052-01, 1052-02, 1052-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Road to Second Job Change

Before giving you the Second Job, Darren wants you to meet and help Lisa, a Designer in Junon.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1052-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- your Job = 2
- your Level ≥ 70
- job variable 0 ≥ 1
- job variable 1 ≤ 1

Then:

- you get the quest [[quests/1052-road-to-second-job-change|Road to Second Job Change]]
- set job variable 1 to 1

### `1052-02`

Happens by talking to [[npcs/1095-designer-lisa|Designer Lisa]].

Checks:

- you have [[quests/1052-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1052-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1052-31`

Checks:

- you have [[quests/1052-road-to-second-job-change|Road to Second Job Change]]
- you carry < 3 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/1052-road-to-second-job-change|Road to Second Job Change]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
