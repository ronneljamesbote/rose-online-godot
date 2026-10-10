---
kind: skill
id: 341
name: Leap Attack
status: in-game
icon: skill/35
type: Damage Action
job: Soldier Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/661-leap-attack|Leap Attack]]'
levels:
- level: 1
  id: 341
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 5'
  learn_points: 10
  cost: HP 30
  power: 50
  cooldown: 9
  duration: 5
  success: 30
  effects: Fainted
- level: 2
  id: 342
  learn_points: 12
  cost: HP 35
  power: 60
  cooldown: 9
  duration: 5
  success: 31
  effects: Fainted
- level: 3
  id: 343
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 6'
  learn_points: 14
  cost: HP 40
  power: 70
  cooldown: 9.2
  duration: 5
  success: 32
  effects: Fainted
- level: 4
  id: 344
  learn_points: 16
  cost: HP 45
  power: 80
  cooldown: 9.2
  duration: 5
  success: 33
  effects: Fainted
- level: 5
  id: 345
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 7'
  learn_points: 18
  cost: HP 50
  power: 90
  cooldown: 9.4
  duration: 5
  success: 34
  effects: Fainted
- level: 6
  id: 346
  learn_points: 20
  cost: HP 55
  power: 100
  cooldown: 9.4
  duration: 5
  success: 35
  effects: Fainted
- level: 7
  id: 347
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 8'
  learn_points: 23
  cost: HP 60
  power: 110
  cooldown: 9.6
  duration: 5
  success: 36
  effects: Fainted
- level: 8
  id: 348
  learn_points: 26
  cost: HP 65
  power: 120
  cooldown: 9.6
  duration: 5
  success: 37
  effects: Fainted
- level: 9
  id: 349
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 9'
  learn_points: 29
  cost: HP 70
  power: 130
  cooldown: 9.8
  duration: 5
  success: 38
  effects: Fainted
- level: 10
  id: 350
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 10'
  learn_points: 32
  cost: HP 80
  power: 150
  cooldown: 9.8
  duration: 5
  success: 40
  effects: Fainted
source:
  data: LIST_SKILL.STB rows 341, 342, 343, 344, 345, 346, 347, 348, 349, 350
  code: module/src/skills.rs
---
# Leap Attack

Enables its user to jump and attack at the same time when using a Melee Weapon. Inflicts Stun effect on the target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
