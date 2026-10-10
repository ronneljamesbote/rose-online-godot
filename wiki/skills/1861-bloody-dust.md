---
kind: skill
id: 1861
name: Bloody Dust
status: in-game
icon: skill/117
type: Area Effect Attack (Self)
job: Raider Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/791-bloody-dust|Bloody Dust]]'
levels:
- level: 1
  id: 1861
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 8'
  learn_points: 35
  cost: MP 70
  power: 60
  area: 14
  cooldown: 16
  duration: 16
  success: 46
  effects: Dodge Rate Down
  changes: Dodge Rate -30%
- level: 2
  id: 1862
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 9'
  learn_points: 41
  cost: MP 75
  power: 68
  area: 14
  cooldown: 16.4
  duration: 18
  success: 48
  effects: Dodge Rate Down
  changes: Dodge Rate -31%
- level: 3
  id: 1863
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 10'
  learn_points: 47
  cost: MP 80
  power: 76
  area: 14
  cooldown: 16.8
  duration: 20
  success: 50
  effects: Dodge Rate Down
  changes: Dodge Rate -32%
- level: 4
  id: 1864
  learn_points: 54
  cost: MP 85
  power: 84
  area: 14
  cooldown: 17.2
  duration: 22
  success: 52
  effects: Dodge Rate Down
  changes: Dodge Rate -33%
- level: 5
  id: 1865
  learn_points: 62
  cost: MP 90
  power: 92
  area: 14
  cooldown: 17.6
  duration: 24
  success: 54
  effects: Dodge Rate Down
  changes: Dodge Rate -34%
- level: 6
  id: 1866
  learn_points: 71
  cost: MP 95
  power: 100
  area: 14
  cooldown: 18
  duration: 26
  success: 56
  effects: Dodge Rate Down
  changes: Dodge Rate -35%
- level: 7
  id: 1867
  learn_points: 81
  cost: MP 100
  power: 108
  area: 14
  cooldown: 18.4
  duration: 28
  success: 58
  effects: Dodge Rate Down
  changes: Dodge Rate -36%
- level: 8
  id: 1868
  learn_points: 93
  cost: MP 105
  power: 116
  area: 14
  cooldown: 18.8
  duration: 30
  success: 60
  effects: Dodge Rate Down
  changes: Dodge Rate -37%
- level: 9
  id: 1869
  learn_points: 106
  cost: MP 110
  power: 124
  area: 14
  cooldown: 19.2
  duration: 32
  success: 62
  effects: Dodge Rate Down
  changes: Dodge Rate -38%
- level: 10
  id: 1870
  learn_points: 121
  cost: MP 120
  power: 140
  area: 14
  cooldown: 19.6
  duration: 34
  success: 64
  effects: Dodge Rate Down
  changes: Dodge Rate -40%
source:
  data: LIST_SKILL.STB rows 1861, 1862, 1863, 1864, 1865, 1866, 1867, 1868, 1869, 1870
  code: module/src/skills.rs
---
# Bloody Dust

Raise dust around the caster to decrease Dodge Rate of nearby enemies.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
