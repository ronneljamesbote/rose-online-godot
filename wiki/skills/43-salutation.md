---
kind: skill
id: 43
name: Salutation
status: in-game
icon: skill/14
type: Emotion
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/633-salutation|Salutation]]'
- '[[items/consumable/973-gracious-bow|Gracious Bow]]'
levels:
- level: 0
  id: 43
  learn_points: 0
  cooldown: 3
source:
  data: LIST_SKILL.STB rows 43
  code: module/src/skills.rs
---
# Salutation

Give a gentlemanly bow in formal greeting.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
