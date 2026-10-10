---
kind: skill
id: 33
name: Donation (10 Zulie)
status: in-game
icon: skill/4
type: Magic Recovery
job: any
max_level: 0
target: Ally
skill_books:
- '[[items/consumable/623-donation-10-zulie|Donation (10 Zulie)]]'
levels:
- level: 0
  id: 33
  learn_points: 0
  cost: Money 10
  range: 5
  changes: Money +10
source:
  data: LIST_SKILL.STB rows 33
  code: module/src/skills.rs
---
# Donation (10 Zulie)

Donate 10 Zulie to another character.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
