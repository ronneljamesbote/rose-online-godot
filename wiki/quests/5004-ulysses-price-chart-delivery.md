---
kind: quest
id: 5004
name: Ulysses' Price Chart Delivery
status: in-game
given_by:
- '[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]'
npcs:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
steps: 4
source:
  data: LIST_QUEST.STB row 5004; QSD triggers 5004-01, 5004-02, 5004-03, 5004-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Ulysses' Price Chart Delivery

Ulysses has asked you to deliver his Daily Price Checklist to Seyon in the Adventurer's Plains.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5004-01`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- your Level ≤ 20
- the NPC [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]
- the NPC is within 0 m

Then:

- you get the quest [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- works on [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- you get 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]

### `5004-02`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- your Level ≤ 25
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- the NPC [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]
- the NPC is within 0 m

Then:

- works on [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `5004-04` is tried instead.

### `5004-03`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- you have [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- the NPC [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]
- the NPC is within 0 m

Then:

- works on [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience: 100 (XP, scaled by your level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `5031-01`

### `5004-04`

Checks:

- you have [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- your Level > 25
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- the NPC [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]
- the NPC is within 0 m

Then:

- works on [[quests/5004-ulysses-price-chart-delivery|Ulysses' Price Chart Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience: 4000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
