---
kind: quest
id: 107
name: Healing Hands
status: in-game
npcs:
- '[[npcs/1034-smith-ronk|Smith Ronk]]'
- '[[npcs/1035-little-street-vendor-pony|Little Street Vendor Pony]]'
- '[[npcs/1037-old-fisherman-myad|Old Fisherman Myad]]'
time_limit_minutes: 15
steps: 6
source:
  data: LIST_QUEST.STB row 107; QSD triggers 106-01, 107-01, 107-02, 107-03, 107-04, 107-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Healing Hands

Myad has asked you to save Pony and Ronk. You're running out of time, so you've got to bring the antidote to them.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `106-01`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/106-healing-hands|Healing Hands]]
- you carry = 0 × [[items/quest/603-soil-of-purification|Soil of Purification]]
- the quest timer > 0

Then:

- works on [[quests/106-healing-hands|Healing Hands]]
- experience, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/103-grapes|Grapes]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/107-healing-hands|Healing Hands]] (progress kept)
- set episode variable 0 to 4

### `107-01`

Happens by talking to [[npcs/1034-smith-ronk|Smith Ronk]].

Checks:

- you have [[quests/107-healing-hands|Healing Hands]]
- you carry ≥ 1 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]]
- quest switch 0 = 0
- the quest timer > 0

Then:

- works on [[quests/107-healing-hands|Healing Hands]]
- 1 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]] is taken
- set quest switch 0 to 1

### `107-02`

Happens by talking to [[npcs/1035-little-street-vendor-pony|Little Street Vendor Pony]].

Checks:

- you have [[quests/107-healing-hands|Healing Hands]]
- you carry ≥ 1 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]]
- quest switch 1 = 0
- the quest timer > 0

Then:

- works on [[quests/107-healing-hands|Healing Hands]]
- 1 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]] is taken
- set quest switch 1 to 1

### `107-03`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/107-healing-hands|Healing Hands]]
- the quest timer > 0
- you carry = 0 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]]
- quest switch 0 = 1
- quest switch 1 = 1

Then:

- works on [[quests/107-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]
- you get [[items/consumable/105-orange|Orange]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/head/5-islamic-bandana|Islamic Bandana]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/hands/5-safe-gloves|Safe Gloves]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/feet/5-dash-shoes|Dash Shoes]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- set episode variable 0 to 5
- then runs step `107-09`

### `107-04`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/107-healing-hands|Healing Hands]]
- the quest timer > 0
- you carry = 0 × [[items/quest/604-karitte-s-antidote|Karitte's Antidote]]
- quest switch 0 = 1
- quest switch 1 = 1

Then:

- works on [[quests/107-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]
- experience, base 400 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/head/5-islamic-bandana|Islamic Bandana]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/body/5-yellow-wild-jeans|Yellow Wild Jeans]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/hands/5-safe-gloves|Safe Gloves]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/feet/5-dash-shoes|Dash Shoes]], base count 1 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- set episode variable 0 to 5
- then runs step `107-09`

### `107-05`

Happens by talking to [[npcs/1037-old-fisherman-myad|Old Fisherman Myad]].

Checks:

- you have [[quests/107-healing-hands|Healing Hands]]
- the quest timer ≤ 0

Then:

- works on [[quests/107-healing-hands|Healing Hands]]
- you get 1 × [[items/quest/605-antidote-recipe|Antidote Recipe]]
- experience, base 250 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- set episode variable 0 to 5
- then runs step `107-09`
