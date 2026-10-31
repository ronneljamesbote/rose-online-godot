---
kind: quest
id: 2407
name: Sharper Steel
status: in-game
given_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2407-sharper-steel.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2407-01, qz01-001/2407-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Sharper Steel

Have Raffle refine a Practice Sword to grade 1 with the materials he gives you, then talk to him again.

Lesson 7 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]] offers it after [[quests/2406-mending-steel|Mending Steel]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2407-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- quest switch 645 is on (you finished [[quests/2406-mending-steel|Mending Steel]])
- quest switch 646 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2407-sharper-steel|Sharper Steel]]
- you get 1 × [[items/weapon/1133-practice-sword|Practice Sword]]
- you get 5 × [[items/material/71-gray-powder|Gray Powder]]
- you get 7 × [[items/material/61-low-essence|Low Essence]]

### `2407-02`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/2407-sharper-steel|Sharper Steel]]

Then:

- works on [[quests/2407-sharper-steel|Sharper Steel]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 646 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
