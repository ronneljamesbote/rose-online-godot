---
kind: skill
id: 1241
name: Magical Aura
status: in-game
icon: skill/209
type: Continuing (Self)
job: Cleric Job
max_level: 10
target: Party Member
skill_books:
- '[[items/consumable/741-magical-aura|Magical Aura]]'
levels:
- level: 1
  id: 1241
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/991-enchant-armor|Enchant Armor]] level 5'
  learn_points: 30
  cost: MP 40
  area: 13
  cooldown: 4
  duration: 160
  effects: Magic Resistance Up
  changes: Magic Resistance +40
- level: 2
  id: 1242
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 33
  cost: MP 45
  area: 13.5
  cooldown: 4
  duration: 175
  effects: Magic Resistance Up
  changes: Magic Resistance +47
- level: 3
  id: 1243
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 37
  cost: MP 50
  area: 14
  cooldown: 4
  duration: 190
  effects: Magic Resistance Up
  changes: Magic Resistance +54
- level: 4
  id: 1244
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 41
  cost: MP 55
  area: 14.5
  cooldown: 4
  duration: 205
  effects: Magic Resistance Up
  changes: Magic Resistance +61
- level: 5
  id: 1245
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 45
  cost: MP 60
  area: 15
  cooldown: 4
  duration: 220
  effects: Magic Resistance Up
  changes: Magic Resistance +68
- level: 6
  id: 1246
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 50
  cost: MP 65
  area: 15.5
  cooldown: 4
  duration: 235
  effects: Magic Resistance Up
  changes: Magic Resistance +75
- level: 7
  id: 1247
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 55
  cost: MP 70
  area: 16
  cooldown: 4
  duration: 250
  effects: Magic Resistance Up
  changes: Magic Resistance +85
- level: 8
  id: 1248
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 60
  cost: MP 75
  area: 16.5
  cooldown: 4
  duration: 265
  effects: Magic Resistance Up
  changes: Magic Resistance +95
- level: 9
  id: 1249
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 66
  cost: MP 80
  area: 17
  cooldown: 4
  duration: 280
  effects: Magic Resistance Up
  changes: Magic Resistance +105
- level: 10
  id: 1250
  needs: '[[skills/821-meditation|Meditation]] level 20'
  learn_points: 72
  cost: MP 90
  area: 20
  cooldown: 4
  duration: 300
  effects: Magic Resistance Up
  changes: Magic Resistance +120
source:
  data: LIST_SKILL.STB rows 1241, 1242, 1243, 1244, 1245, 1246, 1247, 1248, 1249, 1250
  code: module/src/skills.rs
---
# Magical Aura

Increase nearby party members' Magic Resistance for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
