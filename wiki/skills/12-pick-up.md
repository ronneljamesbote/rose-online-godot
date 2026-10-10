---
kind: skill
id: 12
name: Pick Up
status: in-game
icon: skill/4
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/602-pick-up|Pick Up]]'
levels:
- level: 0
  id: 12
  learn_points: 0
  range: 0
source:
  data: LIST_SKILL.STB rows 12
  code: module/src/skills.rs
---
# Pick Up

Allows its user to pick up nearby items dropped on the ground.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
