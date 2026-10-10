---
kind: quest
id: 1072
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]'
steps: 4
source:
  data: LIST_QUEST.STB row 1072; QSD triggers 1071-01, 1072-01, 1072-31, 1072-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

The story that Sharlin told you about Kirnale and Lillith was really romantic. She seemed really eager to see the Phoenix Writing Quill that Kirnale gave to Lillith. Search the Gorge of Silence to see if you can find it.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1071-01`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]].

Checks:

- you have [[quests/1071-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]

Then:

- works on [[quests/1071-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- the quest becomes [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

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

### `1072-31`

Checks:

- you have [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- quest variable 2 < 20

Then:

- works on [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- add 1 to quest variable 2

If the checks fail, step `1072-32` is tried instead.

### `1072-32`

Checks:

- you have [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- quest variable 2 ≥ 20
- you carry < 1 × [[items/quest/219-phoenix-writing-quill|Phoenix Writing Quill]]

Then:

- works on [[quests/1072-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/219-phoenix-writing-quill|Phoenix Writing Quill]]

If the checks fail, step `1078-31` is tried instead.
