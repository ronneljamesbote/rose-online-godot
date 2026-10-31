---
kind: quest
id: 2205
name: Ford Watch
status: in-game
given_by:
- '[[npcs/1041-warden-corvel|Warden Corvel]]'
monsters:
- '[[monsters/5-red-jelly-bean|Red Jelly Bean]]'
level: 5
steps: 6
source:
  data: 'godot/data/quests/2205-ford-watch.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2205-01, qa22-001/2205-02, qa22-001/2205-21, qa22-001/2205-22, qa22-001/2205-23, qa22-001/2205-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Ford Watch

Warden Corvel wants the Red Jelly Beans cleared from the Riverford bridges. Bring him 8 Red Jelly Drops.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1041-warden-corvel|Warden Corvel]] offers it from level 5, after [[quests/104-mysterious-emblem|Mysterious Emblem]].

- **Start:** Bring Corvel 8 Red Jelly Drops from the Red Jelly Beans by the bridges.
- **End:** The bridges are clear again.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2205-01`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- episode variable 0 ≥ 3 (you finished [[quests/104-mysterious-emblem|Mysterious Emblem]])
- your Level ≥ 5
- quest switch 627 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2205-ford-watch|Ford Watch]]

### `2205-02`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/2205-ford-watch|Ford Watch]]
- you carry ≥ 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

Then:

- works on [[quests/2205-ford-watch|Ford Watch]]
- 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]] is taken
- experience, base 160 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/21-mana-vial-s|Mana Vial (S)]]
- you get 1 × [[items/weapon/1129-riverford-sword|Riverford Sword]]
- quest switch 627 on
- the quest ends (removed from your list)

### `2205-21`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/2205-ford-watch|Ford Watch]]
- you carry ≥ 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

Then:

- works on [[quests/2205-ford-watch|Ford Watch]]
- 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]] is taken
- experience, base 160 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/21-mana-vial-s|Mana Vial (S)]]
- you get 1 × [[items/weapon/1130-riverford-spear|Riverford Spear]]
- quest switch 627 on
- the quest ends (removed from your list)

### `2205-22`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/2205-ford-watch|Ford Watch]]
- you carry ≥ 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

Then:

- works on [[quests/2205-ford-watch|Ford Watch]]
- 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]] is taken
- experience, base 160 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/21-mana-vial-s|Mana Vial (S)]]
- you get 1 × [[items/weapon/1131-riverford-bow|Riverford Bow]]
- you get 300 × [[items/material/301-wooden-arrow|Wooden Arrow]]
- quest switch 627 on
- the quest ends (removed from your list)

### `2205-23`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/2205-ford-watch|Ford Watch]]
- you carry ≥ 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

Then:

- works on [[quests/2205-ford-watch|Ford Watch]]
- 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]] is taken
- experience, base 160 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/21-mana-vial-s|Mana Vial (S)]]
- you get 1 × [[items/weapon/1132-riverford-gun|Riverford Gun]]
- you get 300 × [[items/material/321-bean-bullet|Bean Bullet]]
- quest switch 627 on
- the quest ends (removed from your list)

### `2205-31`

Happens by killing [[monsters/5-red-jelly-bean|Red Jelly Bean]].

Checks:

- you have [[quests/2205-ford-watch|Ford Watch]]
- a random roll 0–99 lands in 0–70
- you carry < 8 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

Then:

- works on [[quests/2205-ford-watch|Ford Watch]]
- you get 1 × [[items/quest/715-red-jelly-drop|Red Jelly Drop]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
