---
kind: skill
id: 2411
name: SubItem Craft
status: in-game
icon: skill/195
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/833-subitem-craft|SubItem Craft]]'
levels:
- level: 1
  id: 2411
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 12
  cost: MP 50
  power: 16
- level: 2
  id: 2412
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 2'
  learn_points: 14
  cost: MP 55
  power: 16
- level: 3
  id: 2413
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 4'
  learn_points: 16
  cost: MP 60
  power: 16
- level: 4
  id: 2414
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 6'
  learn_points: 18
  cost: MP 65
  power: 16
- level: 5
  id: 2415
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 8'
  learn_points: 20
  cost: MP 70
  power: 16
- level: 6
  id: 2416
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 10'
  learn_points: 22
  cost: MP 75
  power: 16
- level: 7
  id: 2417
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 24
  cost: MP 80
  power: 16
- level: 8
  id: 2418
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 13'
  learn_points: 26
  cost: MP 85
  power: 16
- level: 9
  id: 2419
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 28
  cost: MP 90
  power: 16
- level: 10
  id: 2420
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 30
  cost: MP 95
  power: 16
- level: 11
  id: 2421
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 33
  cost: MP 100
  power: 16
- level: 12
  id: 2422
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 36
  cost: MP 105
  power: 16
- level: 13
  id: 2423
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 39
  cost: MP 110
  power: 16
- level: 14
  id: 2424
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 42
  cost: MP 115
  power: 16
- level: 15
  id: 2425
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 45
  cost: MP 120
  power: 16
source:
  data: LIST_SKILL.STB rows 2411, 2412, 2413, 2414, 2415, 2416, 2417, 2418, 2419, 2420, 2421, 2422, 2423, 2424, 2425
  code: module/src/skills.rs
---
# SubItem Craft

Craft Shields, Bags, Back equipment, Wings, etc.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
