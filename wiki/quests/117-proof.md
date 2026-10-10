---
kind: quest
id: 117
name: Proof
status: in-game
given_by:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
npcs:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
steps: 3
source:
  data: LIST_QUEST.STB row 117; QSD triggers 116-01, 116-02, 117-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Proof

To earn Harin's trust, you've decided to help her by finding Bellia, the Gypsy Jewel Seller, so you can get some sort of secret medicine.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `116-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/116-a-favor-for-karitte|A Favor for Karitte]]

Then:

- works on [[quests/116-a-favor-for-karitte|A Favor for Karitte]]
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]
- experience, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/117-proof|Proof]] (progress kept)
- set episode variable 0 to 14

### `116-02`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- episode variable 0 = 14

Then:

- you get the quest [[quests/117-proof|Proof]]
- works on [[quests/117-proof|Proof]]
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

### `117-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/117-proof|Proof]]
- you carry = 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

Then:

- works on [[quests/117-proof|Proof]]
- 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]] is taken
- the quest becomes [[quests/118-forbidden-potion|Forbidden Potion]] (progress kept)
- set episode variable 0 to 15
