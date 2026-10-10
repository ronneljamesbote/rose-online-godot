---
kind: quest
id: 857
name: Living as a True Soldier
status: in-game
npcs:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
monsters:
- '[[monsters/68-queen-bibi|Queen Bibi]]'
steps: 5
source:
  data: LIST_QUEST.STB row 857; QSD triggers 856-01, 857-01, 857-02, 857-03, 857-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Living as a True Soldier

Your final exercise is to fight Queen Bibis and bring 4 Queen Bibi's Staffs to Raffle.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `856-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- quest variable 0 ≥ 15
- quest variable 1 ≥ 15

Then:

- works on [[quests/856-living-as-a-true-soldier|Living as a True Soldier]]
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]

### `857-01`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- you carry ≥ 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]]

Then:

- works on [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]] is taken
- add 2 to job variable 0
- you get [[items/weapon/5-long-sword|Long Sword]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `857-02`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- you carry ≥ 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]]

Then:

- works on [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]] is taken
- add 2 to job variable 0
- you get [[items/weapon/164-scimitar|Scimitar]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `857-03`

Happens by talking to [[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]].

Checks:

- you have [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- you carry ≥ 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]]

Then:

- works on [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]] is taken
- add 2 to job variable 0
- you get [[items/weapon/132-small-axe|Small Axe]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `857-31`

Happens by killing [[monsters/68-queen-bibi|Queen Bibi]].

Checks:

- you have [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- you carry < 4 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]]

Then:

- works on [[quests/857-living-as-a-true-soldier|Living as a True Soldier]]
- you get 1 × [[items/quest/131-queen-bibi-s-staff|Queen Bibi's Staff]]

If the checks fail, step `118-31` is tried instead.
