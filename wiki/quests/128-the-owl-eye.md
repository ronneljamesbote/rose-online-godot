---
kind: quest
id: 128
name: The Owl Eye
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
monsters:
- '[[monsters/208-guardian-tree|Guardian Tree]]'
steps: 4
source:
  data: LIST_QUEST.STB row 128; QSD triggers 127-02, 127-03, 128-01, 128-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Owl Eye

Now that you've brought the Gold Fragments, you need fruit from Guardian Trees to complete the Owl Eye. Go to the Forest of Wisdom and get 10 fruit from the Guardian Trees.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `127-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 27

Then:

- you get the quest [[quests/128-the-owl-eye|The Owl Eye]]

### `127-03`

Checks: none.

Then:

- you get the quest [[quests/128-the-owl-eye|The Owl Eye]]

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

### `128-31`

Happens by killing [[monsters/208-guardian-tree|Guardian Tree]].

Checks:

- you have [[quests/128-the-owl-eye|The Owl Eye]]
- you carry < 10 × [[items/quest/613-hard-fruit|Hard Fruit]]

Then:

- works on [[quests/128-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/613-hard-fruit|Hard Fruit]]
