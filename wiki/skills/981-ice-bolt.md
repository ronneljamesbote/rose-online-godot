---
kind: skill
id: 981
name: Ice Bolt
status: in-game
icon: skill/93
type: Magic Spell
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/715-ice-bolt|Ice Bolt]]'
levels:
- level: 1
  id: 981
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 2'
  learn_points: 10
  cost: MP 30
  power: 60
  range: 25
  cooldown: 8.4
- level: 2
  id: 982
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 4'
  learn_points: 13
  cost: MP 37
  power: 80
  range: 25.3
  cooldown: 8.8
- level: 3
  id: 983
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 6'
  learn_points: 17
  cost: MP 44
  power: 100
  range: 25.6
  cooldown: 9.2
- level: 4
  id: 984
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 8'
  learn_points: 22
  cost: MP 51
  power: 120
  range: 25.9
  cooldown: 9.6
- level: 5
  id: 985
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 9'
  learn_points: 28
  cost: MP 58
  power: 140
  range: 26.2
  cooldown: 10
- level: 6
  id: 986
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10'
  learn_points: 36
  cost: MP 65
  power: 160
  range: 26.5
  cooldown: 10.4
  duration: 12
  success: 30
  effects: Slow
  changes: Movement Speed -40%
- level: 7
  id: 987
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 46
  cost: MP 75
  power: 180
  range: 26.8
  cooldown: 10.6
  duration: 13
  success: 34
  effects: Slow
  changes: Movement Speed -44%
- level: 8
  id: 988
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 14'
  learn_points: 59
  cost: MP 85
  power: 200
  range: 27.1
  cooldown: 10.8
  duration: 14
  success: 38
  effects: Slow
  changes: Movement Speed -48%
- level: 9
  id: 989
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 16'
  learn_points: 75
  cost: MP 95
  power: 220
  range: 27.4
  cooldown: 11
  duration: 15
  success: 42
  effects: Slow
  changes: Movement Speed -52%
- level: 10
  id: 990
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 18'
  learn_points: 96
  cost: MP 110
  power: 250
  range: 27.7
  cooldown: 11.2
  duration: 20
  success: 50
  effects: Slow
  changes: Movement Speed -60%
source:
  data: LIST_SKILL.STB rows 981, 982, 983, 984, 985, 986, 987, 988, 989, 990
  code: module/src/skills.rs
---
# Ice Bolt

Shoot an ice bolt at a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
