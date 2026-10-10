---
kind: skill
id: 51
name: Disillusion
status: in-game
icon: skill/68
type: Emotion
job: any
max_level: 10
target: All Members
warps_to: zone 76
levels:
- level: 0
  id: 51
  learn_points: 0
  cost: MP 10
  area: 14
  cooldown: 12
  changes: Stamina +10
- level: 1
  id: 101
  needs_level: 1
  learn_points: 2
  cost: HP 10
  cooldown: 0.2
  success: 100
- level: 2
  id: 102
  needs_level: 1
  learn_points: 2
  cost: HP 11
  cooldown: 0.2
  success: 100
- level: 3
  id: 103
  needs_level: 1
  learn_points: 2
  cost: HP 12
  cooldown: 0.2
  success: 100
- level: 4
  id: 104
  needs_level: 1
  learn_points: 2
  cost: HP 13
  cooldown: 0.2
  success: 100
- level: 5
  id: 105
  needs_level: 1
  learn_points: 2
  cost: HP 14
  cooldown: 0.2
  success: 100
- level: 6
  id: 106
  needs_level: 1
  learn_points: 2
  cost: HP 15
  cooldown: 0.2
  success: 100
- level: 7
  id: 107
  needs_level: 1
  learn_points: 2
  cost: HP 17
  cooldown: 0.2
  success: 100
- level: 8
  id: 108
  needs_level: 1
  learn_points: 2
  cost: HP 19
  cooldown: 0.2
  success: 100
- level: 9
  id: 109
  needs_level: 1
  learn_points: 2
  cost: HP 21
  cooldown: 0.2
  success: 100
- level: 10
  id: 110
  needs_level: 1
  learn_points: 2
  cost: HP 23
  cooldown: 0.2
  success: 100
source:
  data: LIST_SKILL.STB rows 51, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110
  code: module/src/skills.rs
---
# Disillusion

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
