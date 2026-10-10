---
kind: skill
id: 1701
name: Hawk Shot
status: in-game
icon: skill/121
type: Magic Spell
job: Scout Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Bow
skill_books:
- '[[items/consumable/775-hawk-shot|Hawk Shot]]'
levels:
- level: 1
  id: 1701
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10, [[skills/1481-aim-shot|Aim Shot]] level 10'
  learn_points: 35
  cost: MP 40
  power: 100
  range: 36
  cooldown: 9.2
- level: 2
  id: 1702
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 11'
  learn_points: 41
  cost: MP 45
  power: 120
  range: 36.5
  cooldown: 9.8
- level: 3
  id: 1703
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 48
  cost: MP 50
  power: 140
  range: 37
  cooldown: 10.4
- level: 4
  id: 1704
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 13'
  learn_points: 56
  cost: MP 55
  power: 160
  range: 37.5
  cooldown: 11
- level: 5
  id: 1705
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 65
  cost: MP 60
  power: 180
  range: 38
  cooldown: 11.6
- level: 6
  id: 1706
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 15'
  learn_points: 75
  cost: MP 70
  power: 200
  range: 38.2
  cooldown: 13
  duration: 6
  success: 30
  effects: Fainted
- level: 7
  id: 1707
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 87
  cost: MP 75
  power: 220
  range: 38.4
  cooldown: 13.6
  duration: 7
  success: 34
  effects: Fainted
- level: 8
  id: 1708
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 17'
  learn_points: 100
  cost: MP 80
  power: 240
  range: 38.6
  cooldown: 14.2
  duration: 8
  success: 38
  effects: Fainted
- level: 9
  id: 1709
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 18'
  learn_points: 115
  cost: MP 85
  power: 260
  range: 38.8
  cooldown: 14.8
  duration: 9
  success: 42
  effects: Fainted
- level: 10
  id: 1710
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 20'
  learn_points: 132
  cost: MP 90
  power: 300
  range: 40
  cooldown: 15.4
  duration: 10
  success: 50
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 1701, 1702, 1703, 1704, 1705, 1706, 1707, 1708, 1709, 1710
  code: module/src/skills.rs
---
# Hawk Shot

Attack an enemy from a long distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
