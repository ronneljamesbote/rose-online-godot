---
kind: quest
id: 113
name: Hourglass of Purification
status: in-game
given_by:
- '[[npcs/1015-resident-luth|Resident Luth]]'
npcs:
- '[[npcs/1014-guide-lena|Guide Lena]]'
monsters:
- '[[monsters/205-woopie-king|Woopie King]]'
steps: 5
source:
  data: LIST_QUEST.STB row 113; QSD triggers 112-01, 112-02, 113-01, 113-31, 113-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hourglass of Purification

That cowardly Luth! How could he throw away the Crystal Key because of a Woopie? Could the big, grey Woopie have been the Woopie King? Hurry and find the Crystal Key!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `112-01`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/112-hourglass-of-purification|Hourglass of Purification]]

Then:

- works on [[quests/112-hourglass-of-purification|Hourglass of Purification]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 10

### `112-02`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- episode variable 0 = 10

Then:

- you get the quest [[quests/113-hourglass-of-purification|Hourglass of Purification]]

### `113-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]], talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- you carry ≥ 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- works on [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- experience, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/105-orange|Orange]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 10 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/114-hourglass-of-purification|Hourglass of Purification]] (progress kept)
- set episode variable 0 to 11

### `113-31`

Happens by killing [[monsters/205-woopie-king|Woopie King]].

Checks:

- you have [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- quest variable 4 < 5
- you carry < 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- works on [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- add 1 to quest variable 4

If the checks fail, step `113-32` is tried instead.

### `113-32`

Checks:

- you have [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- quest variable 4 ≥ 5
- you carry < 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- works on [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- you get 1 × [[items/quest/606-crystal-key|Crystal Key]]
