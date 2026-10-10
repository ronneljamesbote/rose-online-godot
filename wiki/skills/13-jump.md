---
kind: skill
id: 13
name: Jump
status: in-game
icon: skill/2
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/603-jump|Jump]]'
levels:
- level: 0
  id: 13
  learn_points: 0
  cost: MP 0
  range: 0
source:
  data: LIST_SKILL.STB rows 13
  code: module/src/skills.rs
---
# Jump

Allows its user to jump.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
