---
kind: quest
id: 147
name: Pavrick, the Craftsman on Lunar
status: in-game
given_by:
- '[[npcs/1117-soldier-winters|Soldier Winters]]'
npcs:
- '[[npcs/1181-smith-pavrick|Smith Pavrick]]'
- '[[npcs/1186-tavern-owner-anzhelika|Tavern Owner Anzhelika]]'
steps: 5
source:
  data: LIST_QUEST.STB row 147; QSD triggers 145-01, 145-02, 147-01, 147-03, 147-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Pavrick, the Craftsman on Lunar

Winters recommended that you see Pavrick, the best craftsman in Luna, to check the Golden Dagger's authenticity. You must ask Alphonso to guide you to Luna.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `145-01`

Happens by talking to [[npcs/1117-soldier-winters|Soldier Winters]].

Checks:

- you have [[quests/145-information-on-lunar|Information on Lunar]]

Then:

- works on [[quests/145-information-on-lunar|Information on Lunar]]
- experience, base 20000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- set episode variable 0 to 45

### `145-02`

Happens by talking to [[npcs/1117-soldier-winters|Soldier Winters]].

Checks:

- episode variable 0 = 45

Then:

- you get the quest [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- works on [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `147-01`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 20

Then:

- works on [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]] is taken
- experience, base 60000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 40000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/12-vital-water-l|Vital Water (L)]], base count 7 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/31-spiritual-water-l|Spiritual Water (L)]], base count 7 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/148-the-royal-golden-dagger|The Royal Golden Dagger]] (progress kept)
- set episode variable 0 to 47

### `147-03`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 0

Then:

- works on [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- set quest variable 1 to 10

### `147-04`

Happens by talking to [[npcs/1186-tavern-owner-anzhelika|Tavern Owner Anzhelika]].

Checks:

- you have [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 10

Then:

- works on [[quests/147-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- set quest variable 1 to 20
- take 500 from your Money
- you get 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]]
