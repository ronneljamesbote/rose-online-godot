---
kind: quest
id: 5031
name: Seyon's Price Checklist Delivery
status: in-game
given_by:
- '[[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]'
npcs:
- '[[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]'
steps: 4
source:
  data: LIST_QUEST.STB row 5031; QSD triggers 5031-01, 5031-02, 5031-03, 5031-04
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Seyon's Price Checklist Delivery

Seyon has asked you to deliver the Daily Price Check to Ulysses in Zant.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5031-01`

Happens by talking to [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]].

Checks:

- your Level ≤ 20
- the NPC [[npcs/1036-ferrell-guild-staff-seyon|Ferrell Guild Staff Seyon]]
- the NPC is within 0 m

Then:

- you get the quest [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- works on [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- you get 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]

### `5031-02`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- your Level ≤ 25
- the NPC [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]
- the NPC is within 0 m

Then:

- works on [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience, base 100 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `5031-04` is tried instead.

### `5031-03`

Happens by talking to [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]].

Checks:

- you have [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- the NPC [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]
- the NPC is within 0 m

Then:

- works on [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience, base 100 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `5004-01`

### `5031-04`

Checks:

- you have [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- your Level > 25
- you carry = 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]]
- the NPC [[npcs/1012-ferrell-guild-staff-ulysses|Ferrell Guild Staff Ulysses]]
- the NPC is within 0 m

Then:

- works on [[quests/5031-seyon-s-price-checklist-delivery|Seyon's Price Checklist Delivery]]
- 1 × [[items/quest/803-daily-price-checklist|Daily Price Checklist]] is taken
- experience, base 4000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
