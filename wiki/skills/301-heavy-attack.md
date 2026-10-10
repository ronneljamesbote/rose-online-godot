---
kind: skill
id: 301
name: Heavy Attack
status: in-game
icon: skill/28
type: Damage Action
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/659-heavy-attack|Heavy Attack]]'
levels:
- level: 1
  id: 301
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 1'
  learn_points: 8
  cost: MP 10
  power: 30
  cooldown: 6
- level: 2
  id: 302
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 2'
  learn_points: 10
  cost: MP 12
  power: 37
  cooldown: 6
- level: 3
  id: 303
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 3'
  learn_points: 12
  cost: MP 14
  power: 44
  cooldown: 6
- level: 4
  id: 304
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 4'
  learn_points: 14
  cost: MP 16
  power: 51
  cooldown: 6.2
- level: 5
  id: 305
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 5'
  learn_points: 16
  cost: MP 18
  power: 58
  cooldown: 6.2
- level: 6
  id: 306
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 6'
  learn_points: 18
  cost: MP 20
  power: 65
  cooldown: 6.2
- level: 7
  id: 307
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 7'
  learn_points: 20
  cost: MP 22
  power: 72
  cooldown: 6.4
- level: 8
  id: 308
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 8'
  learn_points: 23
  cost: MP 24
  power: 80
  cooldown: 6.4
- level: 9
  id: 309
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 9'
  learn_points: 26
  cost: MP 26
  power: 88
  cooldown: 6.4
- level: 10
  id: 310
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 10'
  learn_points: 29
  cost: MP 30
  power: 100
  cooldown: 6.6
source:
  data: LIST_SKILL.STB rows 301, 302, 303, 304, 305, 306, 307, 308, 309, 310
  code: module/src/skills.rs
---
# Heavy Attack

Bash a target with crushing force while using a Melee Weapon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
