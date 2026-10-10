---
kind: skill
id: 40
name: Breakdown
status: in-game
icon: skill/11
type: Emotion
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/638-tantrum|Tantrum]]'
- '[[items/consumable/976-breakdown|Breakdown]]'
levels:
- level: 0
  id: 48
  learn_points: 0
  cooldown: 3
source:
  data: LIST_SKILL.STB rows 48
  code: module/src/skills.rs
---
# Breakdown

Collapse to the floor in despair.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
