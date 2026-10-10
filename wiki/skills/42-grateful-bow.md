---
kind: skill
id: 42
name: Grateful Bow
status: in-game
icon: skill/15
type: Emotion
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/632-grateful-bow|Grateful Bow]]'
- '[[items/consumable/972-bow|Bow]]'
levels:
- level: 0
  id: 42
  learn_points: 0
  cooldown: 3
source:
  data: LIST_SKILL.STB rows 42
  code: module/src/skills.rs
---
# Grateful Bow

Give a polite bow to express gratitude.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
