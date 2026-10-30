---
kind: quest
id: 2112
name: 'Light for Luxem'
status: in-game
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
level: 18
steps: 2
source:
  data: 'godot/data/quests/2112-light-for-luxem.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2112-02, ql21-001/2112-03'
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

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2112-02`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/2112-light-for-luxem|Light for Luxem]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 1 × [[items/head/8-rodeo-hat|Rodeo Hat]]
- set quest switch 611 to 1
- the quest ends (removed from your list)

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
