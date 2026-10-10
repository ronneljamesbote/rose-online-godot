---
kind: skill
id: 521
name: Lightning Crasher
status: in-game
icon: skill/48
type: Damage Action
job: Knight Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/677-lightning-crasher|Lightning Crasher]]'
levels:
- level: 1
  id: 521
  needs: '[[skills/301-heavy-attack|Heavy Attack]] level 10'
  learn_points: 30
  cost: MP 50
  power: 120
  cooldown: 8
  duration: 4
  success: 50
  effects: Fainted
- level: 2
  id: 522
  learn_points: 35
  cost: MP 54
  power: 135
  cooldown: 8.2
  duration: 4
  success: 52
  effects: Fainted
- level: 3
  id: 523
  learn_points: 41
  cost: MP 58
  power: 150
  cooldown: 8.4
  duration: 4
  success: 54
  effects: Fainted
- level: 4
  id: 524
  learn_points: 48
  cost: MP 62
  power: 165
  cooldown: 8.6
  duration: 4
  success: 56
  effects: Fainted
- level: 5
  id: 525
  learn_points: 56
  cost: MP 66
  power: 180
  cooldown: 8.8
  duration: 4
  success: 58
  effects: Fainted
- level: 6
  id: 526
  learn_points: 65
  cost: MP 70
  power: 195
  cooldown: 9
  duration: 5
  success: 60
  effects: Fainted
- level: 7
  id: 527
  learn_points: 75
  cost: MP 74
  power: 210
  cooldown: 9.2
  duration: 5
  success: 62
  effects: Fainted
- level: 8
  id: 528
  learn_points: 87
  cost: MP 78
  power: 225
  cooldown: 9.4
  duration: 5
  success: 64
  effects: Fainted
- level: 9
  id: 529
  learn_points: 100
  cost: MP 82
  power: 240
  cooldown: 9.6
  duration: 5
  success: 66
  effects: Fainted
- level: 10
  id: 530
  learn_points: 115
  cost: MP 90
  power: 270
  cooldown: 9.8
  duration: 5
  success: 70
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 521, 522, 523, 524, 525, 526, 527, 528, 529, 530
  code: module/src/skills.rs
---
# Lightning Crasher

Deliver multiple blows after confusing the target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
