---
kind: skill
id: 531
name: Holy Blood
status: in-game
icon: skill/57
type: Area Effect Attack (Self)
job: Knight Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/678-holy-blood|Holy Blood]]'
levels:
- level: 1
  id: 531
  needs: '[[skills/651-blood-attack|Blood Attack]] level 5, [[skills/291-spiritual-training|Spiritual Training]] level 10'
  learn_points: 30
  cost: HP 150
  power: 140
  area: 10
  cooldown: 14
- level: 2
  id: 532
  learn_points: 36
  cost: HP 160
  power: 150
  area: 10.2
  cooldown: 15
- level: 3
  id: 533
  learn_points: 43
  cost: HP 170
  power: 160
  area: 10.4
  cooldown: 16
- level: 4
  id: 534
  learn_points: 51
  cost: HP 180
  power: 170
  area: 10.6
  cooldown: 17
- level: 5
  id: 535
  learn_points: 60
  cost: HP 190
  power: 180
  area: 10.8
  cooldown: 18
- level: 6
  id: 536
  learn_points: 71
  cost: HP 200
  power: 190
  area: 11
  cooldown: 19
- level: 7
  id: 537
  learn_points: 83
  cost: HP 210
  power: 200
  area: 11.2
  cooldown: 20
- level: 8
  id: 538
  learn_points: 97
  cost: HP 220
  power: 210
  area: 11.4
  cooldown: 21
- level: 9
  id: 539
  learn_points: 114
  cost: HP 230
  power: 220
  area: 11.6
  cooldown: 22
- level: 10
  id: 540
  learn_points: 133
  cost: HP 250
  power: 240
  area: 12
  cooldown: 23
source:
  data: LIST_SKILL.STB rows 531, 532, 533, 534, 535, 536, 537, 538, 539, 540
  code: module/src/skills.rs
---
# Holy Blood

Sacrifice one's HP to inflict more damage onto nearby enemies.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
