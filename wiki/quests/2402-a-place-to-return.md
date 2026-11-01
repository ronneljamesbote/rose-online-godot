---
kind: quest
id: 2402
name: A Place to Return
status: in-game
given_by:
- '[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2402-a-place-to-return.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2402-01, qz01-001/2402-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# A Place to Return

Make Zant your save point with Francis, then talk to him again.

Lesson 2 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]] offers it after [[quests/2401-welcome-to-zant|Welcome to Zant]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2402-01`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- quest switch 640 is on (you finished [[quests/2401-welcome-to-zant|Welcome to Zant]])
- quest switch 641 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2402-a-place-to-return|A Place to Return]]

### `2402-02`

Happens by talking to [[npcs/1003-co-founder-of-the-junon-order-francis|Co-Founder of the Junon Order Francis]].

Checks:

- you have [[quests/2402-a-place-to-return|A Place to Return]]

Then:

- works on [[quests/2402-a-place-to-return|A Place to Return]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 641 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
