---
kind: quest
id: 134
name: The Scheme
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
monsters:
- '[[monsters/153-doonga-origin|Doonga Origin]]'
steps: 4
source:
  data: LIST_QUEST.STB row 134; QSD triggers 133-01, 133-02, 134-01, 134-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Scheme

Surprisingly, Eva understands Shroon's plans and you now know that Shroon's secret is in the Pyramid in El Verloon Desert. But since Shroon protected that area with a barrier, you'll need 15 Bottles of Odorous Sweat. Be careful, they smell terrible!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `133-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/133-forbidden-spell|Forbidden Spell]]

Then:

- works on [[quests/133-forbidden-spell|Forbidden Spell]]
- experience, base 10000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/134-the-scheme|The Scheme]]
- set episode variable 0 to 33

### `133-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 33

Then:

- you get the quest [[quests/134-the-scheme|The Scheme]]

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

### `134-31`

Happens by killing [[monsters/153-doonga-origin|Doonga Origin]].

Checks:

- you have [[quests/134-the-scheme|The Scheme]]
- a random roll 0–99 lands in 0–60
- you carry < 15 × [[items/quest/618-odorous-sweat|Odorous Sweat]]

Then:

- works on [[quests/134-the-scheme|The Scheme]]
- you get 1 × [[items/quest/618-odorous-sweat|Odorous Sweat]]
