---
kind: quest
id: 2210
name: Over the Ridge
status: in-game
given_by:
- '[[npcs/1041-warden-corvel|Warden Corvel]]'
npcs:
- '[[npcs/1054-caravan-master-dorran|Caravan Master Dorran]]'
level: 9
steps: 3
source:
  data: 'godot/data/quests/2210-over-the-ridge.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2210-01, qa22-001/2210-02, qa22-001/2210-04'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Over the Ridge

Corvel asked you to take the Enigmatic Emblem over the ridge to Caravan Master Dorran at the Sandwind Caravan Camp in the Valley of Luxem Tower.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1041-warden-corvel|Warden Corvel]] offers it from level 9, after [[quests/2209-the-jelly-king|The Jelly King]].

- **Start:** Take the Enigmatic Emblem to Dorran at the Sandwind Caravan Camp, through either Luxem gate.
- **End:** Dorran has the emblem.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2210-01`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- quest switch 631 is on (you finished [[quests/2209-the-jelly-king|The Jelly King]])
- quest switch 632 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2210-over-the-ridge|Over the Ridge]]
- then runs step `2210-04` ([[quests/2210-over-the-ridge|Over the Ridge]])

### `2210-02`

Happens by talking to [[npcs/1054-caravan-master-dorran|Caravan Master Dorran]].

Checks:

- you have [[quests/2210-over-the-ridge|Over the Ridge]]
- you carry ≥ 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/2210-over-the-ridge|Over the Ridge]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- experience, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 632 on
- the quest ends (removed from your list)

### `2210-04`

Checks:

- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
