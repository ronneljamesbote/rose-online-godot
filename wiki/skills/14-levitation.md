---
kind: skill
id: 14
name: Levitation
status: in-game
icon: skill/3
type: Basic Action
job: any
max_level: 0
target: Yourself
needs_weapon: Wings
skill_books:
- '[[items/consumable/604-levitation|Levitation]]'
levels:
- level: 0
  id: 14
  learn_points: 0
  cost: MP 5
  range: 0
source:
  data: LIST_SKILL.STB rows 14
  code: module/src/skills.rs
---
# Levitation

Leap high into the sky while wearing a Wing item.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
