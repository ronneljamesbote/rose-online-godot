---
kind: quest
id: 102
name: Dreadful Epidemic
status: in-game
given_by:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
steps: 4
source:
  data: LIST_QUEST.STB row 102; QSD triggers 101-02, 102-01, 102-03, 102-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dreadful Epidemic

Seyon said that the mushrooms started to rot after a group of suspicious people garbed in red were camping near them. Let's search the area around the mushrooms for clues.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `101-02`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/101-dreadful-epidemic|Dreadful Epidemic]]

Then:

- works on [[quests/101-dreadful-epidemic|Dreadful Epidemic]]
- the quest becomes [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- set episode variable 0 to 1

### `102-01`

Checks:

- you have [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- the quest becomes [[quests/103-mysterious-emblem|Mysterious Emblem]] (progress kept)

### `102-03`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- episode variable 0 = 1

Then:

- you get the quest [[quests/102-dreadful-epidemic|Dreadful Epidemic]]

### `102-04`

Checks:

- you have [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- client script `mushroom`

If the checks fail, step `102-05` is tried instead.
