---
kind: quest
id: 854
name: Solider Job Change Quest
status: in-game
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
steps: 2
source:
  data: LIST_QUEST.STB row 854; QSD triggers 853-01, 854-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solider Job Change Quest

At last, Leonard has written a recommendation letter for you. With this, Warren will trust you enough to be a Soldier.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `853-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- you carry ≥ 3 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- 3 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]] is taken
- you get 1 × [[items/quest/102-soldier-job-recommendation|Soldier Job Recommendation]]
- the quest becomes [[quests/854-solider-job-change-quest|Solider Job Change Quest]] (progress kept)

### `854-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- you have [[quests/854-solider-job-change-quest|Solider Job Change Quest]]
- you carry = 1 × [[items/quest/102-soldier-job-recommendation|Soldier Job Recommendation]]

Then:

- works on [[quests/854-solider-job-change-quest|Solider Job Change Quest]]
- 1 × [[items/quest/102-soldier-job-recommendation|Soldier Job Recommendation]] is taken
- set your Job to 111
- add 1 to job variable 0
- HP set to 100% and MP to 100%
- you get [[items/hands/31-soldier-gloves|Soldier Gloves]] (item count: 1)
- the quest ends (removed from your list)
