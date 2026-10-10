---
kind: skill
id: 1681
name: Speed Shot
status: in-game
icon: skill/119
type: Continuing (Self)
job: Hawker Job
max_level: 10
target: Yourself
needs_weapon: Bow, Crossbow
warps_to: '[[zones/24-el-verloon-desert|El Verloon Desert]]'
skill_books:
- '[[items/consumable/773-speed-shot|Speed Shot]]'
levels:
- level: 1
  id: 1681
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 8'
  learn_points: 15
  cost: MP 30
  cooldown: 110
  duration: 80
  effects: Haste Attack
  changes: Attack Speed +30
- level: 2
  id: 1682
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 9'
  learn_points: 18
  cost: MP 32
  cooldown: 111.8
  duration: 82
  effects: Haste Attack
  changes: Attack Speed +34
- level: 3
  id: 1683
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10'
  learn_points: 22
  cost: MP 34
  cooldown: 113.6
  duration: 84
  effects: Haste Attack
  changes: Attack Speed +38
- level: 4
  id: 1684
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 11'
  learn_points: 27
  cost: MP 36
  cooldown: 115.4
  duration: 86
  effects: Haste Attack
  changes: Attack Speed +42
- level: 5
  id: 1685
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 32
  cost: MP 38
  cooldown: 117.2
  duration: 88
  effects: Haste Attack
  changes: Attack Speed +46
- level: 6
  id: 1686
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 13'
  learn_points: 39
  cost: MP 40
  cooldown: 119
  duration: 90
  effects: Haste Attack
  changes: Attack Speed +50
- level: 7
  id: 1687
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 47
  cost: MP 42
  cooldown: 120.8
  duration: 92
  effects: Haste Attack
  changes: Attack Speed +54
- level: 8
  id: 1688
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 15'
  learn_points: 56
  cost: MP 44
  cooldown: 122.6
  duration: 94
  effects: Haste Attack
  changes: Attack Speed +58
- level: 9
  id: 1689
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 67
  cost: MP 46
  cooldown: 124.4
  duration: 96
  effects: Haste Attack
  changes: Attack Speed +62
- level: 10
  id: 1690
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 17'
  learn_points: 80
  cost: MP 50
  cooldown: 126.2
  duration: 100
  effects: Haste Attack
  changes: Attack Speed +66
source:
  data: LIST_SKILL.STB rows 1681, 1682, 1683, 1684, 1685, 1686, 1687, 1688, 1689, 1690
  code: module/src/skills.rs
---
# Speed Shot

Increase Attack Speed of Bow and Crossbow weapons for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
