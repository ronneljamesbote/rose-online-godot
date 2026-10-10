---
kind: quest
id: 3405
name: Supplying Training Gloves
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3405; QSD triggers 3405-01, 3405-02, 3405-03, 3405-04, 3405-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Supplying Training Gloves

Lately, we've been having trouble supplying training equipment for new recruits in the Righteous Crusaders. Help us out by hunting Grunter Warriors and bringing back 8 Training Gloves.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3405-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3
- the NPC [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]
- the NPC's variable 0 = 10
- the NPC's variable 8 < 10

Then:

- you get the quest [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- add 1 to the NPC's variable 8
- then runs step `3405-02`

### `3405-02`

Checks:

- the NPC [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]
- the NPC's variable 0 = 10
- the NPC's variable 8 ≥ 10

Then:

- set the NPC's variable 0 to 11
- set the NPC's variable 8 to 0

### `3405-03`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- your Faction = 3
- your Level ≤ 70
- you carry ≥ 8 × [[items/quest/305-training-gloves|Training Gloves]]

Then:

- works on [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- 8 × [[items/quest/305-training-gloves|Training Gloves]] is taken
- add 8 to your UnionPoint3
- experience, base 120 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3405-04` is tried instead.

### `3405-04`

Checks:

- you have [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- your Faction = 3
- your Level > 70
- you carry ≥ 8 × [[items/quest/305-training-gloves|Training Gloves]]

Then:

- works on [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- 8 × [[items/quest/305-training-gloves|Training Gloves]] is taken
- add 3 to your UnionPoint3
- experience, base 12000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3405-31`

Checks:

- you have [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- a random roll 0–99 lands in 0–15
- you carry < 8 × [[items/quest/305-training-gloves|Training Gloves]]

Then:

- works on [[quests/3405-supplying-training-gloves|Supplying Training Gloves]]
- you get 1 × [[items/quest/305-training-gloves|Training Gloves]]
