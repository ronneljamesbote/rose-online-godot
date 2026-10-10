---
kind: skill
id: 671
name: Champion Hit
status: in-game
icon: skill/61
type: Damage Action
job: Champion Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/692-champion-hit|Champion Hit]]'
levels:
- level: 1
  id: 671
  needs: '[[skills/341-leap-attack|Leap Attack]] level 10'
  learn_points: 35
  cost: MP 70
  power: 160
  cooldown: 12
- level: 2
  id: 672
  learn_points: 41
  cost: MP 74
  power: 177
  cooldown: 12.4
- level: 3
  id: 673
  learn_points: 48
  cost: MP 78
  power: 194
  cooldown: 12.8
- level: 4
  id: 674
  learn_points: 56
  cost: MP 82
  power: 211
  cooldown: 13.2
- level: 5
  id: 675
  learn_points: 65
  cost: MP 86
  power: 228
  cooldown: 13.6
- level: 6
  id: 676
  learn_points: 75
  cost: MP 90
  power: 245
  cooldown: 14
- level: 7
  id: 677
  learn_points: 87
  cost: MP 94
  power: 262
  cooldown: 14.4
- level: 8
  id: 678
  learn_points: 100
  cost: MP 98
  power: 279
  cooldown: 14.8
- level: 9
  id: 679
  learn_points: 115
  cost: MP 102
  power: 296
  cooldown: 15.2
- level: 10
  id: 680
  learn_points: 132
  cost: MP 110
  power: 330
  cooldown: 15.6
source:
  data: LIST_SKILL.STB rows 671, 672, 673, 674, 675, 676, 677, 678, 679, 680
  code: module/src/skills.rs
---
# Champion Hit

Enables its caster to leap high into the air and strike the top of the enemy’s skull.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
