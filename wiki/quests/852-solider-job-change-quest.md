---
kind: quest
id: 852
name: Solider Job Change Quest
status: in-game
npcs:
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
monsters:
- '[[monsters/62-honeybee|HoneyBee]]'
steps: 4
source:
  data: LIST_QUEST.STB row 852; QSD triggers 852-01, 852-02, 852-03, 852-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solider Job Change Quest

Leonard wants you to collect 10 HoneyBee Stingers as part of his test to prove you're worthy of becoming a Soldier.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `852-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/851-solider-job-change-quest|Solider Job Change Quest]]

Then:

- works on [[quests/851-solider-job-change-quest|Solider Job Change Quest]]
- the quest becomes [[quests/852-solider-job-change-quest|Solider Job Change Quest]]

### `852-02`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- you carry ≥ 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]] is taken
- the quest becomes [[quests/853-solider-job-change-quest|Solider Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `852-03`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- you carry ≥ 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- set quest switch 4 to 1

### `852-31`

Happens by killing [[monsters/62-honeybee|HoneyBee]].

Checks:

- you have [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 10 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
- you get 1 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]
