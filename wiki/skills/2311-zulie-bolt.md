---
kind: skill
id: 2311
name: Zulie Bolt
status: in-game
icon: skill/198
type: Magic Spell
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/825-zulie-bolt|Zulie Bolt]]'
levels:
- level: 1
  id: 2311
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 8'
  learn_points: 20
  cost: Money 40, MP 10
  power: 110
  range: 25
  cooldown: 7
- level: 2
  id: 2312
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 9'
  learn_points: 24
  cost: Money 48, MP 11
  power: 130
  range: 25.4
  cooldown: 6.8
- level: 3
  id: 2313
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10'
  learn_points: 29
  cost: Money 56, MP 12
  power: 150
  range: 25.8
  cooldown: 6.6
- level: 4
  id: 2314
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 34
  cost: Money 64, MP 13
  power: 170
  range: 26.2
  cooldown: 6.4
- level: 5
  id: 2315
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 40
  cost: Money 72, MP 14
  power: 190
  range: 26.6
  cooldown: 6.2
- level: 6
  id: 2316
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 13'
  learn_points: 47
  cost: Money 80, MP 15
  power: 215
  range: 27
  cooldown: 6
- level: 7
  id: 2317
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 14'
  learn_points: 55
  cost: Money 90, MP 16
  power: 240
  range: 27.5
  cooldown: 5.8
- level: 8
  id: 2318
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 15'
  learn_points: 64
  cost: Money 100, MP 17
  power: 265
  range: 28
  cooldown: 5.6
- level: 9
  id: 2319
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 16'
  learn_points: 75
  cost: Money 110, MP 18
  power: 290
  range: 28.5
  cooldown: 5.4
- level: 10
  id: 2320
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 17'
  learn_points: 87
  cost: Money 130, MP 20
  power: 330
  range: 30
  cooldown: 5.2
source:
  data: LIST_SKILL.STB rows 2311, 2312, 2313, 2314, 2315, 2316, 2317, 2318, 2319, 2320
  code: module/src/skills.rs
---
# Zulie Bolt

Throw a coin at a target to cause damage from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
