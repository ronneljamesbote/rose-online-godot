---
kind: quest
id: 5064
name: Darling Judy (1)
status: in-game
given_by:
- '[[npcs/1201-event-info-judy|Event Info Judy]]'
monsters:
- Cherry Smouly (NPC 309)
- Cherry Smouly (NPC 310)
- Cherry Smouly (NPC 311)
- Cherry Smouly (NPC 312)
- Cherry Smouly (NPC 313)
steps: 7
source:
  data: LIST_QUEST.STB row 5064; QSD triggers 5064-01, 5064-02, 5064-31, 5064-33, 5064-35, 5064-37, 5064-39
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Darling Judy (1)

Every spring, Judy used to make a bunch of flowers for her grandfather. But this time she has a bad cold, so she's asking you to collect 20 Cherry Blossoms. You can get Cherry Blossoms by hunting Cherry Smoulies.  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `5064-01`

Happens by talking to [[npcs/1201-event-info-judy|Event Info Judy]].

Checks:

- the NPC [[npcs/1201-event-info-judy|Event Info Judy]]
- the NPC's variable 0 = 3
- quest switch 488 is off

Then:

- you get the quest [[quests/5064-darling-judy-1|Darling Judy (1)]]

### `5064-02`

Happens by talking to [[npcs/1201-event-info-judy|Event Info Judy]].

Checks:

- the NPC [[npcs/1201-event-info-judy|Event Info Judy]]
- the NPC's variable 0 = 3
- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry ≥ 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]] is taken
- you get [[items/head/154-spring-flower-hat|Spring Flower Hat]] (item count: 1)
- you get [[items/consumable/438-cherry-smouly|Cherry Smouly]] (item count: 2)
- the quest becomes [[quests/5065-darling-judy-2|Darling Judy (2)]]
- quest switch 488 on

### `5064-31`

Happens by killing Cherry Smouly (NPC 309).

Checks:

- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

If the checks fail, step `5064-32` is tried instead.

### `5064-33`

Happens by killing Cherry Smouly (NPC 310).

Checks:

- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

If the checks fail, step `5064-34` is tried instead.

### `5064-35`

Happens by killing Cherry Smouly (NPC 311).

Checks:

- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

If the checks fail, step `5064-36` is tried instead.

### `5064-37`

Happens by killing Cherry Smouly (NPC 312).

Checks:

- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

If the checks fail, step `5064-38` is tried instead.

### `5064-39`

Happens by killing Cherry Smouly (NPC 313).

Checks:

- you have [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5064-darling-judy-1|Darling Judy (1)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

If the checks fail, step `5064-40` is tried instead.
