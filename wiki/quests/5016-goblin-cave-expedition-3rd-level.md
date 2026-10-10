---
kind: quest
id: 5016
name: Goblin Cave Expedition (3rd Level)
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
monsters:
- '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
- '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
- '[[monsters/281-goblin-mage|Goblin Mage]]'
- '[[monsters/282-gem-goblin-mage|Gem Goblin Mage]]'
steps: 7
source:
  data: LIST_QUEST.STB row 5016; QSD triggers 5014-10, 5016-01, 5016-02, 5016-31, 5016-32, 5016-33, 5016-34
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Goblin Cave Expedition (3rd Level)

The Goblin Cave (B2) is teeming with Diamonds that have been dug up by the Goblins. However, Goblins merely hold on to these jewels and don't know what to do with them. Bring a bunch of Small Diamonds to Bellia, and he might pay you a good price for them.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5014-10`

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 2 ≥ 20

Then:

- set the NPC's variable 0 to 2

### `5016-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 20
- your Level ≥ 71
- your Level ≤ 110

Then:

- you get the quest [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- add 1 to the NPC's variable 2
- then runs step `5014-10`

### `5016-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- you carry ≥ 1 × [[items/quest/812-small-diamond|Small Diamond]]

Then:

- works on [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- Zuly, base 810 (reward formula 2: base × times the quest was repeated, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5016-31`

Happens by killing [[monsters/277-gem-goblin-guard|Gem Goblin Guard]].

Checks:

- you have [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- a random roll 0–99 lands in 0–15
- your Level ≥ 71
- your Level ≤ 110
- you carry < 58 × [[items/quest/812-small-diamond|Small Diamond]]

Then:

- works on [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- you get 1 × [[items/quest/812-small-diamond|Small Diamond]]
- add 1 to quest variable 9

### `5016-32`

Happens by killing [[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]].

Checks:

- you have [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- a random roll 0–99 lands in 0–18
- your Level ≥ 71
- your Level ≤ 110
- you carry < 58 × [[items/quest/812-small-diamond|Small Diamond]]

Then:

- works on [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- you get 1 × [[items/quest/812-small-diamond|Small Diamond]]
- add 1 to quest variable 9

### `5016-33`

Happens by killing [[monsters/281-goblin-mage|Goblin Mage]].

Checks:

- you have [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- a random roll 0–99 lands in 0–18
- your Level ≥ 71
- your Level ≤ 110
- you carry < 58 × [[items/quest/812-small-diamond|Small Diamond]]

Then:

- works on [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- you get 2 × [[items/quest/812-small-diamond|Small Diamond]]
- add 2 to quest variable 9

### `5016-34`

Happens by killing [[monsters/282-gem-goblin-mage|Gem Goblin Mage]].

Checks:

- you have [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- a random roll 0–99 lands in 0–18
- your Level ≥ 71
- your Level ≤ 110
- you carry < 58 × [[items/quest/812-small-diamond|Small Diamond]]

Then:

- works on [[quests/5016-goblin-cave-expedition-3rd-level|Goblin Cave Expedition (3rd Level)]]
- you get 2 × [[items/quest/812-small-diamond|Small Diamond]]
- add 2 to quest variable 9
