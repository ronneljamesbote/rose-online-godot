---
kind: quest
id: 106
name: Healing Hands
status: in-game
npcs:
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
time_limit_minutes: 30
steps: 6
source:
  data: LIST_QUEST.STB row 106; QSD triggers 102-05, 102-06, 105-02, 106-01, 106-02, 106-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Healing Hands

You have to complete this quest before the Soil of Purification and Antidote lose their effectiveness. There's not much time left, so you better place the Soil of Purification around the mushrooms and bring the antidote to Myad. Karitte said that the effects of the soil and antidote last only 30 minutes.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `102-05`

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- the quest timer > 0
- you carry = 1 × [[items/quest/603-soil-of-purification|Soil of Purification]]

Then:

- client script `mushroom`

If the checks fail, step `102-06` is tried instead.

### `102-06`

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- the quest timer ≤ 0

Then:

- client script `mushroom`

### `105-02`

Checks: none.

Then:

- you get the quest [[quests/106-healing-hands|Healing Hands]]
- works on [[quests/106-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/603-soil-of-purification|Soil of Purification]]
- you get 2 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]]

### `106-01`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- you carry = 0 × [[items/quest/603-soil-of-purification|Soil of Purification]]
- the quest timer > 0

Then:

- works on [[quests/106-healing-hands|Healing Hands]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/103-grapes|Grapes]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/107-healing-hands|Healing Hands]] (progress kept)
- set episode variable 0 to 4

### `106-02`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- the quest timer ≤ 0

Then:

- works on [[quests/106-healing-hands|Healing Hands]]
- the quest becomes [[quests/105-healing-hands|Healing Hands]]

### `106-03`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- the quest timer > 0
- you carry = 1 × [[items/quest/603-soil-of-purification|Soil of Purification]]

Then:

- works on [[quests/106-healing-hands|Healing Hands]]
- 1 × [[items/quest/603-soil-of-purification|Soil of Purification]] is taken
