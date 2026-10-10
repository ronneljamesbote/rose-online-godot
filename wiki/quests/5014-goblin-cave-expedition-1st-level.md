---
kind: quest
id: 5014
name: Goblin Cave Expedition (1st Level)
status: in-game
given_by:
- '[[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]'
monsters:
- '[[monsters/271-goblin-worker|Goblin Worker]]'
- '[[monsters/274-goblin-server|Goblin Server]]'
steps: 6
source:
  data: LIST_QUEST.STB row 5014; QSD triggers 5014-01, 5014-02, 5014-10, 5014-31, 5014-32, 5014-33
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Goblin Cave Expedition (1st Level)

The Goblin Cave (B1) is teeming with Rubies that have been dug up by the Goblins. However, Goblins merely hold on to these jewels and don't know what to do with them. Bring a bunch of Small Rubies to Bellia, and he might pay you a good price for them.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5014-01`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 0 = 1
- the NPC's variable 2 < 20
- your Level ≥ 36
- your Level ≤ 55

Then:

- you get the quest [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- add 1 to the NPC's variable 2
- then runs step `5014-10`

### `5014-02`

Happens by talking to [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]].

Checks:

- you have [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- you carry ≥ 1 × [[items/quest/810-small-ruby|Small Ruby]]

Then:

- works on [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- money: 450 (money, fixed, see [[rules/quests|Quests]])
- the quest ends (removed from your list)

### `5014-10`

Checks:

- the NPC [[npcs/1092-gypsy-jewel-seller-bellia|Gypsy Jewel Seller Bellia]]
- the NPC's variable 2 ≥ 20

Then:

- set the NPC's variable 0 to 2

### `5014-31`

Happens by killing [[monsters/271-goblin-worker|Goblin Worker]].

Checks:

- you have [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- a random roll 0–99 lands in 0–15
- your Level ≥ 36
- your Level ≤ 55
- you carry < 45 × [[items/quest/810-small-ruby|Small Ruby]]

Then:

- works on [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- you get 1 × [[items/quest/810-small-ruby|Small Ruby]]
- add 1 to quest variable 9

### `5014-32`

Checks:

- you have [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- a random roll 0–99 lands in 0–18
- your Level ≥ 36
- your Level ≤ 55
- you carry < 45 × [[items/quest/810-small-ruby|Small Ruby]]

Then:

- works on [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- you get 1 × [[items/quest/810-small-ruby|Small Ruby]]

### `5014-33`

Happens by killing [[monsters/274-goblin-server|Goblin Server]].

Checks:

- you have [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- a random roll 0–99 lands in 0–20
- your Level ≥ 36
- your Level ≤ 55
- you carry < 45 × [[items/quest/810-small-ruby|Small Ruby]]

Then:

- works on [[quests/5014-goblin-cave-expedition-1st-level|Goblin Cave Expedition (1st Level)]]
- you get 1 × [[items/quest/810-small-ruby|Small Ruby]]
- add 1 to quest variable 9
