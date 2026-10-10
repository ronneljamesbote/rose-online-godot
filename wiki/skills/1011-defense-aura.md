---
kind: skill
id: 1011
name: Defense Aura
status: in-game
icon: skill/86
type: Continuing (Self)
job: Cleric Job
max_level: 10
target: Party Member
skill_books:
- '[[items/consumable/718-defense-aura|Defense Aura]]'
levels:
- level: 1
  id: 1011
  needs: '[[skills/821-meditation|Meditation]] level 10, [[skills/991-enchant-armor|Enchant Armor]] level 5'
  learn_points: 30
  cost: MP 40
  area: 13
  cooldown: 4
  duration: 160
  effects: Def Up
  changes: Defense +25
- level: 2
  id: 1012
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 33
  cost: MP 45
  area: 13.5
  cooldown: 4
  duration: 175
  effects: Def Up
  changes: Defense +30
- level: 3
  id: 1013
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 37
  cost: MP 50
  area: 14
  cooldown: 4
  duration: 190
  effects: Def Up
  changes: Defense +35
- level: 4
  id: 1014
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 41
  cost: MP 55
  area: 14.5
  cooldown: 4
  duration: 205
  effects: Def Up
  changes: Defense +40
- level: 5
  id: 1015
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 45
  cost: MP 60
  area: 15
  cooldown: 4
  duration: 220
  effects: Def Up
  changes: Defense +45
- level: 6
  id: 1016
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 50
  cost: MP 65
  area: 15.5
  cooldown: 4
  duration: 235
  effects: Def Up
  changes: Defense +50
- level: 7
  id: 1017
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 55
  cost: MP 70
  area: 16
  cooldown: 4
  duration: 250
  effects: Def Up
  changes: Defense +55
- level: 8
  id: 1018
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 60
  cost: MP 75
  area: 16.5
  cooldown: 4
  duration: 265
  effects: Def Up
  changes: Defense +60
- level: 9
  id: 1019
  needs: '[[skills/821-meditation|Meditation]] level 18'
  learn_points: 66
  cost: MP 80
  area: 17
  cooldown: 4
  duration: 280
  effects: Def Up
  changes: Defense +65
- level: 10
  id: 1020
  needs: '[[skills/821-meditation|Meditation]] level 19'
  learn_points: 72
  cost: MP 90
  area: 20
  cooldown: 4
  duration: 300
  effects: Def Up
  changes: Defense +75
source:
  data: LIST_SKILL.STB rows 1011, 1012, 1013, 1014, 1015, 1016, 1017, 1018, 1019, 1020
  code: module/src/skills.rs
---
# Defense Aura

Increase nearby party members' Defense for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
