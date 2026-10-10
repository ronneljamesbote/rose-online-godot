---
kind: quest
id: 1064
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1064; QSD triggers 1063-01, 1064-01, 1064-31, 1064-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

Mina is certain that Namielle's Wedding Ring still exists and suggested that you investigate the Goblin Warriors in the 2nd level of the Goblin Cave.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1063-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]

Then:

- works on [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- the quest becomes [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1064-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]]

Then:

- works on [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]] is taken
- the quest becomes [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1064-31`

Checks:

- you have [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- quest variable 1 < 20

Then:

- works on [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- add 1 to quest variable 1

If the checks fail, step `1064-32` is tried instead.

### `1064-32`

Checks:

- you have [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- quest variable 1 ≥ 20
- you carry < 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]]

Then:

- works on [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you get 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]]

If the checks fail, step `1070-31` is tried instead.
