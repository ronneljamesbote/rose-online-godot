---
kind: skill
id: 1581
name: Trap Shot
status: in-game
icon: skill/115
type: Magic Spell
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Bow, Crossbow
warps_to: '[[zones/23-breezy-hills|Breezy Hills]]'
skill_books:
- '[[items/consumable/762-trap-shot|Trap Shot]]'
levels:
- level: 1
  id: 1581
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 5'
  learn_points: 10
  cost: MP 20
  power: 30
  cooldown: 8.4
  duration: 20
  success: 70
  effects: Slow
  changes: Movement Speed +40%
- level: 2
  id: 1582
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 6'
  learn_points: 14
  cost: MP 23
  power: 36
  cooldown: 8.6
  duration: 21
  success: 72
  effects: Slow
  changes: Movement Speed +42%
- level: 3
  id: 1583
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 7'
  learn_points: 18
  cost: MP 26
  power: 42
  cooldown: 8.8
  duration: 22
  success: 74
  effects: Slow
  changes: Movement Speed +44%
- level: 4
  id: 1584
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 8'
  learn_points: 23
  cost: MP 29
  power: 48
  cooldown: 9
  duration: 23
  success: 76
  effects: Slow
  changes: Movement Speed +46%
- level: 5
  id: 1585
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 9'
  learn_points: 30
  cost: MP 32
  power: 54
  cooldown: 9.2
  duration: 24
  success: 78
  effects: Slow
  changes: Movement Speed +48%
- level: 6
  id: 1586
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10'
  learn_points: 38
  cost: MP 35
  power: 60
  cooldown: 10
  duration: 24
  success: 80
  effects: Slow, Slow Attack
  changes: Movement Speed +49%, Attack Speed +30%
- level: 7
  id: 1587
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 47
  cost: MP 38
  power: 66
  cooldown: 10.4
  duration: 25
  success: 82
  effects: Slow, Slow Attack
  changes: Movement Speed +50%, Attack Speed +35%
- level: 8
  id: 1588
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 58
  cost: MP 41
  power: 72
  cooldown: 10.8
  duration: 25
  success: 84
  effects: Slow, Slow Attack
  changes: Movement Speed +51%, Attack Speed +40%
- level: 9
  id: 1589
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 72
  cost: MP 44
  power: 78
  cooldown: 11.2
  duration: 26
  success: 86
  effects: Slow, Slow Attack
  changes: Movement Speed +52%, Attack Speed +45%
- level: 10
  id: 1590
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 18'
  learn_points: 89
  cost: MP 50
  power: 85
  cooldown: 11.6
  duration: 26
  success: 90
  effects: Slow, Slow Attack
  changes: Movement Speed +53%, Attack Speed +50%
source:
  data: LIST_SKILL.STB rows 1581, 1582, 1583, 1584, 1585, 1586, 1587, 1588, 1589, 1590
  code: module/src/skills.rs
---
# Trap Shot

Shoot an arrow to bind a target to the ground, decreasing target's Movement Speed.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
