---
kind: quest
id: 1054
name: Road to Second Job Change
status: in-game
given_by:
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
npcs:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1054; QSD triggers 1054-01, 1054-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Road to Second Job Change

Before giving you the Second Job, Darren wants you to meet and help Mildun, a Ferrell Guild Merchant in Junon.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1054-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- your Job = 4
- your Level ≥ 70
- job variable 0 ≥ 1
- job variable 1 ≤ 1

Then:

- you get the quest [[quests/1054-road-to-second-job-change|Road to Second Job Change]]
- set job variable 0 to 1

### `1054-02`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/1054-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1054-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)
