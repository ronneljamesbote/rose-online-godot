---
kind: skill
id: 391
name: Divine Force
status: in-game
icon: skill/39
type: Magic Spell
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: magic attack
warps_to: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
skill_books:
- '[[items/consumable/2-health-vial-m|Health Vial (M)]]'
- '[[items/consumable/25-mana-bottle-m|Mana Bottle (M)]]'
- '[[items/consumable/30-spiritual-water-m|Spiritual Water (M)]]'
- '[[items/consumable/132-ice-cherry|Ice Cherry]]'
- '[[items/consumable/135-crystal-berry|Crystal Berry]]'
- '[[items/consumable/177-stamina-400|Stamina (+400)]]'
- '[[items/consumable/665-divine-force|Divine Force]]'
levels:
- level: 1
  id: 391
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 6'
  learn_points: 15
  cost: MP 40
  power: 90
  range: 20
  cooldown: 10
  duration: 15
  success: 60
  effects: Magic Resistance Down
  changes: Magic Resistance +25%
- level: 2
  id: 392
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 7'
  learn_points: 19
  cost: MP 42
  power: 100
  range: 20.5
  cooldown: 10.2
  duration: 15
  success: 62
  effects: Magic Resistance Down
  changes: Magic Resistance +26%
- level: 3
  id: 393
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 8'
  learn_points: 24
  cost: MP 44
  power: 110
  range: 21
  cooldown: 10.4
  duration: 15
  success: 64
  effects: Magic Resistance Down
  changes: Magic Resistance +27%
- level: 4
  id: 394
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 9'
  learn_points: 30
  cost: MP 46
  power: 120
  range: 21.5
  cooldown: 10.6
  duration: 15
  success: 66
  effects: Magic Resistance Down
  changes: Magic Resistance +28%
- level: 5
  id: 395
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 10'
  learn_points: 37
  cost: MP 48
  power: 130
  range: 22
  cooldown: 10.8
  duration: 15
  success: 68
  effects: Magic Resistance Down
  changes: Magic Resistance +29%
- level: 6
  id: 396
  learn_points: 46
  cost: MP 51
  power: 140
  range: 22.5
  area: 4
  cooldown: 11
  duration: 18
  success: 70
  effects: Magic Resistance Down
  changes: Magic Resistance +30%
- level: 7
  id: 397
  learn_points: 57
  cost: MP 54
  power: 150
  range: 23
  area: 4.5
  cooldown: 11.2
  duration: 19
  success: 72
  effects: Magic Resistance Down
  changes: Magic Resistance +31%
- level: 8
  id: 398
  learn_points: 70
  cost: MP 57
  power: 160
  range: 23.5
  area: 5
  cooldown: 11.4
  duration: 20
  success: 74
  effects: Magic Resistance Down
  changes: Magic Resistance +32%
- level: 9
  id: 399
  learn_points: 86
  cost: MP 60
  power: 170
  range: 24
  area: 6
  cooldown: 11.6
  duration: 21
  success: 76
  effects: Magic Resistance Down
  changes: Magic Resistance +33%
- level: 10
  id: 400
  learn_points: 105
  cost: MP 65
  power: 185
  range: 25
  area: 8
  cooldown: 11.8
  duration: 23
  success: 78
  effects: Magic Resistance Down
  changes: Magic Resistance +35%
source:
  data: LIST_SKILL.STB rows 391, 392, 393, 394, 395, 396, 397, 398, 399, 400
  code: module/src/skills.rs
---
# Divine Force

Shoot a holy sphere at a target from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
