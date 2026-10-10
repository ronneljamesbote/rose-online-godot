---
kind: skill
id: 361
name: Berserk
status: in-game
icon: skill/29
type: Continuing (Self)
job: Soldier Job
max_level: 20
target: Yourself
skill_books:
- '[[items/consumable/115-hamburger|Hamburger]]'
- '[[items/consumable/662-berserk|Berserk]]'
- '[[items/consumable/663-shield-protect|Shield Protect]]'
levels:
- level: 1
  id: 361
  needs: '[[skills/231-physical-training|Physical Training]] level 3'
  learn_points: 6
  cost: MP 20
  cooldown: 6
  duration: 120
  effects: Atk Power Increased, Def Down
  changes: Attack Power -30, Defense -16
- level: 2
  id: 362
  needs: '[[skills/231-physical-training|Physical Training]] level 4'
  learn_points: 8
  cost: MP 24
  cooldown: 6
  duration: 125
  effects: Atk Power Increased, Def Down
  changes: Attack Power -33, Defense -17
- level: 3
  id: 363
  needs: '[[skills/231-physical-training|Physical Training]] level 5'
  learn_points: 10
  cost: MP 28
  cooldown: 6
  duration: 130
  effects: Atk Power Increased, Def Down
  changes: Attack Power -36, Defense -18
- level: 4
  id: 364
  needs: '[[skills/231-physical-training|Physical Training]] level 6'
  learn_points: 12
  cost: MP 32
  cooldown: 6
  duration: 135
  effects: Atk Power Increased, Def Down
  changes: Attack Power -39, Defense -19
- level: 5
  id: 365
  needs: '[[skills/231-physical-training|Physical Training]] level 7'
  learn_points: 14
  cost: MP 36
  cooldown: 6
  duration: 140
  effects: Atk Power Increased, Def Down
  changes: Attack Power -42, Defense -20
- level: 6
  id: 366
  needs: '[[skills/231-physical-training|Physical Training]] level 8'
  learn_points: 16
  cost: MP 40
  cooldown: 6
  duration: 145
  effects: Atk Power Increased, Def Down
  changes: Attack Power -45, Defense -21
- level: 7
  id: 367
  needs: '[[skills/231-physical-training|Physical Training]] level 9'
  learn_points: 18
  cost: MP 44
  cooldown: 6
  duration: 150
  effects: Atk Power Increased, Def Down
  changes: Attack Power -48, Defense -22
- level: 8
  id: 368
  needs: '[[skills/231-physical-training|Physical Training]] level 10'
  learn_points: 21
  cost: MP 48
  cooldown: 6
  duration: 155
  effects: Atk Power Increased, Def Down
  changes: Attack Power -51, Defense -23
- level: 9
  id: 369
  needs: '[[skills/231-physical-training|Physical Training]] level 11'
  learn_points: 24
  cost: MP 52
  cooldown: 6
  duration: 160
  effects: Atk Power Increased, Def Down
  changes: Attack Power -54, Defense -24
- level: 10
  id: 370
  needs: '[[skills/231-physical-training|Physical Training]] level 12'
  learn_points: 27
  cost: MP 60
  cooldown: 6
  duration: 170
  effects: Atk Power Increased, Def Down
  changes: Attack Power -57, Defense -25
- level: 11
  id: 371
  needs: '[[skills/231-physical-training|Physical Training]] level 13'
  learn_points: 30
  cost: MP 62
  cooldown: 6
  duration: 172
  effects: Atk Power Increased, Def Down
  changes: Attack Power -61, Defense -26
- level: 12
  id: 372
  needs: '[[skills/231-physical-training|Physical Training]] level 14'
  learn_points: 33
  cost: MP 64
  cooldown: 6
  duration: 174
  effects: Atk Power Increased, Def Down
  changes: Attack Power -65, Defense -27
- level: 13
  id: 373
  needs: '[[skills/231-physical-training|Physical Training]] level 15'
  learn_points: 36
  cost: MP 66
  cooldown: 6
  duration: 176
  effects: Atk Power Increased, Def Down
  changes: Attack Power -69, Defense -28
- level: 14
  id: 374
  needs: '[[skills/231-physical-training|Physical Training]] level 16'
  learn_points: 39
  cost: MP 68
  cooldown: 6
  duration: 178
  effects: Atk Power Increased, Def Down
  changes: Attack Power -73, Defense -29
- level: 15
  id: 375
  needs: '[[skills/231-physical-training|Physical Training]] level 17'
  learn_points: 42
  cost: MP 70
  cooldown: 6
  duration: 180
  effects: Atk Power Increased, Def Down
  changes: Attack Power -77, Defense -30
- level: 16
  id: 376
  needs: '[[skills/231-physical-training|Physical Training]] level 18'
  learn_points: 46
  cost: MP 72
  cooldown: 6
  duration: 182
  effects: Atk Power Increased, Def Down
  changes: Attack Power -81, Defense -31
- level: 17
  id: 377
  needs: '[[skills/231-physical-training|Physical Training]] level 19'
  learn_points: 50
  cost: MP 74
  cooldown: 6
  duration: 184
  effects: Atk Power Increased, Def Down
  changes: Attack Power -85, Defense -32
- level: 18
  id: 378
  needs: '[[skills/231-physical-training|Physical Training]] level 20'
  learn_points: 54
  cost: MP 76
  cooldown: 6
  duration: 186
  effects: Atk Power Increased, Def Down
  changes: Attack Power -89, Defense -33
- level: 19
  id: 379
  learn_points: 58
  cost: MP 78
  cooldown: 6
  duration: 188
  effects: Atk Power Increased, Def Down
  changes: Attack Power -93, Defense -34
- level: 20
  id: 380
  learn_points: 62
  cost: MP 80
  cooldown: 6
  duration: 190
  effects: Atk Power Increased, Def Down
  changes: Attack Power -100, Defense -35
source:
  data: LIST_SKILL.STB rows 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379, 380
  code: module/src/skills.rs
---
# Berserk

Increase Attack Power of caster while decreasing caster's Defense for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
