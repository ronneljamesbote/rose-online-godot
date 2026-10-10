---
kind: quest
id: 3807
name: New Recovery Potion
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3807; QSD triggers 3807-01, 3807-02, 3807-03, 3807-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# New Recovery Potion

The Ferrell Guild has decided to create new healing potions to help their long time allies, the Righteous Crusaders, in the Faction Wars. Do your part by bringing 30 units of Sticky Fluid that can be obtained from Kaiman Warriors.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3807-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5
- your Level ≥ 55

Then:

- you get the quest [[quests/3807-new-recovery-potion|New Recovery Potion]]

### `3807-02`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3807-new-recovery-potion|New Recovery Potion]]
- your Faction = 5
- your Level ≤ 70
- you carry ≥ 30 × [[items/quest/317-sticky-fluid|Sticky Fluid]]

Then:

- works on [[quests/3807-new-recovery-potion|New Recovery Potion]]
- 30 × [[items/quest/317-sticky-fluid|Sticky Fluid]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 8 to your UnionPoint5
- the quest ends (removed from your list)

If the checks fail, step `3807-03` is tried instead.

### `3807-03`

Checks:

- you have [[quests/3807-new-recovery-potion|New Recovery Potion]]
- your Faction = 5
- your Level > 70
- you carry ≥ 30 × [[items/quest/317-sticky-fluid|Sticky Fluid]]

Then:

- works on [[quests/3807-new-recovery-potion|New Recovery Potion]]
- 30 × [[items/quest/317-sticky-fluid|Sticky Fluid]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 3 to your UnionPoint5
- the quest ends (removed from your list)

### `3807-31`

Checks:

- you have [[quests/3807-new-recovery-potion|New Recovery Potion]]
- a random roll 0–99 lands in 0–35
- you carry < 30 × [[items/quest/317-sticky-fluid|Sticky Fluid]]

Then:

- works on [[quests/3807-new-recovery-potion|New Recovery Potion]]
- you get 1 × [[items/quest/317-sticky-fluid|Sticky Fluid]]
