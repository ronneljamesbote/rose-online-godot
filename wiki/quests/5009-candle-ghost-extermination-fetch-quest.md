---
kind: quest
id: 5009
name: Candle Ghost Extermination (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1001-village-chief-cornell|Village Chief Cornell]]'
monsters:
- Candle Ghost (NPC 711)
- Candle Ghost (NPC 712)
- Candle Ghost (NPC 713)
- Candle Ghost (NPC 714)
- Candle Ghost (NPC 715)
- Candle Ghost (NPC 716)
- Candle Ghost (NPC 717)
- Candle Ghost (NPC 718)
- Candle Ghost (NPC 719)
- Candle Ghost (NPC 720)
- Candle Ghost (NPC 721)
- Candle Ghost (NPC 722)
- Candle Ghost (NPC 723)
- Candle Ghost (NPC 724)
- Candle Ghost (NPC 725)
- Candle Ghost (NPC 726)
- Candle Ghost (NPC 727)
- Candle Ghost (NPC 728)
- Candle Ghost (NPC 729)
- Candle Ghost (NPC 730)
- Candle Ghost (NPC 731)
time_limit_minutes: 360
steps: 32
source:
  data: LIST_QUEST.STB row 5009; QSD triggers 5009-01, 5009-02, 5009-03, 5009-04, 5009-05, 5009-06, 5009-07, 5009-08, 5009-09, 5009-10, 5009-11, 5009-31, 5009-32, 5009-33, 5009-34, 5009-35, 5009-36, 5009-37, 5009-38, 5009-39, 5009-40, 5009-41, 5009-42, 5009-43, 5009-44, 5009-45, 5009-46, 5009-47, 5009-48, 5009-49, 5009-50, 5009-51
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Candle Ghost Extermination (Fetch Quest)

Cornell, the mayor of Zant, has organized a campaign to exterminate the Candle Ghosts. Bring him 20 Candle Ghost Masks from the Candle Ghosts you've defeated, and he'll reward you according to your performance in comparison to your competition.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5009-01`

Happens by talking to [[npcs/1001-village-chief-cornell|Village Chief Cornell]].

Checks:

- your Level ≥ 15

Then:

- you get the quest [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]

### `5009-02`

Happens by talking to [[npcs/1001-village-chief-cornell|Village Chief Cornell]].

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-03`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 = 0

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/12-vital-water-l|Vital Water (L)]] (item count: 15)
- you get [[items/consumable/31-spiritual-water-l|Spiritual Water (L)]] (item count: 15)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5009-04` is tried instead.

### `5009-04`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 = 1

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/12-vital-water-l|Vital Water (L)]] (item count: 10)
- you get [[items/consumable/31-spiritual-water-l|Spiritual Water (L)]] (item count: 10)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5009-05` is tried instead.

### `5009-05`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 = 2

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/11-vital-water-m|Vital Water (M)]] (item count: 10)
- you get [[items/consumable/30-spiritual-water-m|Spiritual Water (M)]] (item count: 10)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5009-06` is tried instead.

### `5009-06`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 ≥ 3
- the NPC's variable 3 ≤ 4

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/10-vital-water-s|Vital Water (S)]] (item count: 10)
- you get [[items/consumable/29-spiritual-water-s|Spiritual Water (S)]] (item count: 10)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5009-07` is tried instead.

### `5009-07`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 ≥ 5
- the NPC's variable 3 ≤ 9

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/10-vital-water-s|Vital Water (S)]] (item count: 5)
- you get [[items/consumable/29-spiritual-water-s|Spiritual Water (S)]] (item count: 5)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5009-08` is tried instead.

### `5009-08`

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 ≥ 10

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/10-vital-water-s|Vital Water (S)]] (item count: 1)
- you get [[items/consumable/29-spiritual-water-s|Spiritual Water (S)]] (item count: 1)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

### `5009-09`

Happens by talking to [[npcs/1001-village-chief-cornell|Village Chief Cornell]].

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the NPC [[npcs/1001-village-chief-cornell|Village Chief Cornell]]
- the NPC's variable 3 = 0
- the NPC's variable 19 = 1

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- you get [[items/consumable/12-vital-water-l|Vital Water (L)]] (item count: 10)
- you get [[items/consumable/31-spiritual-water-l|Spiritual Water (L)]] (item count: 10)
- add 1 to the NPC's variable 3
- set the NPC's variable 19 to 0
- the quest ends (removed from your list)

If the checks fail, step `5009-03` is tried instead.

### `5009-10`

Happens by talking to [[npcs/1001-village-chief-cornell|Village Chief Cornell]].

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer ≤ 0

### `5009-11`

Happens by talking to [[npcs/1001-village-chief-cornell|Village Chief Cornell]].

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-31`

Happens by killing Candle Ghost (NPC 711).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 23
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-32`

Happens by killing Candle Ghost (NPC 712).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 27
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-33`

Happens by killing Candle Ghost (NPC 713).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- time of day 1190–1439 minutes
- the quest timer > 0
- your Level ≤ 31
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-34`

Happens by killing Candle Ghost (NPC 714).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 35
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-35`

Happens by killing Candle Ghost (NPC 715).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 39
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-36`

Happens by killing Candle Ghost (NPC 716).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 43
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-31` is tried instead.

### `5009-37`

Happens by killing Candle Ghost (NPC 717).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 47
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-32` is tried instead.

### `5009-38`

Happens by killing Candle Ghost (NPC 718).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 51
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-33` is tried instead.

### `5009-39`

Happens by killing Candle Ghost (NPC 719).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 55
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-34` is tried instead.

### `5009-40`

Happens by killing Candle Ghost (NPC 720).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 59
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-35` is tried instead.

### `5009-41`

Happens by killing Candle Ghost (NPC 721).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 63
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-36` is tried instead.

### `5009-42`

Happens by killing Candle Ghost (NPC 722).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 67
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-37` is tried instead.

### `5009-43`

Happens by killing Candle Ghost (NPC 723).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 71
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

If the checks fail, step `5018-38` is tried instead.

### `5009-44`

Happens by killing Candle Ghost (NPC 724).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 75
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-45`

Happens by killing Candle Ghost (NPC 725).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 79
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-46`

Happens by killing Candle Ghost (NPC 726).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 83
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-47`

Happens by killing Candle Ghost (NPC 727).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 87
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-48`

Happens by killing Candle Ghost (NPC 728).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 91
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-49`

Happens by killing Candle Ghost (NPC 729).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 95
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-50`

Happens by killing Candle Ghost (NPC 730).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 99
- a random roll 0–99 lands in 0–20
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5009-51`

Happens by killing Candle Ghost (NPC 731).

Checks:

- you have [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- the quest timer > 0
- time of day 1190–1439 minutes
- your Level ≤ 103
- a random roll 0–99 lands in 0–50
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5009-candle-ghost-extermination-fetch-quest|Candle Ghost Extermination (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
