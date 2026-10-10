---
kind: skill
id: 1451
name: Hawker Spirit
status: in-game
icon: skill/107
type: Continuing (Self)
job: Hawker Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/754-hawker-spirit|Hawker Spirit]]'
levels:
- level: 1
  id: 1451
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 3'
  learn_points: 6
  cost: MP 20
  cooldown: 6
  duration: 150
  effects: Atk Accuracy Up
  changes: Attack Accuracy +20
- level: 2
  id: 1452
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 4'
  learn_points: 8
  cost: MP 22
  cooldown: 6
  duration: 156
  effects: Atk Accuracy Up
  changes: Attack Accuracy +25
- level: 3
  id: 1453
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 5'
  learn_points: 11
  cost: MP 24
  cooldown: 6
  duration: 162
  effects: Atk Accuracy Up
  changes: Attack Accuracy +30
- level: 4
  id: 1454
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 6'
  learn_points: 14
  cost: MP 26
  cooldown: 6
  duration: 168
  effects: Atk Accuracy Up
  changes: Attack Accuracy +35
- level: 5
  id: 1455
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 7'
  learn_points: 18
  cost: MP 28
  cooldown: 6
  duration: 174
  effects: Atk Accuracy Up
  changes: Attack Accuracy +40
- level: 6
  id: 1456
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 8'
  learn_points: 23
  cost: MP 30
  cooldown: 6
  duration: 180
  effects: Atk Accuracy Up
  changes: Attack Accuracy +45
- level: 7
  id: 1457
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 29
  cost: MP 32
  cooldown: 6
  duration: 186
  effects: Atk Accuracy Up
  changes: Attack Accuracy +50
- level: 8
  id: 1458
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 36
  cost: MP 34
  cooldown: 6
  duration: 192
  effects: Atk Accuracy Up
  changes: Attack Accuracy +55
- level: 9
  id: 1459
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 44
  cost: MP 36
  cooldown: 6
  duration: 198
  effects: Atk Accuracy Up
  changes: Attack Accuracy +60
- level: 10
  id: 1460
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 54
  cost: MP 40
  cooldown: 6
  duration: 210
  effects: Atk Accuracy Up
  changes: Attack Accuracy +80
source:
  data: LIST_SKILL.STB rows 1451, 1452, 1453, 1454, 1455, 1456, 1457, 1458, 1459, 1460
  code: module/src/skills.rs
---
# Hawker Spirit

Increase the caster's Attack Accuracy for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
