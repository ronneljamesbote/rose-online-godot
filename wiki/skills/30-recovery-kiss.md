---
kind: skill
id: 30
name: Recovery Kiss
status: in-game
icon: skill/5
type: Magic Recovery
job: any
max_level: 1
target: Ally
warps_to: zone 16
skill_books:
- '[[items/consumable/161-mp-point-30|MP Point (+30)]]'
- '[[items/consumable/190-clan-point-30|Clan Point (+30)]]'
- '[[items/consumable/620-recovery-kiss|Recovery Kiss]]'
levels:
- level: 1
  id: 30
  learn_points: 0
  cost: MP 10
  range: 1.6
  cooldown: 3
  changes: HP +20
source:
  data: LIST_SKILL.STB rows 30
  code: module/src/skills.rs
---
# Recovery Kiss

Kiss another character to restore his or her HP.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
