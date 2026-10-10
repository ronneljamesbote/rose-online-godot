---
kind: quest
id: 1073
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]'
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1073; QSD triggers 1072-01, 1073-01, 1073-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

After giving her the Phoenix Writing Quill, Sharlin has written a recommendation for you. Take her recommendation letter to Darren, mayor of Junon Polis, so that you can change to the Second Job.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1072-01`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]].

Checks:

- you have [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you carry ≥ 1 × [[items/quest/219-phoenix-writing-quill|Phoenix Writing Quill]]

Then:

- works on [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/219-phoenix-writing-quill|Phoenix Writing Quill]] is taken
- you get 1 × [[items/quest/222-sharlin-s-recommendation|Sharlin's Recommendation]]
- the quest becomes [[quests/1073-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

### `1073-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1073-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1073-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/222-sharlin-s-recommendation|Sharlin's Recommendation]] is taken
- take 200000 from your Money
- set your Job to 321
- set job variable 1 to 2
- the quest ends (removed from your list)

### `1073-02`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1073-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1073-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/222-sharlin-s-recommendation|Sharlin's Recommendation]] is taken
- take 200000 from your Money
- set your Job to 322
- set job variable 1 to 2
- the quest ends (removed from your list)
