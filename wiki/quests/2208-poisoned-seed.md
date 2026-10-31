---
kind: quest
id: 2208
name: Poisoned Seed
status: in-game
given_by:
- '[[npcs/1044-scout-rhys|Scout Rhys]]'
level: 7
steps: 3
source:
  data: 'godot/data/quests/2208-poisoned-seed.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2208-01, qa22-001/2208-02, qa22-001/2208-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Poisoned Seed

Rhys found the strangers' camp full of rotten seed. He wants 6 Rotten Seed Sacks back from the Choropies that carried them off.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1044-scout-rhys|Scout Rhys]] offers it from level 7, after [[quests/2207-tracks-to-the-ford|Tracks to the Ford]].

- **Start:** Bring Rhys 6 Rotten Seed Sacks from the Spotted and Mother Choropies round the camp.
- **End:** Rhys has seen enough: the seed was poisoned on purpose.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2208-01`

Happens by talking to [[npcs/1044-scout-rhys|Scout Rhys]].

Checks:

- quest switch 629 is on (you finished [[quests/2207-tracks-to-the-ford|Tracks to the Ford]])
- quest switch 630 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2208-poisoned-seed|Poisoned Seed]]

### `2208-02`

Happens by talking to [[npcs/1044-scout-rhys|Scout Rhys]].

Checks:

- you have [[quests/2208-poisoned-seed|Poisoned Seed]]
- you carry ≥ 6 × [[items/quest/717-rotten-seed-sack|Rotten Seed Sack]]

Then:

- works on [[quests/2208-poisoned-seed|Poisoned Seed]]
- 6 × [[items/quest/717-rotten-seed-sack|Rotten Seed Sack]] is taken
- experience, base 260 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 300 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- quest switch 630 on
- the quest ends (removed from your list)

### `2208-31`

Happens by killing [[monsters/15-spotted-choropy|Spotted Choropy]], [[monsters/13-mother-choropy|Mother Choropy]]: their kill steps `952-32` ([[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]), `952-31` ([[quests/952-hawker-job-change-quest|Hawker Job Change Quest]]) run it next, so a kill counts for both quests.

Checks:

- you have [[quests/2208-poisoned-seed|Poisoned Seed]]
- a random roll 0–99 lands in 0–60
- you carry < 6 × [[items/quest/717-rotten-seed-sack|Rotten Seed Sack]]

Then:

- works on [[quests/2208-poisoned-seed|Poisoned Seed]]
- you get 1 × [[items/quest/717-rotten-seed-sack|Rotten Seed Sack]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
