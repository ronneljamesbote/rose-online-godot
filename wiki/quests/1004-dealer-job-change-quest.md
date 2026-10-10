---
kind: quest
id: 1004
name: Dealer Job Change Quest
status: in-game
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
steps: 2
source:
  data: LIST_QUEST.STB row 1004; QSD triggers 1003-01, 1004-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dealer Job Change Quest

Crow appreciates your work for getting him the materials, saying that he can sell them at a good price. He has written a recommendation that you can take to Warren to become a Dealer.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1003-01`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry ≥ 15 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- 15 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]] is taken
- you get 1 × [[items/quest/109-dealer-job-recommendation|Dealer Job Recommendation]]
- the quest becomes [[quests/1004-dealer-job-change-quest|Dealer Job Change Quest]] (progress kept)

### `1004-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- you have [[quests/1004-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry = 1 × [[items/quest/109-dealer-job-recommendation|Dealer Job Recommendation]]

Then:

- works on [[quests/1004-dealer-job-change-quest|Dealer Job Change Quest]]
- 1 × [[items/quest/109-dealer-job-recommendation|Dealer Job Recommendation]] is taken
- set your Job to 411
- add 1 to job variable 0
- HP set to 100% and MP to 100%
- you get [[items/head/121-brown-turban|Brown Turban]] (item count: 1)
- the quest ends (removed from your list)
