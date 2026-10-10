---
kind: quest
id: 121
name: Banned Recipe
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 4
source:
  data: LIST_QUEST.STB row 121; QSD triggers 120-01, 120-02, 121-01, 121-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Banned Recipe

Harin says that she'll need Five-Color Scales to make the secret medicine. Hunt Aqua Warriors to get these Five-Color Scales.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `120-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/120-dangerous-book|Dangerous Book]]
- you carry = 1 × [[items/quest/609-dreadful-book-vol-2|Dreadful Book Vol 2]]

Then:

- works on [[quests/120-dangerous-book|Dangerous Book]]
- 1 × [[items/quest/609-dreadful-book-vol-2|Dreadful Book Vol 2]] is taken
- the quest becomes [[quests/121-banned-recipe|Banned Recipe]] (progress kept)
- set episode variable 0 to 18

### `120-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 18

Then:

- you get the quest [[quests/121-banned-recipe|Banned Recipe]]

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

### `121-31`

Checks:

- you have [[quests/121-banned-recipe|Banned Recipe]]
- a random roll 0–99 lands in 0–40
- you carry < 5 × [[items/quest/610-five-color-scale|Five-Color Scale]]

Then:

- works on [[quests/121-banned-recipe|Banned Recipe]]
- you get 1 × [[items/quest/610-five-color-scale|Five-Color Scale]]
