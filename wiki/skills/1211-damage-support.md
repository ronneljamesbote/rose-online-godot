---
kind: skill
id: 1211
name: Damage Support
status: in-game
icon: skill/213
type: Continuing
job: Cleric Job
max_level: 10
target: Ally
damage_type: continuous attack
skill_books:
- '[[items/consumable/738-damage-support|Damage Support]]'
levels:
- level: 1
  id: 1211
  needs: '[[skills/1021-power-support|Power Support]] level 10'
  learn_points: 35
  cost: MP 50
  power: 14
  range: 20
  cooldown: 2.4
  duration: 300
  effects: Additional Damage
- level: 2
  id: 1212
  learn_points: 39
  cost: MP 55
  power: 16
  range: 20
  cooldown: 2.4
  duration: 320
  effects: Additional Damage
- level: 3
  id: 1213
  learn_points: 43
  cost: MP 60
  power: 18
  range: 20
  cooldown: 2.4
  duration: 340
  effects: Additional Damage
- level: 4
  id: 1214
  learn_points: 47
  cost: MP 65
  power: 20
  range: 20
  cooldown: 2.4
  duration: 360
  effects: Additional Damage
- level: 5
  id: 1215
  learn_points: 52
  cost: MP 70
  power: 22
  range: 20
  cooldown: 2.4
  duration: 380
  effects: Additional Damage
- level: 6
  id: 1216
  learn_points: 57
  cost: MP 75
  power: 24
  range: 20
  cooldown: 2.4
  duration: 400
  effects: Additional Damage
- level: 7
  id: 1217
  learn_points: 63
  cost: MP 80
  power: 26
  range: 20
  cooldown: 2.4
  duration: 420
  effects: Additional Damage
- level: 8
  id: 1218
  learn_points: 69
  cost: MP 85
  power: 28
  range: 20
  cooldown: 2.4
  duration: 440
  effects: Additional Damage
- level: 9
  id: 1219
  learn_points: 76
  cost: MP 90
  power: 30
  range: 20
  cooldown: 2.4
  duration: 460
  effects: Additional Damage
- level: 10
  id: 1220
  learn_points: 83
  cost: MP 100
  power: 33
  range: 20
  cooldown: 2.4
  duration: 500
  effects: Additional Damage
source:
  data: LIST_SKILL.STB rows 1211, 1212, 1213, 1214, 1215, 1216, 1217, 1218, 1219, 1220
  code: module/src/skills.rs
---
# Damage Support

Increase target's normal attack and skill damage for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
