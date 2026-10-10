---
kind: skill
id: 2491
name: Magic Weapon Craft
status: in-game
icon: skill/153
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/837-magic-weapon-craft|Magic Weapon Craft]]'
levels:
- level: 1
  id: 2491
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 14
  cost: MP 50
  power: 15
- level: 2
  id: 2492
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 3'
  learn_points: 16
  cost: MP 55
  power: 15
- level: 3
  id: 2493
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 5'
  learn_points: 18
  cost: MP 60
  power: 15
- level: 4
  id: 2494
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 7'
  learn_points: 20
  cost: MP 65
  power: 15
- level: 5
  id: 2495
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 9'
  learn_points: 22
  cost: MP 70
  power: 15
- level: 6
  id: 2496
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 24
  cost: MP 75
  power: 15
- level: 7
  id: 2497
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 12'
  learn_points: 26
  cost: MP 80
  power: 15
- level: 8
  id: 2498
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 13'
  learn_points: 28
  cost: MP 85
  power: 15
- level: 9
  id: 2499
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 30
  cost: MP 90
  power: 15
- level: 10
  id: 2500
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 33
  cost: MP 95
  power: 15
- level: 11
  id: 2501
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 36
  cost: MP 100
  power: 15
- level: 12
  id: 2502
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 39
  cost: MP 105
  power: 15
- level: 13
  id: 2503
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 42
  cost: MP 110
  power: 15
- level: 14
  id: 2504
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 45
  cost: MP 115
  power: 15
- level: 15
  id: 2505
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 48
  cost: MP 120
  power: 15
source:
  data: LIST_SKILL.STB rows 2491, 2492, 2493, 2494, 2495, 2496, 2497, 2498, 2499, 2500, 2501, 2502, 2503, 2504, 2505
  code: module/src/skills.rs
---
# Magic Weapon Craft

Create Magic Weapons such as Staffs or Wands.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
