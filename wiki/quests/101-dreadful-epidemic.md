---
kind: quest
id: 101
name: Dreadful Epidemic
status: in-game
given_by:
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
npcs:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
steps: 2
source:
  data: LIST_QUEST.STB row 101; QSD triggers 101-01, 101-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dreadful Epidemic

Myad recently told you about the epidemic in the village, and said that Seyon might be able to help.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `101-01`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- episode variable 0 = 0

Then:

- you get the quest [[quests/101-dreadful-epidemic|Dreadful Epidemic]]

### `101-02`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/101-dreadful-epidemic|Dreadful Epidemic]]

Then:

- works on [[quests/101-dreadful-epidemic|Dreadful Epidemic]]
- the quest becomes [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- set episode variable 0 to 1
