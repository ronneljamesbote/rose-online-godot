---
kind: quest
id: 3606
name: Magic Weapon Production
status: in-game
given_by:
- '[[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3606; QSD triggers 3606-01, 3606-02, 3606-03, 3606-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Magic Weapon Production

The Righteous Crusaders have been waiting for their chance to crush our allies, the Junon Order. The Arumics have to help the Junon Order, and contribute to battle preparations. Go and defeat Doongas, and bring back 30 Doonga Claws to help make magic weapons.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3606-01`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- your Faction = 4
- your Level ≥ 51

Then:

- you get the quest [[quests/3606-magic-weapon-production|Magic Weapon Production]]

### `3606-02`

Happens by talking to [[npcs/1112-arumic-researcher-carasia|Arumic Researcher Carasia]].

Checks:

- you have [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- your Faction = 4
- your Level ≤ 60
- you carry ≥ 30 × [[items/quest/313-doonga-claw|Doonga Claw]]

Then:

- works on [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- 30 × [[items/quest/313-doonga-claw|Doonga Claw]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 6 to your UnionPoint4
- the quest ends (removed from your list)

If the checks fail, step `3606-03` is tried instead.

### `3606-03`

Checks:

- you have [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- your Faction = 4
- your Level > 60
- you carry ≥ 30 × [[items/quest/313-doonga-claw|Doonga Claw]]

Then:

- works on [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- 30 × [[items/quest/313-doonga-claw|Doonga Claw]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to your UnionPoint4
- the quest ends (removed from your list)

### `3606-31`

Checks:

- you have [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- you carry < 30 × [[items/quest/313-doonga-claw|Doonga Claw]]
- your Faction = 4
- a random roll 0–99 lands in 0–40

Then:

- works on [[quests/3606-magic-weapon-production|Magic Weapon Production]]
- you get 1 × [[items/quest/313-doonga-claw|Doonga Claw]]

If the checks fail, step `5013-31` is tried instead.
