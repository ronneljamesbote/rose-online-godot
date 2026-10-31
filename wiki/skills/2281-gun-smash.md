---
kind: skill
id: 2281
name: Gun Smash
status: in-game
icon: skill/192
type: Damage Action
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/822-gun-smash|Gun Smash]]'
levels:
- level: 1
  id: 2281
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 5, [[skills/2241-intensify-weapon|Intensify Weapon]] level 3'
  learn_points: 15
  cost: MP 20
  power: 136
  range: 3
  cooldown: 9
- level: 2
  id: 2282
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 6'
  learn_points: 18
  cost: MP 22
  power: 154
  range: 3
  cooldown: 9
- level: 3
  id: 2283
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 7'
  learn_points: 22
  cost: MP 24
  power: 172
  range: 3
  cooldown: 9
- level: 4
  id: 2284
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 8'
  learn_points: 26
  cost: MP 26
  power: 189
  range: 3
  cooldown: 9
- level: 5
  id: 2285
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 9'
  learn_points: 31
  cost: MP 28
  power: 206
  range: 3
  cooldown: 9
- level: 6
  id: 2286
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 10'
  learn_points: 36
  cost: MP 30
  power: 223
  range: 3
  cooldown: 9
- level: 7
  id: 2287
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 11'
  learn_points: 42
  cost: MP 32
  power: 240
  range: 3
  cooldown: 9
- level: 8
  id: 2288
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 12'
  learn_points: 49
  cost: MP 34
  power: 257
  range: 3
  cooldown: 9
- level: 9
  id: 2289
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 13'
  learn_points: 57
  cost: MP 36
  power: 274
  range: 3
  cooldown: 9
- level: 10
  id: 2290
  needs: '[[skills/2141-arms-mastery|Arms Mastery]] level 14'
  learn_points: 66
  cost: MP 40
  power: 308
  range: 3
  cooldown: 9
  duration: 5
  success: 25
  effects: Fainted
source:
  data: godot/data/skills/2281-gun-smash/ (levels 2281, 2282, 2283, 2284, 2285, 2286, 2287, 2288, 2289, 2290)
  code: module/src/skills.rs
---
# Gun Smash

Hit a nearby target with the barrel of your gun.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
