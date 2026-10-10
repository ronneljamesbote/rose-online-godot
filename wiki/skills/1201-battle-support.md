---
kind: skill
id: 1201
name: Battle Support
status: in-game
icon: skill/212
type: 'Continuing '
job: Cleric Job
max_level: 10
target: Ally
warps_to: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
skill_books:
- '[[items/consumable/737-battle-support|Battle Support]]'
levels:
- level: 1
  id: 1201
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/1021-power-support|Power Support]] level 5'
  learn_points: 30
  cost: MP 40
  range: 20
  cooldown: 2.4
  duration: 300
  effects: Haste Attack
  changes: Attack Speed +18
- level: 2
  id: 1202
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 33
  cost: MP 43
  range: 20
  cooldown: 2.4
  duration: 320
  effects: Haste Attack
  changes: Attack Speed +20
- level: 3
  id: 1203
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 37
  cost: MP 46
  range: 20
  cooldown: 2.4
  duration: 340
  effects: Haste Attack
  changes: Attack Speed +22
- level: 4
  id: 1204
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 41
  cost: MP 49
  range: 20
  cooldown: 2.4
  duration: 360
  effects: Haste Attack
  changes: Attack Speed +24
- level: 5
  id: 1205
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 45
  cost: MP 52
  range: 20
  cooldown: 2.4
  duration: 380
  effects: Haste Attack
  changes: Attack Speed +26
- level: 6
  id: 1206
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 50
  cost: MP 55
  range: 20
  cooldown: 2.4
  duration: 400
  effects: Haste Attack
  changes: Attack Speed +28
- level: 7
  id: 1207
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 55
  cost: MP 58
  range: 20
  cooldown: 2.4
  duration: 420
  effects: Haste Attack
  changes: Attack Speed +30
- level: 8
  id: 1208
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 60
  cost: MP 61
  range: 20
  cooldown: 2.4
  duration: 440
  effects: Haste Attack
  changes: Attack Speed +32
- level: 9
  id: 1209
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 66
  cost: MP 64
  range: 20
  cooldown: 2.4
  duration: 460
  effects: Haste Attack
  changes: Attack Speed +34
- level: 10
  id: 1210
  needs: '[[skills/821-meditation|Meditation]] level 20'
  learn_points: 72
  cost: MP 70
  range: 20
  cooldown: 2.4
  duration: 500
  effects: Haste Attack
  changes: Attack Speed +36
source:
  data: LIST_SKILL.STB rows 1201, 1202, 1203, 1204, 1205, 1206, 1207, 1208, 1209, 1210
  code: module/src/skills.rs
---
# Battle Support

Increase the Attack Speed of a target for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
