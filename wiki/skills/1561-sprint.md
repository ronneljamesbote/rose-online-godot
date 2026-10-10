---
kind: skill
id: 1561
name: Sprint
status: in-game
icon: skill/108
type: Continuing (Self)
job: Hawker Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/760-sprint|Sprint]]'
levels:
- level: 1
  id: 1561
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 4'
  learn_points: 10
  cost: MP 25
  cooldown: 6
  duration: 150
  effects: Dash
  changes: Movement Speed +85
- level: 2
  id: 1562
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 5'
  learn_points: 12
  cost: MP 28
  cooldown: 6
  duration: 155
  effects: Dash
  changes: Movement Speed +100
- level: 3
  id: 1563
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 6'
  learn_points: 15
  cost: MP 31
  cooldown: 6
  duration: 160
  effects: Dash
  changes: Movement Speed +115
- level: 4
  id: 1564
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 7'
  learn_points: 18
  cost: MP 34
  cooldown: 6
  duration: 165
  effects: Dash
  changes: Movement Speed +130
- level: 5
  id: 1565
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 8'
  learn_points: 22
  cost: MP 37
  cooldown: 6
  duration: 170
  effects: Dash
  changes: Movement Speed +145
- level: 6
  id: 1566
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 9'
  learn_points: 26
  cost: MP 40
  cooldown: 6
  duration: 175
  effects: Dash
  changes: Movement Speed +160
- level: 7
  id: 1567
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10'
  learn_points: 31
  cost: MP 43
  cooldown: 6
  duration: 180
  effects: Dash
  changes: Movement Speed +180
- level: 8
  id: 1568
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 36
  cost: MP 46
  cooldown: 6
  duration: 185
  effects: Dash
  changes: Movement Speed +200
- level: 9
  id: 1569
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 42
  cost: MP 49
  cooldown: 6
  duration: 190
  effects: Dash
  changes: Movement Speed +220
- level: 10
  id: 1570
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 49
  cost: MP 55
  cooldown: 6
  duration: 200
  effects: Dash
  changes: Movement Speed +240
source:
  data: LIST_SKILL.STB rows 1561, 1562, 1563, 1564, 1565, 1566, 1567, 1568, 1569, 1570
  code: module/src/skills.rs
---
# Sprint

Increase the caster's Movement Speed for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
