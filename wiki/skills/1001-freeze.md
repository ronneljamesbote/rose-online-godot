---
kind: skill
id: 1001
name: Freeze
status: in-game
icon: skill/202
type: Continuing
job: Mage Job
max_level: 5
target: Hostile Character
skill_books:
- '[[items/consumable/717-freeze|Freeze]]'
levels:
- level: 1
  id: 1001
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10, [[skills/981-ice-bolt|Ice Bolt]] level 5'
  learn_points: 25
  cost: MP 30
  range: 25
  cooldown: 8
  duration: 20
  success: 60
  effects: Slow
  changes: Movement Speed -50%
- level: 2
  id: 1002
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 29
  cost: MP 33
  range: 25.5
  cooldown: 8
  duration: 22
  success: 63
  effects: Slow
  changes: Movement Speed -55%
- level: 3
  id: 1003
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 14'
  learn_points: 33
  cost: MP 36
  range: 26
  cooldown: 8
  duration: 24
  success: 66
  effects: Slow
  changes: Movement Speed -60%
- level: 4
  id: 1004
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 16'
  learn_points: 38
  cost: MP 39
  range: 26.5
  cooldown: 8
  duration: 26
  success: 69
  effects: Slow
  changes: Movement Speed -65%
- level: 5
  id: 1005
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 18'
  learn_points: 44
  cost: MP 45
  range: 27
  cooldown: 8
  duration: 30
  success: 72
  effects: Slow
  changes: Movement Speed -70%
source:
  data: LIST_SKILL.STB rows 1001, 1002, 1003, 1004, 1005
  code: module/src/skills.rs
---
# Freeze

Decrease the Movement Speed of a target for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
