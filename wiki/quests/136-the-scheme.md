---
kind: quest
id: 136
name: The Scheme
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
npcs:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 3
source:
  data: LIST_QUEST.STB row 136; QSD triggers 135-01, 135-02, 136-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Scheme

Eva says that the Red Heart is the medium for Shroon's spell. While Eva deals with the spell, you've got to try to talk to Shroon again.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `135-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 0 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 1 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/135-the-scheme|The Scheme]]
- 1 × [[items/quest/619-scarlet-heart|Scarlet Heart]] is taken
- the quest becomes [[quests/136-the-scheme|The Scheme]] (progress kept)
- set episode variable 0 to 35

### `135-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 35

Then:

- you get the quest [[quests/136-the-scheme|The Scheme]]

### `136-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/136-the-scheme|The Scheme]]

Then:

- works on [[quests/136-the-scheme|The Scheme]]
- experience, base 70000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/137-the-road-to-justice|The Road to Justice]]
- set episode variable 0 to 36
