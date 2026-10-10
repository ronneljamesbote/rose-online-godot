---
kind: quest
id: 151
name: Lunarian Coffee
status: in-game
given_by:
- '[[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]]'
monsters:
- '[[monsters/158-doonga-captain|Doonga Captain]]'
steps: 4
source:
  data: LIST_QUEST.STB row 151; QSD triggers 150-01, 150-02, 151-01, 151-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Lunarian Coffee

The secret to the rich flavor and sumptuous fragrance of Lunarian Coffee is in the Doonga Claws. You've got to get 20 Doonga Claws so that you can get some Lunarian Coffee.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `150-01`

Happens by talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- you have [[quests/150-kay-s-secret|Kay's Secret]]

Then:

- works on [[quests/150-kay-s-secret|Kay's Secret]]
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/151-lunarian-coffee|Lunarian Coffee]]
- set episode variable 0 to 50

### `150-02`

Happens by talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- episode variable 0 = 50

Then:

- you get the quest [[quests/151-lunarian-coffee|Lunarian Coffee]]

### `151-01`

Happens by talking to [[npcs/1143-ferrell-guild-merchant-bith|Ferrell Guild Merchant Bith]].

Checks:

- you have [[quests/151-lunarian-coffee|Lunarian Coffee]]
- you carry ≥ 20 × [[items/quest/313-doonga-claw|Doonga Claw]]

Then:

- works on [[quests/151-lunarian-coffee|Lunarian Coffee]]
- 20 × [[items/quest/313-doonga-claw|Doonga Claw]] is taken
- you get [[items/consumable/6-health-bottle-l|Health Bottle (L)]], base count 30 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/26-mana-bottle-l|Mana Bottle (L)]], base count 30 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 30000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest ends (removed from your list)
- then runs step `151-03`

### `151-31`

Happens by killing [[monsters/158-doonga-captain|Doonga Captain]].

Checks:

- you have [[quests/151-lunarian-coffee|Lunarian Coffee]]
- you carry < 20 × [[items/quest/313-doonga-claw|Doonga Claw]]
- a random roll 0–99 lands in 0–60

Then:

- works on [[quests/151-lunarian-coffee|Lunarian Coffee]]
- you get 1 × [[items/quest/313-doonga-claw|Doonga Claw]]
