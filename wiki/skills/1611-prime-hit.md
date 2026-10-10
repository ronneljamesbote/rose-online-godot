---
kind: skill
id: 1611
name: Prime Hit
status: in-game
icon: skill/216
type: Damage Action
job: Raider Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/765-prime-hit|Prime Hit]]'
levels:
- level: 1
  id: 1611
  needs: '[[skills/1591-screw-attack|Screw Attack]] level 10'
  learn_points: 40
  cost: MP 70
  power: 220
  range: 4
  cooldown: 14
- level: 2
  id: 1612
  learn_points: 46
  cost: MP 75
  power: 242
  range: 4.2
  cooldown: 14.4
- level: 3
  id: 1613
  learn_points: 53
  cost: MP 80
  power: 264
  range: 4.4
  cooldown: 14.8
- level: 4
  id: 1614
  learn_points: 61
  cost: MP 85
  power: 286
  range: 4.6
  cooldown: 15.2
- level: 5
  id: 1615
  learn_points: 70
  cost: MP 90
  power: 310
  range: 4.8
  cooldown: 15.6
- level: 6
  id: 1616
  learn_points: 80
  cost: MP 95
  power: 334
  range: 5
  cooldown: 16
- level: 7
  id: 1617
  learn_points: 91
  cost: MP 100
  power: 358
  range: 5.2
  cooldown: 16.4
- level: 8
  id: 1618
  learn_points: 104
  cost: MP 106
  power: 382
  range: 5.4
  cooldown: 16.8
- level: 9
  id: 1619
  learn_points: 119
  cost: MP 112
  power: 406
  range: 5.6
  cooldown: 17.2
- level: 10
  id: 1620
  learn_points: 135
  cost: MP 120
  power: 440
  range: 5.8
  cooldown: 17.6
source:
  data: LIST_SKILL.STB rows 1611, 1612, 1613, 1614, 1615, 1616, 1617, 1618, 1619, 1620
  code: module/src/skills.rs
---
# Prime Hit

Inflict a great amount of damage to a nearby enemy.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
