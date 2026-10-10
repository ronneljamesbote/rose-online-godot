---
kind: quest
id: 111
name: The Truth to the Rumors
status: in-game
given_by:
- '[[npcs/1015-resident-luth|Resident Luth]]'
npcs:
- '[[npcs/1014-guide-lena|Guide Lena]]'
steps: 3
source:
  data: LIST_QUEST.STB row 111; QSD triggers 110-01, 110-02, 111-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Truth to the Rumors

Why was Luth so angry with you, and what could be so special about the hourglass he's mentioned? You better ask Lena and find out what you can.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `110-01`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]

Then:

- works on [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]
- experience: 300 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]
- set episode variable 0 to 8

### `110-02`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- episode variable 0 = 8

Then:

- you get the quest [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]

### `111-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]

Then:

- works on [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]
- experience: 300 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/112-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 9
