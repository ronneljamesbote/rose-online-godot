---
kind: quest
id: 1066
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1066; QSD triggers 1065-01, 1066-01, 1066-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

Lutis said that you might be able to find the Hairpins of Oblivious by hunting Stone Golems in the Gorge of Silence. When you find 5 Hairpins of Oblivious, return to Mina.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1065-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]

Then:

- works on [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- the quest becomes [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1066-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you carry ≥ 5 × [[items/quest/212-hairpin-of-oblivious|Hairpin of Oblivious]]

Then:

- works on [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 5 × [[items/quest/212-hairpin-of-oblivious|Hairpin of Oblivious]] is taken
- you get 1 × [[items/quest/213-mina-s-recommendation|Mina's Recommendation]]
- the quest becomes [[quests/1067-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1066-31`

Checks:

- you have [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- a random roll 0–99 lands in 0–15
- you carry < 5 × [[items/quest/212-hairpin-of-oblivious|Hairpin of Oblivious]]

Then:

- works on [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you get 1 × [[items/quest/212-hairpin-of-oblivious|Hairpin of Oblivious]]

If the checks fail, step `1072-31` is tried instead.
