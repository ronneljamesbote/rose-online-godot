---
kind: quest
id: 2101
name: 'Dust on the Road'
status: in-game
given_by:
- '[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]]'
npcs:
- '[[npcs/1055-scout-mira|Scout Mira]]'
level: 9
steps: 2
source:
  data: 'godot/data/quests/2101-dust-on-the-road.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2101-01, ql21-001/2101-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dust on the Road

Dorran asked you to report to Mira, the camp's scout, on her lookout rock above the Sandwind Caravan Camp.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]] offers it from level 9.

- **Start:** Go and see Mira on the lookout rock. She watches the whole valley.
- **End:** Mira welcomed you to the camp.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2101-01`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- your Level ≥ 9
- quest switch 600 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2101-dust-on-the-road|Dust on the Road]]

### `2101-02`

Happens by talking to [[npcs/1055-scout-mira|Scout Mira]].

Checks:

- you have [[quests/2101-dust-on-the-road|Dust on the Road]]

Then:

- works on [[quests/2101-dust-on-the-road|Dust on the Road]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 600 to 1
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
