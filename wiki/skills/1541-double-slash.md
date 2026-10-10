---
kind: skill
id: 1541
name: Double Slash
status: in-game
icon: skill/110
type: Damage Action
job: Hawker Job
max_level: 20
target: Hostile Character
damage_type: continuous attack
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/759-double-slash|Double Slash]]'
levels:
- level: 1
  id: 1541
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 1'
  learn_points: 8
  cost: MP 16
  power: 40
  cooldown: 5.4
- level: 2
  id: 1542
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 2'
  learn_points: 10
  cost: MP 18
  power: 45
  cooldown: 5.2
- level: 3
  id: 1543
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 3'
  learn_points: 12
  cost: MP 20
  power: 50
  cooldown: 5
- level: 4
  id: 1544
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 4'
  learn_points: 14
  cost: MP 22
  power: 55
  cooldown: 4.8
- level: 5
  id: 1545
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 5'
  learn_points: 16
  cost: MP 24
  power: 60
  cooldown: 4.6
- level: 6
  id: 1546
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 6'
  learn_points: 19
  cost: MP 26
  power: 65
  cooldown: 4.4
- level: 7
  id: 1547
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 7'
  learn_points: 22
  cost: MP 28
  power: 70
  cooldown: 4.2
- level: 8
  id: 1548
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 8'
  learn_points: 25
  cost: MP 30
  power: 75
  cooldown: 4
- level: 9
  id: 1549
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 9'
  learn_points: 29
  cost: MP 32
  power: 82
  cooldown: 3.8
- level: 10
  id: 1550
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 10'
  learn_points: 33
  cost: MP 34
  power: 90
  cooldown: 3.6
- level: 11
  id: 1551
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 11'
  learn_points: 37
  cost: MP 35
  power: 45
  cooldown: 6
- level: 12
  id: 1552
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 12'
  learn_points: 41
  cost: MP 37
  power: 50
  cooldown: 6
- level: 13
  id: 1553
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 13'
  learn_points: 45
  cost: MP 39
  power: 55
  cooldown: 5.8
- level: 14
  id: 1554
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 14'
  learn_points: 49
  cost: MP 41
  power: 60
  cooldown: 5.8
- level: 15
  id: 1555
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 15'
  learn_points: 53
  cost: MP 43
  power: 65
  cooldown: 5.6
- level: 16
  id: 1556
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 16'
  learn_points: 58
  cost: MP 45
  power: 70
  cooldown: 5.6
- level: 17
  id: 1557
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 17'
  learn_points: 63
  cost: MP 47
  power: 75
  cooldown: 5.4
- level: 18
  id: 1558
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 18'
  learn_points: 68
  cost: MP 49
  power: 80
  cooldown: 5.2
- level: 19
  id: 1559
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 19'
  learn_points: 74
  cost: MP 51
  power: 88
  cooldown: 5
  duration: 4
  success: 15
  effects: Fainted
- level: 20
  id: 1560
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 20'
  learn_points: 80
  cost: MP 55
  power: 100
  cooldown: 4.8
  duration: 6
  success: 20
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 1541, 1542, 1543, 1544, 1545, 1546, 1547, 1548, 1549, 1550, 1551, 1552, 1553, 1554, 1555, 1556, 1557, 1558, 1559, 1560
  code: module/src/skills.rs
---
# Double Slash

Deliver 2 blows at a target at once.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
