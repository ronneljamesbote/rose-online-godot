---
kind: quest
id: 3805
name: Arms Distribution Effort
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3805; QSD triggers 3805-01, 3805-02, 3805-03, 3805-04, 3805-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Arms Distribution Effort

As the Ferrell Guild becomes more frequently targeted for robbery, its merchants must be equipped with guns to protect themselves. For this purpose, collect 17 Iron Fragments by hunting Grunter Warriors to help create more guns.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3805-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5
- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC's variable 0 = 7
- the NPC's variable 6 < 10

Then:

- you get the quest [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- add 1 to the NPC's variable 6
- then runs step `3805-02`

### `3805-02`

Checks:

- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC's variable 0 = 7
- the NPC's variable 6 ≥ 10

Then:

- set the NPC's variable 0 to 8
- set the NPC's variable 6 to 0

### `3805-03`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- your Faction = 5
- your Level ≤ 70
- you carry ≥ 17 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- 17 × [[items/quest/22-iron-fragment|Iron Fragment]] is taken
- add 8 to your UnionPoint5
- experience, base 120 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3805-04` is tried instead.

### `3805-04`

Checks:

- you have [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- your Faction = 5
- your Level > 70
- you carry ≥ 17 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- 17 × [[items/quest/22-iron-fragment|Iron Fragment]] is taken
- add 3 to your UnionPoint5
- experience, base 12000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `3805-31`

Checks:

- you have [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- a random roll 0–99 lands in 0–30
- you carry < 17 × [[items/quest/22-iron-fragment|Iron Fragment]]

Then:

- works on [[quests/3805-arms-distribution-effort|Arms Distribution Effort]]
- you get 1 × [[items/quest/22-iron-fragment|Iron Fragment]]

If the checks fail, step `3405-31` is tried instead.
