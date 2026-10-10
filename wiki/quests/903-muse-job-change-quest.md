---
kind: quest
id: 903
name: Muse Job Change Quest
status: in-game
npcs:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
monsters:
- '[[monsters/22-big-pumpkin|Big Pumpkin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 903; QSD triggers 902-01, 903-01, 903-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Muse Job Change Quest

After receiving the Flanae Seeds, Tryteh has asked you to bring 5 Pumpkin Seeds. Fight Big Pumpkins to find some Pumpkin Seeds.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `902-01`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- you carry ≥ 20 × [[items/quest/106-flanae-seed|Flanae Seed]]

Then:

- works on [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
- 20 × [[items/quest/106-flanae-seed|Flanae Seed]] is taken
- the quest becomes [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- you get [[items/consumable/1-health-vial-s|Health Vial (S)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `903-01`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- you carry ≥ 5 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- 5 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]] is taken
- you get 1 × [[items/quest/105-nectar-of-deep-sleep|Nectar of Deep Sleep]]
- the quest becomes [[quests/904-muse-job-change-quest|Muse Job Change Quest]] (progress kept)

### `903-31`

Happens by killing [[monsters/22-big-pumpkin|Big Pumpkin]].

Checks:

- you have [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- a random roll 0–99 lands in 0–90
- you carry < 5 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]

Then:

- works on [[quests/903-muse-job-change-quest|Muse Job Change Quest]]
- you get 1 × [[items/quest/129-pumpkin-seed|Pumpkin Seed]]
