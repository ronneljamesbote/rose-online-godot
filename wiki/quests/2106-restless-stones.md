---
kind: quest
id: 2106
name: 'Restless Stones'
status: in-game
given_by:
- '[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]]'
level: 13
steps: 5
source:
  data: 'godot/data/quests/2106-restless-stones.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2106-01, ql21-001/2106-02, ql21-001/2106-31, ql21-001/2106-32, ql21-001/2106-33'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Restless Stones

Shannon says the Pomics are crowding the last of the water. Bring her a mana-stained Pomic shard.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]] offers it from level 13, after [[quests/2105-stolen-supplies|Stolen Supplies]].

- **Start:** Bring Shannon a Mana-stained Pomic Shard from the Pomics by the water.
- **End:** Shannon sent you on to the Arumic camp with the shard.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2106-01`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- your Level ≥ 13
- quest switch 604 = 1 (you finished [[quests/2105-stolen-supplies|Stolen Supplies]])
- quest switch 605 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2106-restless-stones|Restless Stones]]

### `2106-02`

Happens by talking to [[npcs/1052-mountain-guide-shannon|Mountain Guide Shannon]].

Checks:

- you have [[quests/2106-restless-stones|Restless Stones]]
- you carry ≥ 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Then:

- works on [[quests/2106-restless-stones|Restless Stones]]
- 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]] is taken
- experience, base 1200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 605 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]]
- works on [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]]
- you get 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

### `2106-31`

Happens by killing [[monsters/71-needle-pomic|Needle Pomic]].

Checks:

- you have [[quests/2106-restless-stones|Restless Stones]]
- a random roll 0–99 lands in 0–19
- you carry < 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Then:

- works on [[quests/2106-restless-stones|Restless Stones]]
- you get 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Whether or not it passes, step `2114-31` runs next.

### `2106-32`

Happens by killing [[monsters/72-pomic|Pomic]].

Checks:

- you have [[quests/2106-restless-stones|Restless Stones]]
- a random roll 0–99 lands in 0–19
- you carry < 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Then:

- works on [[quests/2106-restless-stones|Restless Stones]]
- you get 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Whether or not it passes, step `2114-32` runs next.

### `2106-33`

Happens by killing [[monsters/73-red-pomic|Red Pomic]].

Checks:

- you have [[quests/2106-restless-stones|Restless Stones]]
- a random roll 0–99 lands in 0–19
- you carry < 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Then:

- works on [[quests/2106-restless-stones|Restless Stones]]
- you get 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
