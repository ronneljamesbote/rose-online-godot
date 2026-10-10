---
kind: skill
id: 2611
name: Gem Cutting
status: in-game
icon: skill/188
type: Crafting Skill
job: Artisan Job
max_level: 7
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/847-gem-cutting|Gem Cutting]]'
levels:
- level: 1
  id: 2611
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 7'
  learn_points: 30
  cost: MP 100
  power: 35
- level: 2
  id: 2612
  learn_points: 35
  cost: MP 120
  power: 35
- level: 3
  id: 2613
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 8'
  learn_points: 40
  cost: MP 140
  power: 35
- level: 4
  id: 2614
  learn_points: 46
  cost: MP 160
  power: 35
- level: 5
  id: 2615
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 9'
  learn_points: 53
  cost: MP 180
  power: 35
- level: 6
  id: 2616
  learn_points: 60
  cost: MP 200
  power: 35
- level: 7
  id: 2617
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 10'
  learn_points: 68
  cost: MP 220
  power: 35
source:
  data: LIST_SKILL.STB rows 2611, 2612, 2613, 2614, 2615, 2616, 2617
  code: module/src/skills.rs
---
# Gem Cutting

Produce a good quality gem from a low quality gem.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
