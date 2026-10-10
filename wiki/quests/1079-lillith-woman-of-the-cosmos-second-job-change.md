---
kind: quest
id: 1079
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1079; QSD triggers 1078-01, 1079-01, 1079-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

Jones praised your efforts, and believed it was a good time for you to be promoted. He's written a recommendation letter that you can take to Darren in Junon Polis so that you can change to the Second Job.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1078-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/225-lillith-s-scales|Lillith's Scales]]

Then:

- works on [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 1 × [[items/quest/225-lillith-s-scales|Lillith's Scales]] is taken
- you get 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]]
- the quest becomes [[quests/1079-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

### `1079-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1079-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1079-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]] is taken
- take 200000 from your Money
- set your Job to 421
- set job variable 1 to 2
- the quest ends (removed from your list)

### `1079-02`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1079-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1079-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]] is taken
- take 200000 from your Money
- set your Job to 422
- set job variable 1 to 2
- the quest ends (removed from your list)
