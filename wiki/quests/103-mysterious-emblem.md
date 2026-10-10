---
kind: quest
id: 103
name: Mysterious Emblem
status: in-game
npcs:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
steps: 2
source:
  data: LIST_QUEST.STB row 103; QSD triggers 102-01, 103-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Mysterious Emblem

You discovered this peculiar emblem near the mushrooms. You better show this to Myad.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `102-01`

Checks:

- you have [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you carry < 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]

Then:

- works on [[quests/102-dreadful-epidemic|Dreadful Epidemic]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- the quest becomes [[quests/103-mysterious-emblem|Mysterious Emblem]] (progress kept)

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
