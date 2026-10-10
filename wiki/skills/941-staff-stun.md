---
kind: skill
id: 941
name: Staff Stun
status: in-game
icon: skill/70
type: Damage Action
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: continuous attack
needs_weapon: Magic Staff
skill_books:
- '[[items/consumable/711-staff-stun|Staff Stun]]'
levels:
- level: 1
  id: 941
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 4'
  learn_points: 8
  cost: MP 30
  power: 30
  cooldown: 10
  duration: 5
  success: 40
  effects: Fainted
- level: 2
  id: 942
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 5'
  learn_points: 11
  cost: MP 32
  power: 34
  cooldown: 9.6
  duration: 5
  success: 43
  effects: Fainted
- level: 3
  id: 943
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 6'
  learn_points: 14
  cost: MP 34
  power: 38
  cooldown: 9.2
  duration: 5
  success: 46
  effects: Fainted
- level: 4
  id: 944
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 7'
  learn_points: 18
  cost: MP 36
  power: 42
  cooldown: 8.8
  duration: 6
  success: 49
  effects: Fainted
- level: 5
  id: 945
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 8'
  learn_points: 23
  cost: MP 38
  power: 46
  cooldown: 8.4
  duration: 6
  success: 52
  effects: Fainted
- level: 6
  id: 946
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 10'
  learn_points: 29
  cost: MP 40
  power: 50
  cooldown: 8
  duration: 6
  success: 55
  effects: Fainted
- level: 7
  id: 947
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 12'
  learn_points: 36
  cost: MP 42
  power: 54
  cooldown: 7.6
  duration: 7
  success: 58
  effects: Fainted
- level: 8
  id: 948
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 14'
  learn_points: 44
  cost: MP 44
  power: 58
  cooldown: 7.2
  duration: 7
  success: 61
  effects: Fainted
- level: 9
  id: 949
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 16'
  learn_points: 54
  cost: MP 46
  power: 62
  cooldown: 6.8
  duration: 7
  success: 64
  effects: Fainted
- level: 10
  id: 950
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 18'
  learn_points: 66
  cost: MP 50
  power: 70
  cooldown: 6
  duration: 8
  success: 67
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 941, 942, 943, 944, 945, 946, 947, 948, 949, 950
  code: module/src/skills.rs
---
# Staff Stun

Use a Staff to strike an enemy with great force and inflict the Stun status.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
