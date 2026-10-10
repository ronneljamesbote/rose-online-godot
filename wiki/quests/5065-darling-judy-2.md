---
kind: quest
id: 5065
name: Darling Judy (2)
status: in-game
given_by:
- '[[npcs/1201-event-info-judy|Event Info Judy]]'
steps: 8
source:
  data: LIST_QUEST.STB row 5065; QSD triggers 5064-02, 5064-03, 5064-04, 5064-32, 5064-34, 5064-36, 5064-38, 5064-40
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Darling Judy (2)

Judy asks you once again to gather 20 Cherry Blossoms, this time for Mayor Darren of Junon Polis. Hunt those Cherry Smoulies to get those flowers!  

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

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

### `5064-03`

Happens by talking to [[npcs/1201-event-info-judy|Event Info Judy]].

Checks:

- the NPC [[npcs/1201-event-info-judy|Event Info Judy]]
- the NPC's variable 0 = 3
- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry ≥ 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]] is taken
- you get [[items/vehicle/405-sun-roof|Sun Roof]] (item count: 1)
- you get [[items/consumable/438-cherry-smouly|Cherry Smouly]] (item count: 3)
- the quest ends (removed from your list)
- quest switch 489 on

### `5064-04`

Happens by talking to [[npcs/1201-event-info-judy|Event Info Judy]].

Checks:

- the NPC [[npcs/1201-event-info-judy|Event Info Judy]]
- the NPC's variable 0 = 3
- quest switch 488 is on
- quest switch 489 is off

Then:

- you get the quest [[quests/5065-darling-judy-2|Darling Judy (2)]]

### `5064-32`

Checks:

- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

### `5064-34`

Checks:

- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

### `5064-36`

Checks:

- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

### `5064-38`

Checks:

- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

### `5064-40`

Checks:

- you have [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you carry < 20 × [[items/quest/904-cherry-blossom|Cherry Blossom]]

Then:

- works on [[quests/5065-darling-judy-2|Darling Judy (2)]]
- you get 1 × [[items/quest/904-cherry-blossom|Cherry Blossom]]
