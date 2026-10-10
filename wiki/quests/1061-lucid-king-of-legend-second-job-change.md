---
kind: quest
id: 1061
name: 'Lucid: King of Legend (Second Job Change)'
status: in-game
npcs:
- '[[npcs/1081-mayor-darren|Mayor Darren]]'
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1061; QSD triggers 1060-01, 1061-01, 1061-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lucid: King of Legend (Second Job Change)

Jones has written a recommendation for you. Show it to Darren in Junon Polis so you that you will be able to change to the Second Job.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1060-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]

Then:

- works on [[quests/1060-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/207-lucid-s-sword|Lucid's Sword]] is taken
- you get 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]]
- the quest becomes [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]] (progress kept)

### `1061-01`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]] is taken
- take 200000 from your Money
- set your Job to 121
- set job variable 1 to 2
- the quest ends (removed from your list)

### `1061-02`

Happens by talking to [[npcs/1081-mayor-darren|Mayor Darren]].

Checks:

- you have [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- your Money ≥ 200000

Then:

- works on [[quests/1061-lucid-king-of-legend-second-job-change|Lucid: King of Legend (Second Job Change)]]
- 1 × [[items/quest/209-jones-recommendation|Jones' Recommendation]] is taken
- take 200000 from your Money
- set your Job to 122
- set job variable 1 to 2
- the quest ends (removed from your list)
