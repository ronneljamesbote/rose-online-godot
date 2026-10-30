---
kind: quest
id: 2112
name: 'Light for Luxem'
status: in-game
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
level: 18
steps: 5
source:
  data: 'godot/data/quests/2112-light-for-luxem.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2112-02, ql21-001/2112-21, ql21-001/2112-22, ql21-001/2112-23, ql21-001/2112-03'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Light for Luxem

Take the Lens Core up the Processional Way to Lutis at the top of Luxem Tower.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

You get it at once when you finish [[quests/2111-the-warden-below|The Warden Below]].

- **Start:** Take the Lens Core to Lutis at the top of Luxem Tower.
- **End:** The tower's crystal shines again.

When you hand it in you pick one reward: [[items/body/651-luxem-guard-armor|Luxem Guard Armor]], [[items/body/652-luxem-acolyte-vest|Luxem Acolyte Vest]], [[items/body/653-luxem-stalker-chest|Luxem Stalker Chest]], [[items/body/654-luxem-broker-vest|Luxem Broker Vest]]. Each pick is its own step below; the steps are the same apart from the item.

The pieces are the Luxem armour set, one piece for each quest of the chain (see the zone page). You pick the job version you want; it doesn't have to be your job, so a Visitor can choose before changing job. Each piece can only be worn by its job.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2112-02`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]] and picking the Luxem Guard Armor.

Checks:

- you have [[quests/2112-light-for-luxem|Light for Luxem]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/body/651-luxem-guard-armor|Luxem Guard Armor]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 611 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2117-on-to-zant|On to Zant]]
- works on [[quests/2117-on-to-zant|On to Zant]]

### `2112-21`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]] and picking the Luxem Acolyte Vest.

Checks:

- you have [[quests/2112-light-for-luxem|Light for Luxem]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/body/652-luxem-acolyte-vest|Luxem Acolyte Vest]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 611 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2117-on-to-zant|On to Zant]]
- works on [[quests/2117-on-to-zant|On to Zant]]

### `2112-22`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]] and picking the Luxem Stalker Chest.

Checks:

- you have [[quests/2112-light-for-luxem|Light for Luxem]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/body/653-luxem-stalker-chest|Luxem Stalker Chest]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 611 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2117-on-to-zant|On to Zant]]
- works on [[quests/2117-on-to-zant|On to Zant]]

### `2112-23`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]] and picking the Luxem Broker Vest.

Checks:

- you have [[quests/2112-light-for-luxem|Light for Luxem]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/body/654-luxem-broker-vest|Luxem Broker Vest]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 611 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2117-on-to-zant|On to Zant]]
- works on [[quests/2117-on-to-zant|On to Zant]]

### `2112-03`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]] after the quest.

Checks:

- quest switch 611 = 1 (you finished [[quests/2112-light-for-luxem|Light for Luxem]])
- quest switch 612 = 0

Then:

- experience, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 612 to 1

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
