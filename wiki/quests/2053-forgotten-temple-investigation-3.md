---
kind: quest
id: 2053
name: Forgotten Temple Investigation (3)
status: in-game
npcs:
- '[[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]]'
monsters:
- '[[monsters/455-seal-stone|Seal Stone]]'
- '[[monsters/456-seal-stone|Seal Stone]]'
time_limit_minutes: 180
steps: 22
source:
  data: LIST_QUEST.STB row 2053; QSD triggers 2053-01, 2053-03, 2053-04, 2053-05, 2053-07, 2053-08, 2053-09, 2053-10, 2053-10-0, 2053-11, 2053-12, 2053-12-0, 2053-13, 2053-14, 2053-15, 2053-15-0, 2053-16, 2053-17, 2053-18, 2053-19, 2053-31, 2053-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forgotten Temple Investigation (3)

Ishtal, the Eucar Judge, has asked you to excavate Shamanic Artifacts in the Forgotten Temple in order to reconstruct forgotten shamanist history. While you're there, he'd like you to clear out any thieves in the Forgotten Temple that you can find.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2053-01`

Checks:

- your Level ≥ 111
- your Level ≤ 130
- you carry ≥ 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- you get the quest [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken

### `2053-03`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 13 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 150000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2053-04` is tried instead.

### `2053-04`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 14 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 17 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 150000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/84-lisent-al|Lisent (Al)]], base count 13 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2053-05` is tried instead.

### `2053-05`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 18 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 80000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/84-lisent-al|Lisent (Al)]], base count 13 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/153-blue-hearts|Blue Hearts]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `2053-07`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `2053-08`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry < 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest ends (removed from your list)

### `2053-09`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- quest switch 0 = 0

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- set quest switch 0 to 1
- you are moved to (5034, 4386) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

### `2053-10`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- you are in a party with party level ≥ 1

Then:

- your team number is set
- your get-up point is set to (5035, 4386)

If the checks fail, step `2053-10-0` is tried instead.

### `2053-10-0`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- your team number is set
- your get-up point is set to (5035, 4386)

If the checks fail, step `2053-12` is tried instead.

### `2053-11`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- you are within 0 m of (5035, 5195) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

Then:

- you are moved to (5034, 4386) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2053-10` is tried instead.

### `2053-12`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- you are moved to (5099, 4247) in [[zones/54-crystal-snowfields|Crystal Snowfields]]

If the checks fail, step `2053-12-0` is tried instead.

### `2053-12-0`

Checks: none.

Then:

- you are moved to (5099, 4247) in [[zones/54-crystal-snowfields|Crystal Snowfields]]

### `2053-13`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you get 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- add 1 to quest variable 9

### `2053-14`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]] is taken
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken
- take 1 from quest variable 9

If the checks fail, step `2053-15` is tried instead.

### `2053-15`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry = 0 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken

If the checks fail, step `2053-15-0` is tried instead.

### `2053-15-0`

Checks: none.

Then:

- you are moved to (5099, 4247) in [[zones/54-crystal-snowfields|Crystal Snowfields]]

### `2053-16`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- quest switch 0 = 1

### `2053-17`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

### `2053-18`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you are moved to (5034, 4386) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

### `2053-19`

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you carry ≥ 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken
- you are moved to (5034, 4386) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

### `2053-31`

Happens by killing [[monsters/455-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–82

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

### `2053-32`

Happens by killing [[monsters/456-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–82

Then:

- works on [[quests/2053-forgotten-temple-investigation-3|Forgotten Temple Investigation (3)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
