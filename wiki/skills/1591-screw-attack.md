---
kind: skill
id: 1591
name: Screw Attack
status: in-game
icon: skill/130
type: Damage Action
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/763-screw-attack|Screw Attack]]'
levels:
- level: 1
  id: 1591
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 6'
  learn_points: 18
  cost: MP 40
  power: 80
  range: 4
  cooldown: 10
  duration: 20
  success: 35
  effects: Def Down
  changes: Defense -20 -10%
- level: 2
  id: 1592
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 7'
  learn_points: 22
  cost: MP 43
  power: 95
  range: 4.2
  cooldown: 10.2
  duration: 20
  success: 37
  effects: Def Down
  changes: Defense -20 -11%
- level: 3
  id: 1593
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 8'
  learn_points: 27
  cost: MP 46
  power: 110
  range: 4.4
  cooldown: 10.4
  duration: 20
  success: 39
  effects: Def Down
  changes: Defense -20 -12%
- level: 4
  id: 1594
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 9'
  learn_points: 32
  cost: MP 49
  power: 125
  range: 4.6
  cooldown: 10.6
  duration: 20
  success: 41
  effects: Def Down
  changes: Defense -20 -13%
- level: 5
  id: 1595
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 10'
  learn_points: 38
  cost: MP 52
  power: 140
  range: 4.8
  cooldown: 10.8
  duration: 20
  success: 43
  effects: Def Down
  changes: Defense -20 -14%
- level: 6
  id: 1596
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 11'
  learn_points: 46
  cost: MP 60
  power: 160
  range: 5
  cooldown: 11
  duration: 20
  success: 45
  effects: Def Down
  changes: Defense -30 -15%
- level: 7
  id: 1597
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 12'
  learn_points: 55
  cost: MP 65
  power: 175
  range: 5.2
  cooldown: 11.2
  duration: 20
  success: 47
  effects: Def Down
  changes: Defense -30 -16%
- level: 8
  id: 1598
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 13'
  learn_points: 66
  cost: MP 70
  power: 190
  range: 5.4
  cooldown: 11.4
  duration: 20
  success: 49
  effects: Def Down
  changes: Defense -30 -17%
- level: 9
  id: 1599
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 14'
  learn_points: 79
  cost: MP 75
  power: 205
  range: 5.6
  cooldown: 11.6
  duration: 20
  success: 51
  effects: Def Down
  changes: Defense -30 -18%
- level: 10
  id: 1600
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 15'
  learn_points: 94
  cost: MP 85
  power: 230
  range: 5.8
  cooldown: 11.8
  duration: 20
  success: 55
  effects: Def Down
  changes: Defense -30 -20%
source:
  data: LIST_SKILL.STB rows 1591, 1592, 1593, 1594, 1595, 1596, 1597, 1598, 1599, 1600
  code: module/src/skills.rs
---
# Screw Attack

Enables its caster to spin wildly on a spot to inflict damage on a target. Requires Katar or Dual Wield weapon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
