---
kind: skill
id: 1251
name: Evasion Aura
status: in-game
icon: skill/210
type: Continuing (Self)
job: Cleric Job
max_level: 10
target: Party Member
warps_to: '[[zones/22-adventurer-s-plain|Adventurer''s Plain]]'
skill_books:
- '[[items/consumable/742-evasion-aura|Evasion Aura]]'
levels:
- level: 1
  id: 1251
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/1051-blessed-mind|Blessed Mind]] level 5'
  learn_points: 35
  cost: MP 50
  area: 13
  cooldown: 4
  duration: 160
  effects: Dodge Rate Up
  changes: Dodge Rate +20
- level: 2
  id: 1252
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 39
  cost: MP 55
  area: 13.5
  cooldown: 4
  duration: 175
  effects: Dodge Rate Up
  changes: Dodge Rate +25
- level: 3
  id: 1253
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 43
  cost: MP 60
  area: 14
  cooldown: 4
  duration: 190
  effects: Dodge Rate Up
  changes: Dodge Rate +30
- level: 4
  id: 1254
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 47
  cost: MP 65
  area: 14.5
  cooldown: 4
  duration: 205
  effects: Dodge Rate Up
  changes: Dodge Rate +35
- level: 5
  id: 1255
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 52
  cost: MP 70
  area: 15
  cooldown: 4
  duration: 220
  effects: Dodge Rate Up
  changes: Dodge Rate +40
- level: 6
  id: 1256
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 57
  cost: MP 75
  area: 15.5
  cooldown: 4
  duration: 235
  effects: Dodge Rate Up
  changes: Dodge Rate +45
- level: 7
  id: 1257
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 63
  cost: MP 80
  area: 16
  cooldown: 4
  duration: 250
  effects: Dodge Rate Up
  changes: Dodge Rate +50
- level: 8
  id: 1258
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 69
  cost: MP 85
  area: 16.5
  cooldown: 4
  duration: 265
  effects: Dodge Rate Up
  changes: Dodge Rate +55
- level: 9
  id: 1259
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 76
  cost: MP 90
  area: 17
  cooldown: 4
  duration: 280
  effects: Dodge Rate Up
  changes: Dodge Rate +60
- level: 10
  id: 1260
  needs: '[[skills/821-meditation|Meditation]] level 20'
  learn_points: 83
  cost: MP 100
  area: 20
  cooldown: 4
  duration: 300
  effects: Dodge Rate Up
  changes: Dodge Rate +70
source:
  data: LIST_SKILL.STB rows 1251, 1252, 1253, 1254, 1255, 1256, 1257, 1258, 1259, 1260
  code: module/src/skills.rs
---
# Evasion Aura

Increase nearby party members' Dodge Rate.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
