---
kind: skill
id: 1071
name: Silence
status: in-game
icon: skill/89
type: Continuing
job: Mage Job
max_level: 5
target: Hostile Character
skill_books:
- '[[items/consumable/724-silence|Silence]]'
levels:
- level: 1
  id: 1071
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10, [[skills/1031-weaken|Weaken]] level 5'
  learn_points: 30
  cost: MP 50
  range: 16
  cooldown: 10
  duration: 20
  success: 40
  effects: Muting
- level: 2
  id: 1072
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 34
  cost: MP 55
  range: 17
  cooldown: 9.8
  duration: 24
  success: 45
  effects: Muting
- level: 3
  id: 1073
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 14'
  learn_points: 38
  cost: MP 60
  range: 18
  cooldown: 9.6
  duration: 28
  success: 50
  effects: Muting
- level: 4
  id: 1074
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 16'
  learn_points: 43
  cost: MP 65
  range: 19
  cooldown: 9.4
  duration: 32
  success: 55
  effects: Muting
- level: 5
  id: 1075
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 18'
  learn_points: 48
  cost: MP 70
  range: 21
  cooldown: 9.2
  duration: 40
  success: 60
  effects: Muting
source:
  data: LIST_SKILL.STB rows 1071, 1072, 1073, 1074, 1075
  code: module/src/skills.rs
---
# Silence

Blocks a target from casting magic spells for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
