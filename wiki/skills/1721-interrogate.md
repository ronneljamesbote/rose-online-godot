---
kind: skill
id: 1721
name: Interrogate
status: in-game
icon: skill/122
type: Continuing
job: Hawker Job
max_level: 10
target: Monster
skill_books:
- '[[items/consumable/777-interrogate|Interrogate]]'
levels:
- level: 1
  id: 1721
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 20
  cost: MP 20
  range: 20
  cooldown: 6
  duration: 16
  success: 80
  effects: Sleep
- level: 2
  id: 1722
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 24
  cost: MP 22
  range: 20.5
  cooldown: 5.6
  duration: 18
  success: 85
  effects: Sleep
- level: 3
  id: 1723
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 29
  cost: MP 24
  range: 21
  cooldown: 5.2
  duration: 20
  success: 90
  effects: Sleep
- level: 4
  id: 1724
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 35
  cost: MP 26
  range: 21.5
  cooldown: 4.8
  duration: 22
  success: 92
  effects: Sleep
- level: 5
  id: 1725
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 42
  cost: MP 28
  range: 22
  cooldown: 4.4
  duration: 24
  success: 94
  effects: Sleep
- level: 6
  id: 1726
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 14'
  learn_points: 50
  cost: MP 30
  range: 22.5
  area: 6
  cooldown: 7
  duration: 26
  success: 96
  effects: Sleep
- level: 7
  id: 1727
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 15'
  learn_points: 59
  cost: MP 32
  range: 23
  area: 6
  cooldown: 6.6
  duration: 28
  success: 98
  effects: Sleep
- level: 8
  id: 1728
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 16'
  learn_points: 69
  cost: MP 34
  range: 23.5
  area: 6
  cooldown: 6.2
  duration: 30
  success: 100
  effects: Sleep
- level: 9
  id: 1729
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 17'
  learn_points: 81
  cost: MP 36
  range: 24
  area: 6
  cooldown: 5.8
  duration: 32
  success: 102
  effects: Sleep
- level: 10
  id: 1730
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 18'
  learn_points: 95
  cost: MP 40
  range: 25
  area: 7
  cooldown: 5.4
  duration: 34
  success: 104
  effects: Sleep
source:
  data: LIST_SKILL.STB rows 1721, 1722, 1723, 1724, 1725, 1726, 1727, 1728, 1729, 1730
  code: module/src/skills.rs
---
# Interrogate

Threaten a target to run away from the caster.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
