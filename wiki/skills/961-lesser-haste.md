---
kind: skill
id: 961
name: Lesser Haste
status: in-game
icon: skill/81
type: Continuing
job: Muse Job
max_level: 10
target: Ally
skill_books:
- '[[items/consumable/713-lesser-haste|Lesser Haste]]'
levels:
- level: 1
  id: 961
  needs: '[[skills/821-meditation|Meditation]] level 4'
  learn_points: 8
  cost: MP 30
  range: 20
  cooldown: 2.8
  duration: 300
  effects: Dash
  changes: Movement Speed +100
- level: 2
  id: 962
  needs: '[[skills/821-meditation|Meditation]] level 5'
  learn_points: 10
  cost: MP 33
  range: 20
  cooldown: 2.8
  duration: 320
  effects: Dash
  changes: Movement Speed +110
- level: 3
  id: 963
  needs: '[[skills/821-meditation|Meditation]] level 6'
  learn_points: 12
  cost: MP 36
  range: 20
  cooldown: 2.8
  duration: 340
  effects: Dash
  changes: Movement Speed +120
- level: 4
  id: 964
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 15
  cost: MP 39
  range: 20
  cooldown: 2.8
  duration: 360
  effects: Dash
  changes: Movement Speed +130
- level: 5
  id: 965
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 18
  cost: MP 42
  range: 20
  cooldown: 2.8
  duration: 380
  effects: Dash
  changes: Movement Speed +140
- level: 6
  id: 966
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 22
  cost: MP 45
  range: 20
  cooldown: 2.8
  duration: 400
  effects: Dash
  changes: Movement Speed +150
- level: 7
  id: 967
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 26
  cost: MP 48
  range: 20
  cooldown: 2.8
  duration: 420
  effects: Dash
  changes: Movement Speed +160
- level: 8
  id: 968
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 31
  cost: MP 51
  range: 20
  cooldown: 2.8
  duration: 440
  effects: Dash
  changes: Movement Speed +170
- level: 9
  id: 969
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 36
  cost: MP 54
  range: 20
  cooldown: 2.8
  duration: 460
  effects: Dash
  changes: Movement Speed +180
- level: 10
  id: 970
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 42
  cost: MP 57
  range: 20
  cooldown: 2.8
  duration: 500
  effects: Dash
  changes: Movement Speed +200
source:
  data: LIST_SKILL.STB rows 961, 962, 963, 964, 965, 966, 967, 968, 969, 970
  code: module/src/skills.rs
---
# Lesser Haste

Increase the Movement Speed of a target for the skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
