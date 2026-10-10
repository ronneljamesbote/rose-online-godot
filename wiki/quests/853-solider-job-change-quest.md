---
kind: quest
id: 853
name: Solider Job Change Quest
status: in-game
npcs:
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
monsters:
- '[[monsters/23-elder-pumpkin|Elder Pumpkin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 853; QSD triggers 852-02, 853-01, 853-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solider Job Change Quest

Leonard now wants you to fight Elder Pumpkins and bring him 3 Pumpkin Seeds.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `852-02`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- you carry ≥ 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]] is taken
- the quest becomes [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]] (item count: 5)

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

### `853-31`

Happens by killing [[monsters/23-elder-pumpkin|Elder Pumpkin]].

Checks:

- you have [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- a random roll 0–99 lands in 0–75
- you carry < 3 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- you get 1 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]
