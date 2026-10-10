---
kind: skill
id: 251
name: Defense Training
status: in-game
icon: skill/30
type: Passive
job: Soldier Job
max_level: 20
target: Yourself
warps_to: '[[zones/53-arumic-valley|Arumic Valley]]'
skill_books:
- '[[items/consumable/655-armor-mastery|Armor Mastery]]'
levels:
- level: 1
  id: 251
  needs: '[[skills/231-physical-training|Physical Training]] level 2'
  learn_points: 4
  changes: Defense +6
- level: 2
  id: 252
  learn_points: 5
  changes: Defense +10
- level: 3
  id: 253
  needs: '[[skills/231-physical-training|Physical Training]] level 3'
  learn_points: 6
  changes: Defense +14
- level: 4
  id: 254
  learn_points: 7
  changes: Defense +18
- level: 5
  id: 255
  needs: '[[skills/231-physical-training|Physical Training]] level 4'
  learn_points: 9
  changes: Defense +22
- level: 6
  id: 256
  learn_points: 11
  changes: Defense +26
- level: 7
  id: 257
  needs: '[[skills/231-physical-training|Physical Training]] level 5'
  learn_points: 13
  changes: Defense +30
- level: 8
  id: 258
  learn_points: 15
  changes: Defense +34
- level: 9
  id: 259
  needs: '[[skills/231-physical-training|Physical Training]] level 6'
  learn_points: 17
  changes: Defense +38
- level: 10
  id: 260
  learn_points: 19
  changes: Defense +43
- level: 11
  id: 261
  needs: '[[skills/231-physical-training|Physical Training]] level 7'
  learn_points: 22
  changes: Defense +48
- level: 12
  id: 262
  learn_points: 25
  changes: Defense +53
- level: 13
  id: 263
  needs: '[[skills/231-physical-training|Physical Training]] level 8'
  learn_points: 28
  changes: Defense +58
- level: 14
  id: 264
  learn_points: 31
  changes: Defense +63
- level: 15
  id: 265
  needs: '[[skills/231-physical-training|Physical Training]] level 9'
  learn_points: 34
  changes: Defense +68
- level: 16
  id: 266
  learn_points: 38
  changes: Defense +74
- level: 17
  id: 267
  needs: '[[skills/231-physical-training|Physical Training]] level 10'
  learn_points: 42
  changes: Defense +80
- level: 18
  id: 268
  learn_points: 46
  changes: Defense +86
- level: 19
  id: 269
  learn_points: 51
  changes: Defense +92
- level: 20
  id: 270
  learn_points: 56
  changes: Defense +100
source:
  data: LIST_SKILL.STB rows 251, 252, 253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270
  code: module/src/skills.rs
---
# Defense Training

Increase Defense.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
