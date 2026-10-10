---
kind: skill
id: 19
name: Party
status: in-game
icon: skill/9
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/609-party|Party]]'
levels:
- level: 0
  id: 19
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 19
  code: module/src/skills.rs
---
# Party

Ask someone to join your party.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
