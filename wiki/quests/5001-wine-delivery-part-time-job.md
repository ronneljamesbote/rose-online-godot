---
kind: quest
id: 5001
name: Wine Delivery (Part Time Job)
status: in-game
given_by:
- '[[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]'
npcs:
- '[[npcs/1062-smith-punwell|Smith Punwell]]'
time_limit_minutes: 30
steps: 4
source:
  data: LIST_QUEST.STB row 5001; QSD triggers 5001-01, 5001-02, 5001-03, 5001-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Wine Delivery (Part Time Job)

Sharlin asked you if you can bring two bottles of wine to Punwell in the Breezy Hills. You have a total of 30 minutes to finish this delivery.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5001-01`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]].

Checks:

- your Level ≤ 18
- the NPC [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]]
- the NPC's variable 0 = 1

Then:

- you get the quest [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- works on [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- you get 2 × [[items/quest/801-red-wine|Red Wine]]
- add 1 to the NPC's variable 1

### `5001-02`

Happens by talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- you carry = 2 × [[items/quest/801-red-wine|Red Wine]]

Then:

- works on [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- 2 × [[items/quest/801-red-wine|Red Wine]] is taken
- Zuly, base 1500 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5001-03`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]], talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- the quest timer = 0

Then:

- the quest ends (removed from your list)

### `5001-04`

Happens by talking to [[npcs/1013-tavern-owner-sharlin|Tavern Owner Sharlin]], talking to [[npcs/1062-smith-punwell|Smith Punwell]].

Checks:

- you have [[quests/5001-wine-delivery-part-time-job|Wine Delivery (Part Time Job)]]
- the quest timer > 0
