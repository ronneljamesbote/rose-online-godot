---
kind: quest
id: 2115
name: 'The Outrider''s Trial'
status: in-game
given_by:
- '[[npcs/1120-outrider-kesh|Outrider Kesh]]'
level: 10
steps: 5
source:
  data: 'godot/data/quests/2115-the-outrider-s-trial.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2115-01, ql21-001/2115-02, ql21-001/2115-21, ql21-001/2115-31, ql21-001/2115-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Outrider's Trial

Kesh, the caravan's outrider, will make you a Hawker if you are quick enough. Bring 10 Honeybee Wings, whole.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1120-outrider-kesh|Outrider Kesh]] offers it from level 10, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Bring Kesh 10 Honeybee Wings from the Honeybees and Needle Honeybees.
- **End:** Kesh made you a Hawker.

A job trial: only a Visitor can take it, and only one of the four trials at the camp at a time (the other mentors hide theirs while you have one). Finishing it makes you a Hawker right at the camp, the same job change the quests in [[zones/1-canyon-city-of-zant|Zant]] give, so you don't have to go to Zant for it.

When you hand it in you pick one reward: [[items/weapon/1125-outrider-bow|Outrider Bow]], [[items/weapon/1126-outrider-katar|Outrider Katar]]. Each pick is its own step below; the steps are the same apart from the item.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2115-01`

Happens by talking to [[npcs/1120-outrider-kesh|Outrider Kesh]].

Checks:

- your Level ≥ 10
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 615 = 0 (you haven't done this quest yet)
- your job is Visitor (job 0)

Then:

- you get the quest [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]

### `2115-02`

Happens by talking to [[npcs/1120-outrider-kesh|Outrider Kesh]] and picking the Outrider Bow.

Checks:

- you have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

Then:

- works on [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- you get 1 × [[items/weapon/1125-outrider-bow|Outrider Bow]]
- 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]] is taken
- your job becomes Hawker (job 311), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 615 to 1
- the quest ends (removed from your list)

### `2115-21`

Happens by talking to [[npcs/1120-outrider-kesh|Outrider Kesh]] and picking the Outrider Katar.

Checks:

- you have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

Then:

- works on [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- you get 1 × [[items/weapon/1126-outrider-katar|Outrider Katar]]
- 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]] is taken
- your job becomes Hawker (job 311), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 615 to 1
- the quest ends (removed from your list)

### `2115-31`

Happens by killing [[monsters/61-needle-honeybee|Needle HoneyBee]].

Checks:

- you have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

Then:

- works on [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- you get 1 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

### `2115-32`

Happens by killing [[monsters/62-honeybee|HoneyBee]].

Checks:

- you have [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 10 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

Then:

- works on [[quests/2115-the-outrider-s-trial|The Outrider's Trial]]
- you get 1 × [[items/quest/710-honeybee-wing|Honeybee Wing]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
