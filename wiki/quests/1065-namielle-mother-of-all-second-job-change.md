---
kind: quest
id: 1065
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1065; QSD triggers 1064-01, 1065-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

In the interest of preventing the disparaging rumors about Namielle from gaining credibility, Mina has decided to safeguard Namielle's Wedding Ring. However, another cause for concern in regards to Namielle's memory and reputation are the Hairpins of Oblivious. Learn more about this by speaking to Lutis in the Valley of Luxem Tower.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1064-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]]

Then:

- works on [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/210-namielle-s-wedding-ring|Namielle's Wedding Ring]] is taken
- the quest becomes [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1065-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]

Then:

- works on [[quests/1065-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- the quest becomes [[quests/1066-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)
