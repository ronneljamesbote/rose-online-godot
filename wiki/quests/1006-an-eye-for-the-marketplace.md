---
kind: quest
id: 1006
name: An Eye for the Marketplace
status: in-game
npcs:
- '[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]'
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
- '[[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]]'
steps: 7
source:
  data: LIST_QUEST.STB row 1006; QSD triggers 1005-04, 1005-05, 1005-06, 1006-01, 1006-02, 1006-03, 1006-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Eye for the Marketplace

Ulysses said that the Beetle Shell price has dropped. Instead, he's offering to give you Cobra Leather that you can sell in the Breezy Hills.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1005-04`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry ≥ 15 × [[items/quest/132-beetle-shell|Beetle Shell]]
- quest switch 2 = 0
- quest switch 3 = 0

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- 15 × [[items/quest/132-beetle-shell|Beetle Shell]] is taken
- you get 5 × [[items/quest/116-cobra-leather|Cobra Leather]]
- the quest becomes [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]] (progress kept)

If the checks fail, step `1005-05` is tried instead.

### `1005-05`

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry ≥ 15 × [[items/quest/132-beetle-shell|Beetle Shell]]
- quest switch 2 = 1

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- 15 × [[items/quest/132-beetle-shell|Beetle Shell]] is taken
- you get 4 × [[items/quest/116-cobra-leather|Cobra Leather]]
- the quest becomes [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]] (progress kept)

If the checks fail, step `1005-06` is tried instead.

### `1005-06`

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry ≥ 15 × [[items/quest/132-beetle-shell|Beetle Shell]]
- quest switch 3 = 1

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- 15 × [[items/quest/132-beetle-shell|Beetle Shell]] is taken
- you get 3 × [[items/quest/116-cobra-leather|Cobra Leather]]
- the quest becomes [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]] (progress kept)

### `1006-01`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry = 5 × [[items/quest/116-cobra-leather|Cobra Leather]]

Then:

- works on [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- Zuly, base 250 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1006-02` is tried instead.

### `1006-02`

Checks:

- you have [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry = 4 × [[items/quest/116-cobra-leather|Cobra Leather]]

Then:

- works on [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- experience, base 200 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1006-03` is tried instead.

### `1006-03`

Checks:

- you have [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry = 3 × [[items/quest/116-cobra-leather|Cobra Leather]]

Then:

- works on [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- Zuly, base 150 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1006-04`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry ≥ 3 × [[items/quest/116-cobra-leather|Cobra Leather]]

Then:

- works on [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- the quest becomes [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]] (progress kept)
