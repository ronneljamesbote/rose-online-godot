---
kind: quest
id: 2102
name: 'Pumpkins on the Road'
status: in-game
given_by:
- '[[npcs/1056-hunter-bekk|Hunter Bekk]]'
level: 9
steps: 4
source:
  data: 'godot/data/quests/2102-pumpkins-on-the-road.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2102-01, ql21-001/2102-02, ql21-001/2102-31, ql21-001/2102-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Pumpkins on the Road

Bekk wants the Pumpkins and Big Pumpkins cleared from the Caravan Road. Bring him 10 Pumpkin Stalks.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1056-hunter-bekk|Hunter Bekk]] offers it from level 9, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Clear Pumpkins and Big Pumpkins and bring Bekk 10 Pumpkin Stalks.
- **End:** Bekk counted the stalks and nodded.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2102-01`

Happens by talking to [[npcs/1056-hunter-bekk|Hunter Bekk]].

Checks:

- your Level ≥ 9
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 601 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]

### `2102-02`

Happens by talking to [[npcs/1056-hunter-bekk|Hunter Bekk]].

Checks:

- you have [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- you carry ≥ 10 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]]

Then:

- works on [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- 10 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]] is taken
- experience, base 600 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 600 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- set quest switch 601 to 1
- the quest ends (removed from your list)

### `2102-31`

Happens by killing [[monsters/21-pumpkin|Pumpkin]].

Checks:

- you have [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- a random roll 0–99 lands in 0–59
- you carry < 10 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]]

Then:

- works on [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- you get 1 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]]

### `2102-32`

Happens by killing [[monsters/22-big-pumpkin|Big Pumpkin]].

Checks:

- you have [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- a random roll 0–99 lands in 0–59
- you carry < 10 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]]

Then:

- works on [[quests/2102-pumpkins-on-the-road|Pumpkins on the Road]]
- you get 1 × [[items/quest/701-pumpkin-stalk|Pumpkin Stalk]]

Whether or not it passes, step `2116-31` runs next.

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
