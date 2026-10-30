---
kind: quest
id: 2114
name: 'The Sage''s Trial'
status: in-game
given_by:
- '[[npcs/1119-oasis-sage-orin|Oasis Sage Orin]]'
level: 10
steps: 5
source:
  data: 'godot/data/quests/2114-the-sage-s-trial.tres; quest triggers in godot/data/quest_triggers/: ql21-001/2114-01, ql21-001/2114-02, ql21-001/2114-21, ql21-001/2114-31, ql21-001/2114-32'
  code:
  - module/src/quests.rs
  - crates/rose-quest/src/lib.rs
---
# The Sage's Trial

Orin, the oasis sage, will make you a Muse if you can find where the valley's mana gathers. Bring 6 Humming Pomic Dust from the Pomics.

Part of the [[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]] story; see "Quests" on the zone page for the whole chain.

[[npcs/1119-oasis-sage-orin|Oasis Sage Orin]] offers it from level 10, after [[quests/2101-dust-on-the-road|Dust on the Road]].

- **Start:** Bring Orin 6 Humming Pomic Dust from the Pomics and Needle Pomics.
- **End:** Orin made you a Muse.

A job trial: only a Visitor can take it, and only one of the four trials at the camp at a time (the other mentors hide theirs while you have one). Finishing it makes you a Muse right at the camp, the same job change the quests in [[zones/1-canyon-city-of-zant|Zant]] give, so you don't have to go to Zant for it.

When you hand it in you pick one reward: [[items/weapon/1123-oasis-staff|Oasis Staff]], [[items/weapon/1124-oasis-wand|Oasis Wand]]. Each pick is its own step below; the steps are the same apart from the item.

## Steps

Each step is a quest trigger. A step runs when all its checks pass; then it gives
everything under "Then". See [[rules/quests|Quests]].

### `2114-01`

Happens by talking to [[npcs/1119-oasis-sage-orin|Oasis Sage Orin]].

Checks:

- your Level ≥ 10
- quest switch 600 = 1 (you finished [[quests/2101-dust-on-the-road|Dust on the Road]])
- quest switch 614 = 0 (you haven't done this quest yet)
- your job is Visitor (job 0)

Then:

- you get the quest [[quests/2114-the-sage-s-trial|The Sage's Trial]]

### `2114-02`

Happens by talking to [[npcs/1119-oasis-sage-orin|Oasis Sage Orin]] and picking the Oasis Staff.

Checks:

- you have [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

Then:

- works on [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- you get 1 × [[items/weapon/1123-oasis-staff|Oasis Staff]]
- 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]] is taken
- your job becomes Muse (job 211), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 614 to 1
- the quest ends (removed from your list)
- then runs step `801-mentor` ([[quests/801-junon-planet-identification|Junon Planet Identification]]): if you have that quest, it ends and your Identification papers are taken

### `2114-21`

Happens by talking to [[npcs/1119-oasis-sage-orin|Oasis Sage Orin]] and picking the Oasis Wand.

Checks:

- you have [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- your job is Visitor (job 0)
- you carry ≥ 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

Then:

- works on [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- you get 1 × [[items/weapon/1124-oasis-wand|Oasis Wand]]
- 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]] is taken
- your job becomes Muse (job 211), as the job change quests in Zant do it
- job variable 0 goes up by 1 (first job done)
- HP and MP are filled
- experience, base 800 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- Zuly, base 500 (reward formula 0: base plus a bonus from Charm, smaller at higher levels, see [[rules/quests|Quests]])
- set quest switch 614 to 1
- the quest ends (removed from your list)
- then runs step `801-mentor` ([[quests/801-junon-planet-identification|Junon Planet Identification]]): if you have that quest, it ends and your Identification papers are taken

### `2114-31`

Happens by killing [[monsters/71-needle-pomic|Needle Pomic]].

Checks:

- you have [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

Then:

- works on [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- you get 1 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

### `2114-32`

Happens by killing [[monsters/72-pomic|Pomic]].

Checks:

- you have [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- a random roll 0–99 lands in 0–49
- you carry < 6 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

Then:

- works on [[quests/2114-the-sage-s-trial|The Sage's Trial]]
- you get 1 × [[items/quest/709-humming-pomic-dust|Humming Pomic Dust]]

## Changed from iROSE

This quest is new: iROSE has no such quest. It was added with the Valley of Luxem Tower redesign.
