---
kind: quest
id: 851
name: Solider Job Change Quest
status: in-game
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]]'
steps: 2
source:
  data: LIST_QUEST.STB row 851; QSD triggers 851-01, 852-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Solider Job Change Quest

Warren says you cannot become a Soldier unless you receive a recommendation. You should talk to Leonard in Zant in order to obtain one.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `851-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/851-solider-job-change-quest|Solider Job Change Quest]]

### `852-01`

Happens by talking to [[npcs/1005-righteous-crusader-leonard|Righteous Crusader Leonard]].

Checks:

- you have [[quests/851-solider-job-change-quest|Solider Job Change Quest]]

Then:

- works on [[quests/851-solider-job-change-quest|Solider Job Change Quest]]
- the quest becomes [[quests/852-solider-job-change-quest|Solider Job Change Quest]]
