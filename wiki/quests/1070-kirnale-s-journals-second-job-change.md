---
kind: quest
id: 1070
name: Kirnale's Journals (Second Job Change)
status: in-game
npcs:
- '[[npcs/1104-historian-jones|Historian Jones]]'
steps: 7
source:
  data: LIST_QUEST.STB row 1070; QSD triggers 1069-01, 1070-01, 1070-31, 1070-32, 1070-33, 1070-34, 1070-35
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kirnale's Journals (Second Job Change)

Jones said that he finally lent Kirnale's Journals to a stubborn adventurer who managed to lose them in the Goblin Cave. Hurry and search the 2nd level of the Goblin Cave for Kirnale's Journals!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1069-01`

Happens by talking to [[npcs/1104-historian-jones|Historian Jones]].

Checks:

- you have [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]

Then:

- works on [[quests/1069-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- 1 × [[items/quest/220-kirnale-s-compass|Kirnale's Compass]] is taken
- 1 × [[items/quest/221-portable-sundial|Portable Sundial]] is taken
- the quest becomes [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]] (progress kept)

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

### `1070-31`

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/214-kirnale-s-journal-vol-1|Kirnale's Journal (Vol. 1)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/214-kirnale-s-journal-vol-1|Kirnale's Journal (Vol. 1)]]

If the checks fail, step `1070-32` is tried instead.

### `1070-32`

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/215-kirnale-s-journal-vol-2|Kirnale's Journal (Vol. 2)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/215-kirnale-s-journal-vol-2|Kirnale's Journal (Vol. 2)]]

If the checks fail, step `1070-33` is tried instead.

### `1070-33`

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/216-kirnale-s-journal-vol-3|Kirnale's Journal (Vol. 3)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/216-kirnale-s-journal-vol-3|Kirnale's Journal (Vol. 3)]]

If the checks fail, step `1070-34` is tried instead.

### `1070-34`

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–20
- you carry < 1 × [[items/quest/217-kirnale-s-journal-vol-4|Kirnale's Journal (Vol. 4)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/217-kirnale-s-journal-vol-4|Kirnale's Journal (Vol. 4)]]

If the checks fail, step `1070-35` is tried instead.

### `1070-35`

Checks:

- you have [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- a random roll 0–99 lands in 0–10
- you carry < 1 × [[items/quest/218-kirnale-s-journal-vol-5|Kirnale's Journal (Vol. 5)]]

Then:

- works on [[quests/1070-kirnale-s-journals-second-job-change|Kirnale's Journals (Second Job Change)]]
- you get 1 × [[items/quest/218-kirnale-s-journal-vol-5|Kirnale's Journal (Vol. 5)]]

If the checks fail, step `1074-31` is tried instead.
