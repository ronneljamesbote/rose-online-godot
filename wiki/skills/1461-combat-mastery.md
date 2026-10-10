---
kind: skill
id: 1461
name: Combat Mastery
status: in-game
icon: skill/105
type: Passive
job: Hawker Job
max_level: 20
target: Yourself
needs_weapon: Bow, Katar, Dual Swords
warps_to: zone 49
skill_books:
- '[[items/consumable/755-combat-mastery|Combat Mastery]]'
levels:
- level: 1
  id: 1461
  learn_points: 4
  changes: Bow Attack Speed +4, Combat Weapon Attack Speed +3
- level: 2
  id: 1462
  learn_points: 5
  changes: Bow Attack Speed +5, Combat Weapon Attack Speed +4
- level: 3
  id: 1463
  learn_points: 7
  changes: Bow Attack Speed +6, Combat Weapon Attack Speed +5
- level: 4
  id: 1464
  learn_points: 9
  changes: Bow Attack Speed +7, Combat Weapon Attack Speed +6
- level: 5
  id: 1465
  learn_points: 11
  changes: Bow Attack Speed +8, Combat Weapon Attack Speed +7
- level: 6
  id: 1466
  learn_points: 13
  changes: Bow Attack Speed +9, Combat Weapon Attack Speed +8
- level: 7
  id: 1467
  learn_points: 15
  changes: Bow Attack Speed +10, Combat Weapon Attack Speed +9
- level: 8
  id: 1468
  learn_points: 18
  changes: Bow Attack Speed +11, Combat Weapon Attack Speed +10
- level: 9
  id: 1469
  learn_points: 21
  changes: Bow Attack Speed +12, Combat Weapon Attack Speed +11
- level: 10
  id: 1470
  learn_points: 24
  changes: Bow Attack Speed +13, Combat Weapon Attack Speed +12
- level: 11
  id: 1471
  needs_level: 70
  learn_points: 27
  changes: Bow Attack Speed +14, Combat Weapon Attack Speed +13
- level: 12
  id: 1472
  learn_points: 30
  changes: Bow Attack Speed +15, Combat Weapon Attack Speed +14
- level: 13
  id: 1473
  learn_points: 33
  changes: Bow Attack Speed +16, Combat Weapon Attack Speed +15
- level: 14
  id: 1474
  learn_points: 36
  changes: Bow Attack Speed +17, Combat Weapon Attack Speed +16
- level: 15
  id: 1475
  learn_points: 40
  changes: Bow Attack Speed +18, Combat Weapon Attack Speed +17
- level: 16
  id: 1476
  learn_points: 44
  changes: Bow Attack Speed +19, Combat Weapon Attack Speed +18
- level: 17
  id: 1477
  learn_points: 48
  changes: Bow Attack Speed +20, Combat Weapon Attack Speed +19
- level: 18
  id: 1478
  learn_points: 52
  changes: Bow Attack Speed +21, Combat Weapon Attack Speed +20
- level: 19
  id: 1479
  learn_points: 57
  changes: Bow Attack Speed +22, Combat Weapon Attack Speed +21
- level: 20
  id: 1480
  learn_points: 62
  changes: Bow Attack Speed +23, Combat Weapon Attack Speed +22
source:
  data: LIST_SKILL.STB rows 1461, 1462, 1463, 1464, 1465, 1466, 1467, 1468, 1469, 1470, 1471, 1472, 1473, 1474, 1475, 1476, 1477, 1478, 1479, 1480
  code: module/src/skills.rs
---
# Combat Mastery

Increase Attack Power and Attack Speed of Katar, Dual Wield and Bow weapons.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
