---
kind: skill
id: 2511
name: Gun Craft
status: in-game
icon: skill/154
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/838-gun-craft|Gun Craft]]'
levels:
- level: 1
  id: 2511
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 15
  cost: MP 50
  power: 14
- level: 2
  id: 2512
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 2'
  learn_points: 17
  cost: MP 55
  power: 14
- level: 3
  id: 2513
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 4'
  learn_points: 19
  cost: MP 60
  power: 14
- level: 4
  id: 2514
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 6'
  learn_points: 21
  cost: MP 65
  power: 14
- level: 5
  id: 2515
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 8'
  learn_points: 23
  cost: MP 70
  power: 14
- level: 6
  id: 2516
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 10'
  learn_points: 25
  cost: MP 75
  power: 14
- level: 7
  id: 2517
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 28
  cost: MP 80
  power: 14
- level: 8
  id: 2518
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 13'
  learn_points: 31
  cost: MP 85
  power: 14
- level: 9
  id: 2519
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 34
  cost: MP 90
  power: 14
- level: 10
  id: 2520
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 37
  cost: MP 95
  power: 14
- level: 11
  id: 2521
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 40
  cost: MP 100
  power: 14
- level: 12
  id: 2522
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 43
  cost: MP 105
  power: 14
- level: 13
  id: 2523
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 47
  cost: MP 110
  power: 14
- level: 14
  id: 2524
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 51
  cost: MP 115
  power: 14
- level: 15
  id: 2525
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 55
  cost: MP 120
  power: 14
source:
  data: LIST_SKILL.STB rows 2511, 2512, 2513, 2514, 2515, 2516, 2517, 2518, 2519, 2520, 2521, 2522, 2523, 2524, 2525
  code: module/src/skills.rs
---
# Gun Craft

Create Guns or Launchers.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
