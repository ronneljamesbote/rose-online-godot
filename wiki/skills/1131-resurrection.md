---
kind: skill
id: 1131
name: Resurrection
status: in-game
icon: skill/221
type: Resurrection
job: Cleric Job
max_level: 5
target: Fainted Ally
skill_books:
- '[[items/consumable/730-resurrection|Resurrection]]'
levels:
- level: 1
  id: 1131
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/931-cure|Cure]] level 5'
  learn_points: 30
  cost: MP 300
  range: 10
  cooldown: 6
- level: 2
  id: 1132
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 40
  cost: MP 450
  power: 15
  range: 10
  cooldown: 6
- level: 3
  id: 1133
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 50
  cost: MP 600
  power: 30
  range: 10
  cooldown: 6
- level: 4
  id: 1134
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 60
  cost: MP 700
  power: 40
  range: 10
  cooldown: 6
- level: 5
  id: 1135
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 70
  cost: MP 800
  power: 50
  range: 10
  cooldown: 6
source:
  data: LIST_SKILL.STB rows 1131, 1132, 1133, 1134, 1135
  code: module/src/skills.rs
---
# Resurrection

Revive a dead character on the current spot.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
