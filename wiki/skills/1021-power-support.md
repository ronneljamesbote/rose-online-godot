---
kind: skill
id: 1021
name: Power Support
status: in-game
icon: skill/71
type: Continuing
job: Muse Job
max_level: 10
target: Ally
skill_books:
- '[[items/consumable/719-power-support|Power Support]]'
levels:
- level: 1
  id: 1021
  needs: '[[skills/821-meditation|Meditation]] level 6, [[skills/961-lesser-haste|Lesser Haste]] level 3'
  learn_points: 20
  cost: MP 30
  range: 20
  cooldown: 2.4
  duration: 300
  effects: Atk Power Increased
  changes: Attack Power +25
- level: 2
  id: 1022
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 23
  cost: MP 33
  range: 20
  cooldown: 2.4
  duration: 320
  effects: Atk Power Increased
  changes: Attack Power +30
- level: 3
  id: 1023
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 27
  cost: MP 36
  range: 20
  cooldown: 2.4
  duration: 340
  effects: Atk Power Increased
  changes: Attack Power +35
- level: 4
  id: 1024
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 31
  cost: MP 39
  range: 20
  cooldown: 2.4
  duration: 360
  effects: Atk Power Increased
  changes: Attack Power +40
- level: 5
  id: 1025
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 36
  cost: MP 42
  range: 20
  cooldown: 2.4
  duration: 380
  effects: Atk Power Increased
  changes: Attack Power +45
- level: 6
  id: 1026
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 41
  cost: MP 45
  range: 20
  cooldown: 2.4
  duration: 400
  effects: Atk Power Increased
  changes: Attack Power +50
- level: 7
  id: 1027
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 47
  cost: MP 48
  range: 20
  cooldown: 2.4
  duration: 420
  effects: Atk Power Increased
  changes: Attack Power +55
- level: 8
  id: 1028
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 54
  cost: MP 51
  range: 20
  cooldown: 2.4
  duration: 440
  effects: Atk Power Increased
  changes: Attack Power +60
- level: 9
  id: 1029
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 61
  cost: MP 54
  range: 20
  cooldown: 2.4
  duration: 460
  effects: Atk Power Increased
  changes: Attack Power +65
- level: 10
  id: 1030
  needs: '[[skills/821-meditation|Meditation]] level 19'
  learn_points: 69
  cost: MP 60
  range: 20
  cooldown: 2.4
  duration: 500
  effects: Atk Power Increased
  changes: Attack Power +70
source:
  data: LIST_SKILL.STB rows 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030
  code: module/src/skills.rs
---
# Power Support

Increase the Attack Power of a target for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
