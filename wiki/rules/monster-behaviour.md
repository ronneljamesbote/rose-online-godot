---
kind: rule
id: monster-behaviour
name: Monster behaviour
status: changed-from-irose
combat_tick_ms: 100
idle_check_s: 1
minimum_idle_interval_s: 1
leash_m: 30
wander_limit_m: 24
home_reached_m: 0.1
attack_range_slack_m: 0.5
chase_repath_m: 1
aip_triggers:
  - { trigger: "Idle", when: "every idle interval, while the monster stands still with nothing to fight", runs: "yes" }
  - { trigger: "Damaged", when: "when it is hit and lives", runs: "yes" }
  - { trigger: "Attack", when: "after each of its swings lands", runs: "only its skill-using events" }
  - { trigger: "Created", when: "when it appears", runs: "no" }
  - { trigger: "Kill", when: "when it kills", runs: "no" }
  - { trigger: "Dead", when: "when it dies", runs: "no" }
source:
  code:
    - module/src/monster_brain.rs (idle_tick, on_damaged, on_attack, Run::check, Run::act, Run::attack, Run::walk, find_nearby)
    - module/src/lib.rs (combat_tick, deal_damage, set_aggro_range, clear_damage_sources)
    - module/src/skills.rs (npc_cast, taunter, taunt_tick, is_invisible, summon_tick)
    - module/src/world.rs (spawn_monster)
    - crates/rose-file-readers/src/aip.rs
  data: LIST_NPC.STB (AI file of each monster), 3DDATA/AI/*.AIP
---
# Monster behaviour

Every monster type has an **AI script** (an AIP file from the iROSE data, named in
LIST_NPC.STB). The script decides whether the monster wanders, attacks players on sight,
fights back, calls friends for help, runs away or uses skills. The server runs the same
scripts the original game used, so each monster behaves as its data says. On top of the
scripts, the server has a few fixed rules: chasing, giving up and walking home.

## How a script works

A script has **triggers**. Each trigger is a list of **events**, and each event is a list of
conditions and a list of actions. When a trigger runs, the **first event whose conditions
all pass** does its actions; the rest are skipped.

Triggers the game runs are in `aip_triggers` above:

- **Idle.** Each monster runs its idle trigger once per idle interval (set in its script,
  at least 1 second). The server looks once a second. It only runs while the monster is
  alive, has no target, is not moving, is not stunned or asleep and is not walking home.
  New monsters start at a random point in their first interval, so they don't all move at
  once. Most idle events make the monster wander; aggressive monsters look for a player
  nearby and attack.
- **Damaged.** When the monster is hit and lives, it runs its damaged trigger: usually fight
  back, sometimes call friends or run from strong players. If it is already fighting
  someone, it only reacts to a new attacker with the chance its script gives (the "new
  target chance"). A monster without a damaged trigger just fights back if it has no target.
  A stunned or asleep monster doesn't react (a hit wakes a sleeper first, so a sleeper does
  react).
- **Attack.** After each of its swings that doesn't kill, the monster runs the events of its
  attack trigger that use a skill, and only their skill actions (mostly self-buffs). Other
  events of this trigger, which in iROSE make it step around mid-fight, are not run.

Monsters only think in zones with a player in them.

### Conditions

A condition can check: players or monsters nearby (how many, within what distance, of what
level difference); the damage just taken; the distance to its home or its target; its own
HP percentage; whether the attacker is its current target; the attacker's or target's
level, attack, defence, magic resistance or HP; its own stats; a random roll; whether it or
its target has buffs or debuffs; day or night, the zone's time and the world's time.
Conditions about clans, channels, owners, calendar days and monster variables are not kept
by the game and always fail.

Hidden players (Stealth) and dead ones are never found by "players nearby" checks.

### Actions

| Action | What the monster does |
| --- | --- |
| Stop | stops moving |
| Move randomly | walks or runs to a random spot up to the given distance from where it is, its home, or a player it found |
| Move away | walks or runs the given distance away from its target |
| Attack nearest / found player | attacks the nearest player of the last "nearby" check, or the one that check found |
| Attack attacker | attacks whoever just hit it |
| Attack by stat | attacks the player (or summon) in range with the lowest or highest level, attack, defence, resistance or HP |
| Call for help | monsters of the same type (or any type, or a given type) within the given metres that have no target attack its target, up to a number if the script gives one |
| Use skill | casts a skill on itself, its target or a player it found (see [[rules/skills|Skills]]) |

Speech and emotes are shown by the game client. Summoning other monsters,
transforming, dropping items and the other iROSE actions are not in the game yet.

A monster never wanders farther than **24 m** from its home: a move that would take it
farther stops at 24 m.

## Attacking first

Whether a monster attacks players on sight comes from its idle trigger. If it is one of
the aggressive kinds, its script finds a player within the distance (and level difference)
it gives, and attacks. Peaceful monsters only fight back. An admin can also make every
monster of one type attack any player within a set distance (the `set_aggro_range`
tool); this is off unless set.

A monster can't pick a hidden player (Stealth) as a new target, but keeps fighting one
it was already fighting. A taunted monster attacks only its taunter (see
[[rules/status-effects|Status effects]]).

## Chasing and fighting

- A monster with a target runs to it at its **run speed** and attacks once within its
  attack range plus **0.5 m**. It re-aims when the target has moved more than **1 m** from
  where it was heading.
- Its attacks follow the normal combat rules ([[rules/combat|Combat]]).
- It stops when its target dies, leaves the zone or logs out. A fight between a monster
  and a player doesn't end any other way, except by giving up.

## Giving up and walking home

Every monster has a **home**: the spot where it appeared at its spawn point
([[rules/monster-spawns|Monster spawns]]).

When a monster is more than **30 m** from home, it gives up:

1. it drops its target and is **healed to full HP** at once,
2. the record of who damaged it is wiped, so players who hurt it before get no experience
   for that damage if it is killed later ([[rules/experience|Experience]]),
3. it runs home at its run speed. On the way it doesn't look for players or wander, until it
   is within **0.1 m** of home.

## Summoned monsters

Monsters summoned by players' skills have no script. They follow their owner and fight for
them; the rules are on [[rules/skills|Skills]]. Monsters fight back against summons as
against players, and "players nearby" checks find summons too.

## Changed from iROSE

- **Attack trigger.** Only the skill-using events of a monster's attack trigger run; the
  other events (moving around mid-fight) don't.
- **Created, kill and dead triggers** are not run, and the script actions that summon
  monsters, transform or drop items are not in the game yet.
- The server has no map geometry for monsters, so they walk through walls and objects.

> Open question: a monster that is hit while walking home turns to fight its attacker, and the 30 m give-up check is not made again until it reaches home. It can then chase a player any distance.
