---
kind: quest
id: 1988
name: Pavrick, the Craftsman on Lunar
status: in-game
npcs:
- '[[npcs/1117-soldier-winters|Soldier Winters]]'
- '[[npcs/1181-smith-pavrick|Smith Pavrick]]'
- '[[npcs/1186-tavern-owner-anzhelika|Tavern Owner Anzhelika]]'
steps: 6
source:
  data: LIST_QUEST.STB row 1988; QSD triggers 1986-01, 1986-02, 1986-03, 1988-01, 1988-02, 1988-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Pavrick, the Craftsman on Lunar

Winters recommended that you see Pavrick, the best craftsman in Luna, to check the Golden Dagger's authenticity. You must ask Alphonso to guide you to Luna.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1986-01`

Happens by talking to [[npcs/1117-soldier-winters|Soldier Winters]].

Checks:

- you have [[quests/1986-information-on-lunar|Information on Lunar]]
- quest switch 133 is off

Then:

- works on [[quests/1986-information-on-lunar|Information on Lunar]]
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- quest switch 133 on

If the checks fail, step `1986-02` is tried instead.

### `1986-02`

Checks:

- you have [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- quest switch 133 is off

Then:

- works on [[quests/1987-the-truth-of-the-golden-dagger-2|The Truth of the Golden Dagger (2)]]
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]] (progress kept)
- quest switch 133 on

### `1986-03`

Checks:

- quest switch 133 is on
- quest switch 134 is off

Then:

- you get the quest [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- works on [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]

### `1988-01`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 0

Then:

- works on [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- set quest variable 1 to 10

### `1988-02`

Happens by talking to [[npcs/1186-tavern-owner-anzhelika|Tavern Owner Anzhelika]].

Checks:

- you have [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 10

Then:

- works on [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- set quest variable 1 to 20
- take 500 from your Money
- you get 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]]

### `1988-03`

Happens by talking to [[npcs/1181-smith-pavrick|Smith Pavrick]].

Checks:

- you have [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- quest variable 1 = 20

Then:

- works on [[quests/1988-pavrick-the-craftsman-on-lunar|Pavrick, the Craftsman on Lunar]]
- 1 × [[items/quest/511-luna-whiskey|Luna Whiskey]] is taken
- experience: 8000 (XP, not scaled by level, see [[rules/quests|Quests]])
- money: 15000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- quest switch 134 on
- the quest becomes [[quests/1989-the-royal-golden-dagger|The Royal Golden Dagger]] (progress kept)
