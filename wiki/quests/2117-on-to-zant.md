---
kind: quest
id: 2117
name: 'On to Zant'
status: in-game
npcs:
- '[[npcs/1014-guide-lena|Guide Lena]]'
level: 18
steps: 1
source:
  data: 'godot/data/quests/2117-on-to-zant.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2117-02'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# On to Zant

Lutis told you your road goes on through the north-west gate of the Valley of Luxem Tower to Zant, the Canyon City. Find Lena, the guide, in Zant.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

You get it at once when you finish [[quests/2112-light-for-luxem|Light for Luxem]].

- **Start:** Go through the north-west gate to Zant, the Canyon City, and find Lena the guide.
- **End:** Lena welcomed you to Zant.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2117-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/2117-on-to-zant|On to Zant]]

Then:

- works on [[quests/2117-on-to-zant|On to Zant]]
- experience, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 617 to 1
- the quest ends (removed from your list)

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
