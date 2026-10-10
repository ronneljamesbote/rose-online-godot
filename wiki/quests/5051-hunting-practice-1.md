---
kind: quest
id: 5051
name: Hunting Practice (1)
status: in-game
given_by:
- '[[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]]'
monsters:
- '[[monsters/2-jelly-bean|Jelly Bean]]'
steps: 5
source:
  data: LIST_QUEST.STB row 5051; QSD triggers 5051-31, 5051-32, 5051-33, 5051-34, T77
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Hunting Practice (1)

The Fairy of Arua suggested that you defeat 3 Jelly Beans in preparation for your real adventures. Complete this task and you'll receive a small reward.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5051-31`

Happens by killing [[monsters/2-jelly-bean|Jelly Bean]].

Checks:

- you have [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you carry < 2 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5051-32` is tried instead.

### `5051-32`

Checks:

- you have [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you carry ≥ 2 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you get [[items/consumable/103-grapes|Grapes]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 77 on
- the quest ends (removed from your list)

If the checks fail, step `5033-31` is tried instead.

### `5051-33`

Checks:

- you have [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you carry < 2 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `5051-34` is tried instead.

### `5051-34`

Checks:

- you have [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you carry ≥ 2 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- you get [[items/consumable/103-grapes|Grapes]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 200 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- quest switch 77 on
- the quest ends (removed from your list)

### `T77`

Happens by talking to [[npcs/1030-visitor-guide-fairy-of-arua|Visitor Guide Fairy of Arua]].

Checks:

- quest switch 77 is off

Then:

- you get the quest [[quests/5051-hunting-practice-1|Hunting Practice (1)]]
- client script `Tutorial07button`
