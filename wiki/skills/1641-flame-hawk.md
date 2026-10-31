---
kind: skill
id: 1641
name: Flame Hawk
status: in-game
icon: skill/134
type: Magic Spell
job: Hawker Job
max_level: 10
target: Hostile Character
damage_type: magic attack
skill_books:
- '[[items/consumable/770-flame-hawk|Flame Hawk]]'
levels:
- level: 1
  id: 1641
  needs_level: 70
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 10, [[skills/1601-heart-hit|Heart Hit]] level 5'
  learn_points: 30
  cost: MP 55
  power: 414
  range: 26
  cooldown: 18
- level: 2
  id: 1642
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 35
  cost: MP 59
  power: 457
  range: 27.5
  cooldown: 18.4
- level: 3
  id: 1643
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 41
  cost: MP 63
  power: 501
  range: 29
  cooldown: 18.8
- level: 4
  id: 1644
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 48
  cost: MP 67
  power: 544
  range: 30.5
  cooldown: 19.2
- level: 5
  id: 1645
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 14'
  learn_points: 56
  cost: MP 71
  power: 588
  range: 32
  cooldown: 19.6
- level: 6
  id: 1646
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 15'
  learn_points: 65
  cost: MP 75
  power: 631
  range: 33.5
  cooldown: 20
- level: 7
  id: 1647
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 16'
  learn_points: 75
  cost: MP 80
  power: 676
  range: 35
  cooldown: 20.4
- level: 8
  id: 1648
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 17'
  learn_points: 87
  cost: MP 85
  power: 718
  range: 36.5
  cooldown: 20.8
- level: 9
  id: 1649
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 18'
  learn_points: 100
  cost: MP 90
  power: 773
  range: 38
  cooldown: 21.2
- level: 10
  id: 1650
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 20'
  learn_points: 115
  cost: MP 100
  power: 848
  range: 40
  cooldown: 21.6
source:
  data: godot/data/skills/1641-flame-hawk/ (levels 1641, 1642, 1643, 1644, 1645, 1646, 1647, 1648, 1649, 1650)
  code: module/src/skills.rs
---
# Flame Hawk

Summon a fire hawk to inflict damage to enemies from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
