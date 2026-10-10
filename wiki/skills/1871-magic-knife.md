---
kind: skill
id: 1871
name: Magic Knife
status: in-game
icon: skill/218
type: Magic Spell
job: Raider Job
max_level: 10
target: Hostile Character
damage_type: magic attack
warps_to: '[[zones/21-valley-of-luxem-tower|Valley of Luxem Tower]]'
skill_books:
- '[[items/consumable/792-magic-knife|Magic Knife]]'
levels:
- level: 1
  id: 1871
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 5'
  learn_points: 30
  cost: MP 30
  power: 80
  range: 25
  cooldown: 10
  duration: 12
  success: 80
  effects: Magic Resistance Down
  changes: Magic Resistance +20 +12%
- level: 2
  id: 1872
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 6'
  learn_points: 33
  cost: MP 32
  power: 86
  range: 25.5
  cooldown: 9.8
  duration: 14
  success: 82
  effects: Magic Resistance Down
  changes: Magic Resistance +22 +14%
- level: 3
  id: 1873
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 7'
  learn_points: 36
  cost: MP 34
  power: 92
  range: 26
  cooldown: 9.6
  duration: 16
  success: 84
  effects: Magic Resistance Down
  changes: Magic Resistance +24 +16%
- level: 4
  id: 1874
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 8'
  learn_points: 40
  cost: MP 36
  power: 98
  range: 26.5
  cooldown: 9.4
  duration: 18
  success: 86
  effects: Magic Resistance Down
  changes: Magic Resistance +26 +18%
- level: 5
  id: 1875
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 9'
  learn_points: 44
  cost: MP 38
  power: 104
  range: 27
  cooldown: 9.2
  duration: 20
  success: 88
  effects: Magic Resistance Down
  changes: Magic Resistance +28 +20%
- level: 6
  id: 1876
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 10'
  learn_points: 48
  cost: MP 40
  power: 110
  range: 27.5
  cooldown: 9
  duration: 22
  success: 90
  effects: Magic Resistance Down
  changes: Magic Resistance +30 +22%
- level: 7
  id: 1877
  learn_points: 52
  cost: MP 42
  power: 116
  range: 28
  cooldown: 8.8
  duration: 24
  success: 92
  effects: Magic Resistance Down
  changes: Magic Resistance +32 +24%
- level: 8
  id: 1878
  learn_points: 57
  cost: MP 44
  power: 122
  range: 28.5
  cooldown: 8.6
  duration: 26
  success: 94
  effects: Magic Resistance Down
  changes: Magic Resistance +34 +26%
- level: 9
  id: 1879
  learn_points: 62
  cost: MP 46
  power: 128
  range: 29
  cooldown: 8.4
  duration: 28
  success: 96
  effects: Magic Resistance Down
  changes: Magic Resistance +36 +28%
- level: 10
  id: 1880
  learn_points: 67
  cost: MP 50
  power: 140
  range: 30
  cooldown: 8.2
  duration: 30
  success: 98
  effects: Magic Resistance Down
  changes: Magic Resistance +40 +30%
source:
  data: LIST_SKILL.STB rows 1871, 1872, 1873, 1874, 1875, 1876, 1877, 1878, 1879, 1880
  code: module/src/skills.rs
---
# Magic Knife

Throw a shuriken at a target to decrease its Magic Resistance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
