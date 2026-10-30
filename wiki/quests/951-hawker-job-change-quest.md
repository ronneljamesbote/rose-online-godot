---
kind: quest
id: 951
name: Hawker Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]'
steps: 3
source:
  data: 'godot/data/quests/951-hawker-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/951-01, qj000-01/951-02, qj000-01/951-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hawker Job Change Quest

**Can't be started any more.** In iROSE Warren in Zant starts this quest when you hand him your
identification papers. In our game first jobs come from the job mentors at the Sandwind Caravan
Camp in the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]], and Warren sends you
there instead (see [[quests/801-junon-planet-identification|Junon Planet Identification]]). A
character already on this quest can still finish it. The steps below are iROSE's and are kept
in the game data.

Reasoning that speed is essential for Hawkers, Warren has asked you to bring him a Tarren Earring from Shannon. You better head straight to the Tower of Luxem Valley.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `951-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- your Level ≥ 10
- your Job = 0
- you have [[quests/801-junon-planet-identification|Junon Planet Identification]]
- you carry = 1 × [[items/quest/101-identification|Identification]]

Then:

- works on [[quests/801-junon-planet-identification|Junon Planet Identification]]
- 1 × [[items/quest/101-identification|Identification]] is taken
- the quest becomes [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]

### `951-02`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]

Then:

- works on [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]
- the quest becomes [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]

### `951-03`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]

Then:

- works on [[quests/951-hawker-job-change-quest|Hawker Job Change Quest]]
- set quest switch 4 to 1
