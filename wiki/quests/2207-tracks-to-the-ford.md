---
kind: quest
id: 2207
name: Tracks to the Ford
status: in-game
given_by:
- '[[npcs/1041-warden-corvel|Warden Corvel]]'
npcs:
- '[[npcs/1044-scout-rhys|Scout Rhys]]'
level: 7
steps: 2
source:
  data: 'godot/data/quests/2207-tracks-to-the-ford.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2207-01, qa22-001/2207-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Tracks to the Ford

Corvel sent you to find Scout Rhys, who is watching the strangers' camp at Bramble Ford in the south-west.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1041-warden-corvel|Warden Corvel]] offers it from level 7, after [[quests/2206-tibbs-farm|Tibbs' Farm]].

- **Start:** Find Scout Rhys at the strangers' camp in Bramble Ford.
- **End:** You found Rhys at the camp.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2207-01`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- quest switch 628 is on (you finished [[quests/2206-tibbs-farm|Tibbs' Farm]])
- your Level ≥ 7
- quest switch 629 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2207-tracks-to-the-ford|Tracks to the Ford]]

### `2207-02`

Happens by talking to [[npcs/1044-scout-rhys|Scout Rhys]].

Checks:

- you have [[quests/2207-tracks-to-the-ford|Tracks to the Ford]]

Then:

- works on [[quests/2207-tracks-to-the-ford|Tracks to the Ford]]
- experience, base 220 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 250 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/head/835-riverford-hat|Riverford Hat]]
- quest switch 629 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
