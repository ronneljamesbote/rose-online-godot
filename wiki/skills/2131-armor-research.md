---
kind: skill
id: 2131
name: Armor Research
status: in-game
icon: skill/170
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/809-armor-research|Armor Research]]'
levels:
- level: 1
  id: 2131
  needs_level: 30
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 3'
  learn_points: 15
  changes: Defense +6
- level: 2
  id: 2132
  needs_level: 45
  learn_points: 18
  changes: Defense +12
- level: 3
  id: 2133
  needs_level: 60
  learn_points: 21
  changes: Defense +18
- level: 4
  id: 2134
  needs_level: 70
  learn_points: 24
  changes: Defense +24
- level: 5
  id: 2135
  needs_level: 85
  learn_points: 27
  changes: Defense +30
- level: 6
  id: 2136
  needs_level: 100
  learn_points: 31
  changes: Defense +36
- level: 7
  id: 2137
  needs_level: 115
  learn_points: 35
  changes: Defense +42
- level: 8
  id: 2138
  needs_level: 130
  learn_points: 40
  changes: Defense +48
- level: 9
  id: 2139
  needs_level: 145
  learn_points: 45
  changes: Defense +54
- level: 10
  id: 2140
  needs_level: 160
  learn_points: 51
  changes: Defense +60
source:
  data: LIST_SKILL.STB rows 2131, 2132, 2133, 2134, 2135, 2136, 2137, 2138, 2139, 2140
  code: module/src/skills.rs
---
# Armor Research

Refine your fundamental armor crafting skills. Increase Defense.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
