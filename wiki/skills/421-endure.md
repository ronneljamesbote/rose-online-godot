---
kind: skill
id: 421
name: Endure
status: in-game
icon: skill/36
type: Continuing (Self)
job: Soldier Job
max_level: 10
target: Yourself
needs_weapon: Shield
warps_to: '[[zones/19-zone-19|Zone 19]]'
skill_books:
- '[[items/consumable/668-endure|Endure]]'
levels:
- level: 1
  id: 421
  needs: '[[skills/251-defense-training|Defense Training]] level 6'
  learn_points: 10
  cost: MP 30
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +50, Movement Speed +200
- level: 2
  id: 422
  needs: '[[skills/251-defense-training|Defense Training]] level 7'
  learn_points: 12
  cost: MP 33
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +60, Movement Speed +190
- level: 3
  id: 423
  needs: '[[skills/251-defense-training|Defense Training]] level 8'
  learn_points: 15
  cost: MP 36
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +70, Movement Speed +180
- level: 4
  id: 424
  needs: '[[skills/251-defense-training|Defense Training]] level 9'
  learn_points: 18
  cost: MP 39
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +80, Movement Speed +170
- level: 5
  id: 425
  needs: '[[skills/251-defense-training|Defense Training]] level 10'
  learn_points: 22
  cost: MP 42
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +90, Movement Speed +160
- level: 6
  id: 426
  needs: '[[skills/251-defense-training|Defense Training]] level 11'
  learn_points: 26
  cost: MP 45
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +100, Movement Speed +150
- level: 7
  id: 427
  needs: '[[skills/251-defense-training|Defense Training]] level 13'
  learn_points: 32
  cost: MP 48
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +115, Movement Speed +145
- level: 8
  id: 428
  needs: '[[skills/251-defense-training|Defense Training]] level 15'
  learn_points: 39
  cost: MP 51
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +130, Movement Speed +140
- level: 9
  id: 429
  needs: '[[skills/251-defense-training|Defense Training]] level 17'
  learn_points: 48
  cost: MP 54
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +145, Movement Speed +135
- level: 10
  id: 430
  needs: '[[skills/251-defense-training|Defense Training]] level 19'
  learn_points: 59
  cost: MP 57
  cooldown: 6
  duration: 60
  effects: Def Up, Slow
  changes: Defense +160, Movement Speed +130
source:
  data: LIST_SKILL.STB rows 421, 422, 423, 424, 425, 426, 427, 428, 429, 430
  code: module/src/skills.rs
---
# Endure

Increase Defense while decreasing the caster's Movement Speed for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
