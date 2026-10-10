---
kind: skill
id: 1421
name: Knuckle Mastery
status: in-game
icon: skill/105
type: Passive
job: Hawker Job
max_level: 20
target: Yourself
needs_weapon: Katar, Dual Swords
skill_books:
- '[[items/consumable/752-knuckle-mastery|Knuckle Mastery]]'
- '[[items/consumable/954-special-shop-a-coupon|Special Shop A Coupon]]'
- '[[items/consumable/955-special-shop-b-coupon|Special Shop B Coupon]]'
- '[[items/consumable/956-special-shop-c-coupon|Special Shop C Coupon]]'
levels:
- level: 1
  id: 1421
  learn_points: 4
  changes: Combat Weapon Attack Power +8
- level: 2
  id: 1422
  learn_points: 5
  changes: Combat Weapon Attack Power +11
- level: 3
  id: 1423
  learn_points: 7
  changes: Combat Weapon Attack Power +14
- level: 4
  id: 1424
  learn_points: 9
  changes: Combat Weapon Attack Power +17
- level: 5
  id: 1425
  learn_points: 11
  changes: Combat Weapon Attack Power +20
- level: 6
  id: 1426
  learn_points: 13
  changes: Combat Weapon Attack Power +23
- level: 7
  id: 1427
  learn_points: 15
  changes: Combat Weapon Attack Power +26
- level: 8
  id: 1428
  learn_points: 18
  changes: Combat Weapon Attack Power +29
- level: 9
  id: 1429
  learn_points: 21
  changes: Combat Weapon Attack Power +32
- level: 10
  id: 1430
  learn_points: 24
  changes: Combat Weapon Attack Power +35
- level: 11
  id: 1431
  learn_points: 27
  changes: Combat Weapon Attack Power +39
- level: 12
  id: 1432
  learn_points: 30
  changes: Combat Weapon Attack Power +43
- level: 13
  id: 1433
  learn_points: 34
  changes: Combat Weapon Attack Power +47
- level: 14
  id: 1434
  learn_points: 38
  changes: Combat Weapon Attack Power +51
- level: 15
  id: 1435
  learn_points: 42
  changes: Combat Weapon Attack Power +55
- level: 16
  id: 1436
  learn_points: 47
  changes: Combat Weapon Attack Power +60
- level: 17
  id: 1437
  learn_points: 52
  changes: Combat Weapon Attack Power +65
- level: 18
  id: 1438
  learn_points: 58
  changes: Combat Weapon Attack Power +70
- level: 19
  id: 1439
  learn_points: 64
  changes: Combat Weapon Attack Power +75
- level: 20
  id: 1440
  learn_points: 71
  changes: Combat Weapon Attack Power +80
source:
  data: LIST_SKILL.STB rows 1421, 1422, 1423, 1424, 1425, 1426, 1427, 1428, 1429, 1430, 1431, 1432, 1433, 1434, 1435, 1436, 1437, 1438, 1439, 1440
  code: module/src/skills.rs
---
# Knuckle Mastery

Increase Attack Power of Katar and Dual Wield weapons.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
