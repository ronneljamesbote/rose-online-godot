---
kind: quest
id: 5003
name: Dalping Hunt (Fetch Quest)
status: in-game
given_by:
- '[[npcs/1010-designer-keenu|Designer Keenu]]'
monsters:
- '[[monsters/32-ranger-dalping|Ranger Dalping]]'
time_limit_minutes: 116
steps: 6
source:
  data: LIST_QUEST.STB row 5003; QSD triggers 5003-01, 5003-02, 5003-03, 5003-04, 5003-10, 5003-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Dalping Hunt (Fetch Quest)

Keenu has a weakness for Dalping Eggs. Bring him 13 Dalping Eggs, and he'll give you a reward depending on how much sooner you bring him the eggs than your rivals.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5003-01`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- your Level ≤ 27
- the NPC [[npcs/1010-designer-keenu|Designer Keenu]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 10

Then:

- you get the quest [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- add 1 to the NPC's variable 2
- then runs step `5003-10`

### `5003-02`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- you carry ≥ 13 × [[items/quest/802-dalping-egg|Dalping Egg]]
- the NPC [[npcs/1010-designer-keenu|Designer Keenu]]
- the NPC's variable 3 ≤ 2

Then:

- works on [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- 13 × [[items/quest/802-dalping-egg|Dalping Egg]] is taken
- Zuly, base 3500 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

If the checks fail, step `5003-03` is tried instead.

### `5003-03`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- you carry ≥ 13 × [[items/quest/802-dalping-egg|Dalping Egg]]
- the NPC [[npcs/1010-designer-keenu|Designer Keenu]]
- the NPC's variable 3 > 2

Then:

- works on [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- 13 × [[items/quest/802-dalping-egg|Dalping Egg]] is taken
- Zuly, base 700 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 1 to the NPC's variable 3
- the quest ends (removed from your list)

### `5003-04`

Happens by talking to [[npcs/1010-designer-keenu|Designer Keenu]].

Checks:

- you have [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- the quest timer > 0

### `5003-10`

Checks:

- the NPC [[npcs/1010-designer-keenu|Designer Keenu]]
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2

### `5003-31`

Happens by killing [[monsters/32-ranger-dalping|Ranger Dalping]].

Checks:

- you have [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- a random roll 0–99 lands in 0–80
- you carry < 13 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/5003-dalping-hunt-fetch-quest|Dalping Hunt (Fetch Quest)]]
- you get 1 × [[items/quest/802-dalping-egg|Dalping Egg]]

If the checks fail, step `1007-32` is tried instead.
