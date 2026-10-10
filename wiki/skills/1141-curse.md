---
kind: skill
id: 1141
name: Curse
status: in-game
icon: skill/117
type: Magic Spell
job: Muse Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/731-curse|Curse]]'
levels:
- level: 1
  id: 1141
  needs: '[[skills/821-meditation|Meditation]] level 8, [[skills/931-cure|Cure]] level 4'
  learn_points: 10
  cost: MP 15
  power: 30
  range: 25
  cooldown: 4
  duration: 3
  success: 20
  effects: Poisoned  4
- level: 2
  id: 1142
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 13
  cost: MP 17
  power: 38
  range: 25.5
  cooldown: 4
  duration: 3
  success: 20
  effects: Poisoned  4
- level: 3
  id: 1143
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 16
  cost: MP 19
  power: 46
  range: 26
  cooldown: 4
  duration: 4
  success: 20
  effects: Poisoned  4
- level: 4
  id: 1144
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 20
  cost: MP 21
  power: 54
  range: 26.5
  cooldown: 4
  duration: 4
  success: 20
  effects: Poisoned  4
- level: 5
  id: 1145
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 24
  cost: MP 23
  power: 62
  range: 27
  cooldown: 4
  duration: 5
  success: 20
  effects: Poisoned  4
- level: 6
  id: 1146
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 29
  cost: MP 25
  power: 70
  range: 27.5
  cooldown: 4
  duration: 5
  success: 20
  effects: Poisoned  4
- level: 7
  id: 1147
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 35
  cost: MP 27
  power: 78
  range: 28
  cooldown: 4
  duration: 6
  success: 20
  effects: Poisoned  4
- level: 8
  id: 1148
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 42
  cost: MP 29
  power: 86
  range: 28.5
  cooldown: 4
  duration: 6
  success: 20
  effects: Poisoned  4
- level: 9
  id: 1149
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 50
  cost: MP 31
  power: 94
  range: 29
  cooldown: 4
  duration: 7
  success: 20
  effects: Poisoned  4
- level: 10
  id: 1150
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 59
  cost: MP 35
  power: 110
  range: 30
  cooldown: 4
  duration: 8
  success: 20
  effects: Poisoned  4
source:
  data: LIST_SKILL.STB rows 1141, 1142, 1143, 1144, 1145, 1146, 1147, 1148, 1149, 1150
  code: module/src/skills.rs
---
# Curse

Decreases the HP of a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
