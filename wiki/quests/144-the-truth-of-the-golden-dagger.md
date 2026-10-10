---
kind: quest
id: 144
name: The Truth of the Golden Dagger
status: in-game
given_by:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
steps: 3
source:
  data: LIST_QUEST.STB row 144; QSD triggers 143-01, 143-02, 144-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Truth of the Golden Dagger

To find out the truth about the Golden Dagger, you should talk to Lutis, who is in the Valley of Luxem Tower.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `143-02`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- episode variable 0 = 43

Then:

- you get the quest [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- works on [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `144-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]

Then:

- works on [[quests/144-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- experience, base 20000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/153-blue-hearts|Blue Hearts]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/145-information-on-lunar|Information on Lunar]] (progress kept)
- set episode variable 0 to 44
