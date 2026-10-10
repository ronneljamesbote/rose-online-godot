---
kind: quest
id: 1078
name: Lillith, Woman of the Cosmos (Second Job Change)
status: in-game
npcs:
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1078; QSD triggers 1077-01, 1078-01, 1078-31, 1078-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lillith, Woman of the Cosmos (Second Job Change)

Jones expressed regret that Lillith's love story is more famous that her contributions to the Akram Kingdom. He suggested that you find Lillith's Scales in order to raise recognition of Lillith's true accomplishments. Go to the Gorge of Silence and investigate the Stone Golems that may be carrying Lillith's Scales.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1077-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]

Then:

- works on [[quests/1077-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- 5 × [[items/quest/224-bottled-love-letter|Bottled Love Letter]] is taken
- the quest becomes [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]] (progress kept)

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

### `1078-31`

Checks:

- you have [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- quest variable 2 < 20

Then:

- works on [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- add 1 to quest variable 2

If the checks fail, step `1078-32` is tried instead.

### `1078-32`

Checks:

- you have [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- quest variable 2 ≥ 20
- you carry < 1 × [[items/quest/225-lillith-s-scales|Lillith's Scales]]

Then:

- works on [[quests/1078-lillith-woman-of-the-cosmos-second-job-change|Lillith, Woman of the Cosmos (Second Job Change)]]
- you get 1 × [[items/quest/225-lillith-s-scales|Lillith's Scales]]
