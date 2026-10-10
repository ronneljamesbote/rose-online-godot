---
kind: quest
id: 153
name: Kay's Request
status: in-game
given_by:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
monsters:
- '[[monsters/166-master-stone-golem|Master Stone Golem]]'
steps: 16
source:
  data: LIST_QUEST.STB row 153; QSD triggers 152-02, 152-03, 153-01, 153-03, 153-04, 153-05, 153-06, 153-07, 153-08, 153-09, 153-10, 153-11, 153-12, 153-13, 153-14, 153-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Kay's Request

From Kay's pitiable story, you learn that Master Stone Golems have the boxes containing Kay's goods. You need to retrieve 11 boxes from the Master Stone Golems for Kay.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `152-02`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- episode variable 0 = 52

Then:

- you get the quest [[quests/153-kay-s-request|Kay's Request]]

### `152-03`

Checks: none.

Then:

- you get the quest [[quests/153-kay-s-request|Kay's Request]]
- set episode variable 0 to 52

### `153-01`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/507-viking-sword|Viking Sword]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-03`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry ≥ 11 × [[items/quest/621-old-box|Old Box]]

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- 11 × [[items/quest/621-old-box|Old Box]] is taken
- you get [[items/weapon/538-morning-star|Morning Star]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/564-dark-bow-gun|Dark Bow Gun]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/585-executioner|Executioner]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/616-silver-axe|Silver Axe]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/646-lightning-spear|Lightning Spear]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/676-half-elf-bow|Half-Elf Bow]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/706-beretta|Beretta]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/734-mythril-launcher|Mythril Launcher]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/767-anima-staff|Anima Staff]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/796-blizzard-wand|Blizzard Wand]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/828-dual-patar|Dual Patar]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
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
- you get [[items/weapon/856-viking-sword-axe|Viking Sword & Axe]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/154-kay-s-request|Kay's Request]] (progress kept)
- set episode variable 0 to 53

### `153-31`

Happens by killing [[monsters/166-master-stone-golem|Master Stone Golem]].

Checks:

- you have [[quests/153-kay-s-request|Kay's Request]]
- you carry < 11 × [[items/quest/621-old-box|Old Box]]
- a random roll 0–99 lands in 0–30

Then:

- works on [[quests/153-kay-s-request|Kay's Request]]
- you get 1 × [[items/quest/621-old-box|Old Box]]
