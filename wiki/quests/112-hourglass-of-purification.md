---
kind: quest
id: 112
name: Hourglass of Purification
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
npcs:
- '[[npcs/1015-resident-luth|Resident Luth]]'
steps: 3
source:
  data: LIST_QUEST.STB row 112; QSD triggers 111-01, 111-02, 112-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hourglass of Purification

According to Lena, Luth messed around with the hourglass in Zant and took the Crystal Key. You better question Luth and see if you can get the Crystal Key back.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `111-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]

Then:

- works on [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/112-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 9

### `111-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- episode variable 0 = 9

Then:

- you get the quest [[quests/112-hourglass-of-purification|Hourglass of Purification]]

### `112-01`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/112-hourglass-of-purification|Hourglass of Purification]]

Then:

- works on [[quests/112-hourglass-of-purification|Hourglass of Purification]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 10
