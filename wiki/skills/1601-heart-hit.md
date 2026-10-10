---
kind: skill
id: 1601
name: Heart Hit
status: in-game
icon: skill/132
type: Magic Spell
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/764-heart-hit|Heart Hit]]'
levels:
- level: 1
  id: 1601
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 7, [[skills/1451-hawker-spirit|Hawker Spirit]] level 5'
  learn_points: 20
  cost: MP 40
  power: 80
  range: 22
  cooldown: 10
  duration: 15
  success: 30
  effects: Slow Attack
  changes: Attack Speed -30%
- level: 2
  id: 1602
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 8'
  learn_points: 25
  cost: MP 43
  power: 95
  range: 22.3
  cooldown: 10.4
  duration: 15
  success: 33
  effects: Slow Attack
  changes: Attack Speed -33%
- level: 3
  id: 1603
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 31
  cost: MP 46
  power: 110
  range: 22.6
  cooldown: 10.8
  duration: 16
  success: 36
  effects: Slow Attack
  changes: Attack Speed -36%
- level: 4
  id: 1604
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 38
  cost: MP 49
  power: 125
  range: 22.9
  cooldown: 11.2
  duration: 16
  success: 39
  effects: Slow Attack
  changes: Attack Speed -39%
- level: 5
  id: 1605
  learn_points: 46
  cost: MP 52
  power: 140
  range: 23.2
  cooldown: 11.6
  duration: 17
  success: 42
  effects: Slow Attack
  changes: Attack Speed -42%
- level: 6
  id: 1606
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 54
  cost: MP 55
  power: 155
  range: 23.5
  cooldown: 12
  duration: 17
  success: 46
  effects: Slow Attack
  changes: Attack Speed -45%
- level: 7
  id: 1607
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 64
  cost: MP 58
  power: 170
  range: 23.8
  cooldown: 12.4
  duration: 18
  success: 50
  effects: Slow Attack
  changes: Attack Speed -48%
- level: 8
  id: 1608
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 75
  cost: MP 61
  power: 185
  range: 24.1
  cooldown: 12.8
  duration: 18
  success: 54
  effects: Slow Attack
  changes: Attack Speed -51%
- level: 9
  id: 1609
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 14'
  learn_points: 88
  cost: MP 64
  power: 200
  range: 24.4
  cooldown: 13.2
  duration: 19
  success: 58
  effects: Slow Attack
  changes: Attack Speed -54%
- level: 10
  id: 1610
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 15'
  learn_points: 103
  cost: MP 70
  power: 220
  range: 26
  cooldown: 13.6
  duration: 20
  success: 62
  effects: Slow Attack
  changes: Attack Speed -60%
source:
  data: LIST_SKILL.STB rows 1601, 1602, 1603, 1604, 1605, 1606, 1607, 1608, 1609, 1610
  code: module/src/skills.rs
---
# Heart Hit

Shoot a heart shaped spirit sphere at a target from a distance. Inflict damage and decrease target's Attack Speed at the same time.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
