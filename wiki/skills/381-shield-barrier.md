---
kind: skill
id: 381
name: Shield Barrier
status: in-game
icon: skill/36
type: Continuing (Self)
job: Soldier Job
max_level: 10
target: Party Member
needs_weapon: Shield
warps_to: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
skill_books:
- '[[items/consumable/664-shield-barrier|Shield Barrier]]'
levels:
- level: 1
  id: 381
  needs: '[[skills/251-defense-training|Defense Training]] level 5'
  learn_points: 8
  cost: MP 30
  area: 15
  cooldown: 10
  duration: 120
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +30
- level: 2
  id: 382
  needs: '[[skills/251-defense-training|Defense Training]] level 6'
  learn_points: 10
  cost: MP 35
  area: 15
  cooldown: 10
  duration: 125
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +40
- level: 3
  id: 383
  needs: '[[skills/251-defense-training|Defense Training]] level 7'
  learn_points: 13
  cost: MP 40
  area: 15
  cooldown: 10
  duration: 130
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +50
- level: 4
  id: 384
  needs: '[[skills/251-defense-training|Defense Training]] level 8'
  learn_points: 16
  cost: MP 45
  area: 15
  cooldown: 10
  duration: 135
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +60
- level: 5
  id: 385
  needs: '[[skills/251-defense-training|Defense Training]] level 10'
  learn_points: 20
  cost: MP 50
  area: 15
  cooldown: 10
  duration: 140
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +70
- level: 6
  id: 386
  needs: '[[skills/251-defense-training|Defense Training]] level 12'
  learn_points: 25
  cost: MP 55
  area: 15
  cooldown: 10
  duration: 145
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +80
- level: 7
  id: 387
  needs: '[[skills/251-defense-training|Defense Training]] level 14'
  learn_points: 32
  cost: MP 60
  area: 15
  cooldown: 10
  duration: 150
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +90
- level: 8
  id: 388
  needs: '[[skills/251-defense-training|Defense Training]] level 16'
  learn_points: 40
  cost: MP 65
  area: 15
  cooldown: 10
  duration: 155
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +100
- level: 9
  id: 389
  needs: '[[skills/251-defense-training|Defense Training]] level 18'
  learn_points: 50
  cost: MP 70
  area: 15
  cooldown: 10
  duration: 160
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +110
- level: 10
  id: 390
  needs: '[[skills/251-defense-training|Defense Training]] level 20'
  learn_points: 62
  cost: MP 80
  area: 15
  cooldown: 10
  duration: 170
  success: 100
  effects: Magic Resistance Up
  changes: Magic Resistance +130
source:
  data: LIST_SKILL.STB rows 381, 382, 383, 384, 385, 386, 387, 388, 389, 390
  code: module/src/skills.rs
---
# Shield Barrier

Increase Magic Resistance of the caster and party members for skill’s duration. Requires a Shield.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
