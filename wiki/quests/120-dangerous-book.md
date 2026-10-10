---
kind: quest
id: 120
name: Dangerous Book
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 5
source:
  data: LIST_QUEST.STB row 120; QSD triggers 119-01, 119-02, 120-01, 120-03, 120-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dangerous Book

Harin asks you to steal 2 Dreadful Books from Bellia during the night. Bellia has bad eyesight, so you probably won't get caught if you're careful.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `119-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/119-forbidden-potion|Forbidden Potion]]
- you carry = 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

Then:

- works on [[quests/119-forbidden-potion|Forbidden Potion]]
- 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]] is taken
- the quest becomes [[quests/120-dangerous-book|Dangerous Book]] (progress kept)
- set episode variable 0 to 17

### `119-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 17

Then:

- you get the quest [[quests/120-dangerous-book|Dangerous Book]]

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

### `120-03`

Checks:

- you have [[quests/120-dangerous-book|Dangerous Book]]
- you carry < 1 × [[items/quest/609-dreadful-book-vol-2|Dreadful Book Vol 2]]

Then:

- works on [[quests/120-dangerous-book|Dangerous Book]]
- you get 1 × [[items/quest/609-dreadful-book-vol-2|Dreadful Book Vol 2]]

### `120-04`

Checks:

- you have [[quests/120-dangerous-book|Dangerous Book]]
- you carry < 1 × [[items/quest/609-dreadful-book-vol-2|Dreadful Book Vol 2]]

Then:

- client script `horriblebook`
