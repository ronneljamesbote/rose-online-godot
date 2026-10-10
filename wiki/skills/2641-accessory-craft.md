---
kind: skill
id: 2641
name: Accessory Craft
status: in-game
icon: skill/187
type: Crafting Skill
job: Artisan Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/850-accessory-craft|Accessory Craft]]'
levels:
- level: 1
  id: 2641
  learn_points: 10
  cost: MP 50
  power: 31
- level: 2
  id: 2642
  learn_points: 13
  cost: MP 55
  power: 31
- level: 3
  id: 2643
  learn_points: 17
  cost: MP 60
  power: 31
- level: 4
  id: 2644
  learn_points: 22
  cost: MP 65
  power: 31
- level: 5
  id: 2645
  learn_points: 28
  cost: MP 70
  power: 31
- level: 6
  id: 2646
  learn_points: 35
  cost: MP 75
  power: 31
- level: 7
  id: 2647
  learn_points: 44
  cost: MP 80
  power: 31
- level: 8
  id: 2648
  learn_points: 55
  cost: MP 85
  power: 31
- level: 9
  id: 2649
  learn_points: 69
  cost: MP 90
  power: 31
- level: 10
  id: 2650
  learn_points: 86
  cost: MP 95
  power: 31
source:
  data: LIST_SKILL.STB rows 2641, 2642, 2643, 2644, 2645, 2646, 2647, 2648, 2649, 2650
  code: module/src/skills.rs
---
# Accessory Craft

Create various accessories such as Rings, Earrings and Necklaces.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
