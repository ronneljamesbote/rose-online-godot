---
kind: quest
id: 953
name: Hawker Job Change Quest
status: changed-from-irose
npcs:
- '[[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]]'
- '[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]'
steps: 2
source:
  data: 'godot/data/quests/953-hawker-job-change-quest.tres; quest triggers in godot/data/quest_triggers/: qj000-01/952-01, qj000-01/953-01'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hawker Job Change Quest

**Can't be done any more.** First jobs come from the job mentors at the Sandwind Caravan Camp in
the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] (see
[[quests/801-junon-planet-identification|Junon Planet Identification]]), and the Zant city hub
redesign took this quest's steps out of the Zant NPCs' dialogs. The steps below are iROSE's and
are kept in the game data.

You traded 20 Crystal Dew for the Tarren Earring. Now you should return to Warren who is waiting for you.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `952-01`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- you carry ≥ 20 × [[items/quest/130-crystal-dew|Crystal Dew]]

Then:

- works on [[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]
- 20 × [[items/quest/130-crystal-dew|Crystal Dew]] is taken
- you get 1 × [[items/quest/108-tarren-earring|Tarren Earring]]
- the quest becomes [[quests/953-hawker-job-change-quest|Hawker Job Change Quest]] (progress kept)

### `953-01`

Happens by talking to [[npcs/1002-akram-kingdom-minister-warren|Akram Kingdom Minister Warren]].

Checks:

- you have [[quests/953-hawker-job-change-quest|Hawker Job Change Quest]]
- you carry = 1 × [[items/quest/108-tarren-earring|Tarren Earring]]

Then:

- works on [[quests/953-hawker-job-change-quest|Hawker Job Change Quest]]
- 1 × [[items/quest/108-tarren-earring|Tarren Earring]] is taken
- set your Job to 311
- add 1 to job variable 0
- HP set to 100% and MP to 100%
- you get [[items/hands/91-hunter-gloves|Hunter Gloves]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

## Changed from iROSE

Not in the game any more: see the note at the top.
