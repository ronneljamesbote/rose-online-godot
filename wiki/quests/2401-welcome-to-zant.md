---
kind: quest
id: 2401
name: Welcome to Zant
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
level: 15
steps: 3
source:
  data: 'godot/data/quests/2401-welcome-to-zant.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2401-01, qz01-001/2401-02, qz01-001/2401-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Welcome to Zant

Read Lena's map of who stands where in Zant, then tell her you're ready.

Lesson 1 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1014-guide-lena|Guide Lena]] offers it from level 15, to anyone who hasn't taken or skipped the tour. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2401-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- your level ≥ 15
- quest switch 640 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2401-welcome-to-zant|Welcome to Zant]]
- you get 1 × [[items/quest/719-lena-s-city-map|Lena's City Map]]

### `2401-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/2401-welcome-to-zant|Welcome to Zant]]

Then:

- works on [[quests/2401-welcome-to-zant|Welcome to Zant]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 640 on
- the quest ends (removed from your list)

### `2401-03`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]], when you choose "I know my way around." and say yes to both of her checks ("I'm sure, I don't need the tour." and "Skip the tour for good.").

Checks:

- your level ≥ 15
- quest switch 640 is off (you haven't started the tour)

Then:

- quest switches 640 to 650 on (lessons 1 to 11 count as done, without their gifts or rewards)
- quest switch 652 on (you skipped the tour)
- you get the quest [[quests/2412-the-wind-road|The Wind Road]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
