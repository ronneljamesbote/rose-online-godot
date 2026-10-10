---
kind: skill
id: 2271
name: Poison Fang
status: in-game
icon: skill/197
type: Damage Action
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/821-poison-fang|Poison Fang]]'
levels:
- level: 1
  id: 2271
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 6'
  learn_points: 15
  cost: MP 40
  power: 80
  area: 8
  cooldown: 16
  duration: 16
  success: 40
  effects: Poisoned 2
- level: 2
  id: 2272
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 7'
  learn_points: 19
  cost: MP 44
  power: 90
  area: 8.5
  cooldown: 16.4
  duration: 17
  success: 42
  effects: Poisoned 2
- level: 3
  id: 2273
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 8'
  learn_points: 24
  cost: MP 48
  power: 100
  area: 9
  cooldown: 16.8
  duration: 18
  success: 44
  effects: Poisoned 2
- level: 4
  id: 2274
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 9'
  learn_points: 30
  cost: MP 52
  power: 110
  area: 9.5
  cooldown: 17.2
  duration: 19
  success: 46
  effects: Poisoned 2
- level: 5
  id: 2275
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10'
  learn_points: 37
  cost: MP 56
  power: 120
  area: 10
  cooldown: 17.6
  duration: 20
  success: 48
  effects: Poisoned 2
- level: 6
  id: 2276
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 45
  cost: MP 60
  power: 130
  area: 10.5
  cooldown: 18
  duration: 21
  success: 50
  effects: Poisoned  3
- level: 7
  id: 2277
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 55
  cost: MP 64
  power: 140
  area: 11
  cooldown: 18.4
  duration: 22
  success: 52
  effects: Poisoned  3
- level: 8
  id: 2278
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 13'
  learn_points: 67
  cost: MP 68
  power: 150
  area: 11.5
  cooldown: 18.8
  duration: 23
  success: 54
  effects: Poisoned  3
- level: 9
  id: 2279
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 14'
  learn_points: 81
  cost: MP 72
  power: 160
  area: 12
  cooldown: 19.2
  duration: 24
  success: 56
  effects: Poisoned  3
- level: 10
  id: 2280
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 15'
  learn_points: 98
  cost: MP 80
  power: 180
  area: 12.5
  cooldown: 19.6
  duration: 25
  success: 60
  effects: Poisoned  4
source:
  data: LIST_SKILL.STB rows 2271, 2272, 2273, 2274, 2275, 2276, 2277, 2278, 2279, 2280
  code: module/src/skills.rs
---
# Poison Fang

Shoot a poisonous bullet at a target to inflict damage and Poisoned status. Enemies near the target will be also poisoned at the same time.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
