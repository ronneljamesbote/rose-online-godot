---
kind: skill
id: 2221
name: Twin Bullets
status: in-game
icon: skill/172
type: Damage Action
job: Dealer Job
max_level: 20
target: Hostile Character
damage_type: continuous attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/817-twin-bullets|Twin Bullets]]'
levels:
- level: 1
  id: 2221
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 1'
  learn_points: 8
  cost: MP 20
  power: 40
  cooldown: 6
- level: 2
  id: 2222
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 2'
  learn_points: 10
  cost: MP 22
  power: 45
  cooldown: 6
- level: 3
  id: 2223
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 3'
  learn_points: 12
  cost: MP 24
  power: 50
  cooldown: 5.8
- level: 4
  id: 2224
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 4'
  learn_points: 14
  cost: MP 26
  power: 55
  cooldown: 5.8
- level: 5
  id: 2225
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 5'
  learn_points: 16
  cost: MP 28
  power: 60
  cooldown: 5.6
- level: 6
  id: 2226
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 6'
  learn_points: 19
  cost: MP 30
  power: 65
  cooldown: 5.6
- level: 7
  id: 2227
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 7'
  learn_points: 22
  cost: MP 32
  power: 70
  cooldown: 5.4
- level: 8
  id: 2228
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 8'
  learn_points: 25
  cost: MP 34
  power: 75
  cooldown: 5.4
- level: 9
  id: 2229
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 9'
  learn_points: 29
  cost: MP 36
  power: 82
  cooldown: 5.2
- level: 10
  id: 2230
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 10'
  learn_points: 33
  cost: MP 38
  power: 90
  cooldown: 5.2
- level: 11
  id: 2231
  needs_level: 70
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 11'
  learn_points: 37
  cost: MP 40
  power: 45
  cooldown: 7
- level: 12
  id: 2232
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 12'
  learn_points: 41
  cost: MP 42
  power: 50
  cooldown: 7
- level: 13
  id: 2233
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 13'
  learn_points: 45
  cost: MP 44
  power: 55
  cooldown: 6.8
- level: 14
  id: 2234
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 14'
  learn_points: 50
  cost: MP 46
  power: 60
  cooldown: 6.8
- level: 15
  id: 2235
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 15'
  learn_points: 55
  cost: MP 48
  power: 65
  cooldown: 6.6
- level: 16
  id: 2236
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 16'
  learn_points: 60
  cost: MP 50
  power: 70
  cooldown: 6.6
- level: 17
  id: 2237
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 17'
  learn_points: 66
  cost: MP 52
  power: 75
  cooldown: 6.4
- level: 18
  id: 2238
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 18'
  learn_points: 72
  cost: MP 54
  power: 80
  cooldown: 6.4
- level: 19
  id: 2239
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 19'
  learn_points: 79
  cost: MP 56
  power: 88
  cooldown: 6.2
  duration: 10
  success: 15
  effects: Slow
  changes: Movement Speed -30%
- level: 20
  id: 2240
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 20'
  learn_points: 86
  cost: MP 60
  power: 100
  cooldown: 6.2
  duration: 15
  success: 20
  effects: Slow
  changes: Movement Speed -50%
source:
  data: LIST_SKILL.STB rows 2221, 2222, 2223, 2224, 2225, 2226, 2227, 2228, 2229, 2230, 2231, 2232, 2233, 2234, 2235, 2236, 2237, 2238, 2239, 2240
  code: module/src/skills.rs
---
# Twin Bullets

Shoot 2 bullets at a target in a single attack.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
