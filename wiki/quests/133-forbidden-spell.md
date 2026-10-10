---
kind: quest
id: 133
name: Forbidden Spell
status: in-game
given_by:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
npcs:
- '[[npcs/1082-guide-eva|Guide Eva]]'
steps: 3
source:
  data: LIST_QUEST.STB row 133; QSD triggers 132-01, 132-02, 133-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forbidden Spell

In the end, you weren't able to dissuade the stubborn Shroon. You'd better meet with Eva and see how she's doing.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `132-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/132-forbidden-spell|Forbidden Spell]]

Then:

- works on [[quests/132-forbidden-spell|Forbidden Spell]]
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/133-forbidden-spell|Forbidden Spell]]
- set episode variable 0 to 32

### `132-02`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- episode variable 0 = 32

Then:

- you get the quest [[quests/133-forbidden-spell|Forbidden Spell]]

### `133-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/133-forbidden-spell|Forbidden Spell]]

Then:

- works on [[quests/133-forbidden-spell|Forbidden Spell]]
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/134-the-scheme|The Scheme]]
- set episode variable 0 to 33
