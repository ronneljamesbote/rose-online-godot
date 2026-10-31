---
kind: quest
id: 102
name: Dreadful Epidemic
status: in-game
given_by:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
npcs:
- '[[npcs/1047-herbalist-linnea|Herbalist Linnea]]'
steps: 4
source:
  data: 'godot/data/quests/102-dreadful-epidemic.tres; quest triggers in godot/data/quest_triggers/: qp103/101-02, qp103/102-01, qp103/102-03, qp103/102-04'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dreadful Epidemic

Seyon said that the mushrooms started to rot after a group of suspicious people garbed in red were camping near them. Let's search the area around the mushrooms for clues.  

The rotten mushroom ring is south-west of Arua's Landing, in Whisperwood. [[npcs/1047-herbalist-linnea|Herbalist Linnea]] stands at the ring: tell her you are looking for clues and dig up the emblem (step `102-01`).

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `101-02`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/101-dreadful-epidemic|Dreadful Epidemic]]

Then:

- works on [[quests/101-dreadful-epidemic|Dreadful Epidemic]]
- the quest becomes [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- set episode variable 0 to 1

### `102-01`

Happens by talking to [[npcs/1047-herbalist-linnea|Herbalist Linnea]] at the mushroom ring.

Checks:

- you have [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- the quest becomes [[quests/103-mysterious-emblem|Mysterious Emblem]] (progress kept)

### `102-03`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- episode variable 0 = 1

Then:

- you get the quest [[quests/102-dreadful-epidemic|Dreadful Epidemic]]

### `102-04`

Checks:

- you have [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- client script `mushroom`

If the checks fail, step `102-05` is tried instead.

## Changed from iROSE

In iROSE you find the emblem by clicking the mushroom clue in the ring, and step `102-04`
plays a client script. Our game doesn't use clickable event objects yet, so
[[npcs/1047-herbalist-linnea|Herbalist Linnea]], new with the Adventurer's Plain redesign,
stands at the ring and runs step `102-01` when you talk to her. Step `102-04` is kept but
nothing runs it.
