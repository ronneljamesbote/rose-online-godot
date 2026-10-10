---
kind: quest
id: 5019
name: Goblin Hunting Competition (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]'
monsters:
- '[[monsters/273-gold-mine-goblin-worker|Gold Mine Goblin Worker]]'
- '[[monsters/275-gold-mine-goblin-server|Gold Mine Goblin Server]]'
- '[[monsters/276-goblin-guard|Goblin Guard]]'
- '[[monsters/278-goblin-warrior|Goblin Warrior]]'
time_limit_minutes: 180
steps: 11
source:
  data: LIST_QUEST.STB row 5019; QSD triggers 5019-01, 5019-02, 5019-03, 5019-04, 5019-05, 5019-06, 5019-07, 5019-31, 5019-32, 5019-33, 5019-34
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Goblin Hunting Competition (Fetch Quest)

Itz, who has a weakness for gambling, has created this game of chance. Defeat 55 Goblins in the Goblin Cave (B2) for a possible reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5019-01`

Happens by talking to [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]].

Checks:

- your Level ≤ 70
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 0 = 1

Then:

- you get the quest [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- add 1 to the NPC's variable 2
- then runs step `5019-02`

### `5019-02`

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2
- the NPC says: "Hooray~!"

### `5019-03`

Happens by talking to [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- the quest timer > 0

### `5019-04`

Happens by talking to [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry ≥ 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 3 = 0

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- money: 30000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/59-vital-jam-10|Vital Jam (+10)]] (item count: 1)
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]] (item count: 1)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5019-05` is tried instead.

### `5019-05`

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry ≥ 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 3 = 1

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- money: 20000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/59-vital-jam-10|Vital Jam (+10)]] (item count: 1)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5019-06` is tried instead.

### `5019-06`

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry ≥ 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 3 = 2

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- money: 10000 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]] (item count: 1)
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5019-07` is tried instead.

### `5019-07`

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry ≥ 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- the quest timer > 0
- the NPC [[npcs/1100-ferrell-guild-staff-itz|Ferrell Guild Staff Itz]]
- the NPC's variable 3 ≥ 3

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- money: 2500 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]] (item count: 1)
- the quest ends (removed from your list)

### `5019-31`

Happens by killing [[monsters/273-gold-mine-goblin-worker|Gold Mine Goblin Worker]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry < 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5015-31` is tried instead.

### `5019-32`

Happens by killing [[monsters/275-gold-mine-goblin-server|Gold Mine Goblin Server]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry < 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5015-32` is tried instead.

### `5019-33`

Happens by killing [[monsters/276-goblin-guard|Goblin Guard]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry < 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5015-33` is tried instead.

### `5019-34`

Happens by killing [[monsters/278-goblin-warrior|Goblin Warrior]].

Checks:

- you have [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you carry < 55 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5019-goblin-hunting-competition-fetch-quest|Goblin Hunting Competition (Fetch Quest)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `1057-31` is tried instead.
