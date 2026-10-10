---
kind: skill
id: 2401
name: Item Craft
status: in-game
icon: skill/148
type: Crafting Skill
job: Dealer Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/832-item-craft|Item Craft]]'
levels:
- level: 1
  id: 2401
  needs: '[[skills/2001-market-research|Market Research]] level 1'
  learn_points: 5
  cost: MP 50
  power: 41
- level: 2
  id: 2402
  needs: '[[skills/2001-market-research|Market Research]] level 2'
  learn_points: 10
  cost: MP 55
  power: 41
- level: 3
  id: 2403
  needs: '[[skills/2001-market-research|Market Research]] level 3'
  learn_points: 15
  cost: MP 60
  power: 41
- level: 4
  id: 2404
  needs: '[[skills/2001-market-research|Market Research]] level 4'
  learn_points: 20
  cost: MP 65
  power: 41
- level: 5
  id: 2405
  needs: '[[skills/2001-market-research|Market Research]] level 5'
  learn_points: 25
  cost: MP 70
  power: 41
- level: 6
  id: 2406
  needs: '[[skills/2001-market-research|Market Research]] level 6'
  learn_points: 30
  cost: MP 75
  power: 41
- level: 7
  id: 2407
  needs: '[[skills/2001-market-research|Market Research]] level 7'
  learn_points: 35
  cost: MP 80
  power: 41
- level: 8
  id: 2408
  needs: '[[skills/2001-market-research|Market Research]] level 8'
  learn_points: 40
  cost: MP 85
  power: 41
- level: 9
  id: 2409
  needs: '[[skills/2001-market-research|Market Research]] level 9'
  learn_points: 45
  cost: MP 90
  power: 41
- level: 10
  id: 2410
  needs: '[[skills/2001-market-research|Market Research]] level 10'
  learn_points: 50
  cost: MP 95
  power: 41
source:
  data: LIST_SKILL.STB rows 2401, 2402, 2403, 2404, 2405, 2406, 2407, 2408, 2409, 2410
  code: module/src/skills.rs
---
# Item Craft

Craft various items.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
