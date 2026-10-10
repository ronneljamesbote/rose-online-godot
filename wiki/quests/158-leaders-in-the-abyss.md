---
kind: quest
id: 158
name: Leaders in the Abyss
status: in-game
given_by:
- '[[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]]'
monsters:
- '[[monsters/287-grandmaster-goblin|Grandmaster Goblin]]'
steps: 4
source:
  data: LIST_QUEST.STB row 158; QSD triggers 157-01, 157-02, 158-01, 158-31
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Leaders in the Abyss

It is said that Hebarn's Red Hearts are sealed in the leaders of the Goblins. Go to the Goblin Cave and fight these high ranked Goblins in order to get at least 3 Red Hearts. You can only succeed by creating a party.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `157-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/157-skin-of-steel|Skin of Steel]]
- you carry ≥ 5 × [[items/quest/622-mana-charger|Mana Charger]]

Then:

- works on [[quests/157-skin-of-steel|Skin of Steel]]
- 5 × [[items/quest/622-mana-charger|Mana Charger]] is taken
- experience: 80000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/material/152-green-hearts|Green Hearts]] (item count: 2)
- you get [[items/material/154-pink-hearts|Pink Hearts]] (item count: 2)
- you get [[items/consumable/383-blood-charm|Blood Charm]] (item count: 10)
- the quest becomes [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]] (progress kept)
- set episode variable 0 to 57

### `157-02`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- episode variable 0 = 57

Then:

- you get the quest [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]

### `158-01`

Happens by talking to [[npcs/1151-ferrell-guild-merchant-med|Ferrell Guild Merchant Med]].

Checks:

- you have [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]
- you carry ≥ 3 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]
- 3 × [[items/quest/619-scarlet-heart|Scarlet Heart]] is taken
- experience: 150000 (XP, not scaled by level, see [[rules/quests|Quests]])
- you get [[items/consumable/437-goblin-king|Goblin King]] (item count: 5)
- you get [[items/material/155-red-hearts|Red Hearts]] (item count: 2)
- you get [[items/material/156-golden-hearts|Golden Hearts]] (item count: 2)
- you get [[items/material/157-white-hearts|White Hearts]] (item count: 2)
- the quest ends (removed from your list)
- set episode variable 0 to 58

### `158-31`

Happens by killing [[monsters/287-grandmaster-goblin|Grandmaster Goblin]].

Checks:

- you have [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]
- you carry < 3 × [[items/quest/619-scarlet-heart|Scarlet Heart]]

Then:

- works on [[quests/158-leaders-in-the-abyss|Leaders in the Abyss]]
- you get 1 × [[items/quest/619-scarlet-heart|Scarlet Heart]]
