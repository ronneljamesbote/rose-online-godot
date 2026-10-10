---
kind: skill
id: 1511
name: Power Burst
status: in-game
icon: skill/215
type: Damage Action
job: Raider Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/767-power-burst|Power Burst]]'
levels:
- level: 1
  id: 1511
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 11, [[skills/1501-power-attack|Power Attack]] level 10'
  learn_points: 30
  cost: MP 50
  power: 120
  cooldown: 8
- level: 2
  id: 1512
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 12'
  learn_points: 35
  cost: MP 55
  power: 140
  cooldown: 8.4
- level: 3
  id: 1513
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 13'
  learn_points: 41
  cost: MP 60
  power: 160
  cooldown: 8.8
- level: 4
  id: 1514
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 14'
  learn_points: 48
  cost: MP 65
  power: 180
  cooldown: 9.2
- level: 5
  id: 1515
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 15'
  learn_points: 56
  cost: MP 70
  power: 200
  cooldown: 9.6
- level: 6
  id: 1516
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 16'
  learn_points: 65
  cost: MP 75
  power: 220
  cooldown: 10
- level: 7
  id: 1517
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 17'
  learn_points: 75
  cost: MP 80
  power: 240
  cooldown: 10.4
- level: 8
  id: 1518
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 18'
  learn_points: 87
  cost: MP 85
  power: 260
  cooldown: 10.8
- level: 9
  id: 1519
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 19'
  learn_points: 100
  cost: MP 90
  power: 280
  cooldown: 11.2
- level: 10
  id: 1520
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 20'
  learn_points: 115
  cost: MP 100
  power: 310
  cooldown: 11.6
source:
  data: LIST_SKILL.STB rows 1511, 1512, 1513, 1514, 1515, 1516, 1517, 1518, 1519, 1520
  code: module/src/skills.rs
---
# Power Burst

Advanced Power Attack skill. Hit a target with awesome might while using a Knuckle or a Melee Weapon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
