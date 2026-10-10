---
kind: skill
id: 44
name: Recovery Kiss
status: in-game
icon: skill/24
type: Magic Recovery
job: any
max_level: 0
target: All Members
skill_books:
- '[[items/consumable/634-recovery-kiss|Recovery Kiss]]'
levels:
- level: 0
  id: 44
  learn_points: 0
  cost: MP 10
  range: 1.6
  cooldown: 3
  changes: HP +20
source:
  data: LIST_SKILL.STB rows 44
  code: module/src/skills.rs
---
# Recovery Kiss

Kiss another character to restore his or her HP.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
