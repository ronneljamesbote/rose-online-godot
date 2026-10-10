---
kind: quest
id: 2752
name: Clan House Extension (2)
status: in-game
given_by:
- '[[npcs/1115-clan-owner-burtland|Clan Owner Burtland]]'
steps: 3
source:
  data: LIST_QUEST.STB row 2752; QSD triggers 2752-01, 2752-02, 2752-03
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Clan House Extension (2)

Burtland the Clan owner asked you to collect items that are necessary for Clan House Extension in order to raise your Clan Grade to 3. You are required to bring 200 Steel, 150 Cinnamon Wood and 50 Earth Stones. These are normal items, not quest items.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2752-01`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- your clan rank = 6
- clan level = 2
- clan points ≥ 5000

Then:

- you get the quest [[quests/2752-clan-house-extension-2|Clan House Extension (2)]]

### `2752-02`

Checks:

- you have [[quests/2752-clan-house-extension-2|Clan House Extension (2)]]
- you carry ≥ 200 × [[items/material/7-steel|Steel]]
- you carry ≥ 150 × [[items/material/37-cinnamon-wood|Cinnamon Wood]]
- you carry ≥ 50 × [[items/material/123-earth-stone|Earth Stone]]

Then:

- works on [[quests/2752-clan-house-extension-2|Clan House Extension (2)]]
- 200 × [[items/material/7-steel|Steel]] is taken
- 150 × [[items/material/37-cinnamon-wood|Cinnamon Wood]] is taken
- 50 × [[items/material/123-earth-stone|Earth Stone]] is taken
- the quest ends (removed from your list)
- clan level +1

If the checks fail, step `2753-02` is tried instead.

### `2752-03`

Happens by talking to [[npcs/1115-clan-owner-burtland|Clan Owner Burtland]].

Checks:

- you have [[quests/2752-clan-house-extension-2|Clan House Extension (2)]]
- you carry ≥ 200 × [[items/material/7-steel|Steel]]
- you carry ≥ 150 × [[items/material/37-cinnamon-wood|Cinnamon Wood]]
- you carry ≥ 50 × [[items/material/123-earth-stone|Earth Stone]]
