---
kind: skill
id: 2301
name: Zulie Storm
status: in-game
icon: skill/184
type: Area Effect Attack (Self)
job: Bourgeois Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/824-zulie-storm|Zulie Storm]]'
levels:
- level: 1
  id: 2301
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10, [[skills/2311-zulie-bolt|Zulie Bolt]] level 5'
  learn_points: 40
  cost: Money 100, MP 20
  power: 160
  area: 13
  cooldown: 16
- level: 2
  id: 2302
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 45
  cost: Money 120, MP 22
  power: 180
  area: 13.3
  cooldown: 16.4
- level: 3
  id: 2303
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 51
  cost: Money 140, MP 24
  power: 200
  area: 13.6
  cooldown: 16.8
- level: 4
  id: 2304
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 13'
  learn_points: 57
  cost: Money 160, MP 26
  power: 220
  area: 13.9
  cooldown: 17.2
- level: 5
  id: 2305
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 14'
  learn_points: 64
  cost: Money 180, MP 28
  power: 240
  area: 14.2
  cooldown: 17.6
- level: 6
  id: 2306
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 15'
  learn_points: 71
  cost: Money 200, MP 30
  power: 260
  area: 14.5
  cooldown: 18
- level: 7
  id: 2307
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 16'
  learn_points: 79
  cost: Money 220, MP 32
  power: 280
  area: 14.8
  cooldown: 18.4
- level: 8
  id: 2308
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 17'
  learn_points: 88
  cost: Money 240, MP 34
  power: 300
  area: 15.1
  cooldown: 18.8
- level: 9
  id: 2309
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 18'
  learn_points: 98
  cost: Money 260, MP 36
  power: 320
  area: 15.4
  cooldown: 19.2
- level: 10
  id: 2310
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 19'
  learn_points: 109
  cost: Money 300, MP 40
  power: 350
  area: 15.7
  cooldown: 19.6
source:
  data: LIST_SKILL.STB rows 2301, 2302, 2303, 2304, 2305, 2306, 2307, 2308, 2309, 2310
  code: module/src/skills.rs
---
# Zulie Storm

Throw many coins to inflict damage on enemies near the caster.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
