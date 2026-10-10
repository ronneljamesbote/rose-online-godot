---
kind: quest
id: 109
name: Sacrificed Soul
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
npcs:
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
steps: 3
source:
  data: LIST_QUEST.STB row 109; QSD triggers 108-01, 108-02, 109-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Sacrificed Soul

Lena, a descendant of Lillith, said that the dark force around Zant Village is caused by a witch staying in the Tower of Luxem Valley. Lena said that Karitte must be stopped, and is spreading bad rumors about her. You should see Karitte first.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `108-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/108-healing-hands|Healing Hands]]

Then:

- works on [[quests/108-healing-hands|Healing Hands]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/109-sacrificed-soul|Sacrificed Soul]]
- set episode variable 0 to 6

### `108-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- episode variable 0 = 6

Then:

- you get the quest [[quests/109-sacrificed-soul|Sacrificed Soul]]

### `109-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]], talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/109-sacrificed-soul|Sacrificed Soul]]

Then:

- works on [[quests/109-sacrificed-soul|Sacrificed Soul]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/110-the-truth-to-the-rumors|The Truth to the Rumors]]
- set episode variable 0 to 7
