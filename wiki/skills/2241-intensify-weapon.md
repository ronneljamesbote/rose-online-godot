---
kind: skill
id: 2241
name: Intensify Weapon
status: in-game
icon: skill/176
type: Continuing (Self)
job: Dealer Job
max_level: 10
target: Yourself
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/818-intensify-weapon|Intensify Weapon]]'
levels:
- level: 1
  id: 2241
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 3'
  learn_points: 10
  cost: MP 30
  cooldown: 130
  duration: 100
  effects: Haste Attack
  changes: Attack Speed +24
- level: 2
  id: 2242
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 4'
  learn_points: 13
  cost: MP 32
  cooldown: 132
  duration: 102
  effects: Haste Attack
  changes: Attack Speed +28
- level: 3
  id: 2243
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 5'
  learn_points: 16
  cost: MP 34
  cooldown: 134
  duration: 104
  effects: Haste Attack
  changes: Attack Speed +32
- level: 4
  id: 2244
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 6'
  learn_points: 20
  cost: MP 36
  cooldown: 136
  duration: 106
  effects: Haste Attack
  changes: Attack Speed +36
- level: 5
  id: 2245
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 7'
  learn_points: 25
  cost: MP 38
  cooldown: 138
  duration: 108
  effects: Haste Attack
  changes: Attack Speed +40
- level: 6
  id: 2246
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 8'
  learn_points: 31
  cost: MP 40
  cooldown: 140
  duration: 110
  effects: Haste Attack
  changes: Attack Speed +44
- level: 7
  id: 2247
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 9'
  learn_points: 38
  cost: MP 42
  cooldown: 142
  duration: 112
  effects: Haste Attack
  changes: Attack Speed +48
- level: 8
  id: 2248
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10'
  learn_points: 46
  cost: MP 44
  cooldown: 144
  duration: 114
  effects: Haste Attack
  changes: Attack Speed +52
- level: 9
  id: 2249
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 55
  cost: MP 46
  cooldown: 146
  duration: 116
  effects: Haste Attack
  changes: Attack Speed +56
- level: 10
  id: 2250
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 66
  cost: MP 48
  cooldown: 148
  duration: 120
  effects: Haste Attack
  changes: Attack Speed +60
source:
  data: LIST_SKILL.STB rows 2241, 2242, 2243, 2244, 2245, 2246, 2247, 2248, 2249, 2250
  code: module/src/skills.rs
---
# Intensify Weapon

Increase the caster's Attack Speed for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
