---
kind: skill
id: 231
name: Physical Training
status: in-game
icon: skill/31
type: Passive
job: Soldier Job
max_level: 20
target: Yourself
skill_books:
- '[[items/consumable/10-vital-water-s|Vital Water (S)]]'
- '[[items/consumable/108-beef-jerky|Beef Jerky]]'
- '[[items/consumable/654-physical-training|Physical Training]]'
levels:
- level: 1
  id: 231
  learn_points: 4
  changes: Max HP +30
- level: 2
  id: 232
  learn_points: 5
  changes: Max HP +52
- level: 3
  id: 233
  learn_points: 6
  changes: Max HP +76
- level: 4
  id: 234
  learn_points: 8
  changes: Max HP +102
- level: 5
  id: 235
  learn_points: 10
  changes: Max HP +130
- level: 6
  id: 236
  learn_points: 12
  changes: Max HP +160
- level: 7
  id: 237
  learn_points: 14
  changes: Max HP +192
- level: 8
  id: 238
  learn_points: 16
  changes: Max HP +226
- level: 9
  id: 239
  learn_points: 18
  changes: Max HP +262
- level: 10
  id: 240
  learn_points: 21
  changes: Max HP +300
- level: 11
  id: 241
  needs_level: 70
  learn_points: 24
  changes: Max HP +340
- level: 12
  id: 242
  learn_points: 27
  changes: Max HP +382
- level: 13
  id: 243
  learn_points: 30
  changes: Max HP +426
- level: 14
  id: 244
  learn_points: 34
  changes: Max HP +472
- level: 15
  id: 245
  learn_points: 38
  changes: Max HP +520
- level: 16
  id: 246
  learn_points: 42
  changes: Max HP +570
- level: 17
  id: 247
  learn_points: 47
  changes: Max HP +622
- level: 18
  id: 248
  learn_points: 52
  changes: Max HP +676
- level: 19
  id: 249
  learn_points: 58
  changes: Max HP +732
- level: 20
  id: 250
  learn_points: 64
  changes: Max HP +800
source:
  data: LIST_SKILL.STB rows 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250
  code: module/src/skills.rs
---
# Physical Training

Increase Maximum HP amount.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
