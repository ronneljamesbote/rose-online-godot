---
kind: rule
id: status-effects
name: Status effects
status: changed-from-irose
check_interval_s: 1
one_effect_per_type: true
poison_never_kills: true
damage_shield_never_kills: true
status_effect_types:
  - { effect: "Max HP Up, Max MP Up", does: "adds to max HP or MP", removed_by: ClearGood, reapply: "yes, if as strong" }
  - { effect: "Dash / Slow", does: "adds to or takes from movement speed", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Haste Attack / Slow Attack", does: "adds to or takes from attack speed", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Atk Power Increased / Atk Power Down", does: "adds to or takes from attack power", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Def Up / Def Down", does: "adds to or takes from defence", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Magic Resistance Up / Down", does: "adds to or takes from magic resistance", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Atk Accuracy Up / Down", does: "adds to or takes from hit", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Cri Up / Down", does: "adds to or takes from critical", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Dodge Rate Up / Down", does: "adds to or takes from avoid", removed_by: "ClearGood / ClearBad", reapply: "yes, if as strong" }
  - { effect: "Poisoned", does: "loses HP every second (never below 1 HP)", removed_by: ClearBad, reapply: "no" }
  - { effect: "Muting (silence)", does: "can't use skills", removed_by: ClearBad, reapply: "no" }
  - { effect: "Sleep", does: "can't act; any hit or miss wakes it", removed_by: ClearBad, reapply: "no" }
  - { effect: "Fainted (stun)", does: "can't act; damage doesn't end it", removed_by: ClearBad, reapply: "no" }
  - { effect: "Camouflage (Stealth)", does: "hidden; ends when you attack or use a skill", removed_by: "ClearGood, Detect", reapply: "no" }
  - { effect: "Cloaking", does: "hidden; attacking doesn't end it", removed_by: "ClearGood, Detect", reapply: "no" }
  - { effect: "Damage shield", does: "returns power% of each hit to the attacker", removed_by: ClearGood, reapply: "yes, if as strong" }
  - { effect: "Additional Damage", does: "adds power% to all damage you deal", removed_by: ClearGood, reapply: "yes, if as strong" }
  - { effect: "Taunt", does: "a monster attacks only the taunter", removed_by: ClearBad, reapply: "no" }
  - { effect: "ClearGood, ClearBad (Recovery), ClearAll, Detection", does: "instant: removes other effects", removed_by: "-", reapply: "-" }
poison_per_second:
  - { status: 7, hp_per_second: 10 }
  - { status: 8, hp_per_second: 20 }
  - { status: 9, hp_per_second: 30 }
  - { status: 10, hp_per_second: 44 }
  - { status: 11, hp_per_second: 60 }
  - { status: 58, hp_per_second: 30, note: "fire" }
source:
  code:
    - module/src/skills.rs (apply_effects, target_value, clear_status, clear_good, clear_all_status, status_tick, status_effects, refresh_entity, disabled_reason, is_invisible, break_disguise, wake, taunter, taunt_tick, shield_reflect, has_status_effects)
    - module/src/lib.rs (deal_damage, combat_tick, attack)
    - module/src/monster_brain.rs (attack, find_nearby)
    - module/src/vehicle.rs (get_on, get_off)
    - module/src/death.rs (getting up clears all effects)
    - crates/rose-game-common/src/components/status_effects.rs (StatusEffects::can_apply)
    - crates/rose-game-common/src/components/ability_values.rs (AbilityValuesAdjust)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_skill_adjust_value)
    - crates/rose-data-irose/src/status_effect_database.rs
  data: LIST_STATUS.STB (type, can be reapplied, cleared by, HP per second), LIST_SKILL.STB (effects, success, duration, changes)
---
# Status effects

Status effects are the buffs and debuffs that skills leave on characters and monsters:
a defence bonus from an aura, a slow from [[skills/1001-freeze|Freeze]], poison, a stun.
Each has a duration in seconds, a chance to land, and rules for what happens when another
effect of the same kind arrives. The skill pages list the effects each skill gives (see
[[rules/skills|Skills]] for how to read them). Potions that heal over time work differently
and are on [[rules/recovery|Recovery]].

## Kinds of status effects

The `status_effect_types` table above lists every kind the game supports. In short:

- **Stat buffs and debuffs.** Each one adds to or takes from one stat: max HP, max MP,
  movement speed, attack speed, attack power, defence, magic resistance, hit, critical or
  avoid. The "up" and "down" of one stat are separate kinds, so both can be on you at once;
  your stat is changed by the up value minus the down value.
- **Poisoned.** You lose HP once a second, by the amount in `poison_per_second` for that
  poison (for example [[skills/1141-curse|Curse]] gives 44 HP a second). Poison never takes
  you below 1 HP, and it doesn't wake a sleeping target.
- **Muting (silence).** You can't use skills. You can still move, attack and use items.
- **Sleep.** You can't move, attack, use skills or items, pick things up or sit. Whatever you
  were doing stops. Any hit or miss that doesn't kill you wakes you up at once.
- **Fainted (stun).** Like sleep, but taking damage doesn't end it.
- **Camouflage ([[skills/1841-stealth|Stealth]]).** Monsters and enemy players can't pick you
  as a new target: aggressive monsters don't notice you, and enemies can't click you for a
  skill or attack. A fight that already started carries on. Others see you faded. It ends
  when you attack or use a skill. [[skills/1671-detect|Detect]] ends it.
- **Cloaking.** The same hiding, but attacking doesn't end it. Only some monster and item
  skills give it.
- **Damage shield.** Each time you are hit, the attacker loses the skill's power in percent
  of the HP you lost (rounded down). The returned damage never takes them below 1 HP and
  doesn't count as a hit (it doesn't wake them or make a monster react).
- **Additional Damage** ([[skills/1211-damage-support|Damage Support]]). All your damage,
  normal attacks and skills, is raised by the skill's power in percent.
- **Taunt** ([[skills/411-taunt|Taunt]]). A taunted monster turns on the taunter and
  attacks nobody else until the taunt ends, the taunter leaves the zone or dies. Taunt has
  no effect on players.
- **Cleanses.** These act at once and don't stay:
  - **ClearGood** removes buffs (everything in the "ClearGood" column above).
  - **ClearBad** ([[skills/1041-purify|Purify]], shown as "Recovery") removes debuffs and
    taunts.
  - **ClearAll** removes both, but not taunts.
  - **Detection** ([[skills/1671-detect|Detect]]) ends Camouflage and Cloaking. A skill
    with Detection can also be aimed at a hidden character.
- Some effects can't be removed by any cleanse: the stat bonuses from item scrolls, and the
  self-debuffs that come with some soldier buffs (for example the Def Down part of a skill
  that gives "Atk Power Increased, Def Down").

## How strong an effect is

The strength of a stat effect comes from the skill's `changes` column, worked out when the
effect lands:

```math
\text{value} = \left\lfloor \text{target's current stat} \times \frac{\text{rate}}{100} + \text{amount} \times \frac{\text{caster INT} + 300}{315} \right\rfloor
```

"Rate" is the percentage in the `changes` column, "amount" the plain number. A monster's
INT counts as its level. For a damage shield or Additional Damage the strength is the
skill's `power` instead.

Example: [[skills/1011-defense-aura|Defense Aura]] level 1 shows Defense +25. Cast with
45 INT it gives $\lfloor 25 \times 345 / 315 \rfloor = \lfloor 27.38 \rfloor = 27$ defence.
[[skills/1001-freeze|Freeze]] level 1 shows Movement Speed -50%: on a
[[monsters/62-honeybee|HoneyBee]] with run speed 440 the slow is
$\lfloor 440 \times 50 / 100 \rfloor = 220$, so it runs at 220.

## Will it land?

A skill with an empty `success` column always lands its effects. Otherwise each effect of
the skill is rolled separately, with a random whole number from 1 to 100:

**Buffs** (effects a ClearGood would remove, plus ClearBad and ClearAll cleanses) land when

```math
\text{roll} \le \text{success} - (\text{target level} - \text{caster level})
```

So on yourself, or on someone your own level, the chance is the `success` value in percent.

**Everything else** (debuffs, stuns, taunts, Detection, ClearGood) lands when

```math
\text{roll} < \frac{\text{success} \times (2 \times \text{caster level} + \text{caster INT} + 20)}{0.6 \times \text{target resistance} + 5 + \text{target avoid}}
```

The target's level doesn't matter here, only its magic resistance and avoid.

Example: a level 15 Muse with 45 INT casts [[skills/1001-freeze|Freeze]] level 1
(success 60).

- On a [[monsters/62-honeybee|HoneyBee]] (resistance 24, avoid 29):
  $60 \times 95 / (14.4 + 5 + 29) = 117.8$. Every roll is below that, so it always lands.
- On a [[monsters/144-kaiman-ranger|Kaiman Ranger]] (resistance 139, avoid 115):
  $60 \times 95 / (83.4 + 5 + 115) = 28.02$. It lands on rolls 1 to 28: a **28%** chance.

Damaging skills try their effects on every target they hit. Skills that damage you and the
target together only try them when the damage was above 0.

## Stacking

- A character has **at most one effect of each kind** (one movement speed bonus, one
  poison, one stun ...).
- When a new effect arrives and one of the same kind is already on, the new one replaces it
  only if that kind **can be reapplied** (the `reapply` column) and the new strength is at
  least the old one. It then starts with its full duration. Otherwise the new effect is
  lost.
- Poison, silence, sleep, stun, Camouflage, Cloaking and taunt can't be reapplied: a second
  one doesn't land until the first has ended.
- Effects of different kinds all work together.

## How effects end

- **Time.** Each effect lasts the skill's `duration` in seconds. Effects are checked once a
  second, so one can last up to a second longer than its duration. Poison does its damage
  at the same checks, so a 3 second poison usually hits 3 times.
- **Cleanses**, as described above.
- **Sleep** ends on any hit or miss; **Camouflage** ends when you attack or use a skill.
- **Getting on or off a cart or castle gear** removes your buffs (as a ClearGood).
- **Getting up after dying** removes every effect (see [[rules/death|Death]]); so does
  being brought back by [[skills/1131-resurrection|Resurrection]].
- **Logging out** or leaving the world removes every effect. A monster's effects go when
  it dies.

When an effect starts or ends, the character's stats are worked out again at once.

## Changed from iROSE

- **Damage shields** return part of every hit. In iROSE they only return magic damage
  (and LIST_STATUS.STB names it a shield for attackers within 4 m; there is no distance
  limit here).

> Open question: the damage shield's "within 4 m" from LIST_STATUS.STB is not applied. Decide whether to add it.
