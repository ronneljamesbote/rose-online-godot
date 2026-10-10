---
kind: skill
id: 2081
name: Craft Mastery
status: in-game
icon: skill/147
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/811-craft-mastery|Craft Mastery]]'
levels:
- level: 1
  id: 2081
  learn_points: 8
  changes: Max MP +50
- level: 2
  id: 2082
  learn_points: 10
  changes: Max MP +75
- level: 3
  id: 2083
  learn_points: 13
  changes: Max MP +100
- level: 4
  id: 2084
  learn_points: 16
  changes: Max MP +125
- level: 5
  id: 2085
  learn_points: 20
  changes: Max MP +150
- level: 6
  id: 2086
  learn_points: 25
  changes: Max MP +175
- level: 7
  id: 2087
  learn_points: 31
  changes: Max MP +200
- level: 8
  id: 2088
  learn_points: 38
  changes: Max MP +225
- level: 9
  id: 2089
  learn_points: 47
  changes: Max MP +250
- level: 10
  id: 2090
  learn_points: 57
  changes: Max MP +300
source:
  data: LIST_SKILL.STB rows 2081, 2082, 2083, 2084, 2085, 2086, 2087, 2088, 2089, 2090
  code: module/src/skills.rs
---
# Craft Mastery

Learn the essentials of crafting. Increase Maximum MP.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
