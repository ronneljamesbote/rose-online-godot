---
kind: skill
id: 2201
name: Mighty Shot
status: in-game
icon: skill/145
type: Damage Action
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/816-mighty-shot|Mighty Shot]]'
levels:
- level: 1
  id: 2201
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 1'
  learn_points: 8
  cost: MP 10
  power: 85
  cooldown: 7
- level: 2
  id: 2202
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 2'
  learn_points: 10
  cost: MP 12
  power: 102
  cooldown: 7
- level: 3
  id: 2203
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 3'
  learn_points: 12
  cost: MP 14
  power: 119
  cooldown: 7.2
- level: 4
  id: 2204
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 4'
  learn_points: 14
  cost: MP 16
  power: 136
  cooldown: 7.2
- level: 5
  id: 2205
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 5'
  learn_points: 16
  cost: MP 18
  power: 153
  cooldown: 7.4
- level: 6
  id: 2206
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 6'
  learn_points: 18
  cost: MP 20
  power: 170
  cooldown: 7.4
- level: 7
  id: 2207
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 7'
  learn_points: 20
  cost: MP 22
  power: 186
  cooldown: 7.6
- level: 8
  id: 2208
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 8'
  learn_points: 23
  cost: MP 24
  power: 206
  cooldown: 7.6
- level: 9
  id: 2209
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 9'
  learn_points: 26
  cost: MP 26
  power: 228
  cooldown: 7.8
- level: 10
  id: 2210
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 10'
  learn_points: 29
  cost: MP 30
  power: 266
  cooldown: 7.8
source:
  data: godot/data/skills/2201-mighty-shot/ (levels 2201, 2202, 2203, 2204, 2205, 2206, 2207, 2208, 2209, 2210)
  code: module/src/skills.rs
---
# Mighty Shot

Increase Gun and Launcher weapon damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
