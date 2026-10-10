---
kind: skill
id: 2261
name: Aim Sight
status: in-game
icon: skill/222
type: Damage Action
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/820-aim-sight|Aim Sight]]'
levels:
- level: 1
  id: 2261
  needs_level: 70
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10, [[skills/2241-intensify-weapon|Intensify Weapon]] level 5'
  learn_points: 35
  cost: MP 40
  power: 100
  range: 37
  cooldown: 13
- level: 2
  id: 2262
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 40
  cost: MP 44
  power: 115
  range: 37.5
  cooldown: 12.6
- level: 3
  id: 2263
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 46
  cost: MP 48
  power: 130
  range: 38
  cooldown: 12.2
- level: 4
  id: 2264
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 13'
  learn_points: 53
  cost: MP 52
  power: 145
  range: 38.5
  cooldown: 11.8
- level: 5
  id: 2265
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 14'
  learn_points: 60
  cost: MP 56
  power: 160
  range: 39
  cooldown: 11.4
- level: 6
  id: 2266
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 15'
  learn_points: 68
  cost: MP 60
  power: 175
  range: 39.5
  cooldown: 11
- level: 7
  id: 2267
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 16'
  learn_points: 77
  cost: MP 64
  power: 190
  range: 40
  cooldown: 10.6
- level: 8
  id: 2268
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 17'
  learn_points: 87
  cost: MP 68
  power: 205
  range: 40.5
  cooldown: 10.2
- level: 9
  id: 2269
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 18'
  learn_points: 98
  cost: MP 72
  power: 220
  range: 41
  cooldown: 9.8
- level: 10
  id: 2270
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 19'
  learn_points: 111
  cost: MP 80
  power: 240
  range: 42
  cooldown: 9.4
source:
  data: LIST_SKILL.STB rows 2261, 2262, 2263, 2264, 2265, 2266, 2267, 2268, 2269, 2270
  code: module/src/skills.rs
---
# Aim Sight

Shoot a target from a considerably long distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
