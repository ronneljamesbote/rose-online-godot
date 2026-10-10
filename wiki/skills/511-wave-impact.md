---
kind: skill
id: 511
name: Wave Impact
status: in-game
icon: skill/47
type: Area Effect Attack (Self)
job: Knight Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/676-wave-impact|Wave Impact]]'
levels:
- level: 1
  id: 511
  needs: '[[skills/341-leap-attack|Leap Attack]] level 10'
  learn_points: 35
  cost: MP 70
  power: 120
  area: 7
  cooldown: 14
- level: 2
  id: 512
  learn_points: 41
  cost: MP 75
  power: 132
  area: 7
  cooldown: 14.6
- level: 3
  id: 513
  learn_points: 48
  cost: MP 80
  power: 144
  area: 7
  cooldown: 15.2
- level: 4
  id: 514
  learn_points: 56
  cost: MP 85
  power: 156
  area: 7
  cooldown: 15.8
- level: 5
  id: 515
  learn_points: 65
  cost: MP 90
  power: 168
  area: 7
  cooldown: 16.4
- level: 6
  id: 516
  learn_points: 75
  cost: MP 95
  power: 180
  area: 7
  cooldown: 17
- level: 7
  id: 517
  learn_points: 87
  cost: MP 100
  power: 192
  area: 7
  cooldown: 17.6
- level: 8
  id: 518
  learn_points: 100
  cost: MP 105
  power: 204
  area: 7
  cooldown: 18.2
- level: 9
  id: 519
  learn_points: 115
  cost: MP 110
  power: 216
  area: 7
  cooldown: 18.8
- level: 10
  id: 520
  learn_points: 132
  cost: MP 120
  power: 240
  area: 8
  cooldown: 19.4
source:
  data: LIST_SKILL.STB rows 511, 512, 513, 514, 515, 516, 517, 518, 519, 520
  code: module/src/skills.rs
---
# Wave Impact

Thrust a sword into the ground and crack the ground’s surface to do splash damage to many foes.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
