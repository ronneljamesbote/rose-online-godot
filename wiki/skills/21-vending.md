---
kind: skill
id: 21
name: Vending
status: in-game
icon: skill/8
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/611-vending|Vending]]'
levels:
- level: 0
  id: 21
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 21
  code: module/src/skills.rs
---
# Vending

Open a Personal Shop.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
