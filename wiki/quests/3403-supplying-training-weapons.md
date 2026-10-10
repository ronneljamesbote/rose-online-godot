---
kind: quest
id: 3403
name: Supplying Training Weapons
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
monsters:
- '[[monsters/134-grunter|Grunter]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3403; QSD triggers 3403-01, 3403-02, 3403-03, 3403-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Supplying Training Weapons

Recently, we haven't been able to provide enough training weapons for new recruits in the Righteous Crusaders. For this purpose, go out and bring back 16 Grunter Axes.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3403-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3

Then:

- you get the quest [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]

### `3403-02`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- your Faction = 3
- your Level ≤ 50
- you carry ≥ 16 × [[items/quest/312-grunter-axe|Grunter Axe]]

Then:

- works on [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- 16 × [[items/quest/312-grunter-axe|Grunter Axe]] is taken
- add 4 to your UnionPoint3
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3403-03` is tried instead.

### `3403-03`

Checks:

- you have [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- your Faction = 3
- your Level > 50
- you carry ≥ 16 × [[items/quest/312-grunter-axe|Grunter Axe]]

Then:

- works on [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- 16 × [[items/quest/312-grunter-axe|Grunter Axe]] is taken
- add 1 to your UnionPoint3
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3403-31`

Happens by killing [[monsters/134-grunter|Grunter]].

Checks:

- you have [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- a random roll 0–99 lands in 0–30
- you carry < 16 × [[items/quest/312-grunter-axe|Grunter Axe]]

Then:

- works on [[quests/3403-supplying-training-weapons|Supplying Training Weapons]]
- you get 1 × [[items/quest/312-grunter-axe|Grunter Axe]]
