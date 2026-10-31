---
kind: quest
id: 2209
name: The Jelly King
status: in-game
given_by:
- '[[npcs/1041-warden-corvel|Warden Corvel]]'
level: 9
steps: 3
source:
  data: 'godot/data/quests/2209-the-jelly-king.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2209-01, qa22-001/2209-02, qa22-001/2209-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Jelly King

Corvel wants the Jelly King driven out of its hollow at Bramble Ford. Bring back its crown. A party helps.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1041-warden-corvel|Warden Corvel]] offers it from level 9, after [[quests/2208-poisoned-seed|Poisoned Seed]].

- **Start:** Defeat the Jelly King in its hollow at Bramble Ford and bring Corvel its crown.
- **End:** The Jelly King is beaten, for now.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2209-01`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- quest switch 630 is on (you finished [[quests/2208-poisoned-seed|Poisoned Seed]])
- your Level ≥ 9
- quest switch 631 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2209-the-jelly-king|The Jelly King]]

### `2209-02`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/2209-the-jelly-king|The Jelly King]]
- you carry ≥ 1 × [[items/quest/718-jelly-king-s-crown|Jelly King's Crown]]

Then:

- works on [[quests/2209-the-jelly-king|The Jelly King]]
- 1 × [[items/quest/718-jelly-king-s-crown|Jelly King's Crown]] is taken
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/consumable/351-zant-return-scroll|Zant Return Scroll]]
- you get 1 × [[items/body/655-riverford-coat|Riverford Coat]]
- quest switch 631 on
- the quest ends (removed from your list)

### `2209-31`

Happens by killing [[monsters/203-jelly-king|Jelly King]]: its kill step `952-33` ([[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]) runs it next, so a kill counts for both quests.

Checks:

- you have [[quests/2209-the-jelly-king|The Jelly King]]
- you carry < 1 × [[items/quest/718-jelly-king-s-crown|Jelly King's Crown]]

Then:

- works on [[quests/2209-the-jelly-king|The Jelly King]]
- you get 1 × [[items/quest/718-jelly-king-s-crown|Jelly King's Crown]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
