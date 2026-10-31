---
kind: quest
id: 2412
name: The Wind Road
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2412-the-wind-road.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2412-01, qz01-001/2412-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Wind Road

Find Caravan Master Odell at the Wind Road Caravan Camp by the east gate.

Lesson 12 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1014-guide-lena|Guide Lena]] offers it after [[quests/2411-beasts-for-hire|Beasts for Hire]]. Nothing in the lesson is checked: talking to Odell again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2412-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- quest switch 650 is on (you finished [[quests/2411-beasts-for-hire|Beasts for Hire]])
- quest switch 651 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2412-the-wind-road|The Wind Road]]

### `2412-02`

Happens by talking to [[npcs/1017-caravan-master-odell|Caravan Master Odell]].

Checks:

- you have [[quests/2412-the-wind-road|The Wind Road]]

Then:

- works on [[quests/2412-the-wind-road|The Wind Road]]
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 651 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
