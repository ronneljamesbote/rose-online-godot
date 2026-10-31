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
min_damage: 1
player_raw_scale: 1.2
monster_raw_scale: 0.5
damage_spread: "90% to 110%"
damage_reduction: "DEF / (DEF + 0.75 x raw + 80), at most 75%"
max_damage_reduction: "75%"
level_gap: "2% per level, at most 10 levels"
pvp_damage_scale: 0.5
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
attack_motions:
  - { weapon: "No weapon", motion: "1.000 s", hit_frame: "0.400 s" }
  - { weapon: "One-handed sword", motion: "1.233 s", hit_frame: "0.566 s" }
  - { weapon: "Two-handed sword", motion: "1.233 s", hit_frame: "0.600 s" }
critical_chance: "5% + 45% x rating / (rating + 1.5 x level + 60), at most 50%"
critical_damage: "150% + 1.2% per point of SEN"
monster_critical: "5% chance, 150% damage"
critical_chances:
  - { level: 1, rating: 25, chance: "18.0%" }
  - { level: 10, rating: 50, chance: "23.0%" }
  - { level: 30, rating: 80, chance: "24.5%" }
  - { level: 60, rating: 100, chance: "23.0%" }
  - { level: 60, rating: 200, chance: "30.7%" }
  - { level: 100, rating: 150, chance: "23.8%" }
damage_examples:
  - { attack: 22, defence: "29 (level 4)", level: 1, damage: 19, critical: 31 }
  - { attack: 180, defence: 160, level: "60 vs 60", damage: 130, critical: "273 (SEN 50)" }
source:
  code:
    - module/src/lib.rs (combat_tick, attack, move_to, stop, sit, cancel_attack, attack_interval_us, attack_windup_us, deal_damage, kill, COMBAT_TICK_MS, RANGE_SLACK_CM, CHASE_REPATH_CM)
    - crates/rose-game-irose/src/data/ability_values.rs (calculate_damage, hit_chance, hit_chance_from, roll_hit, roll_damage, hit_damage, damage_reduction, critical_chance, critical_chance_of, critical_damage_percent, level_gap_multiplier, calculate_npc; the constants above them)
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
**critically**, and the damage is your attack, less a share that the target's defence (or
magic resistance for magic) takes off. Monsters fight the same way, with the same formulas.

Where attack, hit, defence and the other values come from is on [[rules/stats|Stats]].
Skills use the same damage steps with their power added, on [[rules/skills|Skills]]. Fighting other players is
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

**Worked example:** a new character's Short Sword (attack speed 111 with 15 DEX, see
[[rules/stats|Stats]]) uses the one-handed sword animation, 1.233 s long with its hit frame
at 0.566 s. A swing takes 1.233 × 100 / 111 = **1.111 s** and the damage lands
0.566 × 100 / 111 = **0.510 s** after it starts. With no weapon (speed 120):
1.000 × 100 / 120 = 0.833 s per swing, hit after 0.333 s.

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

A miss shows "Miss" over the target.

## Critical hits

Each swing that hits rolls for a critical hit. Your **critical rating** is your CON plus
your SEN (plus critical from items, passive skills and buffs, see [[rules/stats|Stats]]).
The chance is

```math
\text{chance} = 5\% + 45\% \times \frac{\text{rating}}{\text{rating} + 1.5 \times L_a + 60}
```

at most **50%**. With no rating it is 5%. The same rating counts for less as you level up,
so it has to keep growing: rating 100 is a 23% chance at level 60 and 19% at level 100.
Table `critical_chances` lists some chances.

A critical hit does **150%** of a normal hit, plus **1.2%** per point of SEN (SEN 50: 210%).

Monsters and summons have no rating: they crit a flat **5%** of the time, for 150%.

**Worked example:** a new character (level 1, CON 15, SEN 10) has rating 25:
5 + 45 × 25 / (25 + 1.5 + 60) = 5 + 13.0 = **18.0%**, and a critical does 150 + 12 = 162%.

## Damage

`ATK` is the attacker's attack, `DEF` the defender's defence (its magic resistance `RES`
for magic), `L_a` and `L_d` the levels. A hit that lands goes through these steps:

1. **Raw damage.** Players: ATK × **1.2**; monsters and summons: ATK × **0.5**. Then a
   random roll from **90% to 110%**.
2. **Defence.** Defence takes off a share of the raw damage:

   ```math
   \text{damage} = \text{raw} \times \left(1 - \frac{\text{DEF}}{\text{DEF} + 0.75 \times \text{raw} + 80}\right)
   ```

   at most **75%** is taken off. A bigger hit loses a smaller share, so defence matters
   most against weak hits and never makes a strong hit useless. Avoid does not lower damage;
   it only lowers the hit chance above.
3. **Magic.** Weapons marked as magic (staffs and wands) and monsters with magic attacks
   use the target's magic resistance instead of defence, and nothing else.
4. **Critical.** A critical hit multiplies it by the critical damage above.
5. **Level gap.** **+2%** for each level the attacker is above the defender, **−2%** for
   each level below, counting at most 10 levels (so 80% to 120%).
6. **Buffs.** Damage-up buffs add their percent (see [[rules/status-effects|Status effects]]).
7. **Hits.** Times the number of hits in the attack animation (1 for most weapons).
8. **Between players** the damage is halved, then kept to at most **25%** of the target's
   max HP (a critical at most **35%**). A summon counts as a monster here.
9. At least **1**, at most **2047**, rounded down.

Table `damage_examples` lists the examples below.

**Worked example:** the level 1 character from [[rules/stats|Stats]] (attack 22) hits a
[[monsters/2-jelly-bean|Jelly Bean]] (level 4, defence 29) with a middle roll (100%):
raw 22 × 1.2 = 26.4; defence takes off 29 / (29 + 19.8 + 80) = 22.5%, leaving 20.46;
3 levels below is −6%: 20.46 × 0.94 = 19.2, so **19** damage. A critical:
19.2 × 1.62 = **31**. A Jelly Bean has 4 × 30 = 120 HP, so it takes about six hits.

**Worked example:** a level 60 player with attack 180 hits a level 60 monster with
defence 160: raw 216, defence takes off 160 / (160 + 162 + 80) = 39.8%, so **130** damage.
With SEN 50 a critical does 130 × 2.1 = **273**.

## Monster attacks

Monsters use the same rules. Their values come from the monster data (see
[[monsters|Monsters]]):

- Max HP is the monster's level × its HP per level (`hp_per_level` on the monster page; the page's `hp` already shows the result).
- Raw damage is their attack × 0.5, and they crit a flat 5% for 150% (above).
- Range and attack speed are the numbers in the data, with nothing added.
- Each monster type has its own attack animation and hit frame, from `export/motions.toml`
  like the player's. Several monsters can share one animation (Mini-Jelly Bean and Jelly
  Bean do), so moving its hit marker moves all of their hits.

**Worked example:** a Jelly Bean (attack 4, level 4) hits the level 1 character (defence
30): raw 4 × 0.5 = 2; defence takes off 30 / (30 + 1.5 + 80) = 26.9%, leaving 1.46; it is 3
levels above: × 1.06 = 1.55, so **1** damage. A level 60 monster with attack 221 against a
level 60 player with defence 140: raw 110.5, defence takes off 140 / (140 + 82.9 + 80) =
46.2%, so **59**.

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
  rolled a success value and missed below 20 unless a lucky roll (rand(1..100) +
  0.6 × level difference ≥ 94) saved it, so each level the target was above you cost about
  2% and 10+ levels above often meant a sure miss, while targets below you could never dodge.
  Between players iROSE divided by the attacker's own avoid, so about 93% of plain attacks
  missed.
- Damage is our own formula (2026-10-31). iROSE multiplied attack by a success roll and
  divided by defence plus avoid, so defence counted twice and fights got much slower with
  level (a same-level monster took about 4 swings at level 1, 50 at level 60 and 130 past
  level 80). Ours takes a share off for defence (at most 75%), so a same-level monster takes
  about 6 to 23 swings at every level. Avoid only changes the hit chance, magic meets only
  magic resistance, hits roll 90% to 110%, and the level gap counts ±2% per level, at most
  10 levels.
- Critical hits: iROSE rolled 16 × (3 × rand(1..100) + level + 30) / (critical + 70) below
  20, and monsters had critical = level × 2.5, so monsters crit 40% of the time at level 30
  and always from level 120. Ours: the chance formula above for players (at most 50%), a
  flat 5% for monsters, and critical damage from SEN.
- Between players iROSE used separate formulas; ours is the same damage halved, with the
  same 25% and 35% caps.
- iROSE makes a hard hit interrupt the target (hit stun). Our server works out the same
  check but does not apply it yet, so being hit never interrupts you.
