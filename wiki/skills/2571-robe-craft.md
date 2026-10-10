---
kind: skill
id: 2571
name: Robe Craft
status: in-game
icon: skill/175
type: Crafting Skill
job: Dealer Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/843-robe-craft|Robe Craft]]'
levels:
- level: 1
  id: 2571
  needs: '[[skills/2131-armor-research|Armor Research]] level 1'
  learn_points: 20
  cost: MP 60
  power: 18
- level: 2
  id: 2572
  needs: '[[skills/2131-armor-research|Armor Research]] level 2'
  learn_points: 24
  cost: MP 65
  power: 18
- level: 3
  id: 2573
  needs: '[[skills/2131-armor-research|Armor Research]] level 3'
  learn_points: 28
  cost: MP 70
  power: 18
- level: 4
  id: 2574
  needs: '[[skills/2131-armor-research|Armor Research]] level 4'
  learn_points: 33
  cost: MP 75
  power: 18
- level: 5
  id: 2575
  needs: '[[skills/2131-armor-research|Armor Research]] level 5'
  learn_points: 39
  cost: MP 80
  power: 18
- level: 6
  id: 2576
  needs: '[[skills/2131-armor-research|Armor Research]] level 6'
  learn_points: 45
  cost: MP 85
  power: 18
- level: 7
  id: 2577
  needs: '[[skills/2131-armor-research|Armor Research]] level 7'
  learn_points: 52
  cost: MP 90
  power: 18
- level: 8
  id: 2578
  needs: '[[skills/2131-armor-research|Armor Research]] level 8'
  learn_points: 60
  cost: MP 95
  power: 18
- level: 9
  id: 2579
  needs: '[[skills/2131-armor-research|Armor Research]] level 9'
  learn_points: 69
  cost: MP 100
  power: 18
- level: 10
  id: 2580
  needs: '[[skills/2131-armor-research|Armor Research]] level 10'
  learn_points: 80
  cost: MP 105
  power: 18
source:
  data: LIST_SKILL.STB rows 2571, 2572, 2573, 2574, 2575, 2576, 2577, 2578, 2579, 2580
  code: module/src/skills.rs
---
# Robe Craft

Create Magic Clothing, Hats, Gloves and Boots.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
