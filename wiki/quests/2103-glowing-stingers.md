---
kind: quest
id: 2103
name: 'Glowing Stingers'
status: in-game
given_by:
- '[[npcs/1055-scout-mira|Scout Mira]]'
level: 10
steps: 4
source:
  data: 'godot/data/quests/2103-glowing-stingers.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2103-01, ql21-001/2103-02, ql21-001/2103-31, ql21-001/2103-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# Glowing Stingers

Mira noticed the Honeybees' stingers glow. Bring her 8 Glowing Stingers from the Honeybees and Needle Honeybees.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1055-scout-mira|Scout Mira]] offers it from level 10, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Bring Mira 8 Glowing Stingers from Honeybees and Needle Honeybees.
- **End:** Mira studied the stingers in the sunlight.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2103-01`

Happens by talking to [[npcs/1055-scout-mira|Scout Mira]].

Checks:

- your Level ≥ 10
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 602 = 0 (you haven't done this quest yet)

Then:

- you get the quest [[quests/2103-glowing-stingers|Glowing Stingers]]

### `2103-02`

Happens by talking to [[npcs/1055-scout-mira|Scout Mira]].

Checks:

- you have [[quests/2103-glowing-stingers|Glowing Stingers]]
- you carry ≥ 8 × [[items/quest/702-glowing-stinger|Glowing Stinger]]

Then:

- works on [[quests/2103-glowing-stingers|Glowing Stingers]]
- 8 × [[items/quest/702-glowing-stinger|Glowing Stinger]] is taken
- experience, base 700 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 700 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- you get 5 × [[items/consumable/1-health-vial-s|Health Vial (S)]]
- set quest switch 602 to 1
- the quest ends (removed from your list)

### `2103-31`

Happens by killing [[monsters/61-needle-honeybee|Needle HoneyBee]].

Checks:

- you have [[quests/2103-glowing-stingers|Glowing Stingers]]
- a random roll 0–99 lands in 0–59
- you carry < 8 × [[items/quest/702-glowing-stinger|Glowing Stinger]]

Then:

- works on [[quests/2103-glowing-stingers|Glowing Stingers]]
- you get 1 × [[items/quest/702-glowing-stinger|Glowing Stinger]]

Whether or not it passes, step `2115-31` runs next.

### `2103-32`

Happens by killing [[monsters/62-honeybee|HoneyBee]].

Checks:

- you have [[quests/2103-glowing-stingers|Glowing Stingers]]
- a random roll 0–99 lands in 0–59
- you carry < 8 × [[items/quest/702-glowing-stinger|Glowing Stinger]]

Then:

- works on [[quests/2103-glowing-stingers|Glowing Stingers]]
- you get 1 × [[items/quest/702-glowing-stinger|Glowing Stinger]]

Whether or not it passes, step `2115-32` runs next.

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
