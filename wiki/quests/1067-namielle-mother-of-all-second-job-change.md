---
kind: quest
id: 1067
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1067; QSD triggers 1066-01, 1067-01, 1067-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

Mina has praised your heroic activities and suggested that you change to the Second Job. Mina has even written you a recommendation letter that you can show to Darren in Junon Polis for the job change.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `1067-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1067-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1067-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/213-mina-s-recommendation|Mina's Recommendation]] is taken
- take 200000 from your Money
- set your Job to 221
- set job variable 1 to 2
- the quest ends (removed from your list)

### `1067-02`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1067-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1067-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/213-mina-s-recommendation|Mina's Recommendation]] is taken
- take 200000 from your Money
- set your Job to 222
- set job variable 1 to 2
- the quest ends (removed from your list)
