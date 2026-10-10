---
kind: quest
id: 154
name: Kay's Request
status: in-game
given_by:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
npcs:
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
steps: 15
source:
  data: LIST_QUEST.STB row 154; QSD triggers 153-01, 153-02, 153-03, 153-04, 153-05, 153-06, 153-07, 153-08, 153-09, 153-10, 153-11, 153-12, 153-13, 153-14, 154-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kay's Request

Kay asks you to find Med's whereabouts to uncover the secret of the Arumic researchers who withdrew from the Gorge of Silence.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `153-01`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/507-viking-sword|Viking Sword]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-02`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- episode variable 0 = 53

Then:

- you get the quest [[quests/154-kay-s-request|Kay's Request]]

### `153-03`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/538-morning-star|Morning Star]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-04`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/564-dark-bow-gun|Dark Bow Gun]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-05`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/585-executioner|Executioner]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-06`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/616-silver-axe|Silver Axe]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-07`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/646-lightning-spear|Lightning Spear]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-08`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/676-half-elf-bow|Half-Elf Bow]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-09`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/706-beretta|Beretta]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-10`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/734-mythril-launcher|Mythril Launcher]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-11`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/767-anima-staff|Anima Staff]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-12`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/796-blizzard-wand|Blizzard Wand]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-13`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/828-dual-patar|Dual Patar]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-14`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/856-viking-sword-axe|Viking Sword & Axe]] (item count: 1)
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `154-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/154-kay-s-request|Kay's Request]]

Then:

- works on [[quests/154-kay-s-request|Kay's Request]]
- experience: 50000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/155-suspicious-researchers|Suspicious Researchers]]
- set episode variable 0 to 54
