---
kind: quest
id: 1001
name: Dealer Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
steps: 2
source:
  data: 'godot/data/quests/1001-dealer-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/1001-01, qj000-01/1001-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dealer Job Change Quest

**Can't be started any more.** In iROSE Warren in Zant starts this quest when you hand him your
identification papers. In our game first jobs come from the job mentors at the Sandwind Caravan
Camp in the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]], and Warren sends you
there instead (see [[quests/801-junon-planet-identification|Junon Planet Identification]]). A
character already on this quest can still finish it. The steps below are iROSE's and are kept
in the game data.

Warren suggested that you meet Crow to learn about the basics of trading.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1001-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]

### `1001-02`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]

Then:

- works on [[quests/1001-dealer-job-change-quest|Dealer Job Change Quest]]
- the quest becomes [[quests/1002-dealer-job-change-quest|Dealer Job Change Quest]]
