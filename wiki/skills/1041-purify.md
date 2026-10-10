---
kind: skill
id: 1041
name: Purify
status: in-game
icon: skill/206
type: Continuing
job: Cleric Job
max_level: 3
target: Ally
skill_books:
- '[[items/consumable/721-purify|Purify]]'
levels:
- level: 1
  id: 1041
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/931-cure|Cure]] level 5'
  learn_points: 30
  cost: MP 30
  range: 16
  cooldown: 4
  duration: 2
  success: 60
  effects: Recovery
- level: 2
  id: 1042
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 34
  cost: MP 35
  range: 20
  cooldown: 3.8
  duration: 2
  success: 80
  effects: Recovery
- level: 3
  id: 1043
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 38
  cost: MP 40
  range: 25
  cooldown: 3.6
  duration: 2
  success: 100
  effects: Recovery
source:
  data: LIST_SKILL.STB rows 1041, 1042, 1043
  code: module/src/skills.rs
---
# Purify

Cancel harmful curse magic cast on a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
