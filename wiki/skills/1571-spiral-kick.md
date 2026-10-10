---
kind: skill
id: 1571
name: Spiral Kick
status: in-game
icon: skill/113
type: Area Effect Attack (Self)
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/761-spiral-kick|Spiral Kick]]'
levels:
- level: 1
  id: 1571
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 8, [[skills/1561-sprint|Sprint]] level 5'
  learn_points: 15
  cost: MP 40
  power: 50
  area: 6
  cooldown: 14
- level: 2
  id: 1572
  learn_points: 19
  cost: MP 44
  power: 58
  area: 6.1
  cooldown: 14.6
- level: 3
  id: 1573
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 24
  cost: MP 48
  power: 66
  area: 6.2
  cooldown: 15.2
- level: 4
  id: 1574
  learn_points: 30
  cost: MP 52
  power: 74
  area: 6.3
  cooldown: 15.8
- level: 5
  id: 1575
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 37
  cost: MP 56
  power: 82
  area: 6.4
  cooldown: 16.4
- level: 6
  id: 1576
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 45
  cost: MP 60
  power: 90
  area: 6.5
  cooldown: 17
- level: 7
  id: 1577
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 55
  cost: MP 64
  power: 98
  area: 6.6
  cooldown: 17.6
- level: 8
  id: 1578
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 67
  cost: MP 68
  power: 106
  area: 6.7
  cooldown: 18.2
- level: 9
  id: 1579
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 14'
  learn_points: 81
  cost: MP 72
  power: 114
  area: 6.8
  cooldown: 18.8
- level: 10
  id: 1580
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 15'
  learn_points: 98
  cost: MP 80
  power: 130
  area: 7
  cooldown: 19.4
source:
  data: LIST_SKILL.STB rows 1571, 1572, 1573, 1574, 1575, 1576, 1577, 1578, 1579, 1580
  code: module/src/skills.rs
---
# Spiral Kick

Enables its caster to spin and kick on a spot to do splash damage to nearby enemies.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
