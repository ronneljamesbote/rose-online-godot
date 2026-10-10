---
kind: rule
id: death
name: Death and Reviving
status: changed-from-irose
xp_loss_min_level: 10
xp_loss_percent: 3
xp_debt_cap: "half of what the level needs"
revive_hp_percent: 30
revive_spread_m: 5
auto_get_up_min: 10
pvp_revive_protection_s: 30
kill_credit_window_min: 5
xp_loss_examples:
  - { level: "1-9", level_needs: "", loss: "0" }
  - { level: 10, level_needs: 2730, loss: 81 }
  - { level: 20, level_needs: 15972, loss: 479 }
  - { level: 30, level_needs: 56320, loss: 1689 }
  - { level: 50, level_needs: 267696, loss: 8030 }
  - { level: 70, level_needs: 764050, loss: 22921 }
  - { level: 100, level_needs: 2314000, loss: 69420 }
  - { level: 150, level_needs: 9053044, loss: 271591 }
resurrection_xp_back:
  - { skill_level: 1, xp_back: "0%" }
  - { skill_level: 2, xp_back: "15%" }
  - { skill_level: 3, xp_back: "30%" }
  - { skill_level: 4, xp_back: "40%" }
  - { skill_level: 5, xp_back: "50%" }
source:
  code:
    - module/src/death.rs (player_died, refund, revive, revive_point, tick, revive_player, set_save_point)
    - module/src/lib.rs (deal_damage, kill, reward_kill, client_disconnected)
    - module/src/character.rs (reward_xp)
    - module/src/skills.rs (resurrect, Resurrection cast)
    - crates/rose-game-irose/src/data/ability_values.rs (levelup_require_xp)
  data: LIST_ZONE.STB (revive points, planet, PvP state); LIST_SKILL.STB (Resurrection power)
---
# Death and Reviving

When your HP reaches 0 you fall. You stay on the ground until you choose where to get up,
or until someone casts [[skills/1131-resurrection|Resurrection]] on you. While you are down
nothing can hurt you, and you get off any cart or castle gear you were driving or riding.

## What a death costs

**Killed by a monster, at level 10 or above:** you lose 3% of the experience your current
level needs (rounded down). You see "You lost N experience".

**Killed by another player, or below level 10:** nothing.

If you have less experience on your bar than the loss, your bar goes to 0 and the rest
becomes an **experience debt**. The debt is added to what your current level needs, so you
must earn it back before you level up. It can't grow beyond half of what the level needs.
It is cleared when you level up.

```math
\text{loss} = \left\lfloor \text{XP the level needs} \times \frac{3}{100} \right\rfloor
```

What each level needs is on [[rules/experience|Experience]]; the table above gives some
examples.

**Worked example.** A level 30 character needs 56320 experience for the level. A death to
a monster costs 56320 x 3 / 100 = 1689. With 5000 on the bar, the bar drops to 3311. With
only 1000 on the bar, the bar drops to 0 and the debt is 689: the next level up now needs
56320 + 689 = 57009. The debt can't go beyond 28160 however often the character dies.

## Getting up

A fallen character picks one of two places:

- **Save point**: where you last saved, by talking to an NPC that offers it or from a quest.
  This works only when the save point is on the same planet as the zone you fell in;
  otherwise you get up at the revive point instead.
- **Revive point**: the zone's revive point nearest to where you fell.

Either way you get up with 30% of your maximum HP (at least 1), somewhere within 5 metres of
the point (up to 5 metres east or west and 5 metres north or south). MP stays as it was.
All buffs and other effects on you end when you get up.

If you get up in a PvP zone, you can't be hurt for 30 seconds.

If you don't choose within **10 minutes**, you get up at the zone's revive point by
yourself. If you leave the game while fallen, you are first got up at the zone's revive
point the same way, so logging out is not a way to get up where you fell.

## Resurrection

A Cleric's [[skills/1131-resurrection|Resurrection]] gets a fallen player up where they
lie, with 30% of their maximum HP. Their buffs end. It also gives back part of the
experience that death cost (table above): the skill's power, in percent of what was taken
from the bar plus the debt that was added. The experience comes back on the bar; the debt
stays.

**Worked example.** The level 30 character above lost 1689 to the death. A level 5
Resurrection gives back 1689 x 50 / 100 = 844 experience.

## When a monster dies

- Everyone who damaged it in the last 5 minutes gets experience for their share of the
  damage (see [[rules/experience|Experience]]); in a party it is shared (see
  [[rules/party|Party]]). Kill experience also gives [[rules/stamina|stamina]].
- The player who landed the killing blow runs the monster's death trigger (quest kill
  counts and quest items).
- It drops items and Zuly (see [[rules/drops|Drops]]), owned at first by the killer.
- Its body disappears at once. When it comes back is on
  [[rules/monster-spawns|Monster spawns]].
- A summoned monster just disappears: no experience, drops or quest triggers.

## PvP deaths

A player killed by another player loses no experience. Both players see "*winner* defeated
*loser*". Getting up works as above; see [[rules/pvp|PvP]] for where players can fight.

## Changed from iROSE

- iROSE waits for ever for a fallen character to choose. Here a character who hasn't chosen
  gets up by itself at the revive point after 10 minutes.

> Open question: Resurrection adds the refunded experience without checking for a level up, so a refund that passes the next level only levels you up at your next kill.

> Open question: LIST_ZONE.STB columns 31 to 33 (more revive settings per zone) are not read; only the revive points are used.
