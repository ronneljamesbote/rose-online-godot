---
kind: quest
id: 1985
name: The Truth of the Golden Dagger
status: in-game
npcs:
- '[[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]]'
- '[[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]]'
steps: 3
source:
  data: LIST_QUEST.STB row 1985; QSD triggers 1984-01, 1985-01, 1985-02
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Truth of the Golden Dagger

To find out the truth about the Golden Dagger, you should talk to Lutis, who is in the Valley of Luxem Tower.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1984-01`

Happens by talking to [[npcs/1141-righteous-crusader-gallahad|Righteous Crusader Gallahad]].

Checks:

- you have [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- quest switch 0 = 1
- quest switch 130 is on
- quest switch 131 is off

Then:

- works on [[quests/1984-the-ominous-kenji-stone|The Ominous Kenji Stone]]
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]] (item count: 10)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 10)
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]
- the quest becomes [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]] (progress kept)
- quest switch 131 on

### `1985-01`

Happens by talking to [[npcs/1051-arumic-resercher-lutis|Arumic Resercher Lutis]].

Checks:

- you have [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- quest switch 131 is on
- quest switch 132 is off

Then:

- works on [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- experience: 1000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/2-health-vial-m|Health Vial (M)]] (item count: 10)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 10)
- the quest becomes [[quests/1986-information-on-lunar|Information on Lunar]] (progress kept)
- quest switch 132 on

### `1985-02`

Checks:

- quest switch 131 is on
- quest switch 132 is off

Then:

- you get the quest [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- works on [[quests/1985-the-truth-of-the-golden-dagger|The Truth of the Golden Dagger]]
- you get 1 × [[items/quest/510-golden-dagger|Golden Dagger]]
