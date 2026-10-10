---
kind: quest
id: 3401
name: Violence by Porkies
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3401; QSD triggers 3401-01, 3401-02, 3401-03, 3401-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Violence by Porkies

Porkies have recently been bullying and tormenting the innocent. Let's show them what we do to oppressors, and kill 17 of them!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3401-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3

Then:

- you get the quest [[quests/3401-violence-by-porkies|Violence by Porkies]]

### `3401-02`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3401-violence-by-porkies|Violence by Porkies]]
- your Faction = 3
- your Level ≤ 50
- you carry ≥ 17 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3401-violence-by-porkies|Violence by Porkies]]
- 17 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 1 to your UnionPoint3
- experience: 80 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3401-03` is tried instead.

### `3401-03`

Checks:

- you have [[quests/3401-violence-by-porkies|Violence by Porkies]]
- your Faction = 3
- your Level > 50
- you carry ≥ 17 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3401-violence-by-porkies|Violence by Porkies]]
- 17 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 1 to your UnionPoint3
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3401-31`

Checks:

- you have [[quests/3401-violence-by-porkies|Violence by Porkies]]
- you carry < 17 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3401-violence-by-porkies|Violence by Porkies]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `3801-31` is tried instead.
