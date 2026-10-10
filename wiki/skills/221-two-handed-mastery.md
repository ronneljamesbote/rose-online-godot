---
kind: skill
id: 221
name: Two-Handed Mastery
status: in-game
icon: skill/27
type: Passive
job: Champion Job
max_level: 10
target: Yourself
needs_weapon: Two-Handed Sword, Spear, Two-Handed Axe
warps_to: zone 43
skill_books:
- '[[items/consumable/653-two-handed-mastery|Two-Handed Mastery]]'
levels:
- level: 1
  id: 221
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 10'
  learn_points: 28
  changes: Two-Handed Weapon Attack Power +52
- level: 2
  id: 222
  learn_points: 32
  changes: Two-Handed Weapon Attack Power +58
- level: 3
  id: 223
  learn_points: 36
  changes: Two-Handed Weapon Attack Power +64
- level: 4
  id: 224
  learn_points: 40
  changes: Two-Handed Weapon Attack Power +70
- level: 5
  id: 225
  learn_points: 45
  changes: Two-Handed Weapon Attack Power +76
- level: 6
  id: 226
  learn_points: 50
  changes: Two-Handed Weapon Attack Power +82
- level: 7
  id: 227
  learn_points: 56
  changes: Two-Handed Weapon Attack Power +88
- level: 8
  id: 228
  learn_points: 62
  changes: Two-Handed Weapon Attack Power +95
- level: 9
  id: 229
  learn_points: 69
  changes: Two-Handed Weapon Attack Power +102
- level: 10
  id: 230
  learn_points: 77
  changes: Two-Handed Weapon Attack Power +110
source:
  data: LIST_SKILL.STB rows 221, 222, 223, 224, 225, 226, 227, 228, 229, 230
  code: module/src/skills.rs
---
# Two-Handed Mastery

Increase Two-Handed Weapon damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
