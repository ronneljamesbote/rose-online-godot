---
kind: quest
id: 3806
name: Reassuring Support
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3806; QSD triggers 3806-01, 3806-02, 3806-03, 3806-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Reassuring Support

The Ferrell Guild can't idly stand by while the Junon Order targets the Righteous Crusaders! Help our allies by defeating Elder Doongas to collect 20 Doonga's Sausages for emergency rations.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3806-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5
- your Level ≥ 51

Then:

- you get the quest [[quests/3806-reassuring-support|Reassuring Support]]

### `3806-02`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3806-reassuring-support|Reassuring Support]]
- your Faction = 5
- your Level ≤ 60
- you carry ≥ 20 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]]

Then:

- works on [[quests/3806-reassuring-support|Reassuring Support]]
- 20 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 6 to your UnionPoint5
- the quest ends (removed from your list)

If the checks fail, step `3806-03` is tried instead.

### `3806-03`

Checks:

- you have [[quests/3806-reassuring-support|Reassuring Support]]
- your Faction = 5
- your Level > 60
- you carry ≥ 20 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]]

Then:

- works on [[quests/3806-reassuring-support|Reassuring Support]]
- 20 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 2 to your UnionPoint5
- the quest ends (removed from your list)

### `3806-31`

Checks:

- you have [[quests/3806-reassuring-support|Reassuring Support]]
- a random roll 0–99 lands in 0–24
- your Faction = 5
- you carry < 20 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]]

Then:

- works on [[quests/3806-reassuring-support|Reassuring Support]]
- you get 1 × [[items/quest/314-doonga-s-sausage|Doonga's Sausage]]

If the checks fail, step `5013-32` is tried instead.
