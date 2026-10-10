---
kind: skill
id: 951
name: Lightning
status: in-game
icon: skill/101
type: Magic Spell
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/712-lightning|Lightning]]'
levels:
- level: 1
  id: 951
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 6'
  learn_points: 12
  cost: MP 30
  power: 30
  range: 22
  cooldown: 12
  duration: 10
  success: 50
  effects: Sleep
- level: 2
  id: 952
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 7'
  learn_points: 15
  cost: MP 32
  power: 40
  range: 22.5
  cooldown: 12
  duration: 12
  success: 55
  effects: Sleep
- level: 3
  id: 953
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 8'
  learn_points: 19
  cost: MP 34
  power: 50
  range: 23
  cooldown: 12
  duration: 14
  success: 60
  effects: Sleep
- level: 4
  id: 954
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 9'
  learn_points: 24
  cost: MP 36
  power: 60
  range: 23.5
  cooldown: 12
  duration: 15
  success: 65
  effects: Sleep
- level: 5
  id: 955
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10'
  learn_points: 30
  cost: MP 38
  power: 70
  range: 24
  cooldown: 12
  duration: 16
  success: 70
  effects: Sleep
- level: 6
  id: 956
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 11'
  learn_points: 37
  cost: MP 60
  power: 100
  range: 24.5
  cooldown: 16
  duration: 5
  success: 50
  effects: Fainted
- level: 7
  id: 957
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 13'
  learn_points: 45
  cost: MP 67
  power: 120
  range: 25
  cooldown: 16.4
  duration: 5
  success: 53
  effects: Fainted
- level: 8
  id: 958
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 15'
  learn_points: 55
  cost: MP 74
  power: 140
  range: 25.5
  cooldown: 16.8
  duration: 6
  success: 56
  effects: Fainted
- level: 9
  id: 959
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 17'
  learn_points: 67
  cost: MP 81
  power: 160
  range: 26
  cooldown: 17.2
  duration: 6
  success: 59
  effects: Fainted
- level: 10
  id: 960
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 19'
  learn_points: 81
  cost: MP 95
  power: 200
  range: 26.5
  area: 5
  cooldown: 17.6
  duration: 7
  success: 65
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 951, 952, 953, 954, 955, 956, 957, 958, 959, 960
  code: module/src/skills.rs
---
# Lightning

Cast Lightning on a target to stun it.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
