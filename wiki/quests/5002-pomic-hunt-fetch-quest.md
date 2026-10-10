---
kind: quest
id: 5002
name: Pomic Hunt (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
monsters:
- '[[monsters/72-pomic|Pomic]]'
time_limit_minutes: 96
steps: 6
source:
  data: LIST_QUEST.STB row 5002; QSD triggers 5002-01, 5002-02, 5002-03, 5002-04, 5002-10, 5002-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Pomic Hunt (Fetch Quest)

Raffle asked you to bring him 10 Pomic Skins, and will give you a better reward if you can complete this task earlier than your rivals.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5002-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- your Level ≤ 18
- the NPC [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 10

Then:

- you get the quest [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- add 1 to the NPC's variable 2
- then runs step `5002-10`

### `5002-02`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/110-pomic-skin|Pomic Skin]]
- the NPC [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]
- the NPC's variable 3 ≤ 2

Then:

- works on [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- 10 × [[items/quest/110-pomic-skin|Pomic Skin]] is taken
- money: 2000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5002-03` is tried instead.

### `5002-03`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/110-pomic-skin|Pomic Skin]]
- the NPC [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]
- the NPC's variable 3 > 2

Then:

- works on [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- add 1 to the NPC's variable 3
- 10 × [[items/quest/110-pomic-skin|Pomic Skin]] is taken
- money: 500 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]] (item count: 1)
- the quest ends (removed from your list)

### `5002-04`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- the quest timer > 0

### `5002-10`

Checks:

- the NPC [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2

### `5002-31`

Happens by killing [[monsters/72-pomic|Pomic]].

Checks:

- you have [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- a random roll 0–99 lands in 0–80
- you carry < 10 × [[items/quest/110-pomic-skin|Pomic Skin]]

Then:

- works on [[quests/5002-pomic-hunt-fetch-quest|Pomic Hunt (Fetch Quest)]]
- you get 1 × [[items/quest/110-pomic-skin|Pomic Skin]]

If the checks fail, step `5006-32` is tried instead.
