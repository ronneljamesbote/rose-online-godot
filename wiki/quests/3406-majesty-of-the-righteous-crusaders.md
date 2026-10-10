---
kind: quest
id: 3406
name: Majesty of the Righteous Crusaders
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
monsters:
- '[[monsters/152-elder-doonga|Elder Doonga]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3406; QSD triggers 3406-01, 3406-02, 3406-03, 3406-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Majesty of the Righteous Crusaders

We've got to keep training if we're going to mete out justice to the Junon Order that opposes us. Defeating 45 Elder Doongas might be rigorous training, but let's do it with pride as Righteous Crusaders.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3406-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3
- your Level ≥ 51

Then:

- you get the quest [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]

### `3406-02`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- your Faction = 3
- your Level ≤ 60
- you carry ≥ 45 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- 45 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 6 to your UnionPoint3
- the quest ends (removed from your list)

If the checks fail, step `3406-03` is tried instead.

### `3406-03`

Checks:

- you have [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- your Faction = 3
- your Level > 60
- you carry ≥ 45 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

Then:

- works on [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- 45 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]] is taken
- you get [[items/consumable/3-health-vial-l|Health Vial (L)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]], base count 5 (reward formula 5: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- add 2 to your UnionPoint3
- the quest ends (removed from your list)

### `3406-31`

Happens by killing [[monsters/152-elder-doonga|Elder Doonga]].

Checks:

- you have [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- you carry < 45 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]
- your Faction = 3

Then:

- works on [[quests/3406-majesty-of-the-righteous-crusaders|Majesty of the Righteous Crusaders]]
- you get 1 × [[items/quest/500-proof-of-monster-extermination|Proof of Monster Extermination]]

If the checks fail, step `3806-31` is tried instead.
