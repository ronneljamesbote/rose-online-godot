---
kind: skill
id: 911
name: Fire Ring
status: in-game
icon: skill/208
type: 'Continuing '
job: Muse Job
max_level: 5
target: Hostile Character
warps_to: '[[zones/19-zone-19|Zone 19]]'
skill_books:
- '[[items/consumable/707-fire-ring|Fire Ring]]'
levels:
- level: 1
  id: 911
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 4'
  learn_points: 10
  cost: MP 25
  range: 25
  cooldown: 6
  duration: 20
  success: 60
  effects: Def Down
  changes: Defense +25%
- level: 2
  id: 912
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 6'
  learn_points: 16
  cost: MP 30
  range: 25.5
  cooldown: 6
  duration: 20
  success: 65
  effects: Def Down
  changes: Defense +28%
- level: 3
  id: 913
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 8'
  learn_points: 24
  cost: MP 35
  range: 26
  cooldown: 6
  duration: 20
  success: 70
  effects: Def Down
  changes: Defense +31%
- level: 4
  id: 914
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 10'
  learn_points: 35
  cost: MP 40
  range: 26.5
  cooldown: 6
  duration: 20
  success: 75
  effects: Def Down
  changes: Defense +34%
- level: 5
  id: 915
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 12'
  learn_points: 50
  cost: MP 45
  range: 27
  cooldown: 6
  duration: 20
  success: 80
  effects: Def Down
  changes: Defense +40%
source:
  data: LIST_SKILL.STB rows 911, 912, 913, 914, 915
  code: module/src/skills.rs
---
# Fire Ring

Decrease the Defense of a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
