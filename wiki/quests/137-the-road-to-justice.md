---
kind: quest
id: 137
name: The Road to Justice
status: in-game
given_by:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
npcs:
- '[[npcs/1082-guide-eva|Guide Eva]]'
steps: 3
source:
  data: LIST_QUEST.STB row 137; QSD triggers 136-01, 136-02, 137-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Road to Justice

Shroon seems angered that you've foiled his plans. Although you've won this time, who knows what Shroon may do in the future. For now, you'd better meet Eva.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `136-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/136-the-scheme|The Scheme]]

Then:

- works on [[quests/136-the-scheme|The Scheme]]
- experience, base 70000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/137-the-road-to-justice|The Road to Justice]]
- set episode variable 0 to 36

### `136-02`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- episode variable 0 = 36

Then:

- you get the quest [[quests/137-the-road-to-justice|The Road to Justice]]

### `137-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/137-the-road-to-justice|The Road to Justice]]

Then:

- works on [[quests/137-the-road-to-justice|The Road to Justice]]
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/138-the-road-to-justice|The Road to Justice]]
- set episode variable 0 to 37
