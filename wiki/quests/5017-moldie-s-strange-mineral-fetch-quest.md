---
kind: quest
id: 5017
name: Moldie's Strange Mineral (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]'
monsters:
- '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
time_limit_minutes: 150
steps: 13
source:
  data: LIST_QUEST.STB row 5017; QSD triggers 5017-01, 5017-02, 5017-03, 5017-04, 5017-05, 5017-06, 5017-07, 5017-31, 5017-32, 5017-33, 5017-34, 5017-35, 5017-36
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Moldie's Strange Mineral (Fetch Quest)

Kiroth has told you that Moldies have been gathering some strange mineral. If you bring 10 Green Minerals and 10 Black Minerals to Kiroth, he will reward you according to your performance compared to your rivals.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5017-01`

Happens by talking to [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]].

Checks:

- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 0 = 1
- your Level ≤ 43

Then:

- you get the quest [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- add 1 to the NPC's variable 2
- then runs step `5017-02`

### `5017-02`

Checks:

- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2
- the NPC says: "We are no longer accepting participants for Candle Ghost's Identity Quest! (Rival) Please try again later~"

### `5017-03`

Happens by talking to [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]].

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- the quest timer > 0

### `5017-04`

Happens by talking to [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]].

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/807-black-mineral|Black Mineral]]
- you carry ≥ 10 × [[items/quest/808-green-mineral|Green Mineral]]
- the quest timer > 0
- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 3 = 0

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- 10 × [[items/quest/807-black-mineral|Black Mineral]] is taken
- 10 × [[items/quest/808-green-mineral|Green Mineral]] is taken
- Zuly, base 15000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5017-05` is tried instead.

### `5017-05`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/807-black-mineral|Black Mineral]]
- you carry ≥ 10 × [[items/quest/808-green-mineral|Green Mineral]]
- the quest timer > 0
- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 3 = 1

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- 10 × [[items/quest/807-black-mineral|Black Mineral]] is taken
- 10 × [[items/quest/808-green-mineral|Green Mineral]] is taken
- Zuly, base 10000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/58-vital-jam-5|Vital Jam (+5)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5017-06` is tried instead.

### `5017-06`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/807-black-mineral|Black Mineral]]
- you carry ≥ 10 × [[items/quest/808-green-mineral|Green Mineral]]
- the quest timer > 0
- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 3 = 2

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- 10 × [[items/quest/807-black-mineral|Black Mineral]] is taken
- 10 × [[items/quest/808-green-mineral|Green Mineral]] is taken
- Zuly, base 5000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5017-07` is tried instead.

### `5017-07`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you carry ≥ 10 × [[items/quest/807-black-mineral|Black Mineral]]
- you carry ≥ 10 × [[items/quest/808-green-mineral|Green Mineral]]
- the quest timer > 0
- the NPC [[npcs/1098-ferrell-guild-staff-kiroth|Ferrell Guild Staff Kiroth]]
- the NPC's variable 3 ≥ 3

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- 10 × [[items/quest/807-black-mineral|Black Mineral]] is taken
- 10 × [[items/quest/808-green-mineral|Green Mineral]] is taken
- Zuly, base 1000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5017-31`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–10
- you carry < 10 × [[items/quest/807-black-mineral|Black Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/807-black-mineral|Black Mineral]]

If the checks fail, step `5017-32` is tried instead.

### `5017-32`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–10
- you carry < 10 × [[items/quest/808-green-mineral|Green Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/808-green-mineral|Green Mineral]]

### `5017-33`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–15
- you carry < 10 × [[items/quest/808-green-mineral|Green Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/808-green-mineral|Green Mineral]]

If the checks fail, step `5017-34` is tried instead.

### `5017-34`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–15
- you carry < 10 × [[items/quest/807-black-mineral|Black Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/807-black-mineral|Black Mineral]]

If the checks fail, step `5011-32` is tried instead.

### `5017-35`

Happens by killing [[monsters/113-gold-mine-moldie|Gold Mine Moldie]].

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–20
- you carry < 10 × [[items/quest/808-green-mineral|Green Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/808-green-mineral|Green Mineral]]

If the checks fail, step `5017-36` is tried instead.

### `5017-36`

Checks:

- you have [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- a random roll 0–99 lands in 0–20
- you carry < 10 × [[items/quest/807-black-mineral|Black Mineral]]

Then:

- works on [[quests/5017-moldie-s-strange-mineral-fetch-quest|Moldie's Strange Mineral (Fetch Quest)]]
- you get 1 × [[items/quest/807-black-mineral|Black Mineral]]

If the checks fail, step `5011-33` is tried instead.
