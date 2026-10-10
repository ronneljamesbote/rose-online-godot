---
kind: skill
id: 2431
name: Sword Craft
status: in-game
icon: skill/150
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/834-sword-craft|Sword Craft]]'
levels:
- level: 1
  id: 2431
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 15
  cost: MP 50
  power: 11
- level: 2
  id: 2432
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 3'
  learn_points: 17
  cost: MP 55
  power: 11
- level: 3
  id: 2433
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 5'
  learn_points: 19
  cost: MP 60
  power: 11
- level: 4
  id: 2434
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 7'
  learn_points: 21
  cost: MP 65
  power: 11
- level: 5
  id: 2435
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 9'
  learn_points: 23
  cost: MP 70
  power: 11
- level: 6
  id: 2436
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 25
  cost: MP 75
  power: 11
- level: 7
  id: 2437
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 12'
  learn_points: 28
  cost: MP 80
  power: 11
- level: 8
  id: 2438
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 13'
  learn_points: 31
  cost: MP 85
  power: 11
- level: 9
  id: 2439
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 34
  cost: MP 90
  power: 11
- level: 10
  id: 2440
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 37
  cost: MP 95
  power: 11
- level: 11
  id: 2441
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 40
  cost: MP 100
  power: 11
- level: 12
  id: 2442
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 43
  cost: MP 105
  power: 11
- level: 13
  id: 2443
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 47
  cost: MP 110
  power: 11
- level: 14
  id: 2444
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 51
  cost: MP 115
  power: 11
- level: 15
  id: 2445
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 55
  cost: MP 120
  power: 11
source:
  data: LIST_SKILL.STB rows 2431, 2432, 2433, 2434, 2435, 2436, 2437, 2438, 2439, 2440, 2441, 2442, 2443, 2444, 2445
  code: module/src/skills.rs
---
# Sword Craft

Create One-Handed Swords, Two-Handed Swords or Dual Wield weapons.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
