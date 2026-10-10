---
kind: quest
id: 1062
name: 'Namielle: Mother of All (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1095-designer-lisa|Designer Lisa]]'
monsters:
- '[[monsters/211-junon-s-kingkong|Junon''s KingKong]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1062; QSD triggers 1052-02, 1062-01, 1062-31, 1062-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Namielle: Mother of All (Second Job Change)

During her adventures with the Seven Knights, Namielle invented many spells that were never passed down. However, the secrets to her legendary magic are supposedly hidden in the Old Priest Chain she lost during a battle with Junon's KingKong. Lisa has asked you to retrieve the Old Priest Chain from Junon's KingKong.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1052-02`

Happens by talking to [[npcs/1095-designer-lisa|Designer Lisa]].

Checks:

- you have [[quests/1052-road-to-second-job-change|Road to Second Job Change]]

Then:

- works on [[quests/1052-road-to-second-job-change|Road to Second Job Change]]
- the quest becomes [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1062-01`

Happens by talking to [[npcs/1095-designer-lisa|Designer Lisa]].

Checks:

- you have [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]]

Then:

- works on [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]] is taken
- the quest becomes [[quests/1063-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]] (progress kept)

### `1062-31`

Happens by killing [[monsters/211-junon-s-kingkong|Junon's KingKong]].

Checks:

- you have [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- quest variable 0 < 8

Then:

- works on [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- add 1 to quest variable 0

If the checks fail, step `1062-32` is tried instead.

### `1062-32`

Checks:

- you have [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- quest variable 0 ≥ 8
- you carry < 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]]

Then:

- works on [[quests/1062-namielle-mother-of-all-second-job-change|Namielle: Mother of All (Second Job Change)]]
- you get 1 × [[items/quest/211-old-priest-chain|Old Priest Chain]]
