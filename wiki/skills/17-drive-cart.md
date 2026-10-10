---
kind: skill
id: 17
name: Drive Cart
status: in-game
icon: skill/6
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/607-drive-cart|Drive Cart]]'
levels:
- level: 0
  id: 17
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 17
  code: module/src/skills.rs
---
# Drive Cart

Drive a Cart. (Get off a Cart when driving.)

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
