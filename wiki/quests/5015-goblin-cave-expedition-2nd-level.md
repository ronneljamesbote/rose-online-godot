---
kind: quest
id: 5015
name: Goblin Cave Expedition (2nd Level)
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
steps: 7
source:
  data: LIST_QUEST.STB row 5015; QSD triggers 5014-10, 5015-01, 5015-02, 5015-31, 5015-32, 5015-33, 5015-34
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Goblin Cave Expedition (2nd Level)

The Goblin Cave (B2) is teeming with Topaz that has been dug up by the Goblins. However, Goblins merely hold on to these jewels and don't know what to do with them. Bring a bunch of Small Topazes to Bellia, and he might pay you a good price for them.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5014-10`

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 2 ≥ 20

Then:

- set the NPC's variable 0 to 2

### `5015-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 20
- your Level ≥ 56
- your Level ≤ 70

Then:

- you get the quest [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- add 1 to the NPC's variable 2
- then runs step `5014-10`

### `5015-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- you carry ≥ 1 × [[items/quest/811-small-topaz|Small Topaz]]

Then:

- works on [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- money: 680 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5015-31`

Checks:

- you have [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- a random roll 0–99 lands in 0–15
- your Level ≥ 56
- your Level ≤ 70
- you carry < 52 × [[items/quest/811-small-topaz|Small Topaz]]

Then:

- works on [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- you get 1 × [[items/quest/811-small-topaz|Small Topaz]]
- add 1 to quest variable 9

### `5015-32`

Checks:

- you have [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- a random roll 0–99 lands in 0–18
- your Level ≥ 56
- your Level ≤ 70
- you carry < 52 × [[items/quest/811-small-topaz|Small Topaz]]

Then:

- works on [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- you get 1 × [[items/quest/811-small-topaz|Small Topaz]]
- add 1 to quest variable 9

### `5015-33`

Checks:

- you have [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- a random roll 0–99 lands in 0–20
- your Level ≥ 56
- your Level ≤ 70
- you carry < 52 × [[items/quest/811-small-topaz|Small Topaz]]

Then:

- works on [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- you get 1 × [[items/quest/811-small-topaz|Small Topaz]]
- add 1 to quest variable 9

### `5015-34`

Checks:

- you have [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- a random roll 0–99 lands in 0–20
- your Level ≥ 56
- your Level ≤ 70
- you carry < 52 × [[items/quest/811-small-topaz|Small Topaz]]

Then:

- works on [[quests/5015-goblin-cave-expedition-2nd-level|Goblin Cave Expedition (2nd Level)]]
- you get 1 × [[items/quest/811-small-topaz|Small Topaz]]
- add 1 to quest variable 9
