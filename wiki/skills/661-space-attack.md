---
kind: skill
id: 661
name: Space Attack
status: in-game
icon: skill/199
type: Magic Spell
job: Champion Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/691-space-attack|Space Attack]]'
levels:
- level: 1
  id: 661
  needs: '[[skills/301-heavy-attack|Heavy Attack]] level 10'
  learn_points: 30
  cost: MP 40
  power: 100
  range: 20
  cooldown: 7
- level: 2
  id: 662
  learn_points: 35
  cost: MP 43
  power: 110
  range: 20
  cooldown: 7.2
- level: 3
  id: 663
  learn_points: 41
  cost: MP 46
  power: 120
  range: 20
  cooldown: 7.4
- level: 4
  id: 664
  learn_points: 48
  cost: MP 49
  power: 130
  range: 20
  cooldown: 7.6
- level: 5
  id: 665
  learn_points: 56
  cost: MP 52
  power: 140
  range: 20
  cooldown: 7.8
- level: 6
  id: 666
  learn_points: 65
  cost: MP 55
  power: 150
  range: 20
  cooldown: 8
- level: 7
  id: 667
  learn_points: 75
  cost: MP 58
  power: 160
  range: 20
  cooldown: 8.2
- level: 8
  id: 668
  learn_points: 87
  cost: MP 61
  power: 170
  range: 20
  cooldown: 8.4
- level: 9
  id: 669
  learn_points: 100
  cost: MP 64
  power: 180
  range: 20
  cooldown: 8.6
- level: 10
  id: 670
  learn_points: 115
  cost: MP 70
  power: 200
  range: 20
  cooldown: 8.8
source:
  data: LIST_SKILL.STB rows 661, 662, 663, 664, 665, 666, 667, 668, 669, 670
  code: module/src/skills.rs
---
# Space Attack

Shoot a spirit sphere at an enemy from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
