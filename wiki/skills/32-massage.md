---
kind: skill
id: 32
name: Massage
status: in-game
icon: skill/5
type: Magic Recovery
job: any
max_level: 0
target: Ally
warps_to: zone 16
skill_books:
- '[[items/consumable/622-massage|Massage]]'
levels:
- level: 0
  id: 32
  learn_points: 0
  cost: MP 0
  range: 1
  changes: HP +100
source:
  data: LIST_SKILL.STB rows 32
  code: module/src/skills.rs
---
# Massage

Give a light massage to another character.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
