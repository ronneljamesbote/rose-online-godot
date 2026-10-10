---
kind: skill
id: 841
name: Spell Mastery
status: in-game
icon: skill/73
type: Passive
job: Muse Job
max_level: 20
target: Yourself
skill_books:
- '[[items/consumable/703-spell-mastery|Spell Mastery]]'
levels:
- level: 1
  id: 841
  learn_points: 4
  changes: MP Cost Reduction +4
- level: 2
  id: 842
  learn_points: 5
  changes: MP Cost Reduction +6
- level: 3
  id: 843
  learn_points: 7
  changes: MP Cost Reduction +8
- level: 4
  id: 844
  learn_points: 9
  changes: MP Cost Reduction +10
- level: 5
  id: 845
  learn_points: 11
  changes: MP Cost Reduction +12
- level: 6
  id: 846
  learn_points: 13
  changes: MP Cost Reduction +14
- level: 7
  id: 847
  learn_points: 16
  changes: MP Cost Reduction +16
- level: 8
  id: 848
  learn_points: 19
  changes: MP Cost Reduction +18
- level: 9
  id: 849
  learn_points: 22
  changes: MP Cost Reduction +19
- level: 10
  id: 850
  learn_points: 26
  changes: MP Cost Reduction +20
- level: 11
  id: 851
  learn_points: 30
  changes: MP Cost Reduction +21
- level: 12
  id: 852
  learn_points: 34
  changes: MP Cost Reduction +22
- level: 13
  id: 853
  learn_points: 38
  changes: MP Cost Reduction +23
- level: 14
  id: 854
  learn_points: 42
  changes: MP Cost Reduction +24
- level: 15
  id: 855
  learn_points: 47
  changes: MP Cost Reduction +25
- level: 16
  id: 856
  learn_points: 52
  changes: MP Cost Reduction +26
- level: 17
  id: 857
  learn_points: 58
  changes: MP Cost Reduction +27
- level: 18
  id: 858
  learn_points: 64
  changes: MP Cost Reduction +28
- level: 19
  id: 859
  learn_points: 71
  changes: MP Cost Reduction +29
- level: 20
  id: 860
  learn_points: 78
  changes: MP Cost Reduction +30
source:
  data: LIST_SKILL.STB rows 841, 842, 843, 844, 845, 846, 847, 848, 849, 850, 851, 852, 853, 854, 855, 856, 857, 858, 859, 860
  code: module/src/skills.rs
---
# Spell Mastery

Reduce MP consumption of skills by a certain percentage.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
