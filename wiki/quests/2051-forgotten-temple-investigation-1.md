---
kind: quest
id: 2051
name: Forgotten Temple Investigation (1)
status: in-game
given_by:
- '[[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]]'
npcs:
- '[[npcs/1191-shamanist-est|Shamanist Est]]'
monsters:
- '[[monsters/451-seal-stone|Seal Stone]]'
- '[[monsters/452-seal-stone|Seal Stone]]'
time_limit_minutes: 180
steps: 19
source:
  data: LIST_QUEST.STB row 2051; QSD triggers 2051-01, 2051-03, 2051-04, 2051-05, 2051-07, 2051-08, 2051-09, 2051-10, 2051-10-0, 2051-12, 2051-13, 2051-14, 2051-15, 2051-16, 2051-17, 2051-18, 2051-19, 2051-31, 2051-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forgotten Temple Investigation (1)

Ishtal, the Eucar Judge, has asked you to excavate Shamanic Artifacts in the Forgotten Temple in order to reconstruct forgotten shamanist history. While you're there, he'd like you to clear out any thieves in the Forgotten Temple that you can find.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2051-01`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- your Level ≥ 71
- your Level ≤ 90
- you carry ≥ 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- you get the quest [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- 5 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken

If the checks fail, step `2052-01` is tried instead.

### `2051-03`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 13 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 80000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2051-04` is tried instead.

### `2051-04`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 14 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≤ 17 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 80000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/82-lisent-cu|Lisent (Cu)]], base count 7 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2051-05` is tried instead.

### `2051-05`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 18 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- Zuly, base 80000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/82-lisent-cu|Lisent (Cu)]], base count 7 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/151-black-hearts|Black Hearts]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `2051-07`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- Zuly, base 3000 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `2052-07` is tried instead.

### `2051-08`

Happens by talking to [[npcs/1171-eucar-judge-ishtal|Eucar Judge Ishtal]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry < 10 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
- you carry < 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest ends (removed from your list)

If the checks fail, step `2052-08` is tried instead.

### `2051-09`

Happens by talking to [[npcs/1191-shamanist-est|Shamanist Est]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- quest switch 0 = 0

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- set quest switch 0 to 1
- you are moved to (5035, 5195) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2052-09` is tried instead.

### `2051-10`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- you are in a party with party level ≥ 1

Then:

- your team number is set
- your get-up point is set to (5035, 5195)

If the checks fail, step `2051-10-0` is tried instead.

### `2051-10-0`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- your team number is set
- your get-up point is set to (5035, 5195)

If the checks fail, step `2051-12` is tried instead.

### `2051-12`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- you are moved to (5099, 4247) in [[zones/54-crystal-snowfields|Crystal Snowfields]]

If the checks fail, step `2052-11` is tried instead.

### `2051-13`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you get 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- add 1 to quest variable 9

If the checks fail, step `2052-13` is tried instead.

### `2051-14`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- 1 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]] is taken
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken
- take 1 from quest variable 9

If the checks fail, step `2051-15` is tried instead.

### `2051-15`

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry = 0 × [[items/quest/498-proof-of-thief-extermination|Proof of Thief Extermination]]
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]] is taken

If the checks fail, step `2052-14` is tried instead.

### `2051-16`

Happens by talking to [[npcs/1191-shamanist-est|Shamanist Est]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- quest switch 0 = 1

If the checks fail, step `2052-16` is tried instead.

### `2051-17`

Happens by talking to [[npcs/1191-shamanist-est|Shamanist Est]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry < 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

If the checks fail, step `2052-17` is tried instead.

### `2051-18`

Happens by talking to [[npcs/1191-shamanist-est|Shamanist Est]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- quest switch 0 = 1
- you carry ≥ 1 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you are moved to (5035, 5195) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2052-18` is tried instead.

### `2051-19`

Happens by talking to [[npcs/1191-shamanist-est|Shamanist Est]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you carry ≥ 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]]

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you get 3 × [[items/quest/513-resurrection-spellbook|Resurrection Spellbook]]
- 15 × [[items/material/144-ice-key-fragment|Ice Key Fragment]] is taken
- you are moved to (5035, 5195) in [[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]

If the checks fail, step `2052-19` is tried instead.

### `2051-31`

Happens by killing [[monsters/451-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–90

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]

### `2051-32`

Happens by killing [[monsters/452-seal-stone|Seal Stone]].

Checks:

- you have [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- the quest timer > 0
- a random roll 0–99 lands in 0–90

Then:

- works on [[quests/2051-forgotten-temple-investigation-1|Forgotten Temple Investigation (1)]]
- you get 1 × [[items/quest/512-shamanic-artifact|Shamanic Artifact]]
