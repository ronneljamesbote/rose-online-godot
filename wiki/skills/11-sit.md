---
kind: skill
id: 11
name: Sit
status: in-game
icon: skill/1
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/601-sit|Sit]]'
levels:
- level: 0
  id: 11
  learn_points: 0
  range: 0
source:
  data: LIST_SKILL.STB rows 11
  code: module/src/skills.rs
---
# Sit

Allows its user to restore HP and MP while sitting.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
