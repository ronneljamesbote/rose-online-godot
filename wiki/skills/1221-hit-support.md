---
kind: skill
id: 1221
name: Hit Support
status: in-game
icon: skill/211
type: 'Continuing '
job: Muse Job
max_level: 10
target: Ally
warps_to: '[[zones/20-birth-island|Birth Island]]'
skill_books:
- '[[items/consumable/739-hit-support|Hit Support]]'
levels:
- level: 1
  id: 1221
  needs: '[[skills/821-meditation|Meditation]] level 8, [[skills/961-lesser-haste|Lesser Haste]] level 4'
  learn_points: 20
  cost: MP 30
  range: 20
  cooldown: 2.4
  duration: 300
  effects: Atk Accuracy Up
  changes: Attack Accuracy +30
- level: 2
  id: 1222
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 23
  cost: MP 34
  range: 20
  cooldown: 2.4
  duration: 320
  effects: Atk Accuracy Up
  changes: Attack Accuracy +35
- level: 3
  id: 1223
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 26
  cost: MP 38
  range: 20
  cooldown: 2.4
  duration: 340
  effects: Atk Accuracy Up
  changes: Attack Accuracy +40
- level: 4
  id: 1224
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 30
  cost: MP 42
  range: 20
  cooldown: 2.4
  duration: 360
  effects: Atk Accuracy Up
  changes: Attack Accuracy +45
- level: 5
  id: 1225
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 34
  cost: MP 46
  range: 20
  cooldown: 2.4
  duration: 380
  effects: Atk Accuracy Up
  changes: Attack Accuracy +50
- level: 6
  id: 1226
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 38
  cost: MP 50
  range: 20
  cooldown: 2.4
  duration: 400
  effects: Atk Accuracy Up
  changes: Attack Accuracy +55
- level: 7
  id: 1227
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 43
  cost: MP 54
  range: 20
  cooldown: 2.4
  duration: 420
  effects: Atk Accuracy Up
  changes: Attack Accuracy +60
- level: 8
  id: 1228
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 48
  cost: MP 58
  range: 20
  cooldown: 2.4
  duration: 440
  effects: Atk Accuracy Up
  changes: Attack Accuracy +65
- level: 9
  id: 1229
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 54
  cost: MP 62
  range: 20
  cooldown: 2.4
  duration: 460
  effects: Atk Accuracy Up
  changes: Attack Accuracy +70
- level: 10
  id: 1230
  needs: '[[skills/821-meditation|Meditation]] level 20'
  learn_points: 60
  cost: MP 66
  range: 20
  cooldown: 2.4
  duration: 500
  effects: Atk Accuracy Up
  changes: Attack Accuracy +80
source:
  data: LIST_SKILL.STB rows 1221, 1222, 1223, 1224, 1225, 1226, 1227, 1228, 1229, 1230
  code: module/src/skills.rs
---
# Hit Support

Increase target's Attack Accuracy for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
