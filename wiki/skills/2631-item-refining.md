---
kind: skill
id: 2631
name: Item Refining
status: in-game
icon: skill/196
type: Crafting Skill
job: Artisan Job
max_level: 1
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/849-item-refining|Item Refining]]'
levels:
- level: 1
  id: 2631
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 5'
  learn_points: 20
  cost: MP 50
  power: 42
source:
  data: LIST_SKILL.STB rows 2631
  code: module/src/skills.rs
---
# Item Refining

Refine equipment to enhance its performance. Increase item class by 1 when successful.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
