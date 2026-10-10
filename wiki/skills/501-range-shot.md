---
kind: skill
id: 501
name: Range Shot
status: in-game
icon: skill/109
type: Magic Spell
job: Knight Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Crossbow
skill_books:
- '[[items/consumable/675-range-shot|Range Shot]]'
levels:
- level: 1
  id: 501
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 6'
  learn_points: 15
  cost: MP 50
  power: 100
  range: 32
  cooldown: 12
- level: 2
  id: 502
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 7'
  learn_points: 18
  cost: MP 55
  power: 115
  range: 32.5
  cooldown: 12.4
- level: 3
  id: 503
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 8'
  learn_points: 22
  cost: MP 60
  power: 130
  range: 33
  cooldown: 12.8
- level: 4
  id: 504
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 9'
  learn_points: 27
  cost: MP 65
  power: 145
  range: 33.5
  cooldown: 13.2
- level: 5
  id: 505
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 10'
  learn_points: 32
  cost: MP 70
  power: 160
  range: 34
  cooldown: 13.6
- level: 6
  id: 506
  learn_points: 38
  cost: MP 75
  power: 175
  range: 34.2
  cooldown: 14
- level: 7
  id: 507
  learn_points: 45
  cost: MP 80
  power: 190
  range: 34.4
  cooldown: 14.4
- level: 8
  id: 508
  learn_points: 53
  cost: MP 85
  power: 205
  range: 34.6
  cooldown: 14.8
- level: 9
  id: 509
  learn_points: 62
  cost: MP 90
  power: 220
  range: 34.8
  cooldown: 15.2
- level: 10
  id: 510
  learn_points: 73
  cost: MP 100
  power: 250
  range: 35
  cooldown: 15.6
source:
  data: LIST_SKILL.STB rows 501, 502, 503, 504, 505, 506, 507, 508, 509, 510
  code: module/src/skills.rs
---
# Range Shot

Fire a powerful arrow shot at a target from a distance. Requires a Crossbow.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
