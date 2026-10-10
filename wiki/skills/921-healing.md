---
kind: skill
id: 921
name: Healing
status: in-game
icon: skill/68
type: Magic Recovery (Self)
job: Muse Job
max_level: 10
target: Party Member
warps_to: zone 16
skill_books:
- '[[items/consumable/709-healing|Healing]]'
levels:
- level: 1
  id: 921
  needs: '[[skills/821-meditation|Meditation]] level 4, [[skills/931-cure|Cure]] level 2'
  learn_points: 10
  cost: MP 35
  area: 13
  cooldown: 7
  changes: HP +160
- level: 2
  id: 922
  needs: '[[skills/821-meditation|Meditation]] level 5'
  learn_points: 13
  cost: MP 40
  area: 13.5
  cooldown: 6.8
  changes: HP +200
- level: 3
  id: 923
  needs: '[[skills/821-meditation|Meditation]] level 6'
  learn_points: 17
  cost: MP 45
  area: 14
  cooldown: 6.6
  changes: HP +240
- level: 4
  id: 924
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 21
  cost: MP 50
  area: 14.5
  cooldown: 6.4
  changes: HP +280
- level: 5
  id: 925
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 26
  cost: MP 55
  area: 15
  cooldown: 6.2
  changes: HP +320
- level: 6
  id: 926
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 32
  cost: MP 60
  area: 15.5
  cooldown: 6
  changes: HP +360
- level: 7
  id: 927
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 38
  cost: MP 65
  area: 16
  cooldown: 5.8
  changes: HP +400
- level: 8
  id: 928
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 45
  cost: MP 70
  area: 16.5
  cooldown: 5.6
  changes: HP +440
- level: 9
  id: 929
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 53
  cost: MP 75
  area: 17
  cooldown: 5.4
  changes: HP +480
- level: 10
  id: 930
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 62
  cost: MP 85
  area: 20
  cooldown: 5.2
  changes: HP +520
source:
  data: LIST_SKILL.STB rows 921, 922, 923, 924, 925, 926, 927, 928, 929, 930
  code: module/src/skills.rs
---
# Healing

Restore HP of the caster and nearby party members at the same time.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
