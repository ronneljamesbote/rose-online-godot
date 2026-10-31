---
kind: rule
id: combat
name: Combat
status: changed-from-irose
tick_ms: 100
range_slack_m: 0.5
chase_repath_m: 1
min_attack_speed: 30
max_damage: 2047
min_damage_normal: 5
min_damage_critical: 10
pvp_damage_cap_normal: "25% of the target's max HP"
pvp_damage_cap_critical: "35% of the target's max HP"
hit_chance: "100 x hit / (hit + avoid x 0.3) + 1.5 x level difference"
hit_avoid_weight: 0.3
hit_percent_per_level: 1.5
hit_level_difference_cap: 10
hit_chance_min: "5%"
hit_chance_max: "95%"
hit_chances:
  - { attacker: "level 30, hit 129", defender: "level 30, avoid 60", chance: "87%" }
  - { attacker: "level 30, hit 129", defender: "level 40, avoid 74", chance: "70%" }
  - { attacker: "level 30, hit 129", defender: "level 50, avoid 87", chance: "68%" }
  - { attacker: "level 30, hit 129", defender: "level 20, avoid 47", chance: "95%" }
  - { attacker: "level 1, hit 63", defender: "level 4, avoid 7", chance: "92%" }
critical_roll: 20
attack_motions:
  - { weapon: "No weapon", motion: "1.000 s", hit_frame: "0.400 s" }
  - { weapon: "One-handed sword", motion: "1.233 s", hit_frame: "0.566 s" }
  - { weapon: "Two-handed sword", motion: "1.233 s", hit_frame: "0.600 s" }
critical_chances:
  - { level: 1, critical: 17, chance: "25%" }
  - { level: 1, critical: 50, chance: "39%" }
  - { level: 10, critical: 30, chance: "28%" }
  - { level: 30, critical: 60, chance: "34%" }
  - { level: 50, critical: 100, chance: "44%" }
  - { level: 100, critical: 150, chance: "48%" }
source:
  code:
    - module/src/lib.rs (combat_tick, attack, move_to, stop, sit, cancel_attack, attack_interval_us, attack_windup_us, deal_damage, kill, COMBAT_TICK_MS, RANGE_SLACK_CM, CHASE_REPATH_CM)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_damage, hit_chance, hit_chance_from, roll_hit, calculate_damage_success_rate, calculate_attack_damage_physical, calculate_attack_damage_magic, calculate_npc)
    - crates/rose-game-common/src/components/ability_values.rs (AbilityValues getters)
    - module/src/character.rs (player_stats)
    - module/src/world.rs (npc_stats_for)
    - module/src/monster_brain.rs (on_damaged, on_attack)
    - crates/rose-game-data/src/lib.rs (load_motions)
    - godot/scripts/main.gd (_unhandled_input)
    - godot/scripts/online.gd (_on_damage, _float_number)
  data: the weapon resources in godot/data/items/weapon/ (attack_power, attack_speed, attack_range, motion_type), TYPE_MOTION.STB and FILE_MOTION.STB (attack animations), export/motions.toml (player, monster and NPC animation lengths and hit frames, exported from the Godot animations), the monster resources in godot/data/monsters/ (monster attack values)
---
# Combat

Click a monster to attack it. If it is too far away your character runs to it, stops when
it is in reach and swings again and again until it dies, you click somewhere else or you
press S (stop). Space attacks the nearest monster. Each swing can **miss**, **hit** or hit
**critically**, and the damage depends on your attack against the target's defence (or
magic resistance for magic). Monsters fight the same way, with the same formulas.

Where attack, hit, defence and the other values come from is on [[rules/stats|Stats]].
Skills use their own damage formulas, on [[rules/skills|Skills]]. Fighting other players is
on [[rules/pvp|PvP]], and what happens when you fall is on [[rules/death|Death]].

## How a fight runs

The server checks every fight every **100 ms** (the combat tick). On each tick, for every
character and monster that has a target:

1. If the target is dead, gone, or (between players) no longer an enemy, the attack stops.
2. If the target is farther than your **attack range + 0.5 m**, you run after it. The chase
   path is updated when the target has moved more than 1 m from where you were heading.
3. In range, you stop and start a swing as soon as your last one has finished.
4. The swing's damage lands at its **hit frame**, part way through the attack animation,
   even if the target has moved out of range by then.

You can't swing while stunned or asleep, or while casting a skill. Attacking stands you up
if you were sitting, and cancels a skill you were casting while it is still in its 0.3 s
grace window; after that the attack starts once the skill takes effect (see
[[rules/skills|Skills]]). Most skills go back to your attack when they end. You can't start a fight with a
character that is hidden.

### Attack range

Your attack range is your weapon's range + 1.2 m (see [[rules/stats|Stats]]); a Short Sword
reaches 2.7 m. A monster's range is the range in its data, with nothing added. Both get
0.5 m of slack, so a Short Sword swings at targets up to 3.2 m away.

### Time between hits

Every weapon type has an attack animation with a hit frame in it. Attack speed shortens or
lengthens both:

```math
\text{time between swings} = \text{animation length} \times \frac{100}{\max(\text{attack speed}, 30)}
```

```math
\text{time to the hit} = \text{hit frame time} \times \frac{100}{\max(\text{attack speed}, 30)}
```

**Worked example:** a Short Sword (attack speed 107) uses the one-handed sword animation,
1.233 s long with its hit frame at 0.566 s. A swing takes 1.233 × 100 / 107 = **1.152 s**
and the damage lands 0.566 × 100 / 107 = **0.529 s** after it starts. With no weapon
(speed 115): 1.000 × 100 / 115 = 0.870 s per swing, hit after 0.348 s.

The player, monster and NPC animations' lengths and hit frames come from `export/motions.toml`, which is
exported from the hit markers of the Godot animations (`godot/assets/README.md`), so the
server and the animation you see always agree. The file holds the same frames as the iROSE
motion files, and the server reads only it, never the iROSE motion files. A motion missing
from it has no animation, as when its iROSE file is missing (a weapon with no animation data,
below). Which animation each weapon, monster and action plays still comes from the iROSE lists
(TYPE_MOTION.STB, FILE_MOTION.STB, LIST_NPC.CHR).

The next swing is timed from when the last one was due, not from when the tick noticed it,
so the 100 ms tick doesn't make you slower. A hit can land up to 100 ms after its hit frame.

Animations are at least 0.3 s long. A weapon with no animation data uses 1 s, with the hit
half way.

### Animation cancelling

- **Before the hit frame**, moving, pressing stop, sitting down or switching targets cancels
  the swing: no damage, and the time is given back, so you can swing again right away.
- **After the hit frame**, you can move at once; the rest of the animation is skipped. Your
  next swing still waits until the full swing time has passed.
- A swing that has started always lands, even if the target walks out of range during the
  wind-up; only you moving, stopping, sitting or switching targets cancels it. Then you
  chase the target and swing again once you are in range. (Cancelling it when the target
  stepped away made a chaser restart its wind-up over and over against a moving target.)

### Ammunition and weapon wear

Bows, guns and launchers need one piece of ammunition per hit of the swing. Without enough
the attack stops with "Out of ammo". The ammunition is used up when the hit lands. Every
swing that lands can wear down your weapon, and every hit you take can wear down your armour (see
[[rules/durability|Durability]]). Attacking from a cart uses fuel (see
[[rules/carts|Carts]]).

## Hit or miss

Each swing rolls once against a **hit chance**, the same for players, monsters and fights
between players:

```math
\text{chance} = \frac{100 \times \text{hit}_a}{\text{hit}_a + \text{avoid}_d \times 0.3} + 1.5 \times \text{clamp}(L_a - L_d, -10, 10)
```

`L_a` and `L_d` are the attacker's and defender's levels. The chance is then kept between
**5%** and **95%**, so every swing can hit and every swing can miss. The swing hits when
rand(0..100) is below the chance.

- Hit against avoid decides most of it: doubling the target's avoid takes a same-level
  chance from about 88% to about 78%.
- Each level the target is above you takes 1.5% off, each level below adds 1.5%, counting
  at most 10 levels either way (±15%).

Table `hit_chances` lists some chances. A typical level 30 character (hit 129) hits a level
30 monster (avoid 60) **87%** of the time, a level 40 one 70%, a level 50 one 68%, and a
level 20 one 95%.

**Worked example:** a level 1 character (hit 63) attacks a
[[monsters/2-jelly-bean|Jelly Bean]] (level 4, avoid 7):
100 × 63 / (63 + 2.1) = 96.8, minus 1.5 × 3 = 4.5, so **92%** of swings hit.

A hit then rolls a **success** value `S` that only scales the damage below (a ±10% effect).
Against a monster, or for a monster attacking you:

```math
v = (L_a + 10) - \lfloor L_d \times 1.1 \rfloor + \text{rand}(1..50)
```

```math
S = \left\lfloor v \times \frac{\text{hit}_a \times 1.1 - \text{avoid}_d \times 0.93 + \text{rand}(1..60) + 5 + L_a \times 0.2}{80} \right\rfloor
```

(`S` is 0 if `v` is 0 or less.) Between two players:

```math
S = 40 - 60 \times \left\lfloor \frac{\text{hit}_a + \text{avoid}_d}{\max(\text{avoid}_a, 1)} \right\rfloor + \text{rand}(1..100)
```

On a hit `S` is raised to at least **20**.

A miss shows "Miss" over the target.

## Critical hits

Each swing that doesn't miss rolls for a critical hit:

```math
c = \left\lfloor \frac{16 \times (3 \times \text{rand}(1..100) + L_a + 30)}{\text{critical}_a + 70} \right\rfloor
```

It is critical when `c` is **below 20**. Higher critical makes it more likely; higher level
makes it slightly less likely. Table `critical_chances` lists some exact chances.

**Worked example:** level 1, critical 17: 16 × (3r + 31) / 87 is below 20 for r up to 25,
so the chance is **25%**. A Jelly Bean (level 4, critical 4 × 2.5 = 10) has 21%.

## Damage

`ATK` is the attacker's attack, `S` the success value, `DEF` the defender's defence, `RES`
its magic resistance and `AVO` its avoid.

### Physical damage

Normal hit:

```math
\text{damage} = \text{ATK} \times (S \times 0.03 + 26) \times \frac{\text{ATK} - \text{DEF} + 250}{145 \times (\text{DEF} + \text{AVO} \times 0.4 + 5)}
```

Critical hit:

```math
\text{damage} = \text{ATK} \times (S \times 0.05 + 29) \times \frac{\text{ATK} - \text{DEF} + 230}{100 \times (\text{DEF} + \text{AVO} \times 0.3 + 5)}
```

### Magic damage

Weapons marked as magic (staffs and wands) and monsters with magic attacks use magic
resistance instead of defence. Normal hit:

```math
\text{damage} = \text{ATK} \times (S \times 0.03 + 30) \times \frac{\text{ATK} - \text{DEF} \times 0.8 + 280}{280 \times (\text{RES} + \text{AVO} \times 0.3 + 5)}
```

Critical hit:

```math
\text{damage} = \text{ATK} \times (S \times 0.05 + 33) \times \frac{\text{ATK} - \text{DEF} \times 0.8 + 310}{200 \times (\text{RES} + \text{AVO} \times 0.3 + 5)}
```

### Between players

When a player hits another player, other numbers are used (a summon counts as a monster
here):

| Hit | Formula |
| --- | --- |
| Physical | ATK × (S × 0.05 + 25) × (ATK - DEF + 400) / (420 × (DEF + AVO × 0.4 + 5)) + 20 |
| Physical critical | ATK × (S × 0.05 + 35) × (ATK - DEF + 430) / (300 × (DEF + AVO × 0.4 + 10)) + 25 |
| Magic | ATK × (S × 0.06 + 29) × (ATK - DEF × 0.8 + 350) / (640 × (RES + AVO × 0.3 + 5)) + 20 |
| Magic critical | ATK × (S × 0.05 + 33) × (ATK - DEF + 340) / (360 × (RES + AVO × 0.3 + 20)) + 25 |

A hit between players does at most 25% of the target's max HP, a critical at most 35%.

### Final steps

1. Damage-up buffs add their percent (see [[rules/status-effects|Status effects]]).
2. The damage is multiplied by the number of hits in the attack animation (1 for most
   weapons), then raised to at least **5** (normal) or **10** (critical).
3. The player against player cap above, then a cap of **2047** for everyone.
4. Rounded down.

**Worked example:** the level 1 character above (attack 21) hits a Jelly Bean (defence 29,
avoid 7) with success 40:
21 × (40 × 0.03 + 26) × (21 - 29 + 250) / (145 × (29 + 2.8 + 5)) = 21 × 27.2 × 242 / 5336
= 25.9, so **25** damage. A critical with the same roll:
21 × 31 × 222 / (100 × 36.1) = 40.0, so **40**. A Jelly Bean has 4 × 30 = 120 HP, so it
takes about five hits.

## Monster attacks

Monsters use the same rules. Their values come from the monster data (see
[[monsters|Monsters]]):

- Max HP is the monster's level × its HP per level (`hp_per_level` on the monster page; the page's `hp` already shows the result).
- Critical is level × 2.5, rounded down.
- Range and attack speed are the numbers in the data, with nothing added.
- Each monster type has its own attack animation and hit frame, from `export/motions.toml`
  like the player's. Several monsters can share one animation (Mini-Jelly Bean and Jelly
  Bean do), so moving its hit marker moves all of their hits.

**Worked example:** a Jelly Bean (attack 4, level 4) hits the level 1 character (defence 30,
avoid 25): 4 × 27.2 × 224 / (145 × 45) = 3.7, raised to the minimum of **5**. Its
critical hits do 10.

A monster that is hit fights back, or runs its own reaction (calling friends, using a
skill). A monster dragged more than 30 m from where its fight began (and no longer
trading blows), or 60 m from its home, gives up, heals fully and walks back. More on [[rules/monster-behaviour|Monster behaviour]].

## Damage numbers

Every hit shows a number rising over the one that was hit:

- white for your hits and other fights,
- yellow and larger for a critical hit,
- red when you are the one hit,
- "Miss" when the swing did no damage.

Killing a monster shows "+N XP" over you (see [[rules/experience|Experience]]).

## Changed from iROSE

- The server resolves fights on a 100 ms tick, with the timing rules above. Cancelling a
  swing before its hit frame gives the time back.
- Hit or miss is our own formula (hit against avoid, ±1.5% per level, 5% to 95%). iROSE
  rolled the success value above and missed below 20 unless a lucky roll (rand(1..100) +
  0.6 × level difference ≥ 94) saved it, so each level the target was above you cost about
  2% and 10+ levels above often meant a sure miss, while targets below you could never dodge.
  Between players iROSE divided by the attacker's own avoid, so about 93% of plain attacks
  missed. In our game the success value only scales damage.
- iROSE makes a hard hit interrupt the target (hit stun). Our server works out the same
  check but does not apply it yet, so being hit never interrupts you.
