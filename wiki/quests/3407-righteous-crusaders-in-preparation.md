---
kind: quest
id: 3407
name: Righteous Crusaders in Preparation
status: in-game
given_by:
- '[[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]]'
monsters:
- '[[monsters/122-krawfy-warrior|Krawfy Warrior]]'
steps: 4
source:
  data: LIST_QUEST.STB row 3407; QSD triggers 3407-01, 3407-02, 3407-03, 3407-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Righteous Crusaders in Preparation

To improve our chances in Faction conflicts, the Righteous Crusaders have decided to get new equipment. Help create new weapons by supplying 30 Krawfy Hard Shells from hunting Krawfy Warriors.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `3407-01`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- your Faction = 3
- your Level ≥ 55

Then:

- you get the quest [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]

### `3407-02`

Happens by talking to [[npcs/1111-righteous-crusader-huffe|Righteous Crusader Huffe]].

Checks:

- you have [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- your Faction = 3
- your Level ≤ 70
- you carry ≥ 30 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- 30 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]] is taken
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 8 to your UnionPoint3
- the quest ends (removed from your list)

If the checks fail, step `3407-03` is tried instead.

### `3407-03`

Checks:

- you have [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- your Faction = 3
- your Level > 70
- you carry ≥ 30 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- 30 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]] is taken
- you get [[items/consumable/5-health-bottle-m|Health Bottle (M)]] (item count: 5)
- you get [[items/consumable/22-mana-vial-m|Mana Vial (M)]] (item count: 5)
- add 3 to your UnionPoint3
- the quest ends (removed from your list)

### `3407-31`

Happens by killing [[monsters/122-krawfy-warrior|Krawfy Warrior]].

Checks:

- you have [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- a random roll 0–99 lands in 0–30
- you carry < 30 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

Then:

- works on [[quests/3407-righteous-crusaders-in-preparation|Righteous Crusaders in Preparation]]
- you get 1 × [[items/quest/315-krawfy-hard-shell|Krawfy Hard Shell]]

If the checks fail, step `3607-31` is tried instead.
