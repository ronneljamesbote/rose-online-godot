---
kind: quest
id: 156
name: Monsters in the Desert
status: in-game
given_by:
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
monsters:
- '[[monsters/201-worm-dragon|Worm Dragon]]'
steps: 4
source:
  data: LIST_QUEST.STB row 156; QSD triggers 155-01, 155-02, 156-01, 156-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Monsters in the Desert

According to Med, Worm Dragons have been chosen to store large amounts of Mana Energy for the Arumics. Fight with about 5 Worm Dragons and search their bodies for Mana Chargers. You can only succeed in this mission by forming a party.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `155-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/155-suspicious-researchers|Suspicious Researchers]]
- you carry = 4 × [[items/quest/622-mana-charger|Mana Charger]]
- quest switch 0 = 1
- quest switch 1 = 1
- quest switch 2 = 1
- quest switch 3 = 1

Then:

- works on [[quests/155-suspicious-researchers|Suspicious Researchers]]
- 4 × [[items/quest/622-mana-charger|Mana Charger]] is taken
- you get [[items/consumable/316-advanced-strength-scroll-solo|Advanced Strength Scroll (Solo)]], base count 3 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/317-advanced-defense-scroll-solo|Advanced Defense Scroll (Solo)]], base count 3 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- experience, base 40000 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/381-ice-charm|Ice Charm]], base count 10 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- the quest becomes [[quests/156-monsters-in-the-desert|Monsters in the Desert]] (progress kept)
- set episode variable 0 to 55

### `155-02`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- episode variable 0 = 55

Then:

- you get the quest [[quests/156-monsters-in-the-desert|Monsters in the Desert]]

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

### `156-31`

Happens by killing [[monsters/201-worm-dragon|Worm Dragon]].

Checks:

- you have [[quests/156-monsters-in-the-desert|Monsters in the Desert]]
- you carry < 5 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/156-monsters-in-the-desert|Monsters in the Desert]]
- you get 1 × [[items/quest/622-mana-charger|Mana Charger]]
