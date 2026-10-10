---
kind: skill
id: 481
name: Heavy Bow Shot
status: in-game
icon: skill/58
type: Damage Action
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Crossbow
skill_books:
- '[[items/consumable/673-heavy-bow-shot|Heavy Bow Shot]]'
levels:
- level: 1
  id: 481
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 1'
  learn_points: 10
  cost: MP 20
  power: 60
  cooldown: 8
- level: 2
  id: 482
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 2'
  learn_points: 13
  cost: MP 24
  power: 70
  cooldown: 8.2
- level: 3
  id: 483
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 3'
  learn_points: 16
  cost: MP 28
  power: 80
  cooldown: 8.4
- level: 4
  id: 484
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 4'
  learn_points: 20
  cost: MP 32
  power: 90
  cooldown: 8.6
- level: 5
  id: 485
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 5'
  learn_points: 25
  cost: MP 36
  power: 100
  cooldown: 8.8
- level: 6
  id: 486
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 6'
  learn_points: 31
  cost: MP 40
  power: 110
  cooldown: 9
- level: 7
  id: 487
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 7'
  learn_points: 38
  cost: MP 44
  power: 120
  cooldown: 9.2
- level: 8
  id: 488
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 8'
  learn_points: 46
  cost: MP 48
  power: 130
  cooldown: 9.4
- level: 9
  id: 489
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 9'
  learn_points: 55
  cost: MP 52
  power: 140
  cooldown: 9.6
- level: 10
  id: 490
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 10'
  learn_points: 66
  cost: MP 60
  power: 160
  cooldown: 9.8
source:
  data: LIST_SKILL.STB rows 481, 482, 483, 484, 485, 486, 487, 488, 489, 490
  code: module/src/skills.rs
---
# Heavy Bow Shot

Crossbow required. By focusing your energy before firing an arrow, you can inflict heavy damage on an enemy.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
