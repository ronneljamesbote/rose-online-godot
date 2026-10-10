---
kind: skill
id: 401
name: Spin Attack
status: in-game
icon: skill/40
type: Area Effect Attack (Self)
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/666-spin-attack|Spin Attack]]'
levels:
- level: 1
  id: 401
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 6'
  learn_points: 20
  cost: MP 50
  power: 80
  area: 7
  cooldown: 16
- level: 2
  id: 402
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 7'
  learn_points: 24
  cost: MP 54
  power: 90
  area: 7
  cooldown: 16.4
- level: 3
  id: 403
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 8'
  learn_points: 29
  cost: MP 58
  power: 100
  area: 7
  cooldown: 16.8
- level: 4
  id: 404
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 9'
  learn_points: 35
  cost: MP 62
  power: 110
  area: 7
  cooldown: 17.2
- level: 5
  id: 405
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 10'
  learn_points: 42
  cost: MP 66
  power: 120
  area: 7
  cooldown: 17.6
- level: 6
  id: 406
  learn_points: 51
  cost: MP 72
  power: 135
  area: 9
  cooldown: 18
- level: 7
  id: 407
  learn_points: 62
  cost: MP 78
  power: 150
  area: 9
  cooldown: 18.4
- level: 8
  id: 408
  learn_points: 75
  cost: MP 84
  power: 165
  area: 9.5
  cooldown: 18.8
- level: 9
  id: 409
  learn_points: 91
  cost: MP 90
  power: 180
  area: 9.5
  cooldown: 19.2
- level: 10
  id: 410
  learn_points: 110
  cost: MP 100
  power: 200
  area: 10
  cooldown: 19.6
source:
  data: LIST_SKILL.STB rows 401, 402, 403, 404, 405, 406, 407, 408, 409, 410
  code: module/src/skills.rs
---
# Spin Attack

Enables its user to spin and attack at the same time when equipping a Two-Handed Weapon. Also does splashed damage to nearby enemies.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
