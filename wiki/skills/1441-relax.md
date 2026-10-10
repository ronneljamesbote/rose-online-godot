---
kind: skill
id: 1441
name: Relax
status: in-game
icon: skill/114
type: Passive
job: Hawker Job
max_level: 10
target: Yourself
warps_to: '[[zones/56-forgotten-temple-b1|Forgotten Temple (B1)]]'
skill_books:
- '[[items/consumable/753-relax|Relax]]'
levels:
- level: 1
  id: 1441
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 2'
  learn_points: 6
  changes: HP Recovery Amount +6
- level: 2
  id: 1442
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 3'
  learn_points: 8
  changes: HP Recovery Amount +12
- level: 3
  id: 1443
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 4'
  learn_points: 11
  changes: HP Recovery Amount +18
- level: 4
  id: 1444
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 5'
  learn_points: 15
  changes: HP Recovery Amount +24
- level: 5
  id: 1445
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 6'
  learn_points: 19
  changes: HP Recovery Amount +30
- level: 6
  id: 1446
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 7'
  learn_points: 24
  changes: HP Recovery Amount +36, MP Recovery Rate +5
- level: 7
  id: 1447
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 8'
  learn_points: 30
  changes: HP Recovery Amount +42, MP Recovery Rate +10
- level: 8
  id: 1448
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 37
  changes: HP Recovery Amount +48, MP Recovery Rate +15
- level: 9
  id: 1449
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 45
  changes: HP Recovery Amount +54, MP Recovery Rate +20
- level: 10
  id: 1450
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 55
  changes: HP Recovery Amount +60, MP Recovery Rate +25
source:
  data: LIST_SKILL.STB rows 1441, 1442, 1443, 1444, 1445, 1446, 1447, 1448, 1449, 1450
  code: module/src/skills.rs
---
# Relax

Increase HP Recovery Rate while resting.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
