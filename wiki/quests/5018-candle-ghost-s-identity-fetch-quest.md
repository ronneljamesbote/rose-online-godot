---
kind: quest
id: 5018
name: Candle Ghost's Identity (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]'
time_limit_minutes: 180
steps: 15
source:
  data: LIST_QUEST.STB row 5018; QSD triggers 5018-01, 5018-02, 5018-03, 5018-04, 5018-05, 5018-06, 5018-07, 5018-31, 5018-32, 5018-33, 5018-34, 5018-35, 5018-36, 5018-37, 5018-38
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Candle Ghost's Identity (Fetch Quest)

No one has ever seen the true form of the Candle Ghosts that appear at night, so it might be a good idea to remove a bunch of their masks. Bring 20 Candle Ghost Masks to Hayen for a reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5018-01`

Happens by talking to [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]].

Checks:

- your Level ≤ 60
- the NPC [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]
- the NPC's variable 0 = 1

Then:

- you get the quest [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- add 1 to the NPC's variable 2
- then runs step `5018-02`

### `5018-02`

Checks:

- the NPC [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]
- the NPC's variable 2 = 10

Then:

- set the NPC's variable 0 to 2
- the NPC says: "We are no longer accepting participants for the Goblin Hunting Competition! (Rival) Please try again later~"

### `5018-03`

Happens by talking to [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]].

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- the quest timer > 0

### `5018-04`

Happens by talking to [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]].

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the quest timer > 0
- the NPC [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]
- the NPC's variable 3 = 0

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- Zuly, base 20000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5018-05` is tried instead.

### `5018-05`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the quest timer > 0
- the NPC [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]
- the NPC's variable 3 = 1

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- Zuly, base 15000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5018-06` is tried instead.

### `5018-06`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the quest timer > 0
- the NPC [[npcs/1099-ferrell-guild-staff-hayen|Ferrell Guild Staff Hayen]]
- the NPC's variable 3 = 2

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- Zuly, base 10000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5018-07` is tried instead.

### `5018-07`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you carry ≥ 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
- the quest timer > 0
- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 3 ≥ 3

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]] is taken
- Zuly, base 2000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5018-31`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–15
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-32`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–17
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-33`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–19
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-34`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–21
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-35`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–23
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-36`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–25
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-37`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–30
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

### `5018-38`

Checks:

- you have [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- a random roll 0–99 lands in 0–35
- you carry < 20 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]

Then:

- works on [[quests/5018-candle-ghost-s-identity-fetch-quest|Candle Ghost's Identity (Fetch Quest)]]
- you get 1 × [[items/quest/809-candle-ghost-mask|Candle Ghost Mask]]
