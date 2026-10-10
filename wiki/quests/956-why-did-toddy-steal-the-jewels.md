---
kind: quest
id: 956
name: Why Did Toddy Steal the Jewels?
status: in-game
npcs:
- '[[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]]'
steps: 6
source:
  data: LIST_QUEST.STB row 956; QSD triggers 955-01, 956-01, 956-02, 956-03, 956-08, 956-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Why Did Toddy Steal the Jewels?

Mina said that his Crystal Necklaces haven't been found yet. Find 7 Crystal Necklaces which were probably picked up by Beetle Fighters.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `955-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you carry ≥ 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]]

Then:

- works on [[quests/955-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- 10 × [[items/quest/113-mina-s-ruby|Mina's Ruby]] is taken
- the quest becomes [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]

### `956-01`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- job variable 0 = 1
- you carry ≥ 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

Then:

- works on [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]] is taken
- add 1 to job variable 0
- you get [[items/weapon/204-orc-bow|Orc Bow]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `956-02`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- job variable 0 = 1
- you carry ≥ 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

Then:

- works on [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]] is taken
- add 1 to job variable 0
- you get [[items/weapon/403-katar|Katar]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `956-03`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- job variable 0 = 1
- you carry ≥ 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

Then:

- works on [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]] is taken
- add 1 to job variable 0
- Zuly, base 1800 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `956-08`

Happens by talking to [[npcs/1007-gypsy-jewel-seller-mina|Gypsy Jewel Seller Mina]].

Checks:

- you have [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you carry < 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

Then:

- works on [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- the quest ends (removed from your list)

### `956-31`

Checks:

- you have [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- a random roll 0–99 lands in 0–30
- you carry < 7 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

Then:

- works on [[quests/956-why-did-toddy-steal-the-jewels|Why Did Toddy Steal the Jewels?]]
- you get 1 × [[items/quest/114-crystal-necklace|Crystal Necklace]]

If the checks fail, step `5007-35` is tried instead.
