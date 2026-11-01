---
kind: quest
id: 2410
name: Stronger Together
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2410-stronger-together.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2410-01, qz01-001/2410-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stronger Together

Listen to Lena about parties, trading and personal shops.

Lesson 10 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1014-guide-lena|Guide Lena]] offers it after [[quests/2409-nothing-goes-to-waste|Nothing Goes to Waste]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2410-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- quest switch 648 is on (you finished [[quests/2409-nothing-goes-to-waste|Nothing Goes to Waste]])
- quest switch 649 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2410-stronger-together|Stronger Together]]

### `2410-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/2410-stronger-together|Stronger Together]]

Then:

- works on [[quests/2410-stronger-together|Stronger Together]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 649 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
