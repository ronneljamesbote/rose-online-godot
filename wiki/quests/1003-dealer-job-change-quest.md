---
kind: quest
id: 1003
name: Dealer Job Change Quest
status: in-game
npcs:
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
monsters:
- '[[monsters/61-needle-honeybee|Needle HoneyBee]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1003; QSD triggers 1002-01, 1003-01, 1003-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dealer Job Change Quest

Crow asked you to bring him 15 HoneyBee Stingers. HoneyBee Stingers can found from hunting Needle HoneyBees.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1002-01`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- you carry = 5 × [[items/quest/111-small-metal-board|Small Metal Board]]
- you carry = 5 × [[items/quest/112-fleece|Fleece]]

Then:

- works on [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
- 5 × [[items/quest/111-small-metal-board|Small Metal Board]] is taken
- 5 × [[items/quest/112-fleece|Fleece]] is taken
- the quest becomes [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

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

### `1003-31`

Happens by killing [[monsters/61-needle-honeybee|Needle HoneyBee]].

Checks:

- you have [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 15 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]

Then:

- works on [[quests/1003-dealer-job-change-quest|Dealer Job Change Quest]]
- you get 1 × [[items/quest/103-honeybee-stinger|HoneyBee Stinger]]
