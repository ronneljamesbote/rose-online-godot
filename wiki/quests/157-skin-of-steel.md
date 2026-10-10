---
kind: quest
id: 157
name: Skin of Steel
status: in-game
given_by:
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
monsters:
- '[[monsters/169-grandmaster-golem|Grandmaster Golem]]'
steps: 4
source:
  data: LIST_QUEST.STB row 157; QSD triggers 156-01, 156-02, 157-01, 157-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Skin of Steel

According to the message in the Charger, you must now fight Grandmaster Golems. Fight with about 5 Grandmaster Golems and look for Mana Chargers in their dead bodies. You can only complete this task by forming a party.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `156-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/156-monsters-in-the-desert|Monsters in the Desert]]
- you carry ≥ 5 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/156-monsters-in-the-desert|Monsters in the Desert]]
- 5 × [[items/quest/622-mana-charger|Mana Charger]] is taken
- you get [[items/material/151-black-hearts|Black Hearts]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/153-blue-hearts|Blue Hearts]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 70000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/382-spark-charm|Spark Charm]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/157-skin-of-steel|Skin of Steel]] (progress kept)
- set episode variable 0 to 56

### `156-02`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- episode variable 0 = 56

Then:

- you get the quest [[quests/157-skin-of-steel|Skin of Steel]]

### `157-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/157-skin-of-steel|Skin of Steel]]
- you carry ≥ 5 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/157-skin-of-steel|Skin of Steel]]
- 5 × [[items/quest/622-mana-charger|Mana Charger]] is taken
- experience, base 80000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/152-green-hearts|Green Hearts]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/material/154-pink-hearts|Pink Hearts]], base count 2 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/383-blood-charm|Blood Charm]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]] (progress kept)
- set episode variable 0 to 57

### `157-31`

Happens by killing [[monsters/169-grandmaster-golem|Grandmaster Golem]].

Checks:

- you have [[quests/157-skin-of-steel|Skin of Steel]]
- you carry < 5 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/157-skin-of-steel|Skin of Steel]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
