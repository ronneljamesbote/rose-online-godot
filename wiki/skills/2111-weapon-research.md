---
kind: skill
id: 2111
name: Weapon Research
status: in-game
icon: skill/147
type: Passive
job: Dealer Job
max_level: 20
target: Yourself
warps_to: zone 13
skill_books:
- '[[items/consumable/808-weapon-research|Weapon Research]]'
levels:
- level: 1
  id: 2111
  needs_level: 20
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 1'
  learn_points: 10
  changes: Concentration +1
- level: 2
  id: 2112
  needs_level: 25
  learn_points: 11
  changes: Concentration +2
- level: 3
  id: 2113
  needs_level: 30
  learn_points: 12
  changes: Concentration +3
- level: 4
  id: 2114
  needs_level: 35
  learn_points: 13
  changes: Concentration +4
- level: 5
  id: 2115
  needs_level: 40
  learn_points: 14
  changes: Concentration +5
- level: 6
  id: 2116
  needs_level: 45
  learn_points: 15
  changes: Concentration +6
- level: 7
  id: 2117
  needs_level: 50
  learn_points: 16
  changes: Concentration +7
- level: 8
  id: 2118
  needs_level: 55
  learn_points: 17
  changes: Concentration +8
- level: 9
  id: 2119
  needs_level: 60
  learn_points: 18
  changes: Concentration +9
- level: 10
  id: 2120
  needs_level: 65
  learn_points: 19
  changes: Concentration +10
- level: 11
  id: 2121
  needs_level: 70
  learn_points: 20
  changes: Concentration +11
- level: 12
  id: 2122
  needs_level: 80
  learn_points: 21
  changes: Concentration +12
- level: 13
  id: 2123
  needs_level: 90
  learn_points: 22
  changes: Concentration +13
- level: 14
  id: 2124
  needs_level: 100
  learn_points: 23
  changes: Concentration +14
- level: 15
  id: 2125
  needs_level: 110
  learn_points: 24
  changes: Concentration +15
- level: 16
  id: 2126
  needs_level: 120
  learn_points: 25
  changes: Concentration +16
- level: 17
  id: 2127
  needs_level: 130
  learn_points: 26
  changes: Concentration +17
- level: 18
  id: 2128
  needs_level: 140
  learn_points: 27
  changes: Concentration +18
- level: 19
  id: 2129
  needs_level: 150
  learn_points: 28
  changes: Concentration +19
- level: 20
  id: 2130
  needs_level: 160
  learn_points: 29
  changes: Concentration +20
source:
  data: LIST_SKILL.STB rows 2111, 2112, 2113, 2114, 2115, 2116, 2117, 2118, 2119, 2120, 2121, 2122, 2123, 2124, 2125, 2126, 2127, 2128, 2129, 2130
  code: module/src/skills.rs
---
# Weapon Research

Allows its user to learn weapon crafting skills. Increase Concentration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
