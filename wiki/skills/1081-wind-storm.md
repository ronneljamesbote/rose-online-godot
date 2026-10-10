---
kind: skill
id: 1081
name: Wind Storm
status: in-game
icon: skill/80
type: Area Effect Attack (Self)
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/725-wind-storm|Wind Storm]]'
levels:
- level: 1
  id: 1081
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 8'
  learn_points: 25
  cost: MP 50
  power: 60
  area: 10
  cooldown: 14
- level: 2
  id: 1082
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 9'
  learn_points: 30
  cost: MP 57
  power: 75
  area: 10.1
  cooldown: 14.4
- level: 3
  id: 1083
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 10'
  learn_points: 36
  cost: MP 64
  power: 90
  area: 10.2
  cooldown: 14.8
- level: 4
  id: 1084
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 11'
  learn_points: 43
  cost: MP 71
  power: 105
  area: 10.3
  cooldown: 15.2
- level: 5
  id: 1085
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 12'
  learn_points: 51
  cost: MP 78
  power: 120
  area: 10.4
  cooldown: 15.6
- level: 6
  id: 1086
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 13'
  learn_points: 60
  cost: MP 85
  power: 135
  area: 13
  cooldown: 16
- level: 7
  id: 1087
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 14'
  learn_points: 71
  cost: MP 92
  power: 150
  area: 13.1
  cooldown: 16.4
- level: 8
  id: 1088
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 16'
  learn_points: 83
  cost: MP 99
  power: 165
  area: 13.2
  cooldown: 16.8
- level: 9
  id: 1089
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 18'
  learn_points: 97
  cost: MP 106
  power: 180
  area: 13.3
  cooldown: 17.2
- level: 10
  id: 1090
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 20'
  learn_points: 114
  cost: MP 120
  power: 200
  area: 14
  cooldown: 17.6
source:
  data: LIST_SKILL.STB rows 1081, 1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090
  code: module/src/skills.rs
---
# Wind Storm

Summon a storm to do enormous damage onto enemies near the caster.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
