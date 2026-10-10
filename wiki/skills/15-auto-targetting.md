---
kind: skill
id: 15
name: Auto Targetting
status: in-game
icon: skill/5
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/187-clan-point-15|Clan Point (+15)]]'
- '[[items/consumable/605-auto-target|Auto Target]]'
levels:
- level: 0
  id: 15
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 15
  code: module/src/skills.rs
---
# Auto Targetting

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
