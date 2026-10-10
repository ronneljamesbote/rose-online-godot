---
kind: quest
id: 1074
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1074; QSD triggers 1054-02, 1074-01, 1074-31, 1074-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

Mildun has asked you to search for Lillith's Family Emblem in order to restore her honor. Investigate the Goblin Warriors in the Goblin Cave (B2) and find the Family Emblem to restore Lillith's legend.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1054-02`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/1054-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1054-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1074-01`

Happens by talking to [[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]].

Checks:

- you have [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/223-family-emblem|Family Emblem]]

Then:

- works on [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 1 × [[items/quest/223-family-emblem|Family Emblem]] is taken
- the quest becomes [[quests/1075-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1074-31`

Checks:

- you have [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- quest variable 1 < 20

Then:

- works on [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- add 1 to quest variable 1

If the checks fail, step `1074-32` is tried instead.

### `1074-32`

Checks:

- you have [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- quest variable 1 ≥ 20
- you carry < 1 × [[items/quest/223-family-emblem|Family Emblem]]

Then:

- works on [[quests/1074-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you get 1 × [[items/quest/223-family-emblem|Family Emblem]]
