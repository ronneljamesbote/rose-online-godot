---
kind: skill
id: 2141
name: Arms Mastery
status: in-game
icon: skill/144
type: Passive
job: Dealer Job
max_level: 20
target: Yourself
needs_weapon: Gun, Launcher
warps_to: zone 50
skill_books:
- '[[items/consumable/810-arms-mastery|Arms Mastery]]'
levels:
- level: 1
  id: 2141
  learn_points: 4
  changes: Gun Attack Speed +3
- level: 2
  id: 2142
  learn_points: 5
  changes: Gun Attack Speed +4
- level: 3
  id: 2143
  learn_points: 7
  changes: Gun Attack Speed +5
- level: 4
  id: 2144
  learn_points: 9
  changes: Gun Attack Speed +6
- level: 5
  id: 2145
  learn_points: 11
  changes: Gun Attack Speed +7
- level: 6
  id: 2146
  learn_points: 13
  changes: Gun Attack Speed +8
- level: 7
  id: 2147
  learn_points: 15
  changes: Gun Attack Speed +9
- level: 8
  id: 2148
  learn_points: 18
  changes: Gun Attack Speed +10
- level: 9
  id: 2149
  learn_points: 21
  changes: Gun Attack Speed +11
- level: 10
  id: 2150
  learn_points: 24
  changes: Gun Attack Speed +12
- level: 11
  id: 2151
  needs_level: 70
  learn_points: 27
  changes: Gun Attack Speed +13
- level: 12
  id: 2152
  learn_points: 30
  changes: Gun Attack Speed +14
- level: 13
  id: 2153
  learn_points: 33
  changes: Gun Attack Speed +15
- level: 14
  id: 2154
  learn_points: 36
  changes: Gun Attack Speed +16
- level: 15
  id: 2155
  learn_points: 40
  changes: Gun Attack Speed +17
- level: 16
  id: 2156
  learn_points: 44
  changes: Gun Attack Speed +18
- level: 17
  id: 2157
  learn_points: 48
  changes: Gun Attack Speed +19
- level: 18
  id: 2158
  learn_points: 52
  changes: Gun Attack Speed +20
- level: 19
  id: 2159
  learn_points: 57
  changes: Gun Attack Speed +21
- level: 20
  id: 2160
  learn_points: 62
  changes: Gun Attack Speed +22
source:
  data: LIST_SKILL.STB rows 2141, 2142, 2143, 2144, 2145, 2146, 2147, 2148, 2149, 2150, 2151, 2152, 2153, 2154, 2155, 2156, 2157, 2158, 2159, 2160
  code: module/src/skills.rs
---
# Arms Mastery

Refine combat techniques with a Gun. Increase Attack Speed when wielding a Gun or Launcher.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
