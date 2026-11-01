---
kind: quest
id: 2411
name: Beasts for Hire
status: in-game
given_by:
- '[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2411-beasts-for-hire.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2411-01, qz01-001/2411-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Beasts for Hire

Use the Jelly Bean capsule Lampa gives you if you like, then talk to him again.

Lesson 11 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]] offers it after [[quests/2410-stronger-together|Stronger Together]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2411-01`

Happens by talking to [[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]].

Checks:

- quest switch 649 is on (you finished [[quests/2410-stronger-together|Stronger Together]])
- quest switch 650 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2411-beasts-for-hire|Beasts for Hire]]
- you get 1 × [[items/consumable/401-jelly-bean|Jelly Bean]]

### `2411-02`

Happens by talking to [[npcs/1016-livestock-farmer-lampa|Livestock Farmer Lampa]].

Checks:

- you have [[quests/2411-beasts-for-hire|Beasts for Hire]]

Then:

- works on [[quests/2411-beasts-for-hire|Beasts for Hire]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 650 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
