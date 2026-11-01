---
kind: quest
id: 2404
name: Potions and Scrolls
status: in-game
given_by:
- '[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2404-potions-and-scrolls.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2404-01, qz01-001/2404-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Potions and Scrolls

Put the potions Tryteh gives you on your hotbar, then talk to him again.

Lesson 4 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]] offers it after [[quests/2403-safe-keeping|Safe Keeping]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2404-01`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- quest switch 642 is on (you finished [[quests/2403-safe-keeping|Safe Keeping]])
- quest switch 643 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2404-potions-and-scrolls|Potions and Scrolls]]
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- you get 5 × [[items/consumable/21-mana-vial-s|Mana Vial (S)]]
- you get 1 × [[items/consumable/351-zant-return-scroll|Zant Return Scroll]]

### `2404-02`

Happens by talking to [[npcs/1006-arumic-merchant-tryteh|Arumic Merchant Tryteh]].

Checks:

- you have [[quests/2404-potions-and-scrolls|Potions and Scrolls]]

Then:

- works on [[quests/2404-potions-and-scrolls|Potions and Scrolls]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 643 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
