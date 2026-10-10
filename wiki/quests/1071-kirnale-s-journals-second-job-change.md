---
kind: quest
id: 1071
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]'
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1071; QSD triggers 1070-01, 1071-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

You found Kirnale's Journals and returned them to Jones, asking him not to lose them again. Jones then suggested that you visit Sharlin, the tavern owner in Zant, if you want to hear the story of Kirnale and Lillith.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1070-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/214-kirnale-s-journal-vol-1|Kirnale's Journal (Vol. 1)]]
- you carry ≥ 1 × [[items/quest/215-kirnale-s-journal-vol-2|Kirnale's Journal (Vol. 2)]]
- you carry ≥ 1 × [[items/quest/216-kirnale-s-journal-vol-3|Kirnale's Journal (Vol. 3)]]
- you carry ≥ 1 × [[items/quest/217-kirnale-s-journal-vol-4|Kirnale's Journal (Vol. 4)]]
- you carry ≥ 1 × [[items/quest/218-kirnale-s-journal-vol-5|Kirnale's Journal (Vol. 5)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/214-kirnale-s-journal-vol-1|Kirnale's Journal (Vol. 1)]] is taken
- 1 × [[items/quest/215-kirnale-s-journal-vol-2|Kirnale's Journal (Vol. 2)]] is taken
- 1 × [[items/quest/216-kirnale-s-journal-vol-3|Kirnale's Journal (Vol. 3)]] is taken
- 1 × [[items/quest/217-kirnale-s-journal-vol-4|Kirnale's Journal (Vol. 4)]] is taken
- 1 × [[items/quest/218-kirnale-s-journal-vol-5|Kirnale's Journal (Vol. 5)]] is taken
- the quest becomes [[quests/1071-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

### `1071-01`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]].

Checks:

- you have [[quests/1071-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]

Then:

- works on [[quests/1071-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- the quest becomes [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)
