---
kind: quest
id: 1007
name: An Eye for the Marketplace
status: in-game
npcs:
- '[[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]]'
monsters:
- '[[monsters/31-dalping|Dalping]]'
steps: 12
source:
  data: LIST_QUEST.STB row 1007; QSD triggers 1006-04, 1007-01, 1007-02, 1007-03, 1007-04, 1007-05, 1007-06, 1007-07, 1007-08, 1007-09, 1007-31, 1007-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# An Eye for the Marketplace

Mairard was very pleased with the Cobra Leather you've given him. Now, if you bring him some Dalping Eggs, he will give you a good weapon.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `1006-04`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you carry ≥ 3 × [[items/quest/116-cobra-leather|Cobra Leather]]

Then:

- works on [[quests/1006-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- the quest becomes [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]] (progress kept)

### `1007-01`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 0
- quest switch 3 = 0
- you carry ≥ 10 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/5-long-sword|Long Sword]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-02` is tried instead.

### `1007-02`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 1
- you carry ≥ 12 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/5-long-sword|Long Sword]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-03` is tried instead.

### `1007-03`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 3 = 1
- you carry ≥ 15 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/5-long-sword|Long Sword]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1007-04`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 0
- quest switch 3 = 0
- you carry ≥ 10 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/233-gloria-gun|Gloria Gun]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-05` is tried instead.

### `1007-05`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 1
- you carry ≥ 12 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/233-gloria-gun|Gloria Gun]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-06` is tried instead.

### `1007-06`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 3 = 1
- you carry ≥ 15 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- you get [[items/weapon/233-gloria-gun|Gloria Gun]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1007-07`

Happens by talking to [[npcs/1064-weapon-craftsman-mairath|Weapon Craftsman Mairath]].

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 0
- quest switch 3 = 0
- you carry ≥ 10 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- Zuly, base 1800 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-08` is tried instead.

### `1007-08`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 2 = 1
- you carry ≥ 12 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- Zuly, base 1800 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

If the checks fail, step `1007-09` is tried instead.

### `1007-09`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- quest switch 3 = 1
- you carry ≥ 15 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- add 2 to job variable 0
- Zuly, base 1800 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `1007-31`

Happens by killing [[monsters/31-dalping|Dalping]].

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- a random roll 0–99 lands in 0–30
- you carry < 15 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you get 1 × [[items/quest/802-dalping-egg|Dalping Egg]]

If the checks fail, step `955-31` is tried instead.

### `1007-32`

Checks:

- you have [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- a random roll 0–99 lands in 0–50
- you carry < 15 × [[items/quest/802-dalping-egg|Dalping Egg]]

Then:

- works on [[quests/1007-an-eye-for-the-marketplace|An Eye for the Marketplace]]
- you get 1 × [[items/quest/802-dalping-egg|Dalping Egg]]

If the checks fail, step `955-32` is tried instead.
