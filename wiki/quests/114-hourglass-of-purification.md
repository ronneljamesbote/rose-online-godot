---
kind: quest
id: 114
name: Hourglass of Purification
status: in-game
given_by:
- '[[npcs/1014-guide-lena|Guide Lena]]'
npcs:
- '[[npcs/1015-resident-luth|Resident Luth]]'
steps: 5
source:
  data: LIST_QUEST.STB row 114; QSD triggers 113-01, 113-02, 114-01, 114-02, 114-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hourglass of Purification

Lena told you to put the Crystal Key into the hourglass. You'll need to search for the hourglass and its keyhole.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `113-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]], talking to [[npcs/1015-resident-luth|Resident Luth]].

Checks:

- you have [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- you carry ≥ 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- works on [[quests/113-hourglass-of-purification|Hourglass of Purification]]
- experience: 1000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/105-orange|Orange]] (item count: 10)
- experience: 10 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/114-hourglass-of-purification|Hourglass of Purification]] (progress kept)
- set episode variable 0 to 11

### `113-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- episode variable 0 = 11

Then:

- you get the quest [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- works on [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- you get 1 × [[items/quest/606-crystal-key|Crystal Key]]

### `114-01`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- quest switch 0 = 0
- you carry = 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- works on [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- 1 × [[items/quest/606-crystal-key|Crystal Key]] is taken
- set quest switch 0 to 1

### `114-02`

Happens by talking to [[npcs/1014-guide-lena|Guide Lena]].

Checks:

- you have [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- quest switch 0 = 1

Then:

- works on [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- the quest becomes [[quests/115-hourglass-of-purification|Hourglass of Purification]]
- set episode variable 0 to 12

### `114-04`

Checks:

- you have [[quests/114-hourglass-of-purification|Hourglass of Purification]]
- quest switch 0 = 0
- you carry = 1 × [[items/quest/606-crystal-key|Crystal Key]]

Then:

- client script `sandglass`
