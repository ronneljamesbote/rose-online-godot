---
kind: skill
id: 50
name: Applause
status: in-game
icon: skill/16
type: Emotion
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/27-silver-pilules|Silver Pilules]]'
- '[[items/consumable/151-hp-point-50|HP Point (+50)]]'
- '[[items/consumable/162-mp-point-50|MP Point (+50)]]'
- '[[items/consumable/171-stamina-50|Stamina (+50)]]'
- '[[items/consumable/294-engine-fuel-m|Engine Fuel (M)]]'
- '[[items/consumable/978-applause|Applause]]'
levels:
- level: 0
  id: 50
  learn_points: 0
  cooldown: 3
source:
  data: LIST_SKILL.STB rows 50
  code: module/src/skills.rs
---
# Applause

Clap your hands in approval.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
