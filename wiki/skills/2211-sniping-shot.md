---
kind: skill
id: 2211
name: Sniping Shot
status: in-game
icon: skill/219
type: Damage Action
job: Dealer Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/815-sniping-shot|Sniping Shot]]'
levels:
- level: 1
  id: 2211
  needs_level: 70
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 10, [[skills/2201-mighty-shot|Mighty Shot]] level 10'
  learn_points: 35
  cost: MP 45
  power: 130
  cooldown: 9.2
- level: 2
  id: 2212
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 11'
  learn_points: 41
  cost: MP 51
  power: 150
  cooldown: 9.6
- level: 3
  id: 2213
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 12'
  learn_points: 48
  cost: MP 57
  power: 170
  cooldown: 10
- level: 4
  id: 2214
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 13'
  learn_points: 56
  cost: MP 63
  power: 190
  cooldown: 10.4
- level: 5
  id: 2215
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 14'
  learn_points: 65
  cost: MP 69
  power: 210
  cooldown: 10.8
- level: 6
  id: 2216
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 15'
  learn_points: 75
  cost: MP 75
  power: 230
  cooldown: 11.2
- level: 7
  id: 2217
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 16'
  learn_points: 87
  cost: MP 81
  power: 250
  cooldown: 11.6
- level: 8
  id: 2218
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 17'
  learn_points: 100
  cost: MP 87
  power: 270
  cooldown: 12
- level: 9
  id: 2219
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 18'
  learn_points: 115
  cost: MP 93
  power: 290
  cooldown: 12.4
- level: 10
  id: 2220
  needs: '[[skills/2021-marksmanship|Marksmanship]] level 19'
  learn_points: 132
  cost: MP 105
  power: 330
  cooldown: 12.8
source:
  data: LIST_SKILL.STB rows 2211, 2212, 2213, 2214, 2215, 2216, 2217, 2218, 2219, 2220
  code: module/src/skills.rs
---
# Sniping Shot

Inflict a great amount of damage to a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
