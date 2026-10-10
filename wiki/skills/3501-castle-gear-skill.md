---
kind: skill
id: 3501
name: Castle Gear Skill
status: in-game
icon: skill/40
type: Area Effect Attack (Self)
job: any
max_level: 1
target: Hostile Character
damage_type: weapon attack
needs_weapon: Castle Gear Body
levels:
- level: 1
  id: 3501
  learn_points: 0
  cost: Fuel 100
  power: 60
  area: 12
  cooldown: 16
source:
  data: LIST_SKILL.STB rows 3501
  code: module/src/skills.rs
---
# Castle Gear Skill

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
