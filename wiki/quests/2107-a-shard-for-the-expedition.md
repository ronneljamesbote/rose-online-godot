---
kind: quest
id: 2107
name: 'A Shard for the Expedition'
status: in-game
npcs:
- '[[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]]'
level: 13
steps: 1
source:
  data: 'godot/data/quests/2107-a-shard-for-the-expedition.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2107-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# A Shard for the Expedition

Shannon sent you to Ardeth, the leader of the Arumic Expedition Camp on the East Terrace, with the mana-stained shard.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

You get it at once when you finish [[quests/2106-restless-stones|Restless Stones]].

- **Start:** Take the Mana-stained Pomic Shard to Ardeth at the Arumic Expedition Camp.
- **End:** Ardeth took the shard and let you help the expedition.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2107-02`

Happens by talking to [[npcs/1058-expedition-leader-ardeth|Expedition Leader Ardeth]].

Checks:

- you have [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]]
- you carry ≥ 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]]

Then:

- works on [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]]
- 1 × [[items/quest/704-mana-stained-pomic-shard|Mana-stained Pomic Shard]] is taken
- experience, base 1500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 1200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 606 to 1
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
