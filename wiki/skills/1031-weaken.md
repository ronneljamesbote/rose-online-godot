---
kind: skill
id: 1031
name: Weaken
status: in-game
icon: skill/205
type: Continuing
job: Muse Job
max_level: 10
target: Hostile Character
skill_books:
- '[[items/consumable/720-weaken|Weaken]]'
levels:
- level: 1
  id: 1031
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 6'
  learn_points: 15
  cost: MP 30
  range: 22
  cooldown: 8
  duration: 20
  success: 50
  effects: Atk Power Down
  changes: Attack Power -16%
- level: 2
  id: 1032
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 7'
  learn_points: 18
  cost: MP 33
  range: 22.5
  cooldown: 8
  duration: 23
  success: 53
  effects: Atk Power Down
  changes: Attack Power -19%
- level: 3
  id: 1033
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 8'
  learn_points: 21
  cost: MP 36
  range: 23
  cooldown: 8
  duration: 26
  success: 56
  effects: Atk Power Down
  changes: Attack Power -22%
- level: 4
  id: 1034
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 9'
  learn_points: 24
  cost: MP 39
  range: 23.5
  cooldown: 8
  duration: 29
  success: 59
  effects: Atk Power Down
  changes: Attack Power -25%
- level: 5
  id: 1035
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10'
  learn_points: 27
  cost: MP 42
  range: 24
  cooldown: 8
  duration: 30
  success: 62
  effects: Atk Power Down
  changes: Attack Power -28%
- level: 6
  id: 1036
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 11'
  learn_points: 32
  cost: MP 45
  range: 24.5
  cooldown: 8
  duration: 30
  success: 65
  effects: Atk Power Down, Slow Attack
  changes: Attack Power -30%, Attack Speed -20%
- level: 7
  id: 1037
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 38
  cost: MP 48
  range: 25
  cooldown: 8
  duration: 33
  success: 68
  effects: Atk Power Down, Slow Attack
  changes: Attack Power -32%, Attack Speed -25%
- level: 8
  id: 1038
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 13'
  learn_points: 45
  cost: MP 51
  range: 25.5
  cooldown: 8
  duration: 36
  success: 71
  effects: Atk Power Down, Slow Attack
  changes: Attack Power -34%, Attack Speed -30%
- level: 9
  id: 1039
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 14'
  learn_points: 53
  cost: MP 54
  range: 26
  cooldown: 8
  duration: 39
  success: 74
  effects: Atk Power Down, Slow Attack
  changes: Attack Power -36%, Attack Speed -35%
- level: 10
  id: 1040
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 15'
  learn_points: 62
  cost: MP 60
  range: 26.5
  cooldown: 8
  duration: 42
  success: 80
  effects: Atk Power Down, Slow Attack
  changes: Attack Power -38%, Attack Speed -40%
source:
  data: LIST_SKILL.STB rows 1031, 1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040
  code: module/src/skills.rs
---
# Weaken

Decrease the Attack Power of a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
