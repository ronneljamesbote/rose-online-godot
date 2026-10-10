---
kind: skill
id: 801
name: Staff Mastery
status: in-game
icon: skill/65
type: Passive
job: Muse Job
max_level: 20
target: Yourself
needs_weapon: Magic Staff, Magic Tool
skill_books:
- '[[items/consumable/701-staff-mastery|Staff Mastery]]'
levels:
- level: 1
  id: 801
  learn_points: 4
  changes: Magic Weapon Attack Power +4
- level: 2
  id: 802
  learn_points: 5
  changes: Magic Weapon Attack Power +7
- level: 3
  id: 803
  learn_points: 7
  changes: Magic Weapon Attack Power +10
- level: 4
  id: 804
  learn_points: 9
  changes: Magic Weapon Attack Power +13
- level: 5
  id: 805
  learn_points: 11
  changes: Magic Weapon Attack Power +16
- level: 6
  id: 806
  learn_points: 13
  changes: Magic Weapon Attack Power +19
- level: 7
  id: 807
  learn_points: 15
  changes: Magic Weapon Attack Power +22
- level: 8
  id: 808
  learn_points: 18
  changes: Magic Weapon Attack Power +25
- level: 9
  id: 809
  learn_points: 21
  changes: Magic Weapon Attack Power +28
- level: 10
  id: 810
  learn_points: 24
  changes: Magic Weapon Attack Power +31
- level: 11
  id: 811
  needs_level: 70
  learn_points: 27
  changes: Magic Weapon Attack Power +35
- level: 12
  id: 812
  learn_points: 31
  changes: Magic Weapon Attack Power +39
- level: 13
  id: 813
  learn_points: 35
  changes: Magic Weapon Attack Power +43
- level: 14
  id: 814
  learn_points: 39
  changes: Magic Weapon Attack Power +47
- level: 15
  id: 815
  learn_points: 44
  changes: Magic Weapon Attack Power +51
- level: 16
  id: 816
  learn_points: 49
  changes: Magic Weapon Attack Power +55
- level: 17
  id: 817
  learn_points: 54
  changes: Magic Weapon Attack Power +59
- level: 18
  id: 818
  learn_points: 60
  changes: Magic Weapon Attack Power +63
- level: 19
  id: 819
  learn_points: 66
  changes: Magic Weapon Attack Power +67
- level: 20
  id: 820
  learn_points: 73
  changes: Magic Weapon Attack Power +71
source:
  data: LIST_SKILL.STB rows 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 811, 812, 813, 814, 815, 816, 817, 818, 819, 820
  code: module/src/skills.rs
---
# Staff Mastery

Increase Magic Weapon damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
