---
kind: skill
id: 16
name: Normal Attack
status: in-game
icon: skill/10
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/606-normal-attack|Normal Attack]]'
levels:
- level: 0
  id: 16
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 16
  code: module/src/skills.rs
---
# Normal Attack

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
