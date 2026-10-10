---
kind: skill
id: 1841
name: Stealth
status: in-game
icon: skill/131
type: Continuing (Self)
job: Raider Job
max_level: 5
target: Yourself
skill_books:
- '[[items/consumable/789-stealth|Stealth]]'
levels:
- level: 1
  id: 1841
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 1'
  learn_points: 40
  cost: MP 30
  cooldown: 80
  duration: 50
  effects: Camouflage
- level: 2
  id: 1842
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 2'
  learn_points: 40
  cost: MP 33
  cooldown: 86
  duration: 60
  effects: Camouflage
- level: 3
  id: 1843
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 3'
  learn_points: 40
  cost: MP 36
  cooldown: 92
  duration: 70
  effects: Camouflage
- level: 4
  id: 1844
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 4'
  learn_points: 40
  cost: MP 39
  cooldown: 98
  duration: 80
  effects: Camouflage
- level: 5
  id: 1845
  needs: '[[skills/1831-mana-blood|Mana Blood]] level 5'
  learn_points: 40
  cost: MP 42
  cooldown: 104
  duration: 90
  effects: Camouflage
source:
  data: LIST_SKILL.STB rows 1841, 1842, 1843, 1844, 1845
  code: module/src/skills.rs
---
# Stealth

Enables its caster to be invisible while moving or resting.  Attacking enemy cancels the invisible mode.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
