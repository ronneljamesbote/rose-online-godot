---
kind: quest
id: 143
name: The Ominous Kenji Stone
status: in-game
given_by:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 5
source:
  data: LIST_QUEST.STB row 143; QSD triggers 142-01, 142-02, 143-01, 143-03, 143-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Ominous Kenji Stone

Gallahad suggests that you investigate the area around the Kenji Stone. Find out what you can over there, and return to Gallahad.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `142-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/142-the-search-for-the-prince|The Search for the Prince]]

Then:

- works on [[quests/142-the-search-for-the-prince|The Search for the Prince]]
- experience, base 20000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- set episode variable 0 to 42

### `142-02`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- episode variable 0 = 42

Then:

- you get the quest [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]

### `143-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 1

Then:

- works on [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]
- experience, base 50000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/308-dexterity-scroll-solo|Dexterity Scroll (Solo)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]] (progress kept)
- set episode variable 0 to 43

### `143-03`

Checks:

- you have [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 0

Then:

- client script `genzistone`

### `143-04`

Checks:

- you have [[quests/143-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 0

Then:

- set quest switch 0 to 1
