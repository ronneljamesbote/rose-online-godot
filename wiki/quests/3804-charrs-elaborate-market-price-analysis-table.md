---
kind: quest
id: 3804
name: Charrs' Elaborate Market Price Analysis Table
status: in-game
given_by:
- '[[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]'
npcs:
- '[[npcs/1061-ferrell-guild-staff-belz|Ferrell Guild Staff Belz]]'
steps: 5
source:
  data: LIST_QUEST.STB row 3804; QSD triggers 3804-01, 3804-02, 3804-03, 3804-04, 3804-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Charrs' Elaborate Market Price Analysis Table

Belz, in the Breezy Hills, has requested Charrs' Market Price Analysis Table. Your job is to make sure it's delivered to Belz.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3804-01`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- your Faction = 5
- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 10
- the NPC is within 0 m

Then:

- you get the quest [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- works on [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- you get 1 × [[items/quest/310-market-price-analysis-table|Market Price Analysis Table]]
- set quest variable 0 to 10
- add 1 to the NPC's variable 2
- then runs step `3804-02`

### `3804-02`

Checks:

- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC's variable 0 = 1
- the NPC's variable 2 ≥ 10

Then:

- set the NPC's variable 0 to 2
- set the NPC's variable 2 to 0

### `3804-03`

Happens by talking to [[npcs/1061-ferrell-guild-staff-belz|Ferrell Guild Staff Belz]].

Checks:

- you have [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- you carry = 1 × [[items/quest/310-market-price-analysis-table|Market Price Analysis Table]]
- quest variable 0 = 10
- the NPC [[npcs/1061-ferrell-guild-staff-belz|Ferrell Guild Staff Belz]]
- the NPC is within 0 m

Then:

- works on [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- 1 × [[items/quest/310-market-price-analysis-table|Market Price Analysis Table]] is taken
- add 10 to quest variable 0

### `3804-04`

Happens by talking to [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]].

Checks:

- you have [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- quest variable 0 = 20
- your Faction = 5
- your Level ≤ 50
- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC is within 0 m

Then:

- works on [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- add 5 to your UnionPoint5
- experience, base 80 (reward formula 1: grows with your level and Charm, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `3804-05` is tried instead.

### `3804-05`

Checks:

- you have [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- quest variable 0 = 20
- your Faction = 5
- your Level > 50
- the NPC [[npcs/1110-ferrell-guild-staff-charrs|Ferrell Guild Staff Charrs]]
- the NPC is within 0 m

Then:

- works on [[quests/3804-charrs-elaborate-market-price-analysis-table|Charrs' Elaborate Market Price Analysis Table]]
- add 2 to your UnionPoint5
- experience, base 5000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/57-vital-jam-2|Vital Jam (+2)]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
