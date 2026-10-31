---
kind: skill
id: 1501
name: Power Attack
status: in-game
icon: skill/28
type: Damage Action
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/757-power-attack|Power Attack]]'
levels:
- level: 1
  id: 1501
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 1'
  learn_points: 8
  cost: MP 10
  power: 115
  cooldown: 5.6
- level: 2
  id: 1502
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 2'
  learn_points: 10
  cost: MP 12
  power: 143
  cooldown: 5.6
- level: 3
  id: 1503
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 3'
  learn_points: 12
  cost: MP 14
  power: 171
  cooldown: 5.6
- level: 4
  id: 1504
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 4'
  learn_points: 14
  cost: MP 16
  power: 199
  cooldown: 5.6
- level: 5
  id: 1505
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 5'
  learn_points: 17
  cost: MP 18
  power: 226
  cooldown: 5.8
- level: 6
  id: 1506
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 6'
  learn_points: 20
  cost: MP 20
  power: 252
  cooldown: 5.8
- level: 7
  id: 1507
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 7'
  learn_points: 23
  cost: MP 22
  power: 279
  cooldown: 5.8
- level: 8
  id: 1508
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 8'
  learn_points: 27
  cost: MP 24
  power: 306
  cooldown: 5.8
- level: 9
  id: 1509
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 9'
  learn_points: 31
  cost: MP 26
  power: 333
  cooldown: 6
- level: 10
  id: 1510
  needs: '[[skills/1421-knuckle-mastery|Knuckle Mastery]] level 10'
  learn_points: 35
  cost: MP 30
  power: 368
  cooldown: 6
source:
  data: godot/data/skills/1501-power-attack/ (levels 1501, 1502, 1503, 1504, 1505, 1506, 1507, 1508, 1509, 1510)
  code: module/src/skills.rs
---
# Power Attack

Hit a target with mighty force while using a Knuckle or a Melee Weapon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
