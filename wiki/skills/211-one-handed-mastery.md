---
kind: skill
id: 211
name: One-Handed Mastery
status: in-game
icon: skill/26
type: Passive
job: Knight Job
max_level: 10
target: Yourself
needs_weapon: One-Handed Sword, One-Handed Blunt Weapon
skill_books:
- '[[items/consumable/107-boiled-egg|Boiled Egg]]'
- '[[items/consumable/652-one-handed-mastery|One-Handed Mastery]]'
levels:
- level: 1
  id: 211
  needs: '[[skills/201-melee-weapon-mastery|Melee Weapon Mastery]] level 10'
  learn_points: 27
  changes: One-Handed Weapon Attack Power +37
- level: 2
  id: 212
  learn_points: 30
  changes: One-Handed Weapon Attack Power +41
- level: 3
  id: 213
  learn_points: 34
  changes: One-Handed Weapon Attack Power +45
- level: 4
  id: 214
  learn_points: 38
  changes: One-Handed Weapon Attack Power +49
- level: 5
  id: 215
  learn_points: 42
  changes: One-Handed Weapon Attack Power +53
- level: 6
  id: 216
  learn_points: 47
  changes: One-Handed Weapon Attack Power +57
- level: 7
  id: 217
  learn_points: 52
  changes: One-Handed Weapon Attack Power +61
- level: 8
  id: 218
  learn_points: 58
  changes: One-Handed Weapon Attack Power +65
- level: 9
  id: 219
  learn_points: 64
  changes: One-Handed Weapon Attack Power +69
- level: 10
  id: 220
  learn_points: 71
  changes: One-Handed Weapon Attack Power +73
source:
  data: LIST_SKILL.STB rows 211, 212, 213, 214, 215, 216, 217, 218, 219, 220
  code: module/src/skills.rs
---
# One-Handed Mastery

Increase One-Handed Weapon damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
