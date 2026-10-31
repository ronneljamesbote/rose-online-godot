---
kind: rule
id: skills
name: Skills
status: changed-from-irose
skill_pages: 4 (basic, active, passive, clan)
skill_slots_per_page: 30
skill_points_per_level: "floor((new level + 2) / 2)"
global_cooldown_s: 0.25 (from when a skill takes effect)
cooldown_unit_s: 0.001 (cooldown_ms in the skill resources; every cooldown converted from iROSE is a step of 0.2 s)
minimum_cast_time_s: 0.3
cast_grace_window_s: 0.3
range_slack_m: 0.5
default_cast_range: the caster's attack range (when the skill has no range)
skill_damage_minimum: 1
skill_damage: "(attack, or magic attack for magic) x (1 + power / 100), then the Combat damage steps"
magic_attack: "attack x 0.8 + INT x 1.2"
skill_damage_maximum: 2047
pvp_skill_damage_cap: 45% of the target's max HP
base_summon_points: 50
summon_spawn_distance_m: 0.5 to 1.5
summon_follow_distance_m: 5.5
summon_follow_to_m: 3.5
summon_stops_fighting_m: 22
resurrection_hp: 30% of max HP
skill_damage_types:
  - { type: weapon attack, hit_check: "the plain attack hit chance (Combat, Hit or miss)", on_miss: "0 damage, no effects" }
  - { type: magic attack, hit_check: "never misses" }
  - { type: natural magic, hit_check: "never misses" }
  - { type: continuous attack, hit_check: "(L_a + 8 - L_d + 1..80) x (Hit - 0.6 Avoid + 1..50 + 50) / 90", miss_below: 10 }
cost_types:
  - { cost: MP, rule: "lowered by the caster's Save Mana %, rounded down" }
  - { cost: HP, rule: "can't take you below 1 HP" }
  - { cost: Stamina, rule: "see Stamina" }
  - { cost: Experience, rule: "taken from your current experience" }
  - { cost: Zuly, rule: "taken from your money" }
  - { cost: Fuel, rule: "taken from your cart or castle gear" }
source:
  code:
    - module/src/skills.rs (learn_skill, level_up_skill, check_requirements, cast_skill, start_cast, advance_cast, cast_locked, queue_if_locked, run_queued, pay_locked_cast, check_can_use, pay_costs, use_cost, motion_us, cast_tick, take_effect, targets, target_allowed, damage, weapon_wear, apply_effects, target_value, summon, summon_capacity, summon_tick, resurrect, npc_cast, use_scroll, consume_scroll)
    - module/src/items.rs (use_item, skill books and scrolls)
    - module/src/character.rs (reward_xp, skill points on level up)
    - module/src/lib.rs (deal_damage, combat_tick, move_to, attack, stop, sit)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_skill_damage, roll_damage, hit_damage, magic_attack, calculate_skill_adjust_value, calculate_npc, calculate_levelup_reward_skill_points)
    - crates/rose-data-irose/src/skill_database.rs (get_cooldown, load_skill)
  data: the skill resources in godot/data/skills/ (a folder per skill, a file per level), the status effects in godot/data/status_effects/, the job classes in godot/data/job_classes/, the monster resources in godot/data/monsters/ (summons)
---
# Skills

Skills are the special moves, spells, buffs and passive bonuses of your character. You
learn a skill from a skill book (or a quest), spend skill points to raise its level, and use
it from the skill window or the hotbar. Every skill has its own page in [[skills|Skills]]
with a table of its levels; this page explains how all skills work and how to read those
tables. Buffs and debuffs are explained in detail on
[[rules/status-effects|Status effects]].

## Learning skills

- **Skill books.** Using a skill book from your inventory teaches its skill at level 1. The
  book is used up. Each skill page lists its books under `skill_books`. Some quests teach
  skills too.
- **Skill points.** You get skill points when you level up: on reaching a new level you get
  $\lfloor (\text{level} + 2) / 2 \rfloor$ points (2 points at level 2, 6 at level 10, 26
  at level 50). Quests can give or take skill points.
- **Requirements.** To learn a skill (or raise it a level) you need all of these, checked
  in this order:
  1. enough skill points for that level (the `learn_points` column),
  2. the right job: the skill page's `job` names a job class (godot/data/job_classes/), and your
     job must be one of the jobs in it ("any" means every job),
  3. the other skills in the `needs` column at the levels shown,
  4. the stats in `needs_level` and `needs` (for example level 25, or a minimum STR).
- Learning a skill takes its `learn_points` from your skill points. You can't learn a
  skill you already know.
- Skills go into one of four pages (basic, active, passive, clan) of **30** slots each, as
  the skill's data says. When the page is full you can't learn more skills on it.

## Levelling skills

Each level of a skill is its own skill resource with its own id (the `id` column of the
levels table), all in the skill's folder in godot/data/skills/. To raise a skill, click its
level-up button in the skill window: the next level must be the one with the same base skill
(`base_skill_id`) and the next `level`, and you must meet that level's
requirements (points, job, skills, stats) as when learning. The cost is that level's
`learn_points`. A skill at its last level (`max_level`) can't go higher.

## Using a skill

1. **Starting.** You pick the skill and a target (a character, a spot on the ground for
   area skills, or nobody for skills on yourself). The game checks at once that you could
   use it (see the checks below), so a refused skill doesn't make you walk anywhere.
   Using a skill makes you stand up, stops your normal attacks and ends a disguise
   ([[skills/1841-stealth|Stealth]]).
2. **Walking into range.** If the target is farther than the skill's `range` (plus
   **0.5 m** of slack), your character walks or runs to it first. A skill with no range
   uses your normal attack range.
3. **Casting.** In range, the checks run again and your character plays the skill's
   casting motion. The skill **takes effect at the end of the casting motion**, then the
   action motion plays. A cast always lasts at least **0.3 s** in total.
4. **Grace window.** For the first **0.3 s** of the casting motion you can still call the
   skill off: moving, attacking, stopping, sitting or using another skill cancels it and
   nothing is spent. Using the **same skill again** on the same target (or again at all,
   for skills on yourself or on the ground) does nothing until it has taken effect, from
   the walk into range to the end of the casting motion, so pressing its hotbar key over
   and over doesn't restart the casting.
5. **Locked in.** After the grace window the costs are paid (checked once more first) and
   the cast can't be cancelled by you any more. The last thing you click while it is locked
   in (a move, an attack, stop, sit or a different skill) waits and happens the moment the
   skill takes effect. Emotes, jumping and scrolls are refused until then. A stun or sleep
   landing on you still cancels the cast, and so does dying; the costs are spent by then.
6. **After the effect.** The action motion can be cut short by anything you do.
7. **When it ends.** Melee skills (those whose action mode is "attack") go on attacking
   their own target. Every other skill goes back to the attack you were in before you used
   it, if that target is still alive and can still be fought. If you clicked something
   while the skill was locked in, that wins instead.

The checks, every time:

- you are alive, not stunned or asleep, and not riding as a passenger,
- you are not silenced ([[rules/status-effects|Status effects]]),
- the skill's own cooldown is over. The **global cooldown** (**0.25 s** after any skill
  takes effect) doesn't refuse a skill: one used during it is accepted, and its casting
  starts the moment the global cooldown ends,
- you can pay every cost (see below),
- you hold a weapon (or shield) of a class the skill needs (`needs_weapon` on the skill
  page), unless it is a cart or castle gear skill, which has its own vehicle rules
  ([[rules/carts|Carts]]),
- the target is one the skill may be used on (the `target` field: yourself, a party
  member, an ally, a hostile character, a monster, a fallen ally ...). Hidden characters
  (Stealth) can't be picked by enemies, except with a skill that reveals them
  ([[skills/1671-detect|Detect]]).

Passive skills work on their own (they add to your stats all the time) and can't be used.
Emotes and jumping only play their motion. While your personal shop is open, the only
action you can use is sitting.

### Costs

The `cost` column lists what one use costs. MP costs are lowered by your **Save Mana**
stat:

```math
\text{MP cost} = \left\lfloor \text{cost} \times \frac{100 - \text{Save Mana}}{100} \right\rfloor
```

Example: [[skills/981-ice-bolt|Ice Bolt]] level 1 costs MP 30. With 10% Save Mana it costs
$\lfloor 30 \times 90 / 100 \rfloor = 27$ MP.

An HP cost never takes you below 1 HP. Other costs (Stamina, experience, Zuly, fuel) are
taken as listed; see [[rules/stamina|Stamina]] and [[rules/carts|Carts]].

### Cooldowns

The `cooldown` column is in seconds (the data stores it in steps of 0.2 s). It counts from
when the casting starts, not when it takes effect, but it is only spent once the cast locks
in (a skill cancelled in its grace window starts no cooldown). Every skill has its own cooldown; the data can
also put several skills in a **cooldown group** that shares one cooldown, but no skill in the
current iROSE data uses a group. On top of that, every skill starts the 0.25 s global
cooldown when it takes effect, so no skill's casting can start sooner than 0.25 s after
the last one took effect. A skill used during it (or waiting from a locked-in cast) starts
when it ends. Emotes and jumping start it when they are used.

While a skill's own cooldown runs, its icon in the hotbar and the skill window is darkened
by a sweep that shrinks clockwise, with the seconds left in the middle. After any skill,
every skill icon also shows a quick sweep (no number) for the 0.25 s global cooldown.

### Scrolls

Scrolls (magic items) cast a skill on you or your target when used, and are used up when
that skill takes effect. If you moved or sold the scroll in the meantime, the skill fails.
Return scrolls are warp skills: they move you to the zone and spot in the skill's data.

## Who a skill reaches

- A skill with an `area` reaches everyone allowed by its `target` within that many metres
  of the aim point: the target character, the spot on the ground, or you for skills on
  yourself. Town NPCs are never reached.
- A skill with no area reaches only its one target (or you).
- **Party skills** (target "Party Member") reach you and the members of your party in the
  area. Clan skills reach only you, because there are no clans yet.
- Players are all on one side, except where they may fight each other
  ([[rules/pvp|PvP]]). Summons are on their owner's side.

## Skill damage

Damaging skills (the ones with a `power` column) hit each target once. First a hit check
decides whether the skill lands, then the skill does the same damage as a plain attack
([[rules/combat|Combat]] "Damage") with its power added.

**Hit check.** It depends on the damage type (see `skill_damage_types` above):

- **Weapon attack** skills roll the same hit chance as a plain attack (hit against avoid,
  ±1.5% per level, 5% to 95%, on [[rules/combat|Combat]] "Hit or miss"). A miss shows
  "Miss", does no damage and gives no status effects.
- **Magic attack** and **natural magic** skills never miss.
- **Continuous attack** skills (the double hits:
  [[skills/321-double-attack|Double Attack]], [[skills/1521-double-shot|Double Shot]],
  [[skills/1541-double-slash|Double Slash]], [[skills/2221-twin-bullets|Twin Bullets]],
  [[skills/941-staff-stun|Staff Stun]]) keep the iROSE check
  $(L_a + 8 - L_d + \text{1..80}) \times (\text{Hit}_a - 0.6\,\text{Avoid}_d + \text{1..50} + 50) / 90$:
  below 10 they miss, otherwise they hit in full.

**Damage.** A skill starts from your attack (weapon attack and continuous attack skills)
or your **magic attack** (magic attack and natural magic skills):

```math
\text{magic attack} = \text{attack} \times 0.8 + \text{INT} \times 1.2
```

and adds its power as a percent:

```math
\text{skill attack} = \text{attack} \times \left(1 + \frac{P}{100}\right)
```

Then every step of a plain attack follows: × 1.2 for players (× 0.5 for monsters), the
90% to 110% roll, the share defence takes off (magic skills: magic resistance only), a
critical roll with your critical chance and damage, ±2% per level of difference (at most 10
levels), Additional Damage buffs ([[skills/1211-damage-support|Damage Support]]), and
between two players half damage, at most **45%** of the defender's max HP. A hit does at
least **1** and at most **2047**, rounded down. A skill with power 0 hits like a plain
swing.

Each skill's power was set once (2026-10-31) so that, at the level its skill book needs
(for monster skills: the level of the monsters that use it), the skill does the same
multiple of a plain swing as it did before. A skill that hit about four times as hard as a
swing still does.

A damaging skill wears your weapon (see [[rules/durability|Durability]]), except natural
magic. Damage from skills counts like any other damage for experience and drops (see
[[rules/combat|Combat]]), wakes a sleeping target and makes a monster react
([[rules/monster-behaviour|Monster behaviour]]).

### Worked example

A level 15 Muse with attack 50, INT 45 and SEN 20 casts
[[skills/981-ice-bolt|Ice Bolt]] level 1 (magic attack, power 120) at a
[[monsters/62-honeybee|HoneyBee]] (level 12, magic resistance 24), with a middle roll and
no critical:

- magic attack: 50 × 0.8 + 45 × 1.2 = 94; with the power: 94 × 2.2 = 206.8;
- raw: 206.8 × 1.2 = 248.2;
- magic resistance takes off 24 / (24 + 186.1 + 80) = 8.3%, leaving 227.7;
- 3 levels above the HoneyBee: × 1.06 = 241.3, shown as **241**.

The roll makes it 215 to 266. The HoneyBee has $12 \times 25 = 300$ HP, so two Ice Bolts
kill it.

## Buffs, debuffs and heals

A skill can carry up to two status effects (the `effects` column), each with a `duration`
and a `success` chance. How they land, stack and end is on
[[rules/status-effects|Status effects]]. Damaging skills try their effects on every target
they hit (not on a miss).

Instant heals: a skill whose `changes` column adds HP heals each target at once by

```math
\text{heal} = \left\lfloor \text{current HP} \times \frac{\text{rate}}{100} + \text{value} \times \frac{I_{caster} + 300}{315} \right\rfloor
```

capped at the target's max HP. An MP change adds its value as it is (no INT bonus).
Example: [[skills/931-cure|Cure]] level 1 (HP +200) cast by a character with 45 INT heals
$\lfloor 200 \times 345 / 315 \rfloor = \lfloor 219.05 \rfloor = 219$ HP.

## Summons

Summoning skills (`summons` on the skill page, for example
[[skills/1151-call-butterfly|Call Butterfly]]) make a monster appear **0.5 to 1.5 m** from
you, on your side.

- **Summon points.** You have **50** summon points, plus what passive skills add. Each
  summon uses the points in its monster's data (`summon_point_requirement`); the ButterFly uses 30. A
  summon that would go over your points fails ("you can't summon any more").
- **Its stats** come from the monster's data, scaled by your level $L_o$ and the skill's
  level $k$ (all rounded down). Its level is your level.

| Stat | Summon's value |
| --- | --- |
| max HP | HP × (k + 16) × (L_o + 85) / 2600 |
| attack | attack × (k + 22) × (L_o + 100) / 4000 |
| hit | hit × (k + 30) × (L_o + 50) / 3200 |
| defence | defence × (k + 30) × (L_o + 80) / 4400 |
| magic resistance | resistance × (k + 24) × (L_o + 90) / 3600 |
| avoid | avoid × (k + 22) × (L_o + 90) / 3400 |

  Example: a level 30 Muse with Call Butterfly level 1 gets a ButterFly
  ([[monsters/811-butterfly|monster 811]]: HP 1154, attack 131) with
  $1154 \times 17 \times 115 / 2600 = 867$ HP and $131 \times 23 \times 130 / 4000 = 97$
  attack.
- **What it does.** It attacks your target first (what you are attacking or casting at).
  Otherwise it fights whoever is attacking you or it. With nothing to fight it follows you:
  once it is more than **5.5 m** away it runs back to **3.5 m** from you. If it gets more
  than **22 m** from you it stops fighting and comes back.
- **Its kills count as yours** (experience, drops and quests). A summon that is killed
  just disappears: no experience or drops.
- **It leaves** when you die, log out or change zone.

## Resurrection

[[skills/1131-resurrection|Resurrection]] targets a fallen ally. They get up where they
fell with **30%** of their max HP (worked out after their buffs and debuffs are cleared),
and get back the skill's power in percent of the experience their death cost. See
[[rules/death|Death]].

## Monster skills

Monsters use the skills their AI script names, when they are hit and after their own
attacks (see [[rules/monster-behaviour|Monster behaviour]]). They play their own casting
motions, pay no costs and have no cooldowns. A monster that is asleep, stunned, silenced
or already casting can't start a skill, and the target must be in the skill's range.

## How to read a skill page

The data block at the top gives the skill as a whole: `type`, `job` (who can learn it),
`max_level`, `target` (who it can be used on), `damage_type` (for damaging skills),
`needs_weapon`, `summons` and `skill_books`. The `levels` table has one row per level:

| Column | Meaning |
| --- | --- |
| level, id | the level, and its id (the number of its skill resource) |
| needs_level | the character level needed to learn this level |
| needs | other skills (at the level shown) and stats needed |
| learn_points | skill points this level costs |
| cost | what one use costs (MP before Save Mana, HP, Stamina ...) |
| power | the skill's power: damage power, or the percentage of an Additional Damage or damage shield buff |
| range | how far away the target can be, in metres (empty: your attack range) |
| area | radius in metres of the area it reaches (empty: one target) |
| cooldown | seconds before you can use it again |
| duration | seconds its status effects last |
| success | the base chance of its status effects landing (see [[rules/status-effects\|Status effects]]); empty means they always land |
| effects | the status effects it gives |
| changes | the stat each effect changes, or the HP/MP it restores |

How to read `changes`:

- A number like **Defense +25** is a flat amount that grows with the caster's INT:
  $\text{value} \times (\text{INT} + 300) / 315$, rounded down. With 45 INT,
  Defense +25 gives $\lfloor 25 \times 345 / 315 \rfloor = 27$ defence.
- A percentage like **Movement Speed +50%** is that share of the target's current value.
- Both can appear together; they add up.
- For a debuff the change is shown as a negative number: [[skills/1001-freeze|Freeze]]
  level 1 shows **Movement Speed -50%**, so a [[monsters/62-honeybee|HoneyBee]] running at
  440 is slowed by $440 \times 50 / 100 = 220$, to 220.

## Changed from iROSE

- **Skill damage** (2026-10-31): iROSE had four formulas of its own (and four more between
  players) built on power, attack, Sense and level, divided by defence plus avoid. Ours is
  the plain attack's damage with the power added as a percent, so skills can crit, defence
  works the same way, magic meets only magic resistance and players take half. Every
  damaging skill's power was rescaled to keep its multiple of a plain swing. Continuous
  attacks no longer have a weak tier.
- **Party skills** reach every party member in the area; **clan skills** reach only the
  caster, because there are no clans yet.
- **Cancelling a cast.** In iROSE you can't call off your own cast once it starts. Here
  you have a 0.3 s grace window that costs nothing; after it the cast is locked in and your
  next click waits for it, as in iROSE.
- **After a skill.** iROSE skills whose action mode is "stop" (most buffs and heals) leave
  you standing still, and "restore" skills go back to what you were doing. Here every
  skill except the "attack" ones goes back to the attack you were in, so buffing or healing
  mid-fight doesn't stop your attacks.
- **Global cooldown.** rose-offline counts its 0.25 s from the start of the casting, so
  with our 0.3 s grace window it would always be over before the skill is paid for. Here
  it counts from when the skill takes effect.
- **Summons** stay until their owner dies, logs out or changes zone. Their lifetime
  status effect (summons slowly losing HP) is not applied.
- **Skill hits.** iROSE gives each damage type its own hit roll, with a weak hit between a
  miss and a full hit, and a miss still dealt the minimum 5 damage with its status effects.
  Here weapon skills share the plain attack hit chance (a miss does 0 damage and no
  effects), and magic and natural magic skills never miss. Spin Attack, Spiral Kick,
  Poison Knife and Magic Knife are weapon skills here (magic in iROSE), so they use the
  weapon formula and can miss.

> Open question: a skill's casting and action motion speeds (`casting_motion_speed` and `action_motion_speed` in the skill resources, iROSE's LIST_SKILL.STB columns 53 and 69) multiply the motion's length, so a speed of 200% makes the cast take twice as long instead of half. Check against iROSE.

> Open question: summons in iROSE lose HP over time (status effect 43, 5 HP per second) and so have a limited life. Our summons don't. Decide whether to add it.
