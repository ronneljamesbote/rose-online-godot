---
kind: quest
id: 3604
name: Lutis, Guard of Luxem Tower
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3604; QSD triggers 3604-01, 3604-02, 3604-03, 3604-04, 3604-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lutis, Guard of Luxem Tower

Lutis, the astronomer in Luxem Tower, has recently been having trouble focusing Mana energy. You've got to help by giving Lutis the Mana Focus Potion.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3604-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4
- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC's variable 0 = 4
- the NPC's variable 4 < 10
- the NPC is within 0 m

Then:

- you get the quest [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- works on [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- you get 3 × [[items/quest/306-mana-focus-potion|Mana Focus Potion]]
- set quest variable 0 to 10
- add 1 to the NPC's variable 4
- then runs step `3604-02`

### `3604-02`

Checks:

- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC's variable 0 = 4
- the NPC's variable 4 ≥ 10

Then:

- set the NPC's variable 0 to 5
- set the NPC's variable 4 to 0

### `3604-03`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- quest variable 0 = 10
- you carry = 3 × [[items/quest/306-mana-focus-potion|Mana Focus Potion]]
- the NPC [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]
- the NPC is within 0 m

Then:

- works on [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- 3 × [[items/quest/306-mana-focus-potion|Mana Focus Potion]] is taken
- add 10 to quest variable 0

### `3604-04`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- quest variable 0 = 20
- your Faction = 4
- your Level ≤ 50
- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC is within 0 m

Then:

- works on [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- add 2 to your UnionPoint4
- experience, base 80 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3604-05` is tried instead.

### `3604-05`

Checks:

- you have [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- quest variable 0 = 20
- your Faction = 4
- your Level > 50
- the NPC [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]
- the NPC is within 0 m

Then:

- works on [[quests/3604-lutis-guard-of-luxem-tower|Lutis, Guard of Luxem Tower]]
- add 5 to your UnionPoint4
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
