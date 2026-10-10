---
kind: quest
id: 131
name: Eva the Sorcerer
status: in-game
given_by:
- '[[npcs/1082-guide-eva|Guide Eva]]'
monsters:
- '[[monsters/143-kaiman-warrior|Kaiman Warrior]]'
- '[[monsters/209-grunter-king|Grunter King]]'
steps: 5
source:
  data: LIST_QUEST.STB row 131; QSD triggers 130-01, 130-02, 131-01, 131-31, 131-32
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Eva the Sorcerer

Something seems to be blocking the power of the Owl Eye. You need for get 20 Flames of Courage from Kaiman Warriors, and 5 Flames of Passion from Grunter Kings, to penetrate the darkness that's interfering with the Owl Eye.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `130-01`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- you have [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]

Then:

- works on [[quests/130-eva-the-sorcerer|Eva the Sorcerer]]
- 1 × [[items/quest/615-small-letter|Small Letter]] is taken
- experience, base 7000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/131-eva-the-sorcerer|Eva the Sorcerer]] (progress kept)
- set episode variable 0 to 30

### `130-02`

Happens by talking to [[npcs/1082-guide-eva|Guide Eva]].

Checks:

- episode variable 0 = 30

Then:

- you get the quest [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]

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
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 20 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/23-mana-vial-l|Mana Vial (L)]], base count 20 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/132-forbidden-spell|Forbidden Spell]] (progress kept)
- set episode variable 0 to 31

### `131-31`

Happens by killing [[monsters/143-kaiman-warrior|Kaiman Warrior]].

Checks:

- you have [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- a random roll 0–99 lands in 0–70
- you carry < 20 × [[items/quest/616-flame-of-courage|Flame of Courage]]

Then:

- works on [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- you get 1 × [[items/quest/616-flame-of-courage|Flame of Courage]]

If the checks fail, step `3807-31` is tried instead.

### `131-32`

Happens by killing [[monsters/209-grunter-king|Grunter King]].

Checks:

- you have [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- you carry < 5 × [[items/quest/617-flame-of-passion|Flame of Passion]]

Then:

- works on [[quests/131-eva-the-sorcerer|Eva the Sorcerer]]
- you get 1 × [[items/quest/617-flame-of-passion|Flame of Passion]]
