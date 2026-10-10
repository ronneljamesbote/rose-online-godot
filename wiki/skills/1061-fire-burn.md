---
kind: skill
id: 1061
name: Fire Burn
status: in-game
icon: skill/96
type: Magic Spell
job: Mage Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/723-fire-burn|Fire Burn]]'
levels:
- level: 1
  id: 1061
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10, [[skills/911-fire-ring|Fire Ring]] level 5'
  learn_points: 40
  cost: MP 90
  power: 180
  range: 22
  cooldown: 10
  duration: 15
  success: 25
  effects: Def Down
  changes: Defense -10%
- level: 2
  id: 1062
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 11'
  learn_points: 45
  cost: MP 97
  power: 200
  range: 22.5
  cooldown: 10.4
  duration: 16
  success: 25
  effects: Def Down
  changes: Defense -11%
- level: 3
  id: 1063
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 51
  cost: MP 104
  power: 220
  range: 23
  cooldown: 10.8
  duration: 17
  success: 25
  effects: Def Down
  changes: Defense -12%
- level: 4
  id: 1064
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 13'
  learn_points: 58
  cost: MP 111
  power: 240
  range: 23.5
  cooldown: 11.2
  duration: 18
  success: 25
  effects: Def Down
  changes: Defense -13%
- level: 5
  id: 1065
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 14'
  learn_points: 65
  cost: MP 118
  power: 260
  range: 24
  cooldown: 11.6
  duration: 19
  success: 25
  effects: Def Down
  changes: Defense -14%
- level: 6
  id: 1066
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 15'
  learn_points: 73
  cost: MP 130
  power: 280
  range: 24.5
  area: 5
  cooldown: 12.8
  duration: 20
  success: 26
  effects: Def Down
  changes: Defense -15%
- level: 7
  id: 1067
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 16'
  learn_points: 82
  cost: MP 140
  power: 300
  range: 25
  area: 5.3
  cooldown: 13.2
  duration: 21
  success: 27
  effects: Def Down
  changes: Defense -16%
- level: 8
  id: 1068
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 17'
  learn_points: 92
  cost: MP 150
  power: 320
  range: 25.5
  area: 5.6
  cooldown: 13.6
  duration: 22
  success: 28
  effects: Def Down
  changes: Defense -17%
- level: 9
  id: 1069
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 18'
  learn_points: 103
  cost: MP 160
  power: 340
  range: 26
  area: 5.9
  cooldown: 14
  duration: 23
  success: 29
  effects: Def Down
  changes: Defense -18%
- level: 10
  id: 1070
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 20'
  learn_points: 115
  cost: MP 175
  power: 360
  range: 27
  area: 6.2
  cooldown: 14.4
  duration: 24
  success: 33
  effects: Def Down
  changes: Defense -20%
source:
  data: LIST_SKILL.STB rows 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068, 1069, 1070
  code: module/src/skills.rs
---
# Fire Burn

Shoot a fire bolt at a target to inflict damage and decrease Defense at the same time.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
