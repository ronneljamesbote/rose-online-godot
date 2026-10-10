---
kind: skill
id: 2021
name: Marksmanship
status: in-game
icon: skill/144
type: Passive
job: Dealer Job
max_level: 20
target: Yourself
needs_weapon: Gun, Launcher
skill_books:
- '[[items/consumable/802-marksmanship|Marksmanship]]'
levels:
- level: 1
  id: 2021
  learn_points: 4
  changes: Gun Attack Power +8
- level: 2
  id: 2022
  learn_points: 5
  changes: Gun Attack Power +11
- level: 3
  id: 2023
  learn_points: 6
  changes: Gun Attack Power +14
- level: 4
  id: 2024
  learn_points: 7
  changes: Gun Attack Power +17
- level: 5
  id: 2025
  learn_points: 9
  changes: Gun Attack Power +20
- level: 6
  id: 2026
  learn_points: 11
  changes: Gun Attack Power +23
- level: 7
  id: 2027
  learn_points: 13
  changes: Gun Attack Power +26
- level: 8
  id: 2028
  learn_points: 15
  changes: Gun Attack Power +29
- level: 9
  id: 2029
  learn_points: 17
  changes: Gun Attack Power +32
- level: 10
  id: 2030
  learn_points: 19
  changes: Gun Attack Power +35
- level: 11
  id: 2031
  needs_level: 70
  learn_points: 22
  changes: Gun Attack Power +39
- level: 12
  id: 2032
  learn_points: 25
  changes: Gun Attack Power +43
- level: 13
  id: 2033
  learn_points: 29
  changes: Gun Attack Power +47
- level: 14
  id: 2034
  learn_points: 33
  changes: Gun Attack Power +51
- level: 15
  id: 2035
  learn_points: 38
  changes: Gun Attack Power +55
- level: 16
  id: 2036
  learn_points: 43
  changes: Gun Attack Power +59
- level: 17
  id: 2037
  learn_points: 49
  changes: Gun Attack Power +63
- level: 18
  id: 2038
  learn_points: 55
  changes: Gun Attack Power +67
- level: 19
  id: 2039
  learn_points: 62
  changes: Gun Attack Power +71
- level: 20
  id: 2040
  learn_points: 70
  changes: Gun Attack Power +75
source:
  data: LIST_SKILL.STB rows 2021, 2022, 2023, 2024, 2025, 2026, 2027, 2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2037, 2038, 2039, 2040
  code: module/src/skills.rs
---
# Marksmanship

Increase Gun and Launcher damage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
