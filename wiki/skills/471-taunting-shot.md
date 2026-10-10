---
kind: skill
id: 471
name: Taunting Shot
status: in-game
icon: skill/200
type: Damage Action
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Crossbow
skill_books:
- '[[items/consumable/672-taunting-shot|Taunting Shot]]'
levels:
- level: 1
  id: 471
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 3'
  learn_points: 12
  cost: MP 16
  power: 30
  cooldown: 7
  duration: 16
  success: 70
  effects: Taunt
- level: 2
  id: 472
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 4'
  learn_points: 15
  cost: MP 19
  power: 36
  cooldown: 7
  duration: 18
  success: 72
  effects: Taunt
- level: 3
  id: 473
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 5'
  learn_points: 18
  cost: MP 22
  power: 43
  cooldown: 7
  duration: 20
  success: 74
  effects: Taunt
- level: 4
  id: 474
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 6'
  learn_points: 22
  cost: MP 25
  power: 51
  cooldown: 7
  duration: 22
  success: 76
  effects: Taunt
- level: 5
  id: 475
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 7'
  learn_points: 26
  cost: MP 28
  power: 60
  cooldown: 7
  duration: 24
  success: 78
  effects: Taunt
- level: 6
  id: 476
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 8'
  learn_points: 31
  cost: MP 31
  power: 70
  cooldown: 7
  duration: 25
  success: 80
  effects: Taunt
- level: 7
  id: 477
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 9'
  learn_points: 37
  cost: MP 34
  power: 81
  cooldown: 7
  duration: 26
  success: 82
  effects: Taunt
- level: 8
  id: 478
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 10'
  learn_points: 44
  cost: MP 37
  power: 93
  cooldown: 7
  duration: 27
  success: 84
  effects: Taunt
- level: 9
  id: 479
  learn_points: 52
  cost: MP 40
  power: 106
  cooldown: 7
  duration: 28
  success: 86
  effects: Taunt
- level: 10
  id: 480
  learn_points: 61
  cost: MP 45
  power: 120
  cooldown: 7
  duration: 30
  success: 95
  effects: Taunt
source:
  data: LIST_SKILL.STB rows 471, 472, 473, 474, 475, 476, 477, 478, 479, 480
  code: module/src/skills.rs
---
# Taunting Shot

Inflict damage and provoke a target at the same time when using a Bow.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
