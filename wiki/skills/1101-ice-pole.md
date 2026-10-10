---
kind: skill
id: 1101
name: Ice Pole
status: in-game
icon: skill/201
type: Area Effect Magic Damage
job: Mage Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/727-ice-pole|Ice Pole]]'
levels:
- level: 1
  id: 1101
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 20, [[skills/981-ice-bolt|Ice Bolt]] level 8'
  learn_points: 40
  cost: MP 120
  power: 150
  range: 25
  area: 12
  cooldown: 14
  duration: 15
  success: 30
  effects: Slow
  changes: Movement Speed -50%
- level: 2
  id: 1102
  learn_points: 45
  cost: MP 126
  power: 160
  range: 25
  area: 12.1
  cooldown: 14.2
  duration: 16
  success: 31
  effects: Slow
  changes: Movement Speed -50%
- level: 3
  id: 1103
  learn_points: 51
  cost: MP 132
  power: 170
  range: 25
  area: 12.2
  cooldown: 14.4
  duration: 17
  success: 32
  effects: Slow
  changes: Movement Speed -50%
- level: 4
  id: 1104
  learn_points: 57
  cost: MP 138
  power: 180
  range: 25
  area: 12.3
  cooldown: 14.6
  duration: 18
  success: 33
  effects: Slow
  changes: Movement Speed -50%
- level: 5
  id: 1105
  learn_points: 64
  cost: MP 144
  power: 190
  range: 25
  area: 12.4
  cooldown: 14.8
  duration: 19
  success: 34
  effects: Slow
  changes: Movement Speed -50%
- level: 6
  id: 1106
  learn_points: 71
  cost: MP 150
  power: 200
  range: 25
  area: 12.5
  cooldown: 15
  duration: 20
  success: 35
  effects: Slow
  changes: Movement Speed -50%
- level: 7
  id: 1107
  learn_points: 79
  cost: MP 156
  power: 210
  range: 25
  area: 12.6
  cooldown: 15.2
  duration: 21
  success: 36
  effects: Slow
  changes: Movement Speed -50%
- level: 8
  id: 1108
  learn_points: 88
  cost: MP 162
  power: 220
  range: 25
  area: 12.7
  cooldown: 15.4
  duration: 22
  success: 37
  effects: Slow
  changes: Movement Speed -50%
- level: 9
  id: 1109
  learn_points: 98
  cost: MP 168
  power: 230
  range: 25
  area: 12.8
  cooldown: 15.6
  duration: 23
  success: 38
  effects: Slow
  changes: Movement Speed -50%
- level: 10
  id: 1110
  learn_points: 109
  cost: MP 174
  power: 240
  range: 25
  area: 13
  cooldown: 15.8
  duration: 24
  success: 42
  effects: Slow
  changes: Movement Speed -50%
source:
  data: LIST_SKILL.STB rows 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110
  code: module/src/skills.rs
---
# Ice Pole

Create ice poles within an area that inflict damage and decrease the Movement Speed of enemies when they walk inside the area.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
