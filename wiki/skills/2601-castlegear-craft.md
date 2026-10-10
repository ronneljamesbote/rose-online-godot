---
kind: skill
id: 2601
name: CastleGear Craft
status: in-game
icon: skill/180
type: Crafting Skill
job: Artisan Job
max_level: 10
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/846-castlegear-craft|CastleGear Craft]]'
levels:
- level: 1
  id: 2601
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 10'
  learn_points: 50
  cost: MP 100
  power: 22
- level: 2
  id: 2602
  learn_points: 54
  cost: MP 120
  power: 22
- level: 3
  id: 2603
  learn_points: 58
  cost: MP 140
  power: 22
- level: 4
  id: 2604
  learn_points: 62
  cost: MP 160
  power: 22
- level: 5
  id: 2605
  learn_points: 67
  cost: MP 180
  power: 22
- level: 6
  id: 2606
  learn_points: 72
  cost: MP 200
  power: 22
- level: 7
  id: 2607
  learn_points: 77
  cost: MP 220
  power: 22
- level: 8
  id: 2608
  learn_points: 83
  cost: MP 240
  power: 22
- level: 9
  id: 2609
  learn_points: 89
  cost: MP 260
  power: 22
- level: 10
  id: 2610
  learn_points: 95
  cost: MP 280
  power: 22
source:
  data: LIST_SKILL.STB rows 2601, 2602, 2603, 2604, 2605, 2606, 2607, 2608, 2609, 2610
  code: module/src/skills.rs
---
# CastleGear Craft

Create various Castle Gear parts.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
