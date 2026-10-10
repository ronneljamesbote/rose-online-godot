---
kind: quest
id: 904
name: Muse Job Change Quest
status: in-game
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
steps: 2
source:
  data: LIST_QUEST.STB row 904; QSD triggers 903-01, 904-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Muse Job Change Quest

Tryteh made a potion for you called the 'Nectar of Deep Sleep.' With this, Warren should allow you to become a Muse.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `904-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- you have [[quests/904-muse-job-change-quest|Muse Job Change Quest]]
- you carry = 1 × [[items/quest/105-nectar-of-deep-sleep|Nectar of Deep Sleep]]

Then:

- works on [[quests/904-muse-job-change-quest|Muse Job Change Quest]]
- 1 × [[items/quest/105-nectar-of-deep-sleep|Nectar of Deep Sleep]] is taken
- set your Job to 211
- add 1 to job variable 0
- HP set to 100% and MP to 100%
- you get [[items/head/61-spiritual-beret|Spiritual Beret]] (item count: 1)
- the quest ends (removed from your list)
