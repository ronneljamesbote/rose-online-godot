---
kind: quest
id: 5041
name: The Lost Engagement Ring
status: in-game
given_by:
- '[[npcs/1072-gypsy-merchant-methio|Gypsy Merchant Methio]]'
monsters:
- '[[monsters/43-red-flanae|Red Flanae]]'
steps: 4
source:
  data: LIST_QUEST.STB row 5041; QSD triggers 5041-01, 5041-02, 5041-03, 5041-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Lost Engagement Ring

Methio lost the engagement ring and without it, he'll never be able to face his fiancee again. You've got to find Methio's Engagement Ring for him.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5041-01`

Happens by talking to [[npcs/1072-gypsy-merchant-methio|Gypsy Merchant Methio]].

Checks:

- your Level ≥ 15
- your Level < 23
- quest switch 32 is off

Then:

- you get the quest [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]

### `5041-02`

Happens by talking to [[npcs/1072-gypsy-merchant-methio|Gypsy Merchant Methio]].

Checks:

- you have [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- you carry ≥ 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]]
- your Level < 23

Then:

- works on [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]] is taken
- experience: 300 (XP, scaled by your level, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]] (item count: 1)
- quest switch 32 on
- the quest ends (removed from your list)

If the checks fail, step `5041-03` is tried instead.

### `5041-03`

Checks:

- you have [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- you carry ≥ 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]]

Then:

- works on [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]] is taken
- experience: 5100 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/56-vital-jam-1|Vital Jam (+1)]] (item count: 1)
- quest switch 32 on
- the quest ends (removed from your list)

### `5041-31`

Happens by killing [[monsters/43-red-flanae|Red Flanae]].

Checks:

- you have [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- a random roll 0–99 lands in 0–7
- your Level ≥ 15
- your Level < 23
- you carry < 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]]

Then:

- works on [[quests/5041-the-lost-engagement-ring|The Lost Engagement Ring]]
- you get 1 × [[items/quest/508-methio-s-engagement-ring|Methio's Engagement Ring]]

If the checks fail, step `5007-31` is tried instead.
