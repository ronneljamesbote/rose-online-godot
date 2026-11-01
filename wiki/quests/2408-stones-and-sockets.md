---
kind: quest
id: 2408
name: Stones and Sockets
status: in-game
given_by:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
level: 15
steps: 2
source:
  data: 'godot/data/quests/2408-stones-and-sockets.tres; quest triggers in godot/data/quest_triggers/: qz01-001/2408-01, qz01-001/2408-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Stones and Sockets

Wear the Practice Ring, set the Practice Stone in its socket, then talk to Mina again.

Lesson 8 of 12 of the Zant city tour; see "Lessons" on the [[zones/1-canyon-city-of-zant|Canyon City of Zant]] page for the whole chain.

[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]] offers it after [[quests/2407-sharper-steel|Sharper Steel]]. Nothing in the lesson is checked: talking to the NPC again finishes it.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2408-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- quest switch 646 is on (you finished [[quests/2407-sharper-steel|Sharper Steel]])
- quest switch 647 is off (you haven't done this lesson yet)

Then:

- you get the quest [[quests/2408-stones-and-sockets|Stones and Sockets]]
- you get [[items/jewellery/253-practice-ring|Practice Ring]], made the way a crafted item is (it always has a socket)
- you get 1 × [[items/gem/371-practice-stone|Practice Stone]]

### `2408-02`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/2408-stones-and-sockets|Stones and Sockets]]

Then:

- works on [[quests/2408-stones-and-sockets|Stones and Sockets]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 647 on
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Zant city hub redesign.
