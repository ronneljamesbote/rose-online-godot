---
kind: skill
id: 1831
name: Mana Blood
status: in-game
icon: skill/126
type: Magic Recovery (Self)
job: Raider Job
max_level: 10
target: Yourself
warps_to: zone 17
skill_books:
- '[[items/consumable/788-mana-blood|Mana Blood]]'
levels:
- level: 1
  id: 1831
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 11'
  learn_points: 20
  cost: HP 100
  cooldown: 24
  changes: MP +100
- level: 2
  id: 1832
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 12'
  learn_points: 23
  cost: HP 120
  cooldown: 23
  changes: MP +130
- level: 3
  id: 1833
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 13'
  learn_points: 26
  cost: HP 140
  cooldown: 22
  changes: MP +160
- level: 4
  id: 1834
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 14'
  learn_points: 29
  cost: HP 160
  cooldown: 21
  changes: MP +190
- level: 5
  id: 1835
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 15'
  learn_points: 32
  cost: HP 180
  cooldown: 20
  changes: MP +220
- level: 6
  id: 1836
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 16'
  learn_points: 36
  cost: HP 200
  cooldown: 19.2
  changes: MP +250
- level: 7
  id: 1837
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 17'
  learn_points: 40
  cost: HP 220
  cooldown: 18.4
  changes: MP +280
- level: 8
  id: 1838
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 18'
  learn_points: 44
  cost: HP 240
  cooldown: 17.6
  changes: MP +310
- level: 9
  id: 1839
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 19'
  learn_points: 49
  cost: HP 260
  cooldown: 16.8
  changes: MP +340
- level: 10
  id: 1840
  needs: '[[skills/1461-combat-mastery|Combat Mastery]] level 20'
  learn_points: 54
  cost: HP 280
  cooldown: 16
  changes: MP +380
source:
  data: LIST_SKILL.STB rows 1831, 1832, 1833, 1834, 1835, 1836, 1837, 1838, 1839, 1840
  code: module/src/skills.rs
---
# Mana Blood

Regain MP by consuming HP.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
