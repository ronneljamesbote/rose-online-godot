---
kind: skill
id: 2471
name: Bow Craft
status: in-game
icon: skill/152
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/836-bow-craft|Bow Craft]]'
levels:
- level: 1
  id: 2471
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 14
  cost: MP 50
  power: 13
- level: 2
  id: 2472
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 3'
  learn_points: 16
  cost: MP 55
  power: 13
- level: 3
  id: 2473
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 5'
  learn_points: 18
  cost: MP 60
  power: 13
- level: 4
  id: 2474
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 7'
  learn_points: 20
  cost: MP 65
  power: 13
- level: 5
  id: 2475
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 9'
  learn_points: 22
  cost: MP 70
  power: 13
- level: 6
  id: 2476
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 24
  cost: MP 75
  power: 13
- level: 7
  id: 2477
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 12'
  learn_points: 26
  cost: MP 80
  power: 13
- level: 8
  id: 2478
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 13'
  learn_points: 28
  cost: MP 85
  power: 13
- level: 9
  id: 2479
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 30
  cost: MP 90
  power: 13
- level: 10
  id: 2480
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 33
  cost: MP 95
  power: 13
- level: 11
  id: 2481
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 36
  cost: MP 100
  power: 13
- level: 12
  id: 2482
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 39
  cost: MP 105
  power: 13
- level: 13
  id: 2483
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 42
  cost: MP 110
  power: 13
- level: 14
  id: 2484
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 45
  cost: MP 115
  power: 13
- level: 15
  id: 2485
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 48
  cost: MP 120
  power: 13
source:
  data: LIST_SKILL.STB rows 2471, 2472, 2473, 2474, 2475, 2476, 2477, 2478, 2479, 2480, 2481, 2482, 2483, 2484, 2485
  code: module/src/skills.rs
---
# Bow Craft

Create Bows or Crossbows.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
