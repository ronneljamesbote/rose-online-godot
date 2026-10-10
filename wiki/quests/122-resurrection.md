---
kind: quest
id: 122
name: Resurrection
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
monsters:
- '[[monsters/253-zombie|Zombie]]'
steps: 7
source:
  data: LIST_QUEST.STB row 122; QSD triggers 121-01, 121-02, 122-01, 122-02, 122-03, 122-04, 122-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Resurrection

You finally have the secret medicine! But there's something suspicious about being asked to sprinkle it around the pyramid in the El Verloon Desert to revive Alex…  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `121-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/121-banned-recipe|Banned Recipe]]
- you carry ≥ 5 × [[items/quest/610-five-color-scale|Five-Color Scale]]

Then:

- works on [[quests/121-banned-recipe|Banned Recipe]]
- 5 × [[items/quest/610-five-color-scale|Five-Color Scale]] is taken
- you get 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]
- experience: 3200 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/309-strength-scroll-solo|Strength Scroll (Solo)]] (item count: 3)
- you get [[items/consumable/310-defense-scroll-solo|Defense Scroll (Solo)]] (item count: 3)
- the quest becomes [[quests/122-resurrection|Resurrection]] (progress kept)
- set episode variable 0 to 19

### `121-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 19

Then:

- you get the quest [[quests/122-resurrection|Resurrection]]
- works on [[quests/122-resurrection|Resurrection]]
- you get 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]

### `122-01`

Checks:

- you have [[quests/122-resurrection|Resurrection]]

Then:

- works on [[quests/122-resurrection|Resurrection]]
- you get 1 × [[items/quest/601-enigmatic-emblem|Enigmatic Emblem]]
- you get 1 × [[items/quest/612-small-necklace|Small Necklace]]
- the quest becomes [[quests/123-resurrection|Resurrection]] (progress kept)

### `122-02`

Happens by killing [[monsters/253-zombie|Zombie]].

Checks:

- you have [[quests/122-resurrection|Resurrection]]

Then:

- client script `piramid02`

### `122-03`

Checks:

- you have [[quests/122-resurrection|Resurrection]]
- you carry = 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]

Then:

- client script `piramid01`

If the checks fail, step `135-03-0` is tried instead.

### `122-04`

Checks:

- you have [[quests/122-resurrection|Resurrection]]
- you carry = 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]

Then:

- works on [[quests/122-resurrection|Resurrection]]
- 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]] is taken
- 1 × [[monsters/253-zombie|Zombie]] appear around you

### `122-05`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/122-resurrection|Resurrection]]
- you carry < 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]

Then:

- works on [[quests/122-resurrection|Resurrection]]
- you get 1 × [[items/quest/611-resurrection-medicine|Resurrection Medicine]]
