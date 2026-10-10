---
kind: skill
id: 411
name: Taunt
status: in-game
icon: skill/62
type: 'Continuing '
job: Soldier Job
max_level: 3
target: Hostile Monster
skill_books:
- '[[items/consumable/667-taunt|Taunt]]'
levels:
- level: 1
  id: 411
  needs: '[[skills/251-defense-training|Defense Training]] level 5'
  learn_points: 15
  cost: MP 20
  range: 16
  cooldown: 6
  duration: 15
  success: 60
  effects: Taunt
- level: 2
  id: 412
  needs: '[[skills/251-defense-training|Defense Training]] level 7'
  learn_points: 20
  cost: MP 24
  range: 20
  cooldown: 5.2
  duration: 22
  success: 75
  effects: Taunt
- level: 3
  id: 413
  needs: '[[skills/251-defense-training|Defense Training]] level 10'
  learn_points: 25
  cost: MP 28
  range: 25
  cooldown: 4.6
  duration: 25
  success: 90
  effects: Taunt
source:
  data: LIST_SKILL.STB rows 411, 412, 413
  code: module/src/skills.rs
---
# Taunt

Directs target's hate towards caster, causing target to attack caster for a while.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
