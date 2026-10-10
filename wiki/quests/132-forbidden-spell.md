---
kind: quest
id: 132
name: Forbidden Spell
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
npcs:
- '[[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]]'
steps: 3
source:
  data: LIST_QUEST.STB row 132; QSD triggers 131-01, 131-02, 132-01
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Forbidden Spell

Eva believes that Shroon may be trying to contract with a demon which may present a dangerous scenario. While Eva looks into other ways to deal with Shroon, you've got to try to convince Shroon to cancel this contract.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `131-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- you carry ≥ 20 × [[items/quest/616-flame-of-courage|Flame of Courage]]
- you carry ≥ 5 × [[items/quest/617-flame-of-passion|Flame of Passion]]

Then:

- works on [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- 20 × [[items/quest/616-flame-of-courage|Flame of Courage]] is taken
- 5 × [[items/quest/617-flame-of-passion|Flame of Passion]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]] (item count: 20)
- you get [[items/consumable/23-mana-vial-l|Mana Vial (L)]] (item count: 20)
- experience: 30000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/132-forbidden-spell|Forbidden Spell]] (progress kept)
- set episode variable 0 to 31

### `131-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 31

Then:

- you get the quest [[quests/132-forbidden-spell|Forbidden Spell]]

### `132-01`

Happens by talking to [[npcs/1121-ikaness-staff-shroon|Ikaness Staff Shroon]].

Checks:

- you have [[quests/132-forbidden-spell|Forbidden Spell]]

Then:

- works on [[quests/132-forbidden-spell|Forbidden Spell]]
- experience: 10000 (XP, not scaled by level, see [[rules/quests|Quests]])
- the quest becomes [[quests/133-forbidden-spell|Forbidden Spell]]
- set episode variable 0 to 32
