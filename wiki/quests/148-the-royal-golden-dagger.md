---
kind: quest
id: 148
name: The Royal Golden Dagger
status: in-game
given_by:
- '[[npcs/1181-smith-pavrick|Smith Pavrick]]'
npcs:
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 3
source:
  data: LIST_QUEST.STB row 148; QSD triggers 147-01, 147-02, 148-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Royal Golden Dagger

Pavrick confirmed that the Golden Dagger really did belong to the royal family. Hence, the warrior who saved Kenji's Beach was really the Prince. But where is he now? First, you need to return the Golden Dagger to Gallahad.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `147-01`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 20

Then:

- works on [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]] is taken
- experience: 60000 (XP, not scaled by level, see [[rules/quests|Quests]])
- money: 40000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/12-vital-water-l|Vital Water (L)]] (item count: 7)
- you get [[items/consumable/31-spiritual-water-l|Spiritual Water (L)]] (item count: 7)
- the quest becomes [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]] (progress kept)
- set episode variable 0 to 47

### `147-02`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- episode variable 0 = 47

Then:

- you get the quest [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]
- works on [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `148-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]

Then:

- works on [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]]
- 1 × [[items/quest/510-golden-dagger|Golden Dagger]] is taken
- experience: 40000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/gem/351-peridot-1|Peridot 1]] (item count: 2)
- you get [[items/gem/331-topaz-1|Topaz 1]] (item count: 2)
- the quest becomes [[quests/149-an-interesting-proposal|An Interesting Proposal]] (progress kept)
- set episode variable 0 to 48
