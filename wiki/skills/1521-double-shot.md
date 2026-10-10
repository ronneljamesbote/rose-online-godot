---
kind: skill
id: 1521
name: Double Shot
status: in-game
icon: skill/109
type: Damage Action
job: Hawker Job
max_level: 20
target: Hostile Character
damage_type: continuous attack
needs_weapon: Bow
skill_books:
- '[[items/consumable/758-double-shot|Double Shot]]'
levels:
- level: 1
  id: 1521
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 1'
  learn_points: 8
  cost: MP 20
  power: 40
  cooldown: 6
- level: 2
  id: 1522
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 2'
  learn_points: 10
  cost: MP 22
  power: 45
  cooldown: 6
- level: 3
  id: 1523
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 3'
  learn_points: 12
  cost: MP 24
  power: 50
  cooldown: 5.8
- level: 4
  id: 1524
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 4'
  learn_points: 14
  cost: MP 26
  power: 55
  cooldown: 5.8
- level: 5
  id: 1525
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 5'
  learn_points: 16
  cost: MP 28
  power: 60
  cooldown: 5.6
- level: 6
  id: 1526
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 6'
  learn_points: 18
  cost: MP 30
  power: 65
  cooldown: 5.6
- level: 7
  id: 1527
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 7'
  learn_points: 20
  cost: MP 32
  power: 70
  cooldown: 5.4
- level: 8
  id: 1528
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 8'
  learn_points: 23
  cost: MP 34
  power: 75
  cooldown: 5.4
- level: 9
  id: 1529
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 9'
  learn_points: 26
  cost: MP 36
  power: 82
  cooldown: 5.2
- level: 10
  id: 1530
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10'
  learn_points: 29
  cost: MP 38
  power: 90
  cooldown: 5.2
- level: 11
  id: 1531
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 11'
  learn_points: 32
  cost: MP 40
  power: 45
  cooldown: 7
- level: 12
  id: 1532
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 36
  cost: MP 42
  power: 50
  cooldown: 7
- level: 13
  id: 1533
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 13'
  learn_points: 40
  cost: MP 44
  power: 55
  cooldown: 6.8
- level: 14
  id: 1534
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 44
  cost: MP 46
  power: 60
  cooldown: 6.8
- level: 15
  id: 1535
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 15'
  learn_points: 49
  cost: MP 48
  power: 65
  cooldown: 6.6
- level: 16
  id: 1536
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 54
  cost: MP 50
  power: 70
  cooldown: 6.6
- level: 17
  id: 1537
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 17'
  learn_points: 59
  cost: MP 52
  power: 75
  cooldown: 6.4
- level: 18
  id: 1538
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 18'
  learn_points: 65
  cost: MP 54
  power: 80
  cooldown: 6.4
- level: 19
  id: 1539
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 19'
  learn_points: 71
  cost: MP 56
  power: 88
  cooldown: 6.2
  duration: 10
  success: 15
  effects: Slow
  changes: Movement Speed -30%
- level: 20
  id: 1540
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 20'
  learn_points: 78
  cost: MP 60
  power: 100
  cooldown: 6.2
  duration: 15
  success: 20
  effects: Slow
  changes: Movement Speed -50%
source:
  data: LIST_SKILL.STB rows 1521, 1522, 1523, 1524, 1525, 1526, 1527, 1528, 1529, 1530, 1531, 1532, 1533, 1534, 1535, 1536, 1537, 1538, 1539, 1540
  code: module/src/skills.rs
---
# Double Shot

Shoot 2 arrows at a target at once.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
