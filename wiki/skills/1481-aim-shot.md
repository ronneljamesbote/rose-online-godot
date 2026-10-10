---
kind: skill
id: 1481
name: Aim Shot
status: in-game
icon: skill/106
type: Damage Action
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Bow
skill_books:
- '[[items/consumable/756-aim-shot|Aim Shot]]'
levels:
- level: 1
  id: 1481
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 1'
  learn_points: 8
  cost: MP 10
  power: 30
  cooldown: 6
- level: 2
  id: 1482
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 2'
  learn_points: 10
  cost: MP 12
  power: 37
  cooldown: 6
- level: 3
  id: 1483
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 3'
  learn_points: 12
  cost: MP 14
  power: 44
  cooldown: 6.2
- level: 4
  id: 1484
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 4'
  learn_points: 14
  cost: MP 16
  power: 51
  cooldown: 6.2
- level: 5
  id: 1485
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 5'
  learn_points: 16
  cost: MP 18
  power: 58
  cooldown: 6.4
- level: 6
  id: 1486
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 6'
  learn_points: 18
  cost: MP 20
  power: 65
  cooldown: 6.4
- level: 7
  id: 1487
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 7'
  learn_points: 20
  cost: MP 22
  power: 72
  cooldown: 6.6
- level: 8
  id: 1488
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 8'
  learn_points: 23
  cost: MP 24
  power: 80
  cooldown: 6.6
- level: 9
  id: 1489
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 9'
  learn_points: 26
  cost: MP 26
  power: 88
  cooldown: 6.8
- level: 10
  id: 1490
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10'
  learn_points: 29
  cost: MP 30
  power: 100
  cooldown: 6.8
source:
  data: LIST_SKILL.STB rows 1481, 1482, 1483, 1484, 1485, 1486, 1487, 1488, 1489, 1490
  code: module/src/skills.rs
---
# Aim Shot

Shoot an arrow at a target with great force.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
