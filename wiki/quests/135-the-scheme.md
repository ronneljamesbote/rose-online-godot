---
kind: quest
id: 135
name: The Scheme
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
monsters:
- '[[monsters/254-stone-golem|Stone Golem]]'
steps: 8
source:
  data: LIST_QUEST.STB row 135; QSD triggers 134-01, 134-02, 135-01, 135-03, 135-03-0, 135-04, 135-05, 135-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Scheme

Eva placed a spell on the Odorous Sweat so that even the dead can smell it. Hurry and sprinkle it around the Pyramid before Shroon notices.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `134-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/134-the-scheme|The Scheme]]
- you carry ≥ 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]

Then:

- works on [[quests/134-the-scheme|The Scheme]]
- Zuly, base 30000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/135-the-scheme|The Scheme]] (progress kept)
- set episode variable 0 to 34

### `134-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 34

Then:

- you get the quest [[quests/135-the-scheme|The Scheme]]
- works on [[quests/135-the-scheme|The Scheme]]
- you get 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]

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

### `135-03`

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 0 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- client script `piramid03`

### `135-03-0`

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 0 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- client script `piramid03`

### `135-04`

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 0 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/135-the-scheme|The Scheme]]
- 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]] is taken
- 1 × [[monsters/254-stone-golem|Stone Golem]] appear around you

### `135-05`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 0 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 0 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/135-the-scheme|The Scheme]]
- you get 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]

### `135-31`

Happens by killing [[monsters/254-stone-golem|Stone Golem]].

Checks:

- you have [[quests/135-the-scheme|The Scheme]]
- you carry = 0 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
- you carry = 0 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/135-the-scheme|The Scheme]]
- you get 1 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

If the checks fail, step `1059-31` is tried instead.
