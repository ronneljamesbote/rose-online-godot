---
kind: quest
id: 3603
name: Grunter's Mysterious Strength
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
monsters:
- '[[monsters/135-grunter-fighter|Grunter Fighter]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3603; QSD triggers 3603-01, 3603-02, 3603-03, 3603-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Grunter's Mysterious Strength

The Arumics have decided to research the Grunter Fighters' mysterious strength in an attempt to discover the source of their power. Help the research efforts by collecting 14 Grunter Feet.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3603-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4

Then:

- you get the quest [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]

### `3603-02`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- your Faction = 4
- your Level ≤ 50
- you carry ≥ 14 × [[items/quest/307-grunter-foot|Grunter Foot]]

Then:

- works on [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- 14 × [[items/quest/307-grunter-foot|Grunter Foot]] is taken
- add 4 to your UnionPoint4
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3603-03` is tried instead.

### `3603-03`

Checks:

- you have [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- your Faction = 4
- your Level > 50
- you carry ≥ 14 × [[items/quest/307-grunter-foot|Grunter Foot]]

Then:

- works on [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- 14 × [[items/quest/307-grunter-foot|Grunter Foot]] is taken
- add 1 to your UnionPoint4
- experience: 10000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3603-31`

Happens by killing [[monsters/135-grunter-fighter|Grunter Fighter]].

Checks:

- you have [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- a random roll 0–99 lands in 0–30
- you carry < 14 × [[items/quest/307-grunter-foot|Grunter Foot]]

Then:

- works on [[quests/3603-grunter-s-mysterious-strength|Grunter's Mysterious Strength]]
- you get 1 × [[items/quest/307-grunter-foot|Grunter Foot]]
