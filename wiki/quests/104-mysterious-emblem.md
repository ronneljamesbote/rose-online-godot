---
kind: quest
id: 104
name: Mysterious Emblem
status: in-game
given_by:
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
npcs:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
- '[[npcs/1041-warden-corvel|Warden Corvel]]'
steps: 4
source:
  data: 'godot/data/quests/104-mysterious-emblem.tres; quest triggers in godot/data/quest_triggers/: qp103/103-01, qp103/103-02, qp103/104-01, qa22-001/104-21'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Mysterious Emblem

Just like Seyon said, those suspicious strangers were also seen by Myad. Myad gave you a pair of Riverford Boots and told you to take the emblem to Warden Corvel at Riverford Post.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `103-01`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]], talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/103-mysterious-emblem|Mysterious Emblem]]
- you carry = 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/103-mysterious-emblem|Mysterious Emblem]]
- experience, base 200 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/104-mysterious-emblem|Mysterious Emblem]] (progress kept)
- set episode variable 0 to 2
- you get 1 × [[items/feet/655-riverford-boots|Riverford Boots]]

### `103-02`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- episode variable 0 = 2

Then:

- you get the quest [[quests/104-mysterious-emblem|Mysterious Emblem]]
- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

### `104-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/104-mysterious-emblem|Mysterious Emblem]]

Then:

- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]] is taken
- the quest becomes [[quests/105-healing-hands|Healing Hands]] (progress kept)
- set episode variable 0 to 3

### `104-21`

Happens by talking to [[npcs/1041-warden-corvel|Warden Corvel]].

Checks:

- you have [[quests/104-mysterious-emblem|Mysterious Emblem]]

Then:

- works on [[quests/104-mysterious-emblem|Mysterious Emblem]]
- set episode variable 0 to 3
- the quest ends (removed from your list)

You keep the emblem when Corvel takes the quest: [[quests/2210-over-the-ridge|Over the Ridge]]
later sends it on to Dorran in the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]].
Karitte's step `104-01` still works, for a character who goes to her first.

## Changed from iROSE

In iROSE the quest ends at Karitte on Luxem Tower. Here Myad sends you to
[[npcs/1041-warden-corvel|Warden Corvel]] at Riverford Post in the
[[zones/22-adventurer-s-plain|Adventurer's Plain]] instead (step `104-21`, new), so the
story of the plain carries on to [[quests/2205-ford-watch|Ford Watch]]. Myad also gives
[[items/feet/655-riverford-boots|Riverford Boots]] at step `103-01`.
