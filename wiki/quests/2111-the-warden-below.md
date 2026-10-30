---
kind: quest
id: 2111
name: 'The Warden Below'
status: in-game
given_by:
- '[[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]]'
npcs:
- '[[npcs/1070-lift-keeper-gus|Lift Keeper Gus]]'
level: 18
steps: 8
source:
  data: 'godot/data/quests/2111-the-warden-below.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2111-01, ql21-001/2111-02, ql21-001/2111-21, ql21-001/2111-22, ql21-001/2111-23, ql21-001/2111-31, ql21-001/2111-03, ql21-001/2111-04'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Warden Below

A sealed vault lies under the dig, guarded by the Ruin Warden. Ride the crane down, defeat the Ruin Warden and bring Ardeth its Lens Core.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] offers it from level 18, after [[quests/2108-hornets-in-the-dig|Hornets in the Dig]] and [[quests/2110-words-in-stone|Words in Stone]].

- **Start:** Ride Ardeth's crane into the sealed vault, defeat the Ruin Warden and bring back the Lens Core.
- **End:** Ardeth holds the Lens Core.

When you hand it in you pick one reward: [[items/head/831-luxem-guard-helm|Luxem Guard Helm]], [[items/head/832-luxem-acolyte-beret|Luxem Acolyte Beret]], [[items/head/833-luxem-stalker-hat|Luxem Stalker Hat]], [[items/head/834-luxem-broker-turban|Luxem Broker Turban]]. Each pick is its own step below; the steps are the same apart from the item.

The pieces are the Luxem armour set, one piece for each quest of the chain (see the zone page). You pick the job version you want; it doesn't have to be your job, so a Visitor can choose before changing job. Each piece can only be worn by its job.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2111-01`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]].

Checks:

- your Level ≥ 18
- quest switch 607 = 1 (you finished [[quests/2108-hornets-in-the-dig|Hornets in the Dig]])
- quest switch 609 = 1 (you finished [[quests/2110-words-in-stone|Words in Stone]])
- quest switch 610 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2111-the-warden-below|The Warden Below]]

### `2111-02`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] and picking the Luxem Guard Helm.

Checks:

- you have [[quests/2111-the-warden-below|The Warden Below]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2111-the-warden-below|The Warden Below]]
- you get 1 × [[items/head/831-luxem-guard-helm|Luxem Guard Helm]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 4500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 4000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 610 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2112-light-for-luxem|Light for Luxem]]
- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/quest/707-lens-core|Lens Core]]

### `2111-21`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] and picking the Luxem Acolyte Beret.

Checks:

- you have [[quests/2111-the-warden-below|The Warden Below]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2111-the-warden-below|The Warden Below]]
- you get 1 × [[items/head/832-luxem-acolyte-beret|Luxem Acolyte Beret]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 4500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 4000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 610 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2112-light-for-luxem|Light for Luxem]]
- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/quest/707-lens-core|Lens Core]]

### `2111-22`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] and picking the Luxem Stalker Hat.

Checks:

- you have [[quests/2111-the-warden-below|The Warden Below]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2111-the-warden-below|The Warden Below]]
- you get 1 × [[items/head/833-luxem-stalker-hat|Luxem Stalker Hat]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 4500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 4000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 610 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2112-light-for-luxem|Light for Luxem]]
- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/quest/707-lens-core|Lens Core]]

### `2111-23`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] and picking the Luxem Broker Turban.

Checks:

- you have [[quests/2111-the-warden-below|The Warden Below]]
- you carry ≥ 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2111-the-warden-below|The Warden Below]]
- you get 1 × [[items/head/834-luxem-broker-turban|Luxem Broker Turban]]
- 1 × [[items/quest/707-lens-core|Lens Core]] is taken
- experience, base 4500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 4000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 610 to 1
- the quest ends (removed from your list)
- you get the quest [[quests/2112-light-for-luxem|Light for Luxem]]
- works on [[quests/2112-light-for-luxem|Light for Luxem]]
- you get 1 × [[items/quest/707-lens-core|Lens Core]]

### `2111-31`

Happens by killing [[monsters/77-ruin-warden|Ruin Warden]].

Checks:

- you have [[quests/2111-the-warden-below|The Warden Below]]
- you carry < 1 × [[items/quest/707-lens-core|Lens Core]]

Then:

- works on [[quests/2111-the-warden-below|The Warden Below]]
- you get 1 × [[items/quest/707-lens-core|Lens Core]]

### `2111-03`

Happens by asking [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]] to lower you into the vault (once [[quests/2110-words-in-stone|Words in Stone]] is done).

Checks:

- your Level ≥ 16

Then:

- you are moved to (5218, 5544) in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]

### `2111-04`

Happens by asking [[npcs/1070-lift-keeper-gus|Lift Keeper Gus]] to take you back up.

Then:

- you are moved to (5248, 5450) in [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
