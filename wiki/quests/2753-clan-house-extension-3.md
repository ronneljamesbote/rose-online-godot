---
kind: quest
id: 2753
name: Clan House Extension (3)
status: in-game
given_by:
- '[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]'
steps: 3
source:
  data: LIST_QUEST.STB row 2753; QSD triggers 2753-01, 2753-02, 2753-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Clan House Extension (3)

Burtland the Clan owner asked you to collect items that are necessary for Clan House Extension in order to raise your Clan Grade to 3. You are required to bring 50 Chromium, 100 Molive and 2 Green Hearts. These are normal items, not quest items.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2753-01`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- your clan rank = 6
- clan level = 2
- clan points ≥ 5000

Then:

- you get the quest [[quests/2753-clan-house-extension-3|Clan House Extension (3)]]

### `2753-02`

Checks:

- you have [[quests/2753-clan-house-extension-3|Clan House Extension (3)]]
- you carry ≥ 100 × [[items/material/14-molive|Molive]]
- you carry ≥ 50 × [[items/material/8-chromium|Chromium]]
- you carry ≥ 2 × [[items/material/152-green-hearts|Green Hearts]]

Then:

- works on [[quests/2753-clan-house-extension-3|Clan House Extension (3)]]
- 100 × [[items/material/14-molive|Molive]] is taken
- 50 × [[items/material/8-chromium|Chromium]] is taken
- 2 × [[items/material/152-green-hearts|Green Hearts]] is taken
- the quest ends (removed from your list)
- clan level +1

### `2753-03`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- you have [[quests/2753-clan-house-extension-3|Clan House Extension (3)]]
- you carry ≥ 100 × [[items/material/14-molive|Molive]]
- you carry ≥ 50 × [[items/material/8-chromium|Chromium]]
- you carry ≥ 2 × [[items/material/152-green-hearts|Green Hearts]]
