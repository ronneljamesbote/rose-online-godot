---
kind: skill
id: 271
name: Crossbow Mastery
status: in-game
icon: skill/42
type: Passive
job: Soldier Job
max_level: 10
target: Yourself
needs_weapon: Crossbow
warps_to: zone 47
skill_books:
- '[[items/consumable/113-plum|Plum]]'
- '[[items/consumable/656-crossbow-mastery|Crossbow Mastery]]'
levels:
- level: 1
  id: 271
  learn_points: 10
  changes: Crossbow Attack Power +10
- level: 2
  id: 272
  learn_points: 13
  changes: Crossbow Attack Power +15
- level: 3
  id: 273
  learn_points: 16
  changes: Crossbow Attack Power +20
- level: 4
  id: 274
  learn_points: 19
  changes: Crossbow Attack Power +26
- level: 5
  id: 275
  learn_points: 23
  changes: Crossbow Attack Power +32
- level: 6
  id: 276
  learn_points: 27
  changes: Crossbow Attack Power +39
- level: 7
  id: 277
  learn_points: 32
  changes: Crossbow Attack Power +46
- level: 8
  id: 278
  learn_points: 38
  changes: Crossbow Attack Power +54
- level: 9
  id: 279
  learn_points: 45
  changes: Crossbow Attack Power +62
- level: 10
  id: 280
  learn_points: 53
  changes: Crossbow Attack Power +70
source:
  data: LIST_SKILL.STB rows 271, 272, 273, 274, 275, 276, 277, 278, 279, 280
  code: module/src/skills.rs
---
# Crossbow Mastery

Increase Crossbow damage. Only Knights can upgrade this skill above Level 5.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
