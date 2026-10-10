---
kind: skill
id: 1051
name: Blessed Mind
status: in-game
icon: skill/95
type: Continuing (Self)
job: Muse Job
max_level: 10
target: Party Member
warps_to: zone 39
skill_books:
- '[[items/consumable/722-blessed-mind|Blessed Mind]]'
levels:
- level: 1
  id: 1051
  needs: '[[skills/821-meditation|Meditation]] level 9, [[skills/971-blessing|Blessing]] level 5'
  learn_points: 20
  cost: MP 40
  area: 13
  cooldown: 4
  duration: 220
  effects: MaxMP Up
  changes: Max MP +150
- level: 2
  id: 1052
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 23
  cost: MP 45
  area: 13.5
  cooldown: 4
  duration: 240
  effects: MaxMP Up
  changes: Max MP +175
- level: 3
  id: 1053
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 26
  cost: MP 50
  area: 14
  cooldown: 4
  duration: 260
  effects: MaxMP Up
  changes: Max MP +200
- level: 4
  id: 1054
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 30
  cost: MP 55
  area: 14.5
  cooldown: 4
  duration: 280
  effects: MaxMP Up
  changes: Max MP +225
- level: 5
  id: 1055
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 34
  cost: MP 60
  area: 15
  cooldown: 4
  duration: 300
  effects: MaxMP Up
  changes: Max MP +250
- level: 6
  id: 1056
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 38
  cost: MP 65
  area: 15.5
  cooldown: 4
  duration: 320
  effects: MaxMP Up
  changes: Max MP +275
- level: 7
  id: 1057
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 43
  cost: MP 70
  area: 16
  cooldown: 4
  duration: 340
  effects: MaxMP Up
  changes: Max MP +300
- level: 8
  id: 1058
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 48
  cost: MP 75
  area: 16.5
  cooldown: 4
  duration: 360
  effects: MaxMP Up
  changes: Max MP +330
- level: 9
  id: 1059
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 54
  cost: MP 80
  area: 17
  cooldown: 4
  duration: 380
  effects: MaxMP Up
  changes: Max MP +360
- level: 10
  id: 1060
  needs: '[[skills/821-meditation|Meditation]] level 20'
  learn_points: 60
  cost: MP 90
  area: 20
  cooldown: 4
  duration: 400
  effects: MaxMP Up
  changes: Max MP +400
source:
  data: LIST_SKILL.STB rows 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060
  code: module/src/skills.rs
---
# Blessed Mind

Increase nearby party members' Maximum MP for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
