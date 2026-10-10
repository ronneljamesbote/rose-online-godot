---
kind: quest
id: 119
name: Forbidden Potion
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
npcs:
- '[[npcs/1097-tavern-owner-harin|Tavern Owner Harin]]'
steps: 3
source:
  data: LIST_QUEST.STB row 119; QSD triggers 118-01, 118-02, 119-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forbidden Potion

Bellia has asked you to ask Harin about the Gypsy Permit she gave to you. You need to speak to Harin and see if you can learn who issued that permit.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `118-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/118-forbidden-potion|Forbidden Potion]]
- you carry ≥ 20 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]]
- you carry ≥ 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]]

Then:

- works on [[quests/118-forbidden-potion|Forbidden Potion]]
- 20 × [[items/quest/607-wild-animal-claw|Wild Animal Claw]] is taken
- 20 × [[items/quest/608-sharp-stinger|Sharp Stinger]] is taken
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]
- experience: 2500 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/123-apple-juice|Apple Juice]] (item count: 30)
- the quest becomes [[quests/119-forbidden-potion|Forbidden Potion]] (progress kept)
- set episode variable 0 to 16

### `118-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 16

Then:

- you get the quest [[quests/119-forbidden-potion|Forbidden Potion]]
- works on [[quests/119-forbidden-potion|Forbidden Potion]]
- you get 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

### `119-01`

Happens by talking to [[npcs/1097-tavern-owner-harin|Tavern Owner Harin]].

Checks:

- you have [[quests/119-forbidden-potion|Forbidden Potion]]
- you carry = 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]]

Then:

- works on [[quests/119-forbidden-potion|Forbidden Potion]]
- 1 × [[items/quest/602-gypsy-s-permit|Gypsy's Permit]] is taken
- the quest becomes [[quests/120-dangerous-book|Dangerous Book]] (progress kept)
- set episode variable 0 to 17
