---
kind: quest
id: 2409
name: Nothing Goes to Waste
status: in-game
given_by:
- '[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2409-nothing-goes-to-waste.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2409-01, qz01-001/2409-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Nothing Goes to Waste

Have Ulysses take the Scrap Gloves apart, then talk to him again.

Lesson 9 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]] offers it after [[quests/2408-stones-and-sockets|Stones and Sockets]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2409-01`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- quest switch 647 is on (you finished [[quests/2408-stones-and-sockets|Stones and Sockets]])
- quest switch 648 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2409-nothing-goes-to-waste|Nothing Goes to Waste]]
- you get 1 × [[items/hands/656-scrap-gloves|Scrap Gloves]]
- Zuly, base 100 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `2409-02`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/2409-nothing-goes-to-waste|Nothing Goes to Waste]]

Then:

- works on [[quests/2409-nothing-goes-to-waste|Nothing Goes to Waste]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 648 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
