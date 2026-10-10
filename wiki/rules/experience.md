---
kind: rule
id: experience
name: Experience
status: changed-from-irose
starting_level: 1
max_level: none
world_xp_rate_percent: 300
damage_counts_for_minutes: 5
party_share_range_m: 50
party_bonus_percent_per_member: 10
xp_to_next_level:
  - { level: 1, xp: 184, total_before: 0 }
  - { level: 2, xp: 294, total_before: 184 }
  - { level: 3, xp: 436, total_before: 478 }
  - { level: 4, xp: 617, total_before: 914 }
  - { level: 5, xp: 840, total_before: 1531 }
  - { level: 6, xp: 1108, total_before: 2371 }
  - { level: 7, xp: 1428, total_before: 3479 }
  - { level: 8, xp: 1801, total_before: 4907 }
  - { level: 9, xp: 2234, total_before: 6708 }
  - { level: 10, xp: 2730, total_before: 8942 }
  - { level: 15, xp: 6300, total_before: 28952 }
  - { level: 16, xp: 7840, total_before: 35252 }
  - { level: 20, xp: 15972, total_before: 77644 }
  - { level: 30, xp: 56320, total_before: 392064 }
  - { level: 50, xp: 267696, total_before: 3178334 }
  - { level: 60, xp: 465124, total_before: 6684184 }
  - { level: 61, xp: 495625, total_before: 7149308 }
  - { level: 100, xp: 2314000, total_before: 55235658 }
  - { level: 113, xp: 3371355, total_before: 91377413 }
  - { level: 114, xp: 3498416, total_before: 94748768 }
  - { level: 150, xp: 9053044, total_before: 307593160 }
  - { level: 151, xp: 9309384, total_before: 316646204 }
  - { level: 189, xp: 22143732, total_before: 885191792 }
  - { level: 190, xp: 36400000, total_before: 907335524 }
  - { level: 200, xp: 443520000, total_before: 2747274075 }
level_up_rewards:
  - { "reaching level": 2, skill_points: 2, stat_points: 11 }
  - { "reaching level": 3, skill_points: 2, stat_points: 12 }
  - { "reaching level": 4, skill_points: 3, stat_points: 13 }
  - { "reaching level": 5, skill_points: 3, stat_points: 14 }
  - { "reaching level": 10, skill_points: 6, stat_points: 18 }
  - { "reaching level": 50, skill_points: 26, stat_points: 50 }
  - { "reaching level": 100, skill_points: 51, stat_points: 90 }
  - { "reaching level": 150, skill_points: 76, stat_points: 130 }
  - { "reaching level": 200, skill_points: 101, stat_points: 170 }
points_by_level:
  - { level: 10, total_skill_points: 34, total_stat_points: 130 }
  - { level: 50, total_skill_points: 674, total_stat_points: 1490 }
  - { level: 100, total_skill_points: 2599, total_stat_points: 4990 }
  - { level: 200, total_skill_points: 10199, total_stat_points: 17990 }
source:
  code:
    - crates/rose-game-irose/src/data/ability_values.rs (levelup_require_xp, calculate_give_xp, calculate_levelup_reward_skill_points, calculate_levelup_reward_stat_points)
    - module/src/character.rs (reward_xp)
    - module/src/lib.rs (reward_kill, deal_damage, add_damage_source, DAMAGE_REWARD_EXPIRE_US, world_rates_row, give_xp)
    - module/src/party.rs (reward_kill_xp, nearby, SHARE_RANGE_CM, XP_BONUS_PERCENT_PER_MEMBER)
    - module/src/death.rs (debt)
  data: LIST_NPC.STB (monster level, HP and experience columns)
---
# Experience

You gain experience (XP) by hurting monsters that then die, and from quests. When your XP
reaches what your level needs, you level up: you get **stat points** to raise your basic
stats (see [[rules/stats|Stats]]) and **skill points** to learn skills (see
[[rules/skills|Skills]]), and your HP and MP fill up.

## XP needed per level

The XP you need to go from a level to the next one:

| Level | XP to the next level |
| --- | --- |
| 1 to 15 | (L + 3) × (L + 5) × (L + 10) × 0.7 |
| 16 to 60 | (L - 5) × (L + 2) × (L + 2) × 2.2 |
| 61 to 113 | (L - 11) × L × (L + 4) × 2.5 |
| 114 to 150 | (L - 31) × (L - 20) × (L + 4) × 3.8 |
| 151 to 189 | (L - 67) × (L - 20) × (L - 10) × 6 |
| 190 and up | (L - 90) × (L - 120) × (L - 60) × (L - 170) × (L - 188) |

`L` is your current level; the result is rounded down. The table `xp_to_next_level` above
lists exact values, with the total XP earned before reaching that level.

**Worked example:** level 1 needs (4 × 6 × 11) × 0.7 = 264 × 0.7 = 184.8, so **184** XP.
Level 16 needs (11 × 18 × 18) × 2.2 = 3564 × 2.2 = **7840**.

XP left over carries into the next level, and one big reward can give several levels at
once. Each new level runs its level up quest events.

If you owe XP from a death (see [[rules/death|Death]]), the debt is added to what your
current level needs, and is cleared when you level up.

> Open question: there is no maximum level. Above level 189 the requirement grows very fast,
> and from level 224 on the number no longer fits and wraps around in the server, so a
> level cap (iROSE players expect one) should be decided.

## XP from monsters

When a monster dies, **everyone who hurt it** in the last 5 minutes gets XP, each worked
out from their own damage. Damage older than 5 minutes doesn't count, and a monster that
gives up and walks home forgets who hurt it. Summons count for their owner. Only the HP
the monster actually lost counts (a 100 damage hit on a monster with 30 HP left counts 30).

```math
\text{XP} = \frac{(L_m + 3) \times \text{XP}_m \times (\text{damage} + \frac{\text{HP}_m}{15} + 30) \times \text{rate}}{\text{HP}_m \times 370}
```

when your level is less than 3 above the monster's, and

```math
\text{XP} = \frac{(L_m + 3) \times \text{XP}_m \times (\text{damage} + \frac{\text{HP}_m}{15} + 30) \times \text{rate}}{\text{HP}_m \times (d + 3) \times 60}
```

when your level is 3 or more above it, where `d` is your level minus the monster's.

- `L_m` is the monster's level, `XP_m` the experience value on its page, `HP_m` its max HP
  (its level × the HP value on its page).
- `damage` is the damage you did to it.
- `rate` is the server's experience rate, **300** (percent) unless an admin changes it.
- The result is rounded down. A result of 0 gives nothing.

Being lower level than the monster gives no extra XP. Being 3 or more levels above it
divides by (d + 3) × 60 instead of 370, so XP drops as the gap grows (at exactly 3 levels
above you get slightly more than at 2).

**Worked example:** a level 1 character kills a [[monsters/2-jelly-bean|Jelly Bean]] (level 4,
HP 4 × 30 = 120, experience 10) alone, doing all 120 damage:
(7 × 10 × (120 + 8 + 30) × 300) / (120 × 370) = 3,318,000 / 44,400 = 74.7, so **74 XP**.
About three Jelly Beans take you to level 2. A level 10 character (d = 6) gets
3,318,000 / (120 × 9 × 60) = 51.2, so 51 XP for the same kill.

If two players each did 60 damage, each gets
(7 × 10 × 98 × 300) / 44,400 = 46.3, so 46 XP: sharing a kill gives more in total.

## Party XP

When a player who earned XP is in a party (see [[rules/party|Party]]), the XP is shared with
party members who are online, in the same zone and within **50 m** of the monster when it
dies. The player who earned it always shares, even from farther away.

With two or more sharing, the XP grows by **10%** for each member besides the first:

```math
\text{total} = \left\lfloor \text{XP} \times \frac{100 + 10 \times (n - 1)}{100} \right\rfloor
```

Then it is split by the party's setting:

- **Equal:** each gets total / n, rounded down.
- **By level:** each gets total × their level / the sum of the sharing members' levels,
  rounded down.

Everyone gets at least 1. Each member's share counts on its own for levelling and
[[rules/stamina|Stamina]].

**Worked example:** the 74 XP Jelly Bean kill above, in a party with a level 9 friend
nearby: total = 74 × 110 / 100 = 81. Equal: 40 each. By level: the level 1 player gets
81 × 1 / 10 = 8 and the level 9 friend 81 × 9 / 10 = 72.

## Points per level

Each level you reach gives:

```math
\text{skill points} = \left\lfloor \frac{\text{new level} + 2}{2} \right\rfloor
```

```math
\text{stat points} = \lfloor \text{new level} \times 0.8 \rfloor + 10
```

**Worked example:** reaching level 2 gives (2 + 2) / 2 = **2** skill points and
1.6 → 1, + 10 = **11** stat points. New characters start at level 1 with 0 of each. The
tables above list more levels and the totals you have by a level.

## Other sources

- Quests give XP with their own reward formulas (see [[quests|Quests]]).
- Admins can give XP for testing.

## Changed from iROSE

- The 10% per member party bonus is our own choice: the iROSE party bonus isn't in the code
  we build on.
- There is no level cap (see the open question above).
