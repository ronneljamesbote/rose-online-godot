---
kind: skill
id: 2591
name: Cart Craft
status: in-game
icon: skill/180
type: Crafting Skill
job: Artisan Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/845-cart-craft|Cart Craft]]'
levels:
- level: 1
  id: 2591
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 8'
  learn_points: 40
  cost: MP 100
  power: 21
- level: 2
  id: 2592
  learn_points: 43
  cost: MP 120
  power: 21
- level: 3
  id: 2593
  learn_points: 47
  cost: MP 140
  power: 21
- level: 4
  id: 2594
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 9'
  learn_points: 51
  cost: MP 160
  power: 21
- level: 5
  id: 2595
  learn_points: 55
  cost: MP 180
  power: 21
- level: 6
  id: 2596
  learn_points: 59
  cost: MP 200
  power: 21
- level: 7
  id: 2597
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 10'
  learn_points: 64
  cost: MP 220
  power: 21
- level: 8
  id: 2598
  learn_points: 69
  cost: MP 240
  power: 21
- level: 9
  id: 2599
  learn_points: 74
  cost: MP 260
  power: 21
- level: 10
  id: 2600
  learn_points: 79
  cost: MP 280
  power: 21
source:
  data: LIST_SKILL.STB rows 2591, 2592, 2593, 2594, 2595, 2596, 2597, 2598, 2599, 2600
  code: module/src/skills.rs
---
# Cart Craft

Create various Cart parts.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
