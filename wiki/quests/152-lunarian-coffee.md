---
kind: quest
id: 152
name: Lunarian Coffee
status: in-game
given_by:
- '[[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]]'
npcs:
- '[[npcs/1131-mountain-guide-kay|Mountain Guide Kay]]'
time_limit_minutes: 10
steps: 6
source:
  data: LIST_QUEST.STB row 152; QSD triggers 151-02, 151-03, 151-04, 152-01, 152-04, 152-05
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lunarian Coffee

You've got to bring this Lunarian Coffee to Kay in 10 minutes before it gets cold. Hurry up!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `151-02`

Happens by talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- episode variable 0 = 51

Then:

- you get the quest [[quests/152-lunarian-coffee|Lunarian Coffee]]
- works on [[quests/152-lunarian-coffee|Lunarian Coffee]]
- you get 1 × [[items/quest/620-lunarian-coffee|Lunarian Coffee]]

### `151-03`

Checks: none.

Then:

- you get the quest [[quests/152-lunarian-coffee|Lunarian Coffee]]
- works on [[quests/152-lunarian-coffee|Lunarian Coffee]]
- you get 1 × [[items/quest/620-lunarian-coffee|Lunarian Coffee]]
- set episode variable 0 to 51

### `151-04`

Checks:

- you have [[quests/152-lunarian-coffee|Lunarian Coffee]]
- the quest timer ≤ 0

Then:

- the quest ends (removed from your list)
- then runs step `151-03`

### `152-01`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]].

Checks:

- you have [[quests/152-lunarian-coffee|Lunarian Coffee]]
- you carry = 1 × [[items/quest/620-lunarian-coffee|Lunarian Coffee]]

Then:

- works on [[quests/152-lunarian-coffee|Lunarian Coffee]]
- 1 × [[items/quest/620-lunarian-coffee|Lunarian Coffee]] is taken
- Zuly, base 60000 (reward formula 3: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/381-ice-charm|Ice Charm]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/382-spark-charm|Spark Charm]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/383-blood-charm|Blood Charm]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `152-03`

### `152-04`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]], talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- you have [[quests/152-lunarian-coffee|Lunarian Coffee]]
- the quest timer > 0

### `152-05`

Happens by talking to [[npcs/1131-mountain-guide-kay|Mountain Guide Kay]], talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- you have [[quests/152-lunarian-coffee|Lunarian Coffee]]
- the quest timer ≤ 0

Then:

- the quest ends (removed from your list)
- then runs step `151-03`
