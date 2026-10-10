---
kind: quest
id: 115
name: Hourglass of Purification
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
npcs:
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
steps: 3
source:
  data: LIST_QUEST.STB row 115; QSD triggers 114-02, 114-03, 115-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hourglass of Purification

Lena recognizes her mistake, and she has asked you to apologize to Karitte for her before she visits her in person.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `114-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- quest switch 0 = 1

Then:

- works on [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- the quest becomes [[quests/115-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 12

### `114-03`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- episode variable 0 = 12

Then:

- you get the quest [[quests/115-hourglass-of-purification|Hourglass of Purification]]

### `115-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/115-hourglass-of-purification|Hourglass of Purification]]

Then:

- works on [[quests/115-hourglass-of-purification|Hourglass of Purification]]
- experience, base 1000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 5000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/jewellery/13-talisman-ring|Talisman Ring]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/116-a-favor-for-karitte|A Favor for Karitte]]
- set episode variable 0 to 13
