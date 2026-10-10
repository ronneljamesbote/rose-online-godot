---
kind: skill
id: 971
name: Blessing
status: in-game
icon: skill/78
type: Continuing (Self)
job: Muse Job
max_level: 10
target: Party Member
warps_to: zone 38
skill_books:
- '[[items/consumable/714-blessing|Blessing]]'
levels:
- level: 1
  id: 971
  needs: '[[skills/821-meditation|Meditation]] level 5'
  learn_points: 10
  cost: MP 30
  area: 13
  cooldown: 4
  duration: 220
  effects: MaxHP Up
  changes: Max HP +200
- level: 2
  id: 972
  needs: '[[skills/821-meditation|Meditation]] level 6'
  learn_points: 13
  cost: MP 35
  area: 13.5
  cooldown: 4
  duration: 240
  effects: MaxHP Up
  changes: Max HP +240
- level: 3
  id: 973
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 16
  cost: MP 40
  area: 14
  cooldown: 4
  duration: 260
  effects: MaxHP Up
  changes: Max HP +280
- level: 4
  id: 974
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 19
  cost: MP 45
  area: 14.5
  cooldown: 4
  duration: 280
  effects: MaxHP Up
  changes: Max HP +320
- level: 5
  id: 975
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 23
  cost: MP 50
  area: 15
  cooldown: 4
  duration: 300
  effects: MaxHP Up
  changes: Max HP +360
- level: 6
  id: 976
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 27
  cost: MP 55
  area: 15.5
  cooldown: 4
  duration: 320
  effects: MaxHP Up
  changes: Max HP +400
- level: 7
  id: 977
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 32
  cost: MP 60
  area: 16
  cooldown: 4
  duration: 340
  effects: MaxHP Up
  changes: Max HP +440
- level: 8
  id: 978
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 38
  cost: MP 65
  area: 16.5
  cooldown: 4
  duration: 360
  effects: MaxHP Up
  changes: Max HP +480
- level: 9
  id: 979
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 45
  cost: MP 70
  area: 17
  cooldown: 4
  duration: 380
  effects: MaxHP Up
  changes: Max HP +520
- level: 10
  id: 980
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 53
  cost: MP 80
  area: 20
  cooldown: 4
  duration: 400
  effects: MaxHP Up
  changes: Max HP +560
source:
  data: LIST_SKILL.STB rows 971, 972, 973, 974, 975, 976, 977, 978, 979, 980
  code: module/src/skills.rs
---
# Blessing

Increase nearby party members' Maximum HP for the skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
