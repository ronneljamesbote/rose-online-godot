---
kind: quest
id: 5053
name: Hunting Practice (3)
status: in-game
given_by:
- '[[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]]'
monsters:
- '[[monsters/12-choropy|Choropy]]'
time_limit_minutes: 10
steps: 5
source:
  data: LIST_QUEST.STB row 5053; QSD triggers 5053-31, 5053-32, 5053-33, 5053-34, T79
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hunting Practice (3)

The Fairy of Arua suggested that you defeat 8 Choropies and Mini-Choropies in preparation for your real adventures. Complete this task and you'll receive a small reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5053-31`

Checks:

- you have [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- the quest timer > 0
- you carry < 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5053-32` is tried instead.

### `5053-32`

Checks:

- you have [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- the quest timer > 0
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- Zuly, base 400 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/102-banana|Banana]], base count 20 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 79 on
- the quest ends (removed from your list)

If the checks fail, step `5034-32` is tried instead.

### `5053-33`

Happens by killing [[monsters/12-choropy|Choropy]].

Checks:

- you have [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- the quest timer > 0
- you carry < 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5053-34` is tried instead.

### `5053-34`

Checks:

- you have [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- the quest timer > 0
- you carry ≥ 7 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
- Zuly, base 400 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/102-banana|Banana]], base count 20 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 79 on
- the quest ends (removed from your list)

If the checks fail, step `5052-33` is tried instead.

### `T79`

Happens by talking to [[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]].

Checks:

- quest switch 79 is off

Then:

- you get the quest [[quests/5053-hunting-practice-3|Hunting Practice (3)]]
