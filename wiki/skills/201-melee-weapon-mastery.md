---
kind: skill
id: 201
name: Melee Weapon Mastery
status: in-game
icon: skill/26
type: Passive
job: Soldier Job
max_level: 10
target: Yourself
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon, Two-Handed Sword, Spear, Two-Handed Axe
skill_books:
- '[[items/consumable/651-melee-weapon-mastery|Melee Weapon Mastery]]'
levels:
- level: 1
  id: 201
  learn_points: 4
  changes: One-Handed Weapon Attack Power +6, Two-Handed Weapon Attack Power +8
- level: 2
  id: 202
  learn_points: 5
  changes: One-Handed Weapon Attack Power +9, Two-Handed Weapon Attack Power +12
- level: 3
  id: 203
  learn_points: 7
  changes: One-Handed Weapon Attack Power +12, Two-Handed Weapon Attack Power +16
- level: 4
  id: 204
  learn_points: 9
  changes: One-Handed Weapon Attack Power +15, Two-Handed Weapon Attack Power +20
- level: 5
  id: 205
  learn_points: 11
  changes: One-Handed Weapon Attack Power +18, Two-Handed Weapon Attack Power +24
- level: 6
  id: 206
  learn_points: 13
  changes: One-Handed Weapon Attack Power +21, Two-Handed Weapon Attack Power +28
- level: 7
  id: 207
  learn_points: 15
  changes: One-Handed Weapon Attack Power +24, Two-Handed Weapon Attack Power +32
- level: 8
  id: 208
  learn_points: 18
  changes: One-Handed Weapon Attack Power +27, Two-Handed Weapon Attack Power +36
- level: 9
  id: 209
  learn_points: 21
  changes: One-Handed Weapon Attack Power +30, Two-Handed Weapon Attack Power +40
- level: 10
  id: 210
  learn_points: 24
  changes: One-Handed Weapon Attack Power +33, Two-Handed Weapon Attack Power +45
source:
  data: LIST_SKILL.STB rows 201, 202, 203, 204, 205, 206, 207, 208, 209, 210
  code: module/src/skills.rs
---
# Melee Weapon Mastery

Increase Melee Weapon damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
