---
kind: skill
id: 2451
name: Mace Craft
status: in-game
icon: skill/151
type: Crafting Skill
job: Dealer Job
max_level: 15
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/835-mace-craft|Mace Craft]]'
levels:
- level: 1
  id: 2451
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 1'
  learn_points: 15
  cost: MP 50
  power: 12
- level: 2
  id: 2452
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 2'
  learn_points: 17
  cost: MP 55
  power: 12
- level: 3
  id: 2453
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 4'
  learn_points: 19
  cost: MP 60
  power: 12
- level: 4
  id: 2454
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 6'
  learn_points: 21
  cost: MP 65
  power: 12
- level: 5
  id: 2455
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 8'
  learn_points: 23
  cost: MP 70
  power: 12
- level: 6
  id: 2456
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 10'
  learn_points: 25
  cost: MP 75
  power: 12
- level: 7
  id: 2457
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 11'
  learn_points: 28
  cost: MP 80
  power: 12
- level: 8
  id: 2458
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 12'
  learn_points: 31
  cost: MP 85
  power: 12
- level: 9
  id: 2459
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 14'
  learn_points: 34
  cost: MP 90
  power: 12
- level: 10
  id: 2460
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 15'
  learn_points: 37
  cost: MP 95
  power: 12
- level: 11
  id: 2461
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 16'
  learn_points: 40
  cost: MP 100
  power: 12
- level: 12
  id: 2462
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 17'
  learn_points: 43
  cost: MP 105
  power: 12
- level: 13
  id: 2463
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 18'
  learn_points: 47
  cost: MP 110
  power: 12
- level: 14
  id: 2464
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 19'
  learn_points: 51
  cost: MP 115
  power: 12
- level: 15
  id: 2465
  needs: '[[skills/2111-weapon-research|Weapon Research]] level 20'
  learn_points: 55
  cost: MP 120
  power: 12
source:
  data: LIST_SKILL.STB rows 2451, 2452, 2453, 2454, 2455, 2456, 2457, 2458, 2459, 2460, 2461, 2462, 2463, 2464, 2465
  code: module/src/skills.rs
---
# Mace Craft

Create Axes, Spears, Katars or One-Handed Maces.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
