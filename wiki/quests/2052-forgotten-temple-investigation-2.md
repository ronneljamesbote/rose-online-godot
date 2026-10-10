---
kind: quest
id: 2052
name: Forgotten Temple Investigation (2)
status: in-game
npcs:
- '[[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]]'
monsters:
- '[[monsters/453-seal-stone|Seal Stone]]'
- '[[monsters/454-seal-stone|Seal Stone]]'
time_limit_minutes: 180
steps: 20
source:
  data: LIST_QUEST.STB row 2052; QSD triggers 2052-01, 2052-03, 2052-04, 2052-05, 2052-07, 2052-08, 2052-09, 2052-10, 2052-10-0, 2052-11, 2052-12, 2052-13, 2052-14, 2052-15, 2052-16, 2052-17, 2052-18, 2052-19, 2052-31, 2052-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forgotten Temple Investigation (2)

Ishtal, the Eucar Judge, has asked you to excavate Shamanic Artifacts in the Forgotten Temple in order to reconstruct forgotten shamanist history. While you're there, he'd like you to clear out any thieves in the Forgotten Temple that you can find.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2052-01`

Checks:

- your Level ≥ 91
- your Level ≤ 110
- you carry ≥ 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- you get the quest [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken

If the checks fail, step `2053-01` is tried instead.

### `2052-03`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 13 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- money: 3000 (money, fixed, see [[rules/quests|Quests]])
- money: 120000 (money, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2052-04` is tried instead.

### `2052-04`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 14 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 17 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- money: 3000 (money, fixed, see [[rules/quests|Quests]])
- money: 120000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/material/83-lisent-pb|Lisent (Pb)]] (item count: 10)
- the quest ends (removed from your list)

If the checks fail, step `2052-05` is tried instead.

### `2052-05`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 18 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- money: 3000 (money, fixed, see [[rules/quests|Quests]])
- money: 120000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/material/83-lisent-pb|Lisent (Pb)]] (item count: 10)
- you get [[items/material/152-green-hearts|Green Hearts]] (item count: 1)
- the quest ends (removed from your list)

### `2052-07`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- money: 3000 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2053-07` is tried instead.

### `2052-08`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry < 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest ends (removed from your list)

If the checks fail, step `2053-08` is tried instead.

### `2052-09`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- quest switch 0 = 0

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- set quest switch 0 to 1
- you are moved to (5664, 4834) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2053-09` is tried instead.

### `2052-10`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- you are in a party with party level ≥ 1

Then:

- your team number is set
- your get-up point is set to (5664, 4835)

If the checks fail, step `2052-10-0` is tried instead.

### `2052-10-0`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- your team number is set
- your get-up point is set to (5664, 4835)

If the checks fail, step `2052-12` is tried instead.

### `2052-11`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- you are within 0 m of (5035, 5195) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

Then:

- you are moved to (5664, 4834) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2052-10` is tried instead.

### `2052-12`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- you are moved to (5099, 4247) in [[zones/54-crystal-snowfields|Crystal Snowfields]]

If the checks fail, step `2053-11` is tried instead.

### `2052-13`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you get 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- add 1 to quest variable 9

If the checks fail, step `2053-13` is tried instead.

### `2052-14`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]] is taken
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken
- take 1 from quest variable 9

If the checks fail, step `2052-15` is tried instead.

### `2052-15`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry = 0 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken

If the checks fail, step `2053-14` is tried instead.

### `2052-16`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- quest switch 0 = 1

If the checks fail, step `2053-16` is tried instead.

### `2052-17`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

If the checks fail, step `2053-17` is tried instead.

### `2052-18`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you are moved to (5664, 4834) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2053-18` is tried instead.

### `2052-19`

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you carry ≥ 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken
- you are moved to (5664, 4834) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2053-19` is tried instead.

### `2052-31`

Happens by killing [[monsters/453-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–86

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

### `2052-32`

Happens by killing [[monsters/454-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–86

Then:

- works on [[quests/2052-forgotten-temple-investigation-2|Forgotten Temple Investigation (2)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
