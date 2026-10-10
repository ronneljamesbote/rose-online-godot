---
kind: skill
id: 2531
name: Clothing Craft
status: in-game
icon: skill/173
type: Crafting Skill
job: Dealer Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/839-clothing-craft|Clothing Craft]]'
levels:
- level: 1
  id: 2531
  needs: '[[skills/2131-armor-research|Armor Research]] level 1'
  learn_points: 20
  cost: MP 60
  power: 19
- level: 2
  id: 2532
  needs: '[[skills/2131-armor-research|Armor Research]] level 2'
  learn_points: 24
  cost: MP 65
  power: 19
- level: 3
  id: 2533
  needs: '[[skills/2131-armor-research|Armor Research]] level 3'
  learn_points: 28
  cost: MP 70
  power: 19
- level: 4
  id: 2534
  needs: '[[skills/2131-armor-research|Armor Research]] level 4'
  learn_points: 33
  cost: MP 75
  power: 19
- level: 5
  id: 2535
  needs: '[[skills/2131-armor-research|Armor Research]] level 5'
  learn_points: 39
  cost: MP 80
  power: 19
- level: 6
  id: 2536
  needs: '[[skills/2131-armor-research|Armor Research]] level 6'
  learn_points: 45
  cost: MP 85
  power: 19
- level: 7
  id: 2537
  needs: '[[skills/2131-armor-research|Armor Research]] level 7'
  learn_points: 52
  cost: MP 90
  power: 19
- level: 8
  id: 2538
  needs: '[[skills/2131-armor-research|Armor Research]] level 8'
  learn_points: 60
  cost: MP 95
  power: 19
- level: 9
  id: 2539
  needs: '[[skills/2131-armor-research|Armor Research]] level 9'
  learn_points: 69
  cost: MP 100
  power: 19
- level: 10
  id: 2540
  needs: '[[skills/2131-armor-research|Armor Research]] level 10'
  learn_points: 80
  cost: MP 105
  power: 19
source:
  data: LIST_SKILL.STB rows 2531, 2532, 2533, 2534, 2535, 2536, 2537, 2538, 2539, 2540
  code: module/src/skills.rs
---
# Clothing Craft

Create normal Clothing, Hats, Gloves, Shoes or Masks.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
