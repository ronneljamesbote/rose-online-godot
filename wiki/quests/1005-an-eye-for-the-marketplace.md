---
kind: quest
id: 1005
name: An Eye for the Marketplace
status: in-game
given_by:
- '[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]'
steps: 8
source:
  data: LIST_QUEST.STB row 1005; QSD triggers 1005-01, 1005-02, 1005-03, 1005-04, 1005-05, 1005-06, 1005-31, 1005-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Eye for the Marketplace

Ulysses said that Beetle Shells are selling for a good price lately. If you bring him 15 Beetle Shells from Beetle Fighters, he'll buy them from you for a good price.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1005-01`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- job variable 0 = 1
- your Level ≥ 20
- your Job = 4

Then:

- you get the quest [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]

### `1005-02`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- set quest switch 2 to 1

### `1005-03`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- set quest switch 2 to 0
- set quest switch 3 to 1

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

### `1005-31`

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- a random roll 0–99 lands in 0–35
- you carry < 15 × [[items/quest/132-beetle-shell|Beetle Shell]]

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you get 1 × [[items/quest/132-beetle-shell|Beetle Shell]]

If the checks fail, step `5007-34` is tried instead.

### `1005-32`

Checks:

- you have [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- a random roll 0–99 lands in 0–45
- you carry < 15 × [[items/quest/132-beetle-shell|Beetle Shell]]

Then:

- works on [[quests/1005-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you get 1 × [[items/quest/132-beetle-shell|Beetle Shell]]

If the checks fail, step `956-31` is tried instead.
