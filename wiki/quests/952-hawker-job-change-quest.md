---
kind: quest
id: 952
name: Hawker Job Change Quest
status: in-game
npcs:
- '[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]'
monsters:
- '[[monsters/13-mother-choropy|Mother Choropy]]'
- '[[monsters/15-spotted-choropy|Spotted Choropy]]'
- '[[monsters/203-jelly-king|Jelly King]]'
steps: 5
source:
  data: LIST_QUEST.STB row 952; QSD triggers 951-02, 952-01, 952-31, 952-32, 952-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hawker Job Change Quest

Before Shannon will give you to Tarren Earring, you must bring twenty 20 Crystal Dew. Hunt Mother Choropy, Spotted Choropy, and Jelly King to get Crystal Dew.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `951-02`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]

Then:

- works on [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]
- the quest becomes [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]

### `952-01`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you carry ≥ 20 × [[items/quest/130-crystal-dew|Crystal Dew]]

Then:

- works on [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- 20 × [[items/quest/130-crystal-dew|Crystal Dew]] is taken
- you get 1 × [[items/quest/108-tarren-earring|Tarren Earring]]
- the quest becomes [[quests/953-hawker-job-change-quest|Hawker Job Change Quest]] (progress kept)

### `952-31`

Happens by killing [[monsters/13-mother-choropy|Mother Choropy]].

Checks:

- you have [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- a random roll 0–99 lands in 0–80
- you carry < 20 × [[items/quest/130-crystal-dew|Crystal Dew]]

Then:

- works on [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you get 1 × [[items/quest/130-crystal-dew|Crystal Dew]]

### `952-32`

Happens by killing [[monsters/15-spotted-choropy|Spotted Choropy]].

Checks:

- you have [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 20 × [[items/quest/130-crystal-dew|Crystal Dew]]

Then:

- works on [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you get 1 × [[items/quest/130-crystal-dew|Crystal Dew]]

### `952-33`

Happens by killing [[monsters/203-jelly-king|Jelly King]].

Checks:

- you have [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you carry < 20 × [[items/quest/130-crystal-dew|Crystal Dew]]

Then:

- works on [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you get 3 × [[items/quest/130-crystal-dew|Crystal Dew]]
