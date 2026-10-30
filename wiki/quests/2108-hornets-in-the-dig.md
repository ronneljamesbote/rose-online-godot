---
kind: quest
id: 2108
name: 'Hornets in the Dig'
status: in-game
given_by:
- '[[npcs/1060-guard-captain-torvin|Guard Captain Torvin]]'
level: 15
steps: 7
source:
  data: 'godot/data/quests/2108-hornets-in-the-dig.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2108-01, ql21-001/2108-02, ql21-001/2108-21, ql21-001/2108-22, ql21-001/2108-23, ql21-001/2108-31, ql21-001/2108-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hornets in the Dig

Hornets have nested in the stonework of the ruins and keep the diggers away. Bring Torvin 8 Hornet Wings.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1060-guard-captain-torvin|Guard Captain Torvin]] offers it from level 15, after [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]].

- **Start:** Clear the Hornets from the sunken ruins and bring Torvin 8 Hornet Wings.
- **End:** Torvin's diggers can work again.

When you hand it in you pick one reward: [[items/feet/651-luxem-guard-boots|Luxem Guard Boots]], [[items/feet/652-luxem-acolyte-shoes|Luxem Acolyte Shoes]], [[items/feet/653-luxem-stalker-boots|Luxem Stalker Boots]], [[items/feet/654-luxem-broker-shoes|Luxem Broker Shoes]]. Each pick is its own step below; the steps are the same apart from the item.

The pieces are the Luxem armour set, one piece for each quest of the chain (see the zone page). You pick the job version you want; it doesn't have to be your job, so a Visitor can choose before changing job. Each piece can only be worn by its job.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2108-01`

Happens by talking to [[npcs/1060-guard-captain-torvin|Guard Captain Torvin]].

Checks:

- your Level ≥ 15
- quest switch 606 = 1 (you finished [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]])
- quest switch 607 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]

### `2108-02`

Happens by talking to [[npcs/1060-guard-captain-torvin|Guard Captain Torvin]] and picking the Luxem Guard Boots.

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you carry ≥ 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/feet/651-luxem-guard-boots|Luxem Guard Boots]]
- 8 × [[items/quest/705-hornet-wing|Hornet Wing]] is taken
- experience, base 2200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 607 to 1
- the quest ends (removed from your list)

### `2108-21`

Happens by talking to [[npcs/1060-guard-captain-torvin|Guard Captain Torvin]] and picking the Luxem Acolyte Shoes.

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you carry ≥ 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/feet/652-luxem-acolyte-shoes|Luxem Acolyte Shoes]]
- 8 × [[items/quest/705-hornet-wing|Hornet Wing]] is taken
- experience, base 2200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 607 to 1
- the quest ends (removed from your list)

### `2108-22`

Happens by talking to [[npcs/1060-guard-captain-torvin|Guard Captain Torvin]] and picking the Luxem Stalker Boots.

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you carry ≥ 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/feet/653-luxem-stalker-boots|Luxem Stalker Boots]]
- 8 × [[items/quest/705-hornet-wing|Hornet Wing]] is taken
- experience, base 2200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 607 to 1
- the quest ends (removed from your list)

### `2108-23`

Happens by talking to [[npcs/1060-guard-captain-torvin|Guard Captain Torvin]] and picking the Luxem Broker Shoes.

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you carry ≥ 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/feet/654-luxem-broker-shoes|Luxem Broker Shoes]]
- 8 × [[items/quest/705-hornet-wing|Hornet Wing]] is taken
- experience, base 2200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 607 to 1
- the quest ends (removed from your list)

### `2108-31`

Happens by killing [[monsters/63-needle-hornet|Needle Hornet]].

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- a random roll 0–99 lands in 0–59
- you carry < 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/quest/705-hornet-wing|Hornet Wing]]

### `2108-32`

Happens by killing [[monsters/64-hornet|Hornet]].

Checks:

- you have [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- a random roll 0–99 lands in 0–69
- you carry < 8 × [[items/quest/705-hornet-wing|Hornet Wing]]

Then:

- works on [[quests/2108-hornets-in-the-dig|Hornets in the Dig]]
- you get 1 × [[items/quest/705-hornet-wing|Hornet Wing]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
