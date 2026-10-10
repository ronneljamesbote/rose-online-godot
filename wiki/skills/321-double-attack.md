---
kind: skill
id: 321
name: Double Attack
status: in-game
icon: skill/32
type: Damage Action
job: Soldier Job
max_level: 20
target: Hostile Character
damage_type: continuous attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/114-sandwich|Sandwich]]'
- '[[items/consumable/660-double-attack|Double Attack]]'
levels:
- level: 1
  id: 321
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 1'
  learn_points: 8
  cost: MP 20
  power: 40
  cooldown: 5.4
- level: 2
  id: 322
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 2'
  learn_points: 10
  cost: MP 22
  power: 45
  cooldown: 5.2
- level: 3
  id: 323
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 3'
  learn_points: 12
  cost: MP 24
  power: 50
  cooldown: 5
- level: 4
  id: 324
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 4'
  learn_points: 14
  cost: MP 26
  power: 55
  cooldown: 4.8
- level: 5
  id: 325
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 5'
  learn_points: 16
  cost: MP 28
  power: 60
  cooldown: 4.6
- level: 6
  id: 326
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 6'
  learn_points: 18
  cost: MP 30
  power: 65
  cooldown: 4.4
- level: 7
  id: 327
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 7'
  learn_points: 20
  cost: MP 32
  power: 70
  cooldown: 4.2
- level: 8
  id: 328
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 8'
  learn_points: 23
  cost: MP 34
  power: 75
  cooldown: 4
- level: 9
  id: 329
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 9'
  learn_points: 26
  cost: MP 36
  power: 82
  cooldown: 3.8
- level: 10
  id: 330
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 10'
  learn_points: 29
  cost: MP 38
  power: 90
  cooldown: 3.6
- level: 11
  id: 331
  needs_level: 70
  learn_points: 32
  cost: MP 40
  power: 45
  cooldown: 6
- level: 12
  id: 332
  learn_points: 35
  cost: MP 42
  power: 50
  cooldown: 6
- level: 13
  id: 333
  learn_points: 38
  cost: MP 44
  power: 55
  cooldown: 5.8
- level: 14
  id: 334
  learn_points: 42
  cost: MP 46
  power: 60
  cooldown: 5.8
- level: 15
  id: 335
  learn_points: 46
  cost: MP 48
  power: 65
  cooldown: 5.6
- level: 16
  id: 336
  learn_points: 50
  cost: MP 50
  power: 70
  cooldown: 5.6
- level: 17
  id: 337
  learn_points: 55
  cost: MP 52
  power: 75
  cooldown: 5.4
- level: 18
  id: 338
  learn_points: 60
  cost: MP 54
  power: 80
  cooldown: 5.2
- level: 19
  id: 339
  learn_points: 65
  cost: MP 56
  power: 88
  cooldown: 5
  duration: 12
  success: 15
  effects: Dodge Rate Down
  changes: Dodge Rate -30 -15%
- level: 20
  id: 340
  learn_points: 71
  cost: MP 60
  power: 100
  cooldown: 4.8
  duration: 15
  success: 20
  effects: Dodge Rate Down
  changes: Dodge Rate -40 -20%
source:
  data: LIST_SKILL.STB rows 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340
  code: module/src/skills.rs
---
# Double Attack

Deliver double blows onto a target while using a Melee Weapon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
