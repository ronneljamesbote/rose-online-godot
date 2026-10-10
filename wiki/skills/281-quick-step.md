---
kind: skill
id: 281
name: Quick Step
status: in-game
icon: skill/171
type: Passive
job: Soldier Job
max_level: 10
target: Yourself
warps_to: '[[zones/52-mana-snowfields|Mana Snowfields]]'
skill_books:
- '[[items/consumable/657-quick-step|Quick Step]]'
levels:
- level: 1
  id: 281
  needs: '[[skills/231-physical-training|Physical Training]] level 5'
  learn_points: 4
  changes: Movement Speed +30
- level: 2
  id: 282
  learn_points: 6
  changes: Movement Speed +40
- level: 3
  id: 283
  needs: '[[skills/231-physical-training|Physical Training]] level 6'
  learn_points: 8
  changes: Movement Speed +52
- level: 4
  id: 284
  learn_points: 11
  changes: Movement Speed +64
- level: 5
  id: 285
  needs: '[[skills/231-physical-training|Physical Training]] level 7'
  learn_points: 14
  changes: Movement Speed +76
- level: 6
  id: 286
  learn_points: 18
  changes: Movement Speed +90
- level: 7
  id: 287
  needs: '[[skills/231-physical-training|Physical Training]] level 8'
  learn_points: 23
  changes: Movement Speed +105
- level: 8
  id: 288
  learn_points: 29
  changes: Movement Speed +120
- level: 9
  id: 289
  needs: '[[skills/231-physical-training|Physical Training]] level 9'
  learn_points: 36
  changes: Movement Speed +135
- level: 10
  id: 290
  learn_points: 44
  changes: Movement Speed +150
source:
  data: LIST_SKILL.STB rows 281, 282, 283, 284, 285, 286, 287, 288, 289, 290
  code: module/src/skills.rs
---
# Quick Step

Increase Movement Speed. Only Champions can upgrade this skill above Level 5.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
