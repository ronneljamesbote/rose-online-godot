---
kind: quest
id: 1063
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
- '[[npcs/1095-designer-lisa|Designer Lisa]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1063; QSD triggers 1062-01, 1063-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

When a faction of the Seven Heroes was opposed to embarking on the Second Expedition, Namielle destroyed her own wedding ring with Lucid's Sword to illustrate her dedication and willingness to sacrifice. However, there is a rumor that her ring was never actually broken. To find out the truth, you should visit Mina, the jewel seller in Zant.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1062-01`

Happens by talking to [[npcs/1095-designer-lisa|Designer Lisa]].

Checks:

- you have [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]]

Then:

- works on [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]] is taken
- the quest becomes [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1063-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]

Then:

- works on [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- the quest becomes [[quests/1064-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)
