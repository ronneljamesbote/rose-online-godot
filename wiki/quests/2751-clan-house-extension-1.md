---
kind: quest
id: 2751
name: Clan House Extension (1)
status: in-game
given_by:
- '[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]'
steps: 3
source:
  data: LIST_QUEST.STB row 2751; QSD triggers 2751-01, 2751-02, 2751-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Clan House Extension (1)

Burtland the Clan owner asked you to collect items that are necessary for Clan House Extension in order to raise your Clan Grade to 3. You are required to bring 600 Silver Iron, 400 Pine Wood and 10 Blue Crystals. These are normal items, not quest items.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2751-01`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- your clan rank = 6
- clan level = 2
- clan points ≥ 5000

Then:

- you get the quest [[quests/2751-clan-house-extension-1|Clan House Extension (1)]]

### `2751-02`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- you have [[quests/2751-clan-house-extension-1|Clan House Extension (1)]]
- you carry ≥ 600 × [[items/material/6-silver-iron|Silver Iron]]
- you carry ≥ 400 × [[items/material/36-pine-wood|Pine Wood]]
- you carry ≥ 10 × [[items/material/162-blue-crystal|Blue Crystal]]

Then:

- works on [[quests/2751-clan-house-extension-1|Clan House Extension (1)]]
- 600 × [[items/material/6-silver-iron|Silver Iron]] is taken
- 400 × [[items/material/36-pine-wood|Pine Wood]] is taken
- 10 × [[items/material/162-blue-crystal|Blue Crystal]] is taken
- the quest ends (removed from your list)
- clan level +1

If the checks fail, step `2752-02` is tried instead.

### `2751-03`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- you have [[quests/2751-clan-house-extension-1|Clan House Extension (1)]]
- you carry ≥ 600 × [[items/material/6-silver-iron|Silver Iron]]
- you carry ≥ 400 × [[items/material/36-pine-wood|Pine Wood]]
- you carry ≥ 10 × [[items/material/162-blue-crystal|Blue Crystal]]
