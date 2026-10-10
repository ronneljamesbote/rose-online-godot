---
kind: quest
id: 3404
name: Lawless Porkie Hooligans
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3404; QSD triggers 3404-01, 3404-02, 3404-03, 3404-04, 3404-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lawless Porkie Hooligans

Lately, word has been going around that those Porkies have been mugging and harassing the innocent. Let's kill at least 30 Porkie Hooligans and show them we mean business.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3404-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3
- the NPC [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]
- the NPC's variable 0 = 4
- the NPC's variable 4 < 10

Then:

- you get the quest [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- add 1 to the NPC's variable 4
- then runs step `3404-02`

### `3404-02`

Checks:

- the NPC [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]
- the NPC's variable 0 = 4
- the NPC's variable 4 ≥ 10

Then:

- set the NPC's variable 0 to 5
- set the NPC's variable 4 to 0

### `3404-03`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- your Faction = 3
- your Level ≤ 50
- you carry ≥ 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 5 to your UnionPoint3
- experience: 80 (XP, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

If the checks fail, step `3404-04` is tried instead.

### `3404-04`

Checks:

- you have [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- your Faction = 3
- your Level > 50
- you carry ≥ 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 2 to your UnionPoint3
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]] (item count: 1)
- the quest ends (removed from your list)

### `3404-31`

Checks:

- you have [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- you carry < 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3404-lawless-porkie-hooligans|Lawless Porkie Hooligans]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `3601-31` is tried instead.
