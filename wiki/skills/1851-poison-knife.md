---
kind: skill
id: 1851
name: Poison Knife
status: in-game
icon: skill/128
type: Magic Spell
job: Raider Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/790-poison-knife|Poison Knife]]'
levels:
- level: 1
  id: 1851
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 3'
  learn_points: 30
  cost: MP 50
  power: 120
  range: 25
  cooldown: 10
  duration: 12
  success: 50
  effects: Poisoned  3
- level: 2
  id: 1852
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 4'
  learn_points: 33
  cost: MP 53
  power: 130
  range: 25.5
  cooldown: 9.8
  duration: 14
  success: 54
  effects: Poisoned  3
- level: 3
  id: 1853
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 5'
  learn_points: 37
  cost: MP 56
  power: 140
  range: 26
  cooldown: 9.6
  duration: 16
  success: 58
  effects: Poisoned  3
- level: 4
  id: 1854
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 6'
  learn_points: 41
  cost: MP 59
  power: 150
  range: 26.5
  cooldown: 9.4
  duration: 18
  success: 62
  effects: Poisoned  3
- level: 5
  id: 1855
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 7'
  learn_points: 45
  cost: MP 62
  power: 160
  range: 27
  cooldown: 9.2
  duration: 20
  success: 66
  effects: Poisoned  3
- level: 6
  id: 1856
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 8'
  learn_points: 50
  cost: MP 65
  power: 170
  range: 27.5
  cooldown: 9
  duration: 22
  success: 70
  effects: Poisoned  4
- level: 7
  id: 1857
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 9'
  learn_points: 55
  cost: MP 68
  power: 180
  range: 28
  cooldown: 8.8
  duration: 24
  success: 74
  effects: Poisoned  4
- level: 8
  id: 1858
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 10'
  learn_points: 60
  cost: MP 71
  power: 190
  range: 28.5
  cooldown: 8.6
  duration: 26
  success: 78
  effects: Poisoned  4
- level: 9
  id: 1859
  learn_points: 66
  cost: MP 74
  power: 200
  range: 29
  cooldown: 8.4
  duration: 28
  success: 82
  effects: Poisoned  4
- level: 10
  id: 1860
  learn_points: 72
  cost: MP 80
  power: 220
  range: 30
  cooldown: 8.2
  duration: 30
  success: 90
  effects: Poisoned  4
source:
  data: LIST_SKILL.STB rows 1851, 1852, 1853, 1854, 1855, 1856, 1857, 1858, 1859, 1860
  code: module/src/skills.rs
---
# Poison Knife

Throw a poison shuriken at a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
