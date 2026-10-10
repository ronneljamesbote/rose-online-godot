---
kind: quest
id: 130
name: Eva the Sorcerer
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
npcs:
- '[[npcs/1082-guide-eva|Guide Eva]]'
steps: 3
source:
  data: LIST_QUEST.STB row 130; QSD triggers 129-01, 129-02, 130-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Eva the Sorcerer

Bellia realizes the power of Shroon's magic and recommends that you get Eva to help you. Hopefully, you can persuade Eva, a woman who abandoned magic and is now living as a Guide, to help you.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `129-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/129-the-owl-eye|The Owl Eye]]
- you carry = 0 × [[items/quest/614-owl-eye|Owl Eye]]

Then:

- works on [[quests/129-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]
- experience, base 7000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/130-eva-the-sorcerer|Eva the Sorcerer]] (progress kept)
- set episode variable 0 to 29

### `129-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 29

Then:

- you get the quest [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]
- works on [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]
- you get 1 × [[items/quest/615-small-letter|Small Letter]]

### `130-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]

Then:

- works on [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]
- 1 × [[items/quest/615-small-letter|Small Letter]] is taken
- experience, base 7000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/131-eva-the-sorcerer|Eva the Sorcerer]] (progress kept)
- set episode variable 0 to 30
