---
kind: quest
id: 2405
name: Dressed for the Road
status: in-game
given_by:
- '[[npcs/1009-armor-seller-carrion|Armor Seller Carrion]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2405-dressed-for-the-road.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2405-01, qz01-001/2405-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dressed for the Road

Look at Carrion's armour shop (buying, selling, armour slots), then talk to him again.

Lesson 5 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1009-armor-seller-carrion|Armor Seller Carrion]] offers it after [[quests/2404-potions-and-scrolls|Potions and Scrolls]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2405-01`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- quest switch 643 is on (you finished [[quests/2404-potions-and-scrolls|Potions and Scrolls]])
- quest switch 644 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2405-dressed-for-the-road|Dressed for the Road]]

### `2405-02`

Happens by talking to [[npcs/1009-armor-seller-carrion|Armor Seller Carrion]].

Checks:

- you have [[quests/2405-dressed-for-the-road|Dressed for the Road]]

Then:

- works on [[quests/2405-dressed-for-the-road|Dressed for the Road]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 644 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
