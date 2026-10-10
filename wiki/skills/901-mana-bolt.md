---
kind: skill
id: 901
name: Mana Bolt
status: in-game
icon: skill/67
type: Magic Spell
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/708-mana-bolt|Mana Bolt]]'
levels:
- level: 1
  id: 901
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 1'
  learn_points: 6
  cost: MP 20
  power: 40
  range: 25
  cooldown: 4.2
- level: 2
  id: 902
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 3'
  learn_points: 9
  cost: MP 24
  power: 56
  range: 25.5
  cooldown: 4.4
- level: 3
  id: 903
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 5'
  learn_points: 12
  cost: MP 28
  power: 72
  range: 26
  cooldown: 4.6
- level: 4
  id: 904
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 7'
  learn_points: 16
  cost: MP 32
  power: 88
  range: 26.5
  cooldown: 4.8
- level: 5
  id: 905
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 9'
  learn_points: 21
  cost: MP 36
  power: 105
  range: 27
  cooldown: 5
- level: 6
  id: 906
  needs_level: 70
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 10'
  learn_points: 27
  cost: MP 40
  power: 115
  range: 27.2
  cooldown: 6
  duration: 20
  success: 22
  effects: Def Down, Magic Resistance Down
  changes: Defense -30%, Magic Resistance -30%
- level: 7
  id: 907
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 12'
  learn_points: 35
  cost: MP 44
  power: 125
  range: 27.4
  cooldown: 6.2
  duration: 20
  success: 24
  effects: Def Down, Magic Resistance Down
  changes: Defense -30%, Magic Resistance -30%
- level: 8
  id: 908
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 14'
  learn_points: 45
  cost: MP 48
  power: 135
  range: 27.6
  cooldown: 6.4
  duration: 20
  success: 26
  effects: Def Down, Magic Resistance Down
  changes: Defense -30%, Magic Resistance -30%
- level: 9
  id: 909
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 16'
  learn_points: 58
  cost: MP 52
  power: 145
  range: 27.8
  cooldown: 6.6
  duration: 20
  success: 28
  effects: Def Down, Magic Resistance Down
  changes: Defense -30%, Magic Resistance -30%
- level: 10
  id: 910
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 18'
  learn_points: 74
  cost: MP 60
  power: 160
  range: 28
  cooldown: 6.8
  duration: 20
  success: 30
  effects: Def Down, Magic Resistance Down
  changes: Defense -30%, Magic Resistance -30%
source:
  data: LIST_SKILL.STB rows 901, 902, 903, 904, 905, 906, 907, 908, 909, 910
  code: module/src/skills.rs
---
# Mana Bolt

Shoot a Mana Bolt at a target from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
