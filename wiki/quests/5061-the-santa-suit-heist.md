---
kind: quest
id: 5061
name: The Santa Suit Heist
status: in-game
monsters:
- Rudolph Santa (NPC 303)
- Rudolph Santa (NPC 304)
- Rudolph Santa (NPC 305)
steps: 11
source:
  data: LIST_QUEST.STB row 5061; QSD triggers 5061-01, 5061-02, 5061-03, 5061-04, 5061-11, 5061-12, 5061-13, 5061-14, 5061-31, 5061-32, 5061-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Santa Suit Heist

Rudolphs have stolen the Santa costumes. You've got to help Santa by retrieving as many Stolen Santa Suits as you can!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5061-01`

Checks: none.

Then:

- you get the quest [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]

### `5061-02`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you are within 1 m of (5231, 5129) in [[zones/1-canyon-city-of-zant|Canyon City of Zant]]

If the checks fail, step `5061-03` is tried instead.

### `5061-03`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you are within 1 m of (5522, 5347) in [[zones/2-city-of-junon-polis|City of Junon Polis]]

### `5061-04`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you are within 1 m of (5338, 4984) in [[zones/51-magic-city-of-the-eucar|Magic City of the Eucar]]

### `5061-11`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry ≥ 8 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- 8 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]] is taken
- you get [[items/feet/152-santa-boots|Santa Boots]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get consumable 912, base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `5061-12`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry ≥ 13 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- 13 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]] is taken
- you get [[items/hands/152-santa-gloves|Santa Gloves]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get consumable 912, base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `5061-13`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry ≥ 20 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- 20 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]] is taken
- you get [[items/head/152-santa-hat|Santa Hat]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get consumable 912, base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `5061-14`

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry ≥ 28 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- 28 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]] is taken
- you get [[items/body/152-santa-sweater|Santa Sweater]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get consumable 912, base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])

### `5061-31`

Happens by killing Rudolph Santa (NPC 303).

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry < 100 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]
- a random roll 0–99 lands in 0–85

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you get 1 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

### `5061-32`

Happens by killing Rudolph Santa (NPC 304).

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry < 100 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]
- a random roll 0–99 lands in 0–85

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you get 1 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]

### `5061-33`

Happens by killing Rudolph Santa (NPC 305).

Checks:

- you have [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you carry < 100 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]
- a random roll 0–99 lands in 0–85

Then:

- works on [[quests/5061-the-santa-suit-heist|The Santa Suit Heist]]
- you get 1 × [[items/quest/901-stolen-santa-suit|Stolen Santa Suit]]
