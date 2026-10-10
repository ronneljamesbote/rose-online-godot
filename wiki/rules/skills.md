---
kind: rule
id: skills
name: Skills
status: changed-from-irose
skill_pages: 4 (basic, active, passive, clan)
skill_slots_per_page: 30
skill_points_per_level: "floor((new level + 2) / 2)"
global_cooldown_s: 0.25
cooldown_unit_s: 0.2 (LIST_SKILL.STB stores cooldowns in steps of 0.2 s)
minimum_cast_time_s: 0.3
range_slack_m: 0.5
default_cast_range: the caster's attack range (when the skill has no range)
skill_damage_minimum: 5
skill_damage_maximum: 2047
pvp_skill_damage_cap: 45% of the target's max HP
base_summon_points: 50
summon_spawn_distance_m: 0.5 to 1.5
summon_follow_distance_m: 5.5
summon_follow_to_m: 3.5
summon_stops_fighting_m: 22
resurrection_hp: 30% of max HP
skill_damage_types:
  - { type: weapon attack, hit_check: "(L_a + 20 - L_d + 1..60) x (Hit - 0.6 Avoid + 1..70 + 10) / 110", miss_below: 10, weak_below: 20 }
  - { type: magic attack, hit_check: "(L_a + 30 - L_d + 1..50) x (Hit - 0.56 Avoid + 1..70 + 10) / 110", miss_below: 8, weak_below: 20 }
  - { type: natural magic, hit_check: "(L_a + 10 - L_d + 1..80) x (Hit - 0.5 Avoid + 1..50 + 50) / 90", miss_below: 6, weak_below: 20 }
  - { type: continuous attack, hit_check: "(L_a + 8 - L_d + 1..80) x (Hit - 0.6 Avoid + 1..50 + 50) / 90", miss_below: 10, weak_below: 20 }
cost_types:
  - { cost: MP, rule: "lowered by the caster's Save Mana %, rounded down" }
  - { cost: HP, rule: "can't take you below 1 HP" }
  - { cost: Stamina, rule: "see Stamina" }
  - { cost: Experience, rule: "taken from your current experience" }
  - { cost: Zuly, rule: "taken from your money" }
  - { cost: Fuel, rule: "taken from your cart or castle gear" }
source:
  code:
    - module/src/skills.rs (learn_skill, level_up_skill, check_requirements, cast_skill, start_cast, advance_cast, check_can_use, pay_costs, use_cost, motion_us, cast_tick, take_effect, targets, target_allowed, damage, weapon_wear, apply_effects, target_value, summon, summon_capacity, summon_tick, resurrect, npc_cast, use_scroll, consume_scroll)
    - module/src/items.rs (use_item, skill books and scrolls)
    - module/src/character.rs (reward_xp, skill points on level up)
    - module/src/lib.rs (deal_damage, combat_tick)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_skill_damage, calculate_skill_adjust_value, calculate_npc, calculate_levelup_reward_skill_points)
    - crates/rose-data-irose/src/skill_database.rs (get_cooldown, load_skill)
  data: LIST_SKILL.STB, LIST_STATUS.STB, LIST_CLASS.STB (job classes), LIST_NPC.STB (summons)
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
  2. the right job: the skill page's `job` names a job class from LIST_CLASS.STB, and your
     job must be one of the jobs in it ("any" means every job),
  3. the other skills in the `needs` column at the levels shown,
  4. the stats in `needs_level` and `needs` (for example level 25, or a minimum STR).
- Learning a skill takes its `learn_points` from your skill points. You can't learn a
  skill you already know.
- Skills go into one of four pages (basic, active, passive, clan) of **30** slots each, as
  the skill's data says. When the page is full you can't learn more skills on it.

## Levelling skills

Each level of a skill is its own row in LIST_SKILL.STB (the `id` column of the levels
table). To raise a skill, click its level-up button in the skill window: the next level
must be the next row with the same base skill, and you must meet that level's
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
3. **Casting.** In range, the checks run again, then the costs are paid and the cooldowns
   start. Your character plays the skill's casting motion. The skill **takes effect at the
   end of the casting motion**, then the action motion plays. A cast always lasts at least
   **0.3 s** in total.
4. **Interrupting.** Moving, attacking or stopping before the skill takes effect cancels
   it. If the casting had already started, the costs and cooldown are spent anyway. A stun
   or sleep landing on you cancels the cast too. Dying cancels it.
5. **After.** Melee skills (those whose action mode is "attack") go back to normal attacks
   on the same target when they finish.

The checks, every time:

- you are alive, not stunned or asleep, and not riding as a passenger,
- you are not silenced ([[rules/status-effects|Status effects]]),
- the **global cooldown** (**0.25 s** after any skill) and the skill's own cooldown are
  over,
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

The `cooldown` column is in seconds (the data stores it in steps of 0.2 s). It starts when
the casting starts, not when it takes effect. Every skill has its own cooldown; the data can
also put several skills in a **cooldown group** that shares one cooldown, but no skill in the
current iROSE data uses a group. On top of that, any skill starts the 0.25 s global
cooldown.

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
decides how well the skill lands, then the damage formula for the skill's `damage_type`
runs. $L_a$ and $L_d$ are the attacker's and defender's levels, $P$ is the skill's power,
$A$ the attacker's attack power, $I$ its INT, $S$ its Sense, $C$ its critical, and $D$, $R$,
$V$ the defender's defence, magic resistance and avoid. $r$ is a random whole number from 1
to 30, and "1..n" is a random whole number from 1 to n.

**Hit check.** Each damage type has its own (see `skill_damage_types` above). Below the
"miss" value the skill misses; between "miss" and 20 it is a weak hit; 20 or more is a
full hit. For a magic attack:

```math
\text{hit} = \frac{(L_a + 30 - L_d + \text{1..50}) \times (\text{Hit}_a - 0.56\,V + \text{1..70} + 10)}{110}
```

**Weapon attack.**

```math
\text{weak} = \frac{0.4\,P\,(A + 50)(r + 1.2\,S + 340)}{(D + R + 20)(250 + L_d - L_a)} + 20
```

```math
\text{full} = \frac{(P + 0.2\,A)(A + 60)(r + 0.7\,S + 370) \times 0.01 \times (120 - L_d + L_a)}{(D + 0.8\,R + 0.4\,V + 20) \times 270} + 20
```

**Magic attack.**

```math
\text{weak} = \frac{0.2\,P\,(0.8\,A + I + 80)(r + 1.3\,S + 280)}{(0.3\,D + R + 30)(250 + L_d - L_a)} + 20
```

```math
\text{full} = \frac{P\,(0.8\,A + 1.2\,I + 100)(r + 0.7\,S + 350) \times 0.01 \times (150 - L_d + L_a)}{(0.3\,D + R + 0.3\,V + 60) \times 350} + 20
```

**Natural magic.**

```math
\text{weak} = \frac{0.2\,P\,(P + I + 80)(r + 2\,S + 290)}{(0.2\,D + R + 30)(250 + L_d - L_a)} + 20
```

```math
\text{full} = \frac{(P + 35)(P + I + 140)(r + S + 380) \times 0.01 \times (150 - L_d + L_a)}{(0.35\,D + 1.2\,R + 0.4\,V + 10) \times 730} + 20
```

**Continuous attack.**

```math
\text{weak} = \frac{0.4\,(P + 40)(A + 40)(r + 0.2\,C + 40)}{(D + 0.3\,R + 0.4\,V + 10) \times 80} + 5
```

```math
\text{full} = \frac{(P + 0.15\,C + 40)\,A\,(r + 0.32\,C + 35) \times 0.01 \times (120 - L_d + L_a)}{(D + 0.3\,R + 0.4\,V + 10) \times 100} + 20
```

**Player against player.** When both sides are player characters, the full hit uses
different numbers:

| Type | Full hit between players |
| --- | --- |
| weapon attack | $(P + 0.2A)(A + 60)(r + 0.7S + 370) \times 0.01 \times (320 - L_d + L_a) / (D + 0.8R + 0.4V + 40) / 1600 + 60$ |
| magic attack | $(P + 50)(0.8A + 1.2I + 100)(r + 0.7S + 350) \times 0.01 \times (380 - L_d + L_a) / (0.4D + R + 0.3V + 60) / 2500 + 60$ |
| natural magic | $(P + 35)(P + I + 140)(r + S + 380) \times 0.01 \times (400 - L_d + L_a) / (0.5D + 1.2R + 0.4V + 20) / 3400 + 40$ |
| continuous attack | $(P + 0.15C + 40)\,A\,(r + 0.32C + 35) \times 0.01 \times (350 - L_d + L_a) / (D + 0.3R + 0.4V + 35) / 400 + 20$ |

**Then, for every type:**

1. An **Additional Damage** buff adds its percentage
   ([[skills/1211-damage-support|Damage Support]]).
2. The damage is at least **5** (a miss counts as 0 here, so it still deals 5).
3. Between two players it is at most **45%** of the defender's max HP.
4. It is at most **2047**.
5. It is rounded down to a whole number. Skill hits are never critical.

A damaging skill wears your weapon (see [[rules/durability|Durability]]), except natural
magic. Damage from skills counts like any other damage for experience and drops (see
[[rules/combat|Combat]]), wakes a sleeping target and makes a monster react
([[rules/monster-behaviour|Monster behaviour]]).

### Worked example

A level 15 Muse with attack power 50, INT 45, Sense 20 and Hit 60 casts
[[skills/981-ice-bolt|Ice Bolt]] level 1 (magic attack, power 60) at a
[[monsters/62-honeybee|HoneyBee]] (level 12, defence 45, magic resistance 24, avoid 29).
Say the random numbers come out as 25 (of 1..50), 35 (of 1..70) and $r = 15$.

Hit check: $(15 + 30 - 12 + 25) \times (60 - 0.56 \times 29 + 35 + 10) / 110 = 58 \times 88.76 / 110 = 46.8$.
That is 20 or more, so it is a full hit against a monster:

```math
\frac{60 \times (0.8 \times 50 + 1.2 \times 45 + 100) \times (15 + 0.7 \times 20 + 350) \times 0.01 \times (150 - 12 + 15)}{(0.3 \times 45 + 24 + 0.3 \times 29 + 60) \times 350} + 20
```

$= 60 \times 194 \times 379 \times 0.01 \times 153 / (106.2 \times 350) + 20 = 181.59 + 20 = 201.59$,
which is shown as **201**. With $r$ from 1 to 30 the full hit ranges from 194 to 208.
The HoneyBee has $12 \times 25 = 300$ HP, so two Ice Bolts kill it.

## Buffs, debuffs and heals

A skill can carry up to two status effects (the `effects` column), each with a `duration`
and a `success` chance. How they land, stack and end is on
[[rules/status-effects|Status effects]]. Damaging skills try their effects on every target
they hit.

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
  summon uses the points in its monster's data (LIST_NPC.STB); the ButterFly uses 30. A
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
| level, id | the level, and its row in LIST_SKILL.STB |
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

- **Party skills** reach every party member in the area; **clan skills** reach only the
  caster, because there are no clans yet.
- **Summons** stay until their owner dies, logs out or changes zone. Their lifetime
  status effect (summons slowly losing HP) is not applied.

> Open question: a skill's casting and action motion speeds (LIST_SKILL.STB columns 53 and 69) multiply the motion's length, so a speed of 200% makes the cast take twice as long instead of half. Check against iROSE.

> Open question: a missed skill hit still deals the minimum 5 damage, and damaging skills try their status effects on a target even when the hit missed. iROSE may deal no damage and give no effects on a miss.

> Open question: summons in iROSE lose HP over time (status effect 43, 5 HP per second) and so have a limited life. Our summons don't. Decide whether to add it.
