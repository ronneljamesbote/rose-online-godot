---
kind: skill
id: 1671
name: Detect
status: in-game
icon: skill/120
type: Continuing (Self)
job: Scout Job
max_level: 3
target: Hostile Character
skill_books:
- '[[items/consumable/772-detect|Detect]]'
levels:
- level: 1
  id: 1671
  needs: '[[skills/1441-relax|Relax]] level 5'
  learn_points: 30
  cost: MP 30
  area: 10
  cooldown: 6
  duration: 2
  effects: Detection
- level: 2
  id: 1672
  needs: '[[skills/1441-relax|Relax]] level 6'
  learn_points: 30
  cost: MP 40
  area: 12
  cooldown: 5
  duration: 2
  effects: Detection
- level: 3
  id: 1673
  needs: '[[skills/1441-relax|Relax]] level 7'
  learn_points: 30
  cost: MP 50
  area: 15
  cooldown: 4
  duration: 2
  effects: Detection
source:
  data: LIST_SKILL.STB rows 1671, 1672, 1673
  code: module/src/skills.rs
---
# Detect

Detect hidden enemies around the caster.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
