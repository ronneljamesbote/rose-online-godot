---
kind: quest
id: 116
name: A Favor for Karitte
status: in-game
given_by:
- '[[npcs/1053-cleric-karitte|Cleric Karitte]]'
npcs:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 116; QSD triggers 115-01, 115-02, 116-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# A Favor for Karitte

Karitte has asked you to meet Harin in Junon Polis, and find out if she's really her older sister.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `115-01`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- you have [[quests/115-hourglass-of-purification|Hourglass of Purification]]

Then:

- works on [[quests/115-hourglass-of-purification|Hourglass of Purification]]
- experience: 1000 (XP, not scaled by level, see [[rules/quests|Quests]])
- money: 5000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/jewellery/13-talisman-ring|Talisman Ring]] (item count: 1)
- the quest becomes [[quests/116-a-favor-for-karitte|A Favor for Karitte]]
- set episode variable 0 to 13

### `115-02`

Happens by talking to [[npcs/1053-cleric-karitte|Cleric Karitte]].

Checks:

- episode variable 0 = 13

Then:

- you get the quest [[quests/116-a-favor-for-karitte|A Favor for Karitte]]

### `116-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/116-a-favor-for-karitte|A Favor for Karitte]]

Then:

- works on [[quests/116-a-favor-for-karitte|A Favor for Karitte]]
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]
- experience: 500 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/117-proof|Proof]] (progress kept)
- set episode variable 0 to 14
