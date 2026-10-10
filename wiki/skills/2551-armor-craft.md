---
kind: skill
id: 2551
name: Armor Craft
status: in-game
icon: skill/174
type: Crafting Skill
job: Dealer Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/841-armor-craft|Armor Craft]]'
levels:
- level: 1
  id: 2551
  needs: '[[skills/2131-armor-research|Armor Research]] level 1'
  learn_points: 20
  cost: MP 60
  power: 17
- level: 2
  id: 2552
  needs: '[[skills/2131-armor-research|Armor Research]] level 2'
  learn_points: 24
  cost: MP 65
  power: 17
- level: 3
  id: 2553
  needs: '[[skills/2131-armor-research|Armor Research]] level 3'
  learn_points: 28
  cost: MP 70
  power: 17
- level: 4
  id: 2554
  needs: '[[skills/2131-armor-research|Armor Research]] level 4'
  learn_points: 33
  cost: MP 75
  power: 17
- level: 5
  id: 2555
  needs: '[[skills/2131-armor-research|Armor Research]] level 5'
  learn_points: 39
  cost: MP 80
  power: 17
- level: 6
  id: 2556
  needs: '[[skills/2131-armor-research|Armor Research]] level 6'
  learn_points: 45
  cost: MP 85
  power: 17
- level: 7
  id: 2557
  needs: '[[skills/2131-armor-research|Armor Research]] level 7'
  learn_points: 52
  cost: MP 90
  power: 17
- level: 8
  id: 2558
  needs: '[[skills/2131-armor-research|Armor Research]] level 8'
  learn_points: 60
  cost: MP 95
  power: 17
- level: 9
  id: 2559
  needs: '[[skills/2131-armor-research|Armor Research]] level 9'
  learn_points: 69
  cost: MP 100
  power: 17
- level: 10
  id: 2560
  needs: '[[skills/2131-armor-research|Armor Research]] level 10'
  learn_points: 80
  cost: MP 105
  power: 17
source:
  data: LIST_SKILL.STB rows 2551, 2552, 2553, 2554, 2555, 2556, 2557, 2558, 2559, 2560
  code: module/src/skills.rs
---
# Armor Craft

Create Armor, Helms, Gauntlets and Boots.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
