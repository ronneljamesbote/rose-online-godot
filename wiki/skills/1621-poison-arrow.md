---
kind: skill
id: 1621
name: Poison Arrow
status: in-game
icon: skill/123
type: Magic Spell
job: Scout Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Bow, Crossbow
skill_books:
- '[[items/consumable/766-poison-arrow|Poison Arrow]]'
levels:
- level: 1
  id: 1621
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10, [[skills/1581-trap-shot|Trap Shot]] level 5'
  learn_points: 30
  cost: MP 30
  power: 60
  cooldown: 6
  duration: 12
  success: 70
  effects: Poisoned  4
- level: 2
  id: 1622
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 11'
  learn_points: 33
  cost: MP 32
  power: 65
  cooldown: 6.2
  duration: 13
  success: 72
  effects: Poisoned  4
- level: 3
  id: 1623
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 37
  cost: MP 34
  power: 70
  cooldown: 6.4
  duration: 14
  success: 74
  effects: Poisoned  4
- level: 4
  id: 1624
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 13'
  learn_points: 41
  cost: MP 36
  power: 75
  cooldown: 6.6
  duration: 15
  success: 76
  effects: Poisoned  4
- level: 5
  id: 1625
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 45
  cost: MP 38
  power: 80
  cooldown: 6.8
  duration: 16
  success: 78
  effects: Poisoned  4
- level: 6
  id: 1626
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 15'
  learn_points: 50
  cost: MP 40
  power: 85
  cooldown: 7
  duration: 17
  success: 80
  effects: Poisoned  4
- level: 7
  id: 1627
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 55
  cost: MP 42
  power: 90
  cooldown: 7.2
  duration: 18
  success: 82
  effects: Poisoned  4
- level: 8
  id: 1628
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 17'
  learn_points: 60
  cost: MP 44
  power: 95
  cooldown: 7.4
  duration: 19
  success: 84
  effects: Poisoned  4
- level: 9
  id: 1629
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 18'
  learn_points: 66
  cost: MP 46
  power: 100
  cooldown: 7.6
  duration: 20
  success: 86
  effects: Poisoned  4
- level: 10
  id: 1630
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 19'
  learn_points: 72
  cost: MP 50
  power: 110
  cooldown: 7.8
  duration: 21
  success: 90
  effects: Poisoned  4
source:
  data: LIST_SKILL.STB rows 1621, 1622, 1623, 1624, 1625, 1626, 1627, 1628, 1629, 1630
  code: module/src/skills.rs
---
# Poison Arrow

Shoot an arrow that will inflict the Poisoned status on an enemy.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
