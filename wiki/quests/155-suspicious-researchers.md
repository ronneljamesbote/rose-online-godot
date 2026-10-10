---
kind: quest
id: 155
name: Suspicious Researchers
status: in-game
given_by:
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
steps: 11
source:
  data: LIST_QUEST.STB row 155; QSD triggers 154-01, 154-02, 155-01, 155-03, 155-04, 155-05, 155-06, 155-07, 155-08, 155-09, 155-10
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Suspicious Researchers

Med asks you to find the Mana Chargers that the researchers have hidden in the village. The message 'Wine Barrel,' was inscribed on one of the Chargers. Maybe you should investigate the Bar to find the next hidden Mana Charger.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `154-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/154-kay-s-request|Kay's Request]]

Then:

- works on [[quests/154-kay-s-request|Kay's Request]]
- experience: 50000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/155-suspicious-researchers|Suspicious Researchers]]
- set episode variable 0 to 54

### `154-02`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- episode variable 0 = 54

Then:

- you get the quest [[quests/155-suspicious-researchers|Suspicious Researchers]]

### `155-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you carry = 4 × [[items/quest/622-mana-charger|Mana Charger]]
- quest switch 0 = 1
- quest switch 1 = 1
- quest switch 2 = 1
- quest switch 3 = 1

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- 4 × [[items/quest/622-mana-charger|Mana Charger]] is taken
- you get [[items/consumable/316-advanced-strength-scroll-solo|Advanced Strength Scroll (Solo)]] (item count: 3)
- you get [[items/consumable/317-advanced-defense-scroll-solo|Advanced Defense Scroll (Solo)]] (item count: 3)
- experience: 40000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/381-ice-charm|Ice Charm]] (item count: 10)
- the quest becomes [[quests/156-monsters-in-the-desert|Monsters in the Desert]] (progress kept)
- set episode variable 0 to 55

### `155-03`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 0 = 0
- you carry = 0 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- client script `mana`

### `155-04`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 0 = 1
- quest switch 1 = 0
- you carry = 1 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- client script `mana`

### `155-05`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 1 = 1
- quest switch 2 = 0
- you carry = 2 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- client script `mana`

### `155-06`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 2 = 1
- quest switch 3 = 0
- you carry = 3 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- client script `mana`

### `155-07`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 0 = 0
- you carry = 0 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
- set quest switch 0 to 1

### `155-08`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 0 = 1
- quest switch 1 = 0
- you carry = 1 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
- set quest switch 1 to 1

### `155-09`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 1 = 1
- quest switch 2 = 0
- you carry = 2 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
- set quest switch 2 to 1

### `155-10`

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- quest switch 2 = 1
- quest switch 3 = 0
- you carry = 3 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
- set quest switch 3 to 1
