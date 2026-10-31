---
kind: quest
id: 901
name: Muse Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
steps: 2
source:
  data: 'godot/data/quests/901-muse-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/901-01, qj000-01/901-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Muse Job Change Quest

**Can't be started any more.** In iROSE Warren in Zant starts this quest when you hand him your
identification papers. In our game first jobs come from the job mentors at the Sandwind Caravan
Camp in the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]], and Warren sends you
there instead (see [[quests/801-junon-planet-identification|Junon Planet Identification]]). Since the Zant city hub
redesign the Zant NPCs no longer run its later steps either, so a character already on it can't
finish it. The steps below are iROSE's and are kept
in the game data.

Warren said that he'd recognize you as a Muse if you had a simple potion created for him. Let’s make one with Tryteh's help.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `901-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/901-muse-job-change-quest|Muse Job Change Quest]]

### `901-02`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/901-muse-job-change-quest|Muse Job Change Quest]]

Then:

- works on [[quests/901-muse-job-change-quest|Muse Job Change Quest]]
- the quest becomes [[quests/902-muse-job-change-quest|Muse Job Change Quest]]
