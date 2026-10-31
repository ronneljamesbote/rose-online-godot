---
kind: quest
id: 2204
name: The Humming Stones
status: in-game
given_by:
- '[[npcs/1040-stone-reader-ilene|Stone-reader Ilene]]'
level: 1
steps: 3
source:
  data: 'godot/data/quests/2204-the-humming-stones.tres; quest triggers in godot/data/quest_triggers/: qa22-001/2204-01, qa22-001/2204-02, qa22-001/2204-31'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Humming Stones

Stone-reader Ilene wants 8 Jelly Cores from the JellyNuts that gather round the standing stones of Stonewatch Meadow.

Part of the [[zones/22-adventurer-s-plain|Adventurer's Plain]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1040-stone-reader-ilene|Stone-reader Ilene]] offers it from level 1, after [[quests/2203-bones-on-the-beach|Bones on the Beach]].

- **Start:** Bring Ilene 8 Jelly Cores from the JellyNuts round the standing stones.
- **End:** Ilene has her cores, and a suspicion.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2204-01`

Happens by talking to [[npcs/1040-stone-reader-ilene|Stone-reader Ilene]].

Checks:

- quest switch 625 is on (you finished [[quests/2203-bones-on-the-beach|Bones on the Beach]])
- quest switch 626 is off (you haven't done this quest yet)

Then:

- you get the quest [[quests/2204-the-humming-stones|The Humming Stones]]

### `2204-02`

Happens by talking to [[npcs/1040-stone-reader-ilene|Stone-reader Ilene]].

Checks:

- you have [[quests/2204-the-humming-stones|The Humming Stones]]
- you carry ≥ 8 × [[items/quest/714-jelly-core|Jelly Core]]

Then:

- works on [[quests/2204-the-humming-stones|The Humming Stones]]
- 8 × [[items/quest/714-jelly-core|Jelly Core]] is taken
- experience, base 120 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 150 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- quest switch 626 on
- the quest ends (removed from your list)

### `2204-31`

Happens by killing [[monsters/3-jellynut|JellyNut]]: its kill step `5033-33` ([[quests/5033-stockpiling-jelly-bean-hides|Stockpiling Jelly Bean Hides]]) runs it next, so a kill counts for both quests.

Checks:

- you have [[quests/2204-the-humming-stones|The Humming Stones]]
- a random roll 0–99 lands in 0–60
- you carry < 8 × [[items/quest/714-jelly-core|Jelly Core]]

Then:

- works on [[quests/2204-the-humming-stones|The Humming Stones]]
- you get 1 × [[items/quest/714-jelly-core|Jelly Core]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Adventurer's Plain redesign.
