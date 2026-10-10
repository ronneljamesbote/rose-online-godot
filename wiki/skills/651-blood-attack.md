---
kind: skill
id: 651
name: Blood Attack
status: in-game
icon: skill/56
type: Damage Absorption
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/690-blood-attack|Blood Attack]]'
levels:
- level: 1
  id: 651
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 5'
  learn_points: 20
  cost: MP 40
  power: 60
  cooldown: 10
  changes: HP +120
- level: 2
  id: 652
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 6'
  learn_points: 24
  cost: MP 43
  power: 70
  cooldown: 10
  changes: HP +140
- level: 3
  id: 653
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 7'
  learn_points: 29
  cost: MP 46
  power: 80
  cooldown: 10
  changes: HP +160
- level: 4
  id: 654
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 8'
  learn_points: 35
  cost: MP 49
  power: 90
  cooldown: 10
  changes: HP +180
- level: 5
  id: 655
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 9'
  learn_points: 42
  cost: MP 52
  power: 100
  cooldown: 10
  changes: HP +200
- level: 6
  id: 656
  needs: '[[skills/291-spiritual-training|Spiritual Training]] level 10'
  learn_points: 51
  cost: MP 56
  power: 115
  cooldown: 10
  changes: HP +230
- level: 7
  id: 657
  learn_points: 61
  cost: MP 60
  power: 130
  cooldown: 10
  changes: HP +260
- level: 8
  id: 658
  learn_points: 73
  cost: MP 64
  power: 145
  cooldown: 10
  changes: HP +290
- level: 9
  id: 659
  learn_points: 87
  cost: MP 68
  power: 160
  cooldown: 10
  changes: HP +320
- level: 10
  id: 660
  learn_points: 104
  cost: MP 75
  power: 180
  cooldown: 10
  changes: HP +370
source:
  data: LIST_SKILL.STB rows 651, 652, 653, 654, 655, 656, 657, 658, 659, 660
  code: module/src/skills.rs
---
# Blood Attack

Special attack in which its user regains an amount of HP calculated by the amount of damage inflicted on a nearby enemy. Only Champions can upgrade this skill above Level 6.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
