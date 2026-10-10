---
kind: quest
id: 3605
name: 'Plant Extract: Magic Fundamental'
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3605; QSD triggers 3605-01, 3605-02, 3605-03, 3605-04, 3605-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Plant Extract: Magic Fundamental

Extracting the magical essence from plants is one of the basics of magic research. Go and hunt Smoulies in order to bring back 13 Smouly Leaves.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3605-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4
- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC's variable 0 = 10
- the NPC's variable 8 < 10

Then:

- you get the quest [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- add 1 to the NPC's variable 8
- then runs step `3605-02`

### `3605-02`

Checks:

- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC's variable 0 = 10
- the NPC's variable 8 ≥ 10

Then:

- set the NPC's variable 0 to 11
- set the NPC's variable 8 to 0

### `3605-03`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- your Faction = 4
- your Level ≤ 70
- you carry ≥ 13 × [[items/quest/309-smouly-leaf|Smouly Leaf]]

Then:

- works on [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- 13 × [[items/quest/309-smouly-leaf|Smouly Leaf]] is taken
- add 8 to your UnionPoint4
- experience: 120 (XP, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

If the checks fail, step `3605-04` is tried instead.

### `3605-04`

Checks:

- you have [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- your Faction = 4
- your Level > 70
- you carry = 13 × [[items/quest/309-smouly-leaf|Smouly Leaf]]

Then:

- works on [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- 13 × [[items/quest/309-smouly-leaf|Smouly Leaf]] is taken
- add 3 to your UnionPoint4
- experience: 12000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

### `3605-31`

Checks:

- you have [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- a random roll 0–99 lands in 0–19
- you carry < 13 × [[items/quest/309-smouly-leaf|Smouly Leaf]]

Then:

- works on [[quests/3605-plant-extract-magic-fundamental|Plant Extract: Magic Fundamental]]
- you get 1 × [[items/quest/309-smouly-leaf|Smouly Leaf]]

If the checks fail, step `3803-31` is tried instead.
