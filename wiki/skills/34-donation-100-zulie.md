---
kind: skill
id: 34
name: Donation (100 Zulie)
status: in-game
icon: skill/4
type: Magic Recovery
job: any
max_level: 0
target: Ally
skill_books:
- '[[items/consumable/624-donation-100-zulie|Donation (100 Zulie)]]'
levels:
- level: 0
  id: 34
  learn_points: 0
  cost: Money 50
  range: 5
  changes: Money +50
source:
  data: LIST_SKILL.STB rows 34
  code: module/src/skills.rs
---
# Donation (100 Zulie)

Donate 100 Zulie to another character.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
