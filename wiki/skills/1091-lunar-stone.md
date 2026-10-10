---
kind: skill
id: 1091
name: Lunar Stone
status: in-game
icon: skill/99
type: Area Effect Magic Damage
job: Mage Job
max_level: 10
target: Hostile Character
damage_type: magic attack
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/726-lunar-stone|Lunar Stone]]'
levels:
- level: 1
  id: 1091
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 20, [[skills/1081-wind-storm|Wind Storm]] level 8'
  learn_points: 40
  cost: MP 120
  power: 160
  range: 20
  area: 12
  cooldown: 20
- level: 2
  id: 1092
  learn_points: 47
  cost: MP 130
  power: 178
  range: 20
  area: 12.1
  cooldown: 21
- level: 3
  id: 1093
  learn_points: 55
  cost: MP 140
  power: 196
  range: 20
  area: 12.2
  cooldown: 22
- level: 4
  id: 1094
  learn_points: 64
  cost: MP 150
  power: 214
  range: 20
  area: 12.3
  cooldown: 23
- level: 5
  id: 1095
  learn_points: 74
  cost: MP 160
  power: 232
  range: 20
  area: 12.4
  cooldown: 24
- level: 6
  id: 1096
  learn_points: 85
  cost: MP 170
  power: 250
  range: 20
  area: 12.5
  cooldown: 25
- level: 7
  id: 1097
  learn_points: 98
  cost: MP 180
  power: 268
  range: 20
  area: 12.6
  cooldown: 26
- level: 8
  id: 1098
  learn_points: 113
  cost: MP 190
  power: 286
  range: 20
  area: 12.7
  cooldown: 27
- level: 9
  id: 1099
  learn_points: 130
  cost: MP 200
  power: 304
  range: 20
  area: 12.8
  cooldown: 28
- level: 10
  id: 1100
  learn_points: 149
  cost: MP 215
  power: 330
  range: 20
  area: 13
  cooldown: 29
source:
  data: LIST_SKILL.STB rows 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099, 1100
  code: module/src/skills.rs
---
# Lunar Stone

Summon a meteor from the sky to inflict damage on many foes.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
