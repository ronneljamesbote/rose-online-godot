---
kind: skill
id: 3511
name: Castle Gear Skill
status: in-game
icon: skill/39
type: Magic Spell
job: any
max_level: 1
target: Hostile Character
damage_type: magic attack
needs_weapon: Castle Gear Body
levels:
- level: 1
  id: 3511
  learn_points: 0
  cost: Fuel 80, MP 100
  power: 90
  range: 25
  cooldown: 10
  duration: 15
  success: 60
  effects: Magic Resistance Down
  changes: Magic Resistance -25%
source:
  data: LIST_SKILL.STB rows 3511
  code: module/src/skills.rs
---
# Castle Gear Skill

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
