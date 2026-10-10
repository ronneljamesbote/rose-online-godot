---
kind: rule
id: monster-spawns
name: Monster spawns
status: in-game
spawn_check_tick_s: 1
minimum_interval_s: 1
tactics_value_max: 500
monster_types_per_point: up to 5 basic groups and 2 reinforcement groups are used
spawn_table:
  - { regen: "0-10", spawns: "basic 1", tactics: "+12" }
  - { regen: "11-15", spawns: "basic 1 (2 fewer), basic 2", tactics: "+15" }
  - { regen: "16-25", spawns: "basic 3", tactics: "+12" }
  - { regen: "26-30", spawns: "basic 1 (1 fewer), basic 3", tactics: "+15" }
  - { regen: "31-40", spawns: "basic 4", tactics: "+12" }
  - { regen: "41-50", spawns: "basic 2, basic 3 (1 fewer)", tactics: "+12" }
  - { regen: "51-65", spawns: "basic 3, basic 4 (2 fewer)", tactics: "+20" }
  - { regen: "66-73", spawns: "basic 4, basic 5", tactics: "+15" }
  - { regen: "74-85", spawns: "basic 1, basic 5 (2 fewer), reinforcement 1 (1 fewer)", tactics: "+15" }
  - { regen: "86-92", spawns: "basic 2, reinforcement 1, reinforcement 2", tactics: "set to 1" }
  - { regen: "93 and up", spawns: "basic 5, reinforcement 1 (1 more), reinforcement 2", tactics: "set to 7" }
source:
  code:
    - module/src/world.rs (setup_zones, spawn_tick, spawn_queue, spawn_monster)
    - module/src/lib.rs (spawn_tick timer, reset_monsters, kill, despawn)
  data: zone IFO files (monster spawn blocks), LIST_ZONE.STB, LIST_NPC.STB
---
# Monster spawns

Monsters come from **spawn points** placed on each zone's map. Every zone page shows its
spawn points (see for example [[zones/20-birth-island|Birth Island]]). A spawn point keeps
a few monsters around itself and brings more, and stronger "reinforcements", the more it is
fought over.

## What a spawn point has

Each spawn point comes from the zone's map files (the IFO files of iROSE) and has:

| On the zone page | Meaning |
| --- | --- |
| x, y | its centre, in metres |
| radius | monsters appear within this many metres of the centre |
| max_alive | how many of its monsters it tries to keep alive (the "limit") |
| respawn_seconds | how often it checks whether to add monsters (the "interval"; at least 1 s) |
| monsters | its **basic groups**, in order: a monster and how many come at once |
| reinforcements | its **reinforcement groups** (iROSE "tactics"), used when it is fought hard |

It also has a hidden **tactic points** number from the map file (100 for most points),
and keeps a running **tactics value** that starts at 0.

## When monsters appear

- Spawn points only work in zones where at least one player is online. The first check
  happens as soon as a player is in the zone; after that a point checks every
  `respawn_seconds`. The server looks at all spawn points once a second.
- A monster belongs to the spawn point it came from. It counts as alive until it dies.
  Dead monsters are removed at once, so they stop counting straight away.
- New monsters appear at a random spot within the radius of the centre (spots near the
  centre are a little more likely). That spot becomes the monster's home: it wanders
  around it and walks back to it after a chase (see
  [[rules/monster-behaviour|Monster behaviour]]).
- Monsters stay in a zone after every player has left it; they just don't think or
  spawn until someone comes back.

## The check

At each check, with $n$ monsters of this point alive:

1. If $n$ is at `max_alive` or more, nothing spawns and the tactics value goes down by 1.
2. Otherwise the **regen value** is worked out (rounded down):

```math
\text{regen} = \left\lfloor \frac{(2 \times \text{max alive} - n) \times \text{tactics} \times 50}{\text{max alive} \times \text{tactic points}} \right\rfloor
```

3. The regen value picks a row of `spawn_table` above. Each group in the row spawns its
   monster, with its count changed as shown ("2 fewer" never goes below 0). A group the
   point doesn't have spawns nothing.
4. The tactics value changes as the row says ("+12" adds, "set to 7" replaces), and never
   goes above **500**.

So a quiet spawn point fills up slowly with its first basic groups. While players kill its
monsters it stays below `max_alive`, the tactics value keeps climbing, and the regen value
grows until reinforcements come. After a reinforcement wave the tactics value drops back to
1 or 7 and the cycle starts again.

`max_alive` is not a hard cap: when a point is below it, a whole wave spawns, which can take
it over (for example from 16 to 20 with a limit of 17).

## Worked example

The first spawn point on [[zones/20-birth-island|Birth Island]] has max alive 17,
respawn every 5 seconds, radius 40 m and tactic points 100. Its basic groups are
[[monsters/1-mini-jelly-bean|Mini-Jelly Bean]] ×2, [[monsters/8-butterfly|ButterFly]] ×1,
[[monsters/11-mini-choropy|Mini-Choropy]] ×2, [[monsters/9-butterfly|ButterFly]] ×2 and
[[monsters/2-jelly-bean|Jelly Bean]] ×2; its reinforcements are Jelly Bean ×2 and
Mini-Jelly Bean ×4.

If nobody fights there, the first checks go like this:

| Check | Alive | Tactics | Regen | Spawns | Tactics after |
| --- | --- | --- | --- | --- | --- |
| 1 | 0 | 0 | 0 | Mini-Jelly Bean ×2 | 12 |
| 2 | 2 | 12 | $\lfloor 32 \times 12 \times 50 / 1700 \rfloor = 11$ | ButterFly ×1 (and Mini-Jelly Bean ×0) | 27 |
| 3 | 3 | 27 | 24 | Mini-Choropy ×2 | 39 |
| 4 | 5 | 39 | 33 | ButterFly ×2 | 51 |
| 5 | 7 | 51 | 40 | ButterFly ×2 | 63 |
| 6 | 9 | 63 | 46 | ButterFly ×1, Mini-Choropy ×1 | 75 |
| 7 | 11 | 75 | 50 | ButterFly ×1, Mini-Choropy ×1 | 87 |
| 8 | 13 | 87 | 53 | Mini-Choropy ×2 | 107 |
| 9 | 15 | 107 | 59 | Mini-Choropy ×2 | 127 |
| 10 | 17 | 127 | - | nothing (full) | 126 |

About 40 seconds after a player arrives the point is full. If players now kill 10 of them,
the next check has 7 alive and tactics about 120:
$\lfloor (34 - 7) \times 120 \times 50 / 1700 \rfloor = 95$, so the reinforcement row
spawns Jelly Bean ×2, Jelly Bean ×3 and Mini-Jelly Bean ×4, and the tactics value is set
to 7.
