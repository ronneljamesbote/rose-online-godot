---
kind: quest
id: 2104
name: 'The Wagon That Never Came'
status: in-game
given_by:
- '[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]]'
npcs:
- '[[npcs/1069-wagon-driver-oskar|Wagon Driver Oskar]]'
level: 11
steps: 3
source:
  data: 'godot/data/quests/2104-the-wagon-that-never-came.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2104-01, ql21-001/2104-02, ql21-001/2104-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Wagon That Never Came

One of Dorran's wagons never reached the camp. Find it in the dry wadi past the oasis and see if the driver is alive.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]] offers it from level 11, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Find Dorran's lost wagon in the dry wadi and talk to its driver.
- **End:** You told Dorran what happened to his wagon.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2104-01`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- your Level ≥ 11
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 603 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]

### `2104-02`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- you have [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]
- quest variable 0 = 1 (you talked to [[npcs/1069-wagon-driver-oskar|Wagon Driver Oskar]])

Then:

- works on [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]
- experience, base 900 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 900 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 603 to 1
- the quest ends (removed from your list)

### `2104-03`

Happens by talking to [[npcs/1069-wagon-driver-oskar|Wagon Driver Oskar]].

Checks:

- you have [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]
- quest variable 0 = 0

Then:

- works on [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]]
- set quest variable 0 to 1

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
