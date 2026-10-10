---
kind: quest
id: 110
name: The Truth to the Rumors
status: in-game
given_by:
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
npcs:
- '[[npcs/1014-guide-lena|Guide Lena]]'
- '[[npcs/1015-resident-luth|Resident Luth]]'
steps: 3
source:
  data: LIST_QUEST.STB row 110; QSD triggers 109-01, 109-02, 110-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Truth to the Rumors

Karitte told you a totally different story than Lena. Something else is behind the dark force affecting the Canyon City. Karitte has told you to check up on Luth, who is well known for causing trouble in Zant.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `109-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]], talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/109-sacrificed-soul|Sacrificed Soul]]

Then:

- works on [[quests/109-sacrificed-soul|Sacrificed Soul]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]
- set episode variable 0 to 7

### `109-02`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- episode variable 0 = 7

Then:

- you get the quest [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]

### `110-01`

Happens by talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]

Then:

- works on [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/111-the-truth-to-the-rumors|The Truth to the Rumors]]
- set episode variable 0 to 8
