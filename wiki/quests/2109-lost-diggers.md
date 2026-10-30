---
kind: quest
id: 2109
name: 'Lost Diggers'
status: in-game
given_by:
- '[[npcs/1059-archaeologist-pell|Archaeologist Pell]]'
npcs:
- '[[npcs/1066-digger-hob|Digger Hob]]'
- '[[npcs/1067-digger-wren|Digger Wren]]'
- '[[npcs/1068-digger-fenn|Digger Fenn]]'
level: 15
steps: 5
source:
  data: 'godot/data/quests/2109-lost-diggers.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2109-01, ql21-001/2109-02, ql21-001/2109-03, ql21-001/2109-04, ql21-001/2109-05'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lost Diggers

Three of Pell's diggers didn't come back from the ruins. Find Hob at the pyramid dig, Wren at the shattered monument and Fenn on the Processional Way.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1059-archaeologist-pell|Archaeologist Pell]] offers it from level 15, after [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]].

- **Start:** Find the three diggers: Hob at the pyramid, Wren at the shattered monument, Fenn on the Processional Way.
- **End:** All three diggers are safe.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2109-01`

Happens by talking to [[npcs/1059-archaeologist-pell|Archaeologist Pell]].

Checks:

- your Level ≥ 15
- quest switch 606 = 1 (you finished [[quests/2107-a-shard-for-the-expedition|A Shard for the Expedition]])
- quest switch 608 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2109-lost-diggers|Lost Diggers]]

### `2109-02`

Happens by talking to [[npcs/1059-archaeologist-pell|Archaeologist Pell]].

Checks:

- you have [[quests/2109-lost-diggers|Lost Diggers]]
- quest variable 0 = 1 (you talked to [[npcs/1066-digger-hob|Digger Hob]])
- quest variable 1 = 1 (you talked to [[npcs/1067-digger-wren|Digger Wren]])
- quest variable 2 = 1 (you talked to [[npcs/1068-digger-fenn|Digger Fenn]])

Then:

- works on [[quests/2109-lost-diggers|Lost Diggers]]
- experience, base 2600 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 2200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 608 to 1
- the quest ends (removed from your list)

### `2109-03`

Happens by talking to [[npcs/1066-digger-hob|Digger Hob]].

Checks:

- you have [[quests/2109-lost-diggers|Lost Diggers]]
- quest variable 0 = 0

Then:

- works on [[quests/2109-lost-diggers|Lost Diggers]]
- set quest variable 0 to 1

### `2109-04`

Happens by talking to [[npcs/1067-digger-wren|Digger Wren]].

Checks:

- you have [[quests/2109-lost-diggers|Lost Diggers]]
- quest variable 1 = 0

Then:

- works on [[quests/2109-lost-diggers|Lost Diggers]]
- set quest variable 1 to 1

### `2109-05`

Happens by talking to [[npcs/1068-digger-fenn|Digger Fenn]].

Checks:

- you have [[quests/2109-lost-diggers|Lost Diggers]]
- quest variable 2 = 0

Then:

- works on [[quests/2109-lost-diggers|Lost Diggers]]
- set quest variable 2 to 1

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
