---
kind: quest
id: 2403
name: Safe Keeping
status: in-game
given_by:
- '[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2403-safe-keeping.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2403-01, qz01-001/2403-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Safe Keeping

Put the Scrap Gloves in Crow's storage and take them out again, then talk to Crow.

Lesson 3 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]] offers it after [[quests/2402-a-place-to-return|A Place to Return]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2403-01`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- quest switch 641 is on (you finished [[quests/2402-a-place-to-return|A Place to Return]])
- quest switch 642 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2403-safe-keeping|Safe Keeping]]
- you get 1 × [[items/hands/656-scrap-gloves|Scrap Gloves]]

### `2403-02`

Happens by talking to [[npcs/1004-ferrell-guild-staff-crow|Ferrell Guild Staff Crow]].

Checks:

- you have [[quests/2403-safe-keeping|Safe Keeping]]

Then:

- works on [[quests/2403-safe-keeping|Safe Keeping]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 642 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
