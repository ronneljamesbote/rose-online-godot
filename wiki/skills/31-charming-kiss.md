---
kind: skill
id: 31
name: Charming Kiss
status: in-game
icon: skill/5
type: Magic Recovery
job: any
max_level: 1
target: Ally
skill_books:
- '[[items/consumable/621-charming-kiss|Charming Kiss]]'
levels:
- level: 1
  id: 31
  learn_points: 0
  cost: MP 10
  range: 1.6
  cooldown: 3
  changes: MP +15
source:
  data: LIST_SKILL.STB rows 31
  code: module/src/skills.rs
---
# Charming Kiss

Kiss another character to help recover his or her MP.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
