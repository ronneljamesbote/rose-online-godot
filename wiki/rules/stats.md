---
kind: rule
id: stats
name: Stats
status: changed-from-irose
max_basic_stat: 300
stat_point_cost: "current value / 5, rounded down"
starting_level: 1
starting_stat_points: 0
starting_skill_points: 0
starting_stats:
  - { stat: "STR (Strength)", value: 15 }
  - { stat: "DEX (Dexterity)", value: 15 }
  - { stat: "INT (Intelligence)", value: 15 }
  - { stat: "CON (Concentration)", value: 15 }
  - { stat: "CHA (Charm)", value: 10 }
  - { stat: "SEN (Sense)", value: 10 }
stat_costs:
  - { "from value": "10-14", "points per +1": 2 }
  - { "from value": "15-19", "points per +1": 3 }
  - { "from value": "20-24", "points per +1": 4 }
  - { "from value": "50-54", "points per +1": 10 }
  - { "from value": "100-104", "points per +1": 20 }
  - { "from value": "295-299", "points per +1": 59 }
level_1_character:
  - { value: "Max HP", amount: 134 }
  - { value: "Max MP", amount: 75 }
  - { value: "Attack (Short Sword)", amount: 21 }
  - { value: "Hit", amount: 63 }
  - { value: "Defence", amount: 30 }
  - { value: "Magic resistance", amount: 27 }
  - { value: "Critical", amount: 17 }
  - { value: "Avoid", amount: 25 }
  - { value: "Attack speed", amount: 107 }
  - { value: "Attack range", amount: "2.7 m" }
  - { value: "Move speed", amount: "4.38 m/s" }
  - { value: "Max weight", amount: 1195 }
job_hp_mp:
  - { job: "Visitor (0)", hp_level_add: 12, hp_per_level: 8, mp_level_add: 4, mp_per_level: 3 }
  - { job: "Soldier (111)", hp_level_add: 7, hp_per_level: 12, mp_level_add: 3, mp_per_level: 4 }
  - { job: "Knight (121)", hp_level_add: -3, hp_per_level: 14, mp_level_add: 0, mp_per_level: 4.5 }
  - { job: "Champion (122)", hp_level_add: 2, hp_per_level: 13, mp_level_add: -6, mp_per_level: 5 }
  - { job: "Muse (211)", hp_level_add: 11, hp_per_level: 10, mp_level_add: 0, mp_per_level: 6 }
  - { job: "Mage (221)", hp_level_add: 11, hp_per_level: 10, mp_level_add: -7, mp_per_level: 7 }
  - { job: "Cleric (222)", hp_level_add: 5, hp_per_level: 11, mp_level_add: -4, mp_per_level: 6.5 }
  - { job: "Hawker (311)", hp_level_add: 10, hp_per_level: 11, mp_level_add: 4, mp_per_level: 4 }
  - { job: "Raider (321)", hp_level_add: 2, hp_per_level: 13, mp_level_add: 4, mp_per_level: 4 }
  - { job: "Scout (322)", hp_level_add: 11, hp_per_level: 11, mp_level_add: 0, mp_per_level: 4.5 }
  - { job: "Dealer (411)", hp_level_add: 12, hp_per_level: 10, mp_level_add: 3, mp_per_level: 4 }
  - { job: "Bourgeois (421)", hp_level_add: 13, hp_per_level: 10, mp_level_add: 3, mp_per_level: 4 }
  - { job: "Artisan (422)", hp_level_add: 6, hp_per_level: 11, mp_level_add: 0, mp_per_level: 4.5 }
second_job_bonus:
  jobs: "Knight, Champion, Mage, Cleric, Raider, Scout, Bourgeois, Artisan"
  max_hp: 300
  attack: 30
  defence: 25
  magic_resistance: 20
attack_formulas:
  - { weapon: "One-handed and two-handed melee", formula: "STR x 0.75 + level x 0.2 + weapon x (STR x 0.05 + 29) / 30" }
  - { weapon: "Bow, crossbow", formula: "DEX x 0.62 + STR x 0.2 + level x 0.2 + arrow + (weapon + arrow x 0.5 + 8) x (DEX x 0.04 + SEN x 0.03 + 29) / 30" }
  - { weapon: "Gun", formula: "DEX x 0.4 + CON x 0.5 + level x 0.2 + bullet + (weapon + bullet x 0.6 + 8) x (CON x 0.03 + SEN x 0.05 + 29) / 30" }
  - { weapon: "Launcher", formula: "STR x 0.52 + CON x 0.5 + level x 0.2 + shell + (weapon + shell + 12) x (CON x 0.04 + SEN x 0.05 + 29) / 30" }
  - { weapon: "Staff (magic melee)", formula: "STR x 0.4 + INT x 0.4 + level x 0.2 + weapon x (INT x 0.05 + 29) / 30" }
  - { weapon: "Wand (magic ranged)", formula: "INT x 0.6 + level x 0.2 + weapon x (SEN x 0.1 + 26) / 27" }
  - { weapon: "Dual swords", formula: "STR x 0.63 + DEX x 0.45 + level x 0.2 + weapon x (DEX x 0.05 + 25) / 26" }
  - { weapon: "Katar", formula: "STR x 0.42 + DEX x 0.55 + level x 0.2 + weapon x (DEX x 0.05 + 20) / 21" }
  - { weapon: "No weapon (or a broken one)", formula: "STR x 0.5 + DEX x 0.3 + level x 0.2" }
source:
  code:
    - crates/rose-game-irose/src/data/ability_values.rs (calculate, calculate_max_health, calculate_max_mana, calculate_attack_power, calculate_attack_speed, calculate_attack_range, calculate_hit, calculate_defence, calculate_resistance, calculate_critical, calculate_avoid, calculate_run_speed, calculate_max_weight, calculate_basic_stat_increase_cost, basic_stat_increase_cost)
    - crates/rose-game-common/src/components/ability_values.rs (AbilityValues getters, AbilityValuesAdjust)
    - module/src/character.rs (new_player, starter_kit, ability_values, player_stats, refresh_player, add_basic_stat)
    - crates/rose-game-data/src/lib.rs (CharacterCreator)
  data: INIT_AVATAR.STB rows 0-1 (starting stats and clothes), LIST_WEAPON.STB (attack, attack speed, range, quality), LIST_FOOT.STB row 0 (barefoot speed 65), LIST_BODY.STB and the other equipment tables (defence, resistance, durability)
---
# Stats

Every character has six **basic stats**: Strength, Dexterity, Intelligence, Concentration,
Charm and Sense. You raise them with **stat points**, which you get every time you level up
(see [[rules/experience|Experience]]). From the basic stats, your level, your job and your
equipment the game works out your **derived values**: HP, MP, attack, defence, hit, avoid
and the rest. The character window (C) shows both.

What the derived values do in a fight is on [[rules/combat|Combat]]. How HP and MP come
back is on [[rules/recovery|Recovery]].

## The six basic stats

| Stat | What it raises |
| --- | --- |
| STR (Strength) | Attack with melee weapons, max HP, defence, max weight |
| DEX (Dexterity) | Attack with bows, guns, dual swords and katars, avoid, move speed |
| INT (Intelligence) | Attack with staffs and wands, max MP, magic resistance, magic skill damage |
| CON (Concentration) | Hit, critical, gun and launcher attack, HP and MP recovery |
| CHA (Charm) | Not used by any derived value; it raises quest rewards and drops (see [[rules/drops\|Drops]]) |
| SEN (Sense) | Critical, bow, gun, launcher and wand attack, skill damage |

A basic stat can go up to **300**. Items, gems and passive skills can add to a stat on top
of what you bought; those bonuses count everywhere a stat is used below.

## New characters

A new character is a level 1 **Visitor** (job 0) on Birth Island with these stats, the same
for men and women:

| STR | DEX | INT | CON | CHA | SEN |
| --- | --- | --- | --- | --- | --- |
| 15 | 15 | 15 | 15 | 10 | 10 |

They start with 0 stat points and 0 skill points, wearing
[[items/body/30-visitor-look|Visitor Look]] and holding a
[[items/weapon/2-short-sword|Short Sword]], with a [[items/weapon/202-short-bow|Short Bow]]
and 999 arrows in the bag.

## Raising a stat

Each +1 costs stat points equal to the stat's current value divided by 5, rounded down:

```math
\text{cost} = \left\lfloor \text{current value} \times 0.2 \right\rfloor
```

**Worked example:** STR 15 costs 15 × 0.2 = 3 points to raise to 16. STR 20 costs 4. Only
the stat you bought counts, not bonuses from items. A stat at 300 can't be raised.

## Derived values

All values below are worked out again whenever your level, stats, equipment, skills or
status effects change. Each one is rounded down to a whole number at the end.

"Bonus" means stat bonuses on equipment and gems. "Passive" means passive skills, which
add either a fixed amount or a percent of the value before them:

```math
\text{final} = \text{base} + \text{passive amount} + \text{base} \times \frac{\text{passive percent}}{100}
```

Buffs and debuffs (see [[rules/status-effects|Status effects]]) are added after that.

### Max HP

```math
\text{max HP} = (\text{level} + a) \times b + \text{STR} \times 2 + \text{bonus}
```

`a` and `b` depend on the job (table `job_hp_mp` above, columns `hp_level_add` and
`hp_per_level`). Second jobs get +300 on top (see below).

**Worked example:** a level 1 Visitor with 15 STR: (1 + 12) × 8 + 15 × 2 = 104 + 30 = **134**.

### Max MP

```math
\text{max MP} = \lfloor (\text{level} + c) \times d \rfloor + \text{INT} \times 4 + \text{bonus}
```

`c` and `d` are `mp_level_add` and `mp_per_level` in the job table.

**Worked example:** level 1 Visitor with 15 INT: (1 + 4) × 3 + 15 × 4 = 15 + 60 = **75**.

### Attack

The formula depends on the weapon in your hand (table `attack_formulas`). "weapon" is the
weapon's attack plus its grade bonus; "arrow", "bullet" and "shell" are the quality of the
ammunition you have equipped. A broken weapon counts as no weapon. Item attack bonuses are
added at the end.

For one-handed and two-handed melee weapons:

```math
\text{attack} = \text{STR} \times 0.75 + \text{level} \times 0.2 + \text{weapon attack} \times \frac{\text{STR} \times 0.05 + 29}{30}
```

**Worked example:** level 1, 15 STR, Short Sword (attack 10):
15 × 0.75 + 1 × 0.2 + 10 × (0.75 + 29) / 30 = 11.25 + 0.2 + 9.92 = 21.37, shown as **21**.
With a weapon of attack 12 it would be 11.25 + 0.2 + 11.9 = 23.35, shown as 23.

With no weapon: 15 × 0.5 + 15 × 0.3 + 0.2 = 12.2, shown as 12.

Whether your attacks do physical or magic damage depends on the weapon too: weapons marked
as magic in the item data (staffs and wands) do magic damage, everything else physical.

### Hit

With a weapon:

```math
\text{hit} = (\text{CON} + 10) \times 0.8 + \text{weapon quality} \times 0.6 + \text{grade hit} + \text{weapon durability} \times 0.8 + \text{bonus}
```

With no weapon (or a broken one): (CON + 10) × 0.5 + 15 + bonus.

**Worked example:** 15 CON, Short Sword (quality 26, durability 35, grade 0):
25 × 0.8 + 26 × 0.6 + 35 × 0.8 = 20 + 15.6 + 28 = 63.6, shown as **63**. Without the
sword it would be 25 × 0.5 + 15 = 27.

### Defence

```math
\text{defence} = \text{item defence} + (\text{STR} + 5) \times 0.35 + (\text{level} + 15) \times 0.7 + \text{bonus}
```

"Item defence" adds up the defence of every worn item (plus its grade bonus) that isn't
broken. With a shield in the off hand, shield passives add their amount or percent on top.

**Worked example:** Visitor Look (defence 12), 15 STR, level 1:
12 + 20 × 0.35 + 16 × 0.7 = 12 + 7 + 11.2 = 30.2, shown as **30**.

### Magic resistance

```math
\text{resistance} = \text{item resistance} + (\text{INT} + 5) \times 0.6 + (\text{level} + 15) \times 0.8 + \text{bonus}
```

**Worked example:** Visitor Look (resistance 3), 15 INT, level 1:
3 + 20 × 0.6 + 16 × 0.8 = 3 + 12 + 12.8 = 27.8, shown as **27**.

### Critical

```math
\text{critical} = \text{SEN} + (\text{CON} + 20) \times 0.2 + \text{bonus}
```

**Worked example:** 10 SEN, 15 CON: 10 + 35 × 0.2 = **17**.

### Avoid

```math
\text{avoid} = (\text{DEX} \times 1.9 + \text{level} \times 0.3 + 10) \times 0.4 + \text{durability} \times 0.3 + \text{grades} + \text{bonus}
```

"durability" adds up the durability of your head, body, back, hands, feet and off-hand
items (not broken ones). "grades" adds up the grade of every worn item that has defence.

**Worked example:** 15 DEX, level 1, Visitor Look (durability 34), nothing else worn:
(28.5 + 0.3 + 10) × 0.4 + 34 × 0.3 = 15.52 + 10.2 = 25.72, shown as **25**.

### Attack speed

```math
\text{attack speed} = \frac{1500}{\text{weapon speed} + 5} + \text{bonus}
```

The weapon speed is the item's speed number (lower is faster). With no weapon the game
uses speed 8. Passive skills for bows, guns and launchers, and for katars and dual swords,
add to it. How attack speed turns into time between hits is on [[rules/combat|Combat]].

**Worked example:** Short Sword (speed 9): 1500 / 14 = 107.1, shown as **107**. No weapon:
1500 / 13 = 115.

### Attack range

```math
\text{range} = \text{weapon range} + 120 \text{ cm}
```

With no weapon the weapon range is 100 cm. **Worked example:** Short Sword (150 cm):
150 + 120 = 270 cm = **2.7 m**.

### Move speed

Characters always run. Their speed is:

```math
\text{speed} = (20 + \text{shoe speed} + \text{back item speed}) \times \frac{\text{DEX} + 500}{100} + \text{bonus}
```

in centimetres per second. Without shoes the shoe speed is 65 (the barefoot row of the shoe
table). A broken shoe or back item counts as not worn.

**Worked example:** no shoes, 15 DEX: (20 + 65) × 515 / 100 = 437.75 cm/s, about
**4.38 m/s**. Driving a cart uses the cart's speed instead (see [[rules/carts|Carts]]).

### Max weight

```math
\text{max weight} = 1100 + \text{level} \times 5 + \text{STR} \times 6 + \text{bonus}
```

Weight passives only count while you wear a bag on your back.

**Worked example:** level 1, 15 STR: 1100 + 5 + 90 = **1195**.

> Open question: max weight is worked out but nothing in the game checks it yet, so
> carrying more than it does not slow you down or stop pickups.

## Second job bonus

Characters in a second job (Knight, Champion, Mage, Cleric, Raider, Scout, Bourgeois,
Artisan) get +300 max HP, +30 attack, +25 defence and +20 magic resistance, added after
everything else.

## Changed from iROSE

- New characters get a [[items/weapon/2-short-sword|Short Sword]] in hand and a
  [[items/weapon/202-short-bow|Short Bow]] with 999 arrows in the bag. In iROSE the
  starting data gives a [[items/weapon/1-wooden-sword|Wooden Sword]]; ours replaces it.
- Max weight has no effect yet (see the open question above).
