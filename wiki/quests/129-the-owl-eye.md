---
kind: quest
id: 129
name: The Owl Eye
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
steps: 5
source:
  data: LIST_QUEST.STB row 129; QSD triggers 128-01, 128-02, 129-01, 129-03, 129-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Owl Eye

Now that you've got the completed Owl Eye, you need to hide it someplace where it can watch Shroon. Be careful: If you're caught, all your efforts will be for nothing!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `128-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/128-the-owl-eye|The Owl Eye]]
- you carry ≥ 10 × [[items/quest/613-hard-fruit|Hard Fruit]]

Then:

- works on [[quests/128-the-owl-eye|The Owl Eye]]
- 10 × [[items/quest/613-hard-fruit|Hard Fruit]] is taken
- you get 1 × [[items/quest/614-owl-eye|Owl Eye]]
- experience: 20000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]] (item count: 20)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 20)
- the quest becomes [[quests/129-the-owl-eye|The Owl Eye]] (progress kept)
- set episode variable 0 to 28

### `128-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 28

Then:

- you get the quest [[quests/129-the-owl-eye|The Owl Eye]]
- works on [[quests/129-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/614-owl-eye|Owl Eye]]

### `129-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/129-the-owl-eye|The Owl Eye]]
- you carry = 0 × [[items/quest/614-owl-eye|Owl Eye]]

Then:

- works on [[quests/129-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]
- experience: 7000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/130-eva-the-sorcerer|Eva the Sorcerer]] (progress kept)
- set episode variable 0 to 29

### `129-03`

Checks:

- you have [[quests/129-the-owl-eye|The Owl Eye]]
- you carry = 1 × [[items/quest/614-owl-eye|Owl Eye]]

Then:

- client script `owl`

### `129-04`

Checks:

- you have [[quests/129-the-owl-eye|The Owl Eye]]
- you carry = 1 × [[items/quest/614-owl-eye|Owl Eye]]

Then:

- works on [[quests/129-the-owl-eye|The Owl Eye]]
- 1 × [[items/quest/614-owl-eye|Owl Eye]] is taken
