---
kind: quest
id: 904
name: Muse Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
steps: 2
source:
  data: 'godot/data/quests/904-muse-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/903-01, qj000-01/904-01'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Muse Job Change Quest

**Can't be done any more.** First jobs come from the job mentors at the Sandwind Caravan Camp in
the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] (see
[[quests/801-junon-planet-identification|Junon Planet Identification]]), and the Zant city hub
redesign took this quest's steps out of the Zant NPCs' dialogs. The steps below are iROSE's and
are kept in the game data.

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
- you get [[items/head/61-spiritual-beret|Spiritual Beret]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

## Changed from iROSE

Not in the game any more: see the note at the top.
