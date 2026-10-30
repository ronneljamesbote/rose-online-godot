---
kind: quest
id: 2105
name: 'Stolen Supplies'
status: in-game
given_by:
- '[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]]'
level: 13
steps: 3
source:
  data: 'godot/data/quests/2105-stolen-supplies.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2105-01, ql21-001/2105-02, ql21-001/2105-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stolen Supplies

The Cowboy Pumpkins took the caravan's crates to Raider's Perch in the Redrock Canyons. Take back 6 Caravan Crates.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]] offers it from level 13, after [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]].

- **Start:** Take back 6 Caravan Crates from the Cowboy Pumpkins in the Redrock Canyons.
- **End:** Dorran's crates are back in the camp.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2105-01`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- your Level ≥ 13
- quest switch 603 = 1 (you finished [[quests/2104-the-wagon-that-never-came|The Wagon That Never Came]])
- quest switch 604 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2105-stolen-supplies|Stolen Supplies]]

### `2105-02`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- you have [[quests/2105-stolen-supplies|Stolen Supplies]]
- you carry ≥ 6 × [[items/quest/703-caravan-crate|Caravan Crate]]

Then:

- works on [[quests/2105-stolen-supplies|Stolen Supplies]]
- 6 × [[items/quest/703-caravan-crate|Caravan Crate]] is taken
- experience, base 1400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 1500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/body/7-yellow-denim-look|Yellow Denim Look]]
- set quest switch 604 to 1
- the quest ends (removed from your list)

### `2105-31`

Happens by killing [[monsters/24-cowboy-pumpkin|Cowboy Pumpkin]].

Checks:

- you have [[quests/2105-stolen-supplies|Stolen Supplies]]
- a random roll 0–99 lands in 0–49
- you carry < 6 × [[items/quest/703-caravan-crate|Caravan Crate]]

Then:

- works on [[quests/2105-stolen-supplies|Stolen Supplies]]
- you get 1 × [[items/quest/703-caravan-crate|Caravan Crate]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
