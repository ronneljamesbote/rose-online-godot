---
kind: quest
id: 5052
name: Hunting Practice (2)
status: in-game
given_by:
- '[[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]]'
monsters:
- '[[monsters/11-mini-choropy|Mini-Choropy]]'
steps: 5
source:
  data: LIST_QUEST.STB row 5052; QSD triggers 5052-31, 5052-32, 5052-33, 5052-34, T78
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hunting Practice (2)

The Fairy of Arua suggested that you defeat 4 Mini-Choropies in preparation for your real adventures. Complete this task and you'll receive a small reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5052-31`

Happens by killing [[monsters/11-mini-choropy|Mini-Choropy]].

Checks:

- you have [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you carry < 3 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you get 1 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5052-32` is tried instead.

### `5052-32`

Checks:

- you have [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you carry ≥ 3 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- money: 250 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/104-kiwi|Kiwi]] (item count: 8)
- quest switch 78 on
- the quest ends (removed from your list)

If the checks fail, step `5053-31` is tried instead.

### `5052-33`

Checks:

- you have [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you carry < 3 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you get 1 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5052-34` is tried instead.

### `5052-34`

Checks:

- you have [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- you carry ≥ 3 × [[items/quest/499-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
- money: 250 (money, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/104-kiwi|Kiwi]] (item count: 8)
- quest switch 78 on
- the quest ends (removed from your list)

If the checks fail, step `5034-31` is tried instead.

### `T78`

Happens by talking to [[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]].

Checks:

- quest switch 78 is off

Then:

- you get the quest [[quests/5052-hunting-practice-2|Hunting Practice (2)]]
