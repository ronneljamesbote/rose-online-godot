---
kind: skill
id: 1401
name: Bow Mastery
status: in-game
icon: skill/104
type: Passive
job: Hawker Job
max_level: 20
target: Yourself
needs_weapon: Bow
warps_to: zone 44
skill_books:
- '[[items/consumable/751-bow-mastery|Bow Mastery]]'
levels:
- level: 1
  id: 1401
  learn_points: 4
  changes: Bow Attack Power +8
- level: 2
  id: 1402
  learn_points: 5
  changes: Bow Attack Power +12
- level: 3
  id: 1403
  learn_points: 7
  changes: Bow Attack Power +16
- level: 4
  id: 1404
  learn_points: 9
  changes: Bow Attack Power +20
- level: 5
  id: 1405
  learn_points: 11
  changes: Bow Attack Power +24
- level: 6
  id: 1406
  learn_points: 13
  changes: Bow Attack Power +28
- level: 7
  id: 1407
  learn_points: 15
  changes: Bow Attack Power +32
- level: 8
  id: 1408
  learn_points: 18
  changes: Bow Attack Power +36
- level: 9
  id: 1409
  learn_points: 21
  changes: Bow Attack Power +40
- level: 10
  id: 1410
  learn_points: 24
  changes: Bow Attack Power +45
- level: 11
  id: 1411
  learn_points: 27
  changes: Bow Attack Power +50
- level: 12
  id: 1412
  learn_points: 30
  changes: Bow Attack Power +55
- level: 13
  id: 1413
  learn_points: 34
  changes: Bow Attack Power +60
- level: 14
  id: 1414
  learn_points: 38
  changes: Bow Attack Power +65
- level: 15
  id: 1415
  learn_points: 42
  changes: Bow Attack Power +70
- level: 16
  id: 1416
  learn_points: 47
  changes: Bow Attack Power +75
- level: 17
  id: 1417
  learn_points: 52
  changes: Bow Attack Power +80
- level: 18
  id: 1418
  learn_points: 58
  changes: Bow Attack Power +85
- level: 19
  id: 1419
  learn_points: 64
  changes: Bow Attack Power +90
- level: 20
  id: 1420
  learn_points: 71
  changes: Bow Attack Power +95
source:
  data: LIST_SKILL.STB rows 1401, 1402, 1403, 1404, 1405, 1406, 1407, 1408, 1409, 1410, 1411, 1412, 1413, 1414, 1415, 1416, 1417, 1418, 1419, 1420
  code: module/src/skills.rs
---
# Bow Mastery

Increase Attack Power of Bow weapons.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
