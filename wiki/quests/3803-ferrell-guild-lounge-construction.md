---
kind: quest
id: 3803
name: Ferrell Guild Lounge Construction
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3803; QSD triggers 3803-01, 3803-02, 3803-03, 3803-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Ferrell Guild Lounge Construction

The Ferrell Guild isn't all business: apparently our leaders see the need for a place to relax. Help out with the lounge construction by hunting Smoulies to collect 12 Thick Pulpwood.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3803-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5

Then:

- you get the quest [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]

### `3803-02`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- your Faction = 5
- your Level ≤ 50
- you carry ≥ 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

Then:

- works on [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]] is taken
- add 4 to your UnionPoint5
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3803-03` is tried instead.

### `3803-03`

Checks:

- you have [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- your Faction = 5
- your Level > 50
- you carry ≥ 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

Then:

- works on [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]] is taken
- add 1 to your UnionPoint5
- experience: 5000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3803-31`

Checks:

- you have [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- a random roll 0–99 lands in 0–30
- you carry < 12 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

Then:

- works on [[quests/3803-ferrell-guild-lounge-construction|Ferrell Guild Lounge Construction]]
- you get 1 × [[items/quest/302-thick-pulpwood|Thick Pulpwood]]

If the checks fail, step `5012-31` is tried instead.
