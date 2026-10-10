---
kind: quest
id: 127
name: The Owl Eye
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
monsters:
- '[[monsters/114-captain-moldie|Captain Moldie]]'
steps: 5
source:
  data: LIST_QUEST.STB row 127; QSD triggers 126-01, 126-02, 127-01, 127-31, 127-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Owl Eye

With a background in alchemy, Bellia is taking a very methodical approach in dealing with Shroon. To learn Shroon's secrets, Bellia asks you to get 20 Gold Fragments to make a tool that can monitor his activities. You can get Gold Fragments by fighting Captain Moldies.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `126-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/126-falsehoods|Falsehoods]]

Then:

- works on [[quests/126-falsehoods|Falsehoods]]
- 1 × [[items/quest/615-small-letter|Small Letter]] is taken
- money: 5000 (money, scaled by your level, see [[rules/quests|Quests]])
- the quest becomes [[quests/127-the-owl-eye|The Owl Eye]] (progress kept)
- set episode variable 0 to 26

### `126-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- episode variable 0 = 26

Then:

- you get the quest [[quests/127-the-owl-eye|The Owl Eye]]

### `127-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/127-the-owl-eye|The Owl Eye]]
- you carry ≥ 20 × [[items/quest/24-gold-fragment|Gold Fragment]]

Then:

- works on [[quests/127-the-owl-eye|The Owl Eye]]
- 20 × [[items/quest/24-gold-fragment|Gold Fragment]] is taken
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- money: 500 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- set episode variable 0 to 27
- then runs step `127-03`

### `127-31`

Happens by killing [[monsters/114-captain-moldie|Captain Moldie]].

Checks:

- you have [[quests/127-the-owl-eye|The Owl Eye]]
- you carry < 20 × [[items/quest/24-gold-fragment|Gold Fragment]]
- a random roll 0–99 lands in 0–80

Then:

- works on [[quests/127-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/24-gold-fragment|Gold Fragment]]

If the checks fail, step `127-32` is tried instead.

### `127-32`

Checks:

- you have [[quests/127-the-owl-eye|The Owl Eye]]
- you carry ≥ 20 × [[items/quest/24-gold-fragment|Gold Fragment]]
- you carry < 50 × [[items/quest/24-gold-fragment|Gold Fragment]]
- a random roll 0–99 lands in 0–65

Then:

- works on [[quests/127-the-owl-eye|The Owl Eye]]
- you get 1 × [[items/quest/24-gold-fragment|Gold Fragment]]
- add 1 to quest variable 9
