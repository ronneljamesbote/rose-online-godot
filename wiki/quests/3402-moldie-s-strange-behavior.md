---
kind: quest
id: 3402
name: Moldie's Strange Behavior
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
monsters:
- '[[monsters/111-moldie|Moldie]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3402; QSD triggers 3402-01, 3402-02, 3402-03, 3402-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Moldie's Strange Behavior

The Moldies that have left the Coal Mines haven't been able to adapt to the outside world. They shouldn't be wandering around and bothering people, so go out and punish 30 of them.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3402-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3

Then:

- you get the quest [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]

### `3402-02`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- your Faction = 3
- your Level ≤ 50
- you carry ≥ 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 3 to your UnionPoint3
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3402-03` is tried instead.

### `3402-03`

Checks:

- you have [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- your Faction = 3
- your Level > 50
- you carry = 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- add 1 to your UnionPoint3
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3402-31`

Happens by killing [[monsters/111-moldie|Moldie]].

Checks:

- you have [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- you carry < 30 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3402-moldie-s-strange-behavior|Moldie's Strange Behavior]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5011-31` is tried instead.
