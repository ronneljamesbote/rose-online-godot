---
kind: skill
id: 1231
name: Critical Support
status: in-game
icon: skill/214
type: Continuing
job: Cleric Job
max_level: 10
target: Ally
skill_books:
- '[[items/consumable/740-critical-support|Critical Support]]'
levels:
- level: 1
  id: 1231
  needs: '[[skills/1221-hit-support|Hit Support]] level 10'
  learn_points: 35
  cost: MP 40
  range: 20
  cooldown: 2.4
  duration: 300
  effects: Cri Up
  changes: Critical +40
- level: 2
  id: 1232
  learn_points: 39
  cost: MP 44
  range: 20
  cooldown: 2.4
  duration: 320
  effects: Cri Up
  changes: Critical +45
- level: 3
  id: 1233
  learn_points: 43
  cost: MP 48
  range: 20
  cooldown: 2.4
  duration: 340
  effects: Cri Up
  changes: Critical +50
- level: 4
  id: 1234
  learn_points: 47
  cost: MP 52
  range: 20
  cooldown: 2.4
  duration: 360
  effects: Cri Up
  changes: Critical +55
- level: 5
  id: 1235
  learn_points: 52
  cost: MP 56
  range: 20
  cooldown: 2.4
  duration: 380
  effects: Cri Up
  changes: Critical +60
- level: 6
  id: 1236
  learn_points: 57
  cost: MP 60
  range: 20
  cooldown: 2.4
  duration: 400
  effects: Cri Up
  changes: Critical +65
- level: 7
  id: 1237
  learn_points: 63
  cost: MP 64
  range: 20
  cooldown: 2.4
  duration: 420
  effects: Cri Up
  changes: Critical +70
- level: 8
  id: 1238
  learn_points: 69
  cost: MP 68
  range: 20
  cooldown: 2.4
  duration: 440
  effects: Cri Up
  changes: Critical +75
- level: 9
  id: 1239
  learn_points: 76
  cost: MP 72
  range: 20
  cooldown: 2.4
  duration: 460
  effects: Cri Up
  changes: Critical +80
- level: 10
  id: 1240
  learn_points: 83
  cost: MP 80
  range: 20
  cooldown: 2.4
  duration: 500
  effects: Cri Up
  changes: Critical +90
source:
  data: LIST_SKILL.STB rows 1231, 1232, 1233, 1234, 1235, 1236, 1237, 1238, 1239, 1240
  code: module/src/skills.rs
---
# Critical Support

Increase target's Critical attack rate for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
