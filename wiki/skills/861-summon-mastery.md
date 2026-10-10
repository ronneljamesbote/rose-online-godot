---
kind: skill
id: 861
name: Summon Mastery
status: in-game
icon: skill/100
type: Passive
job: Muse Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/704-summon-mastery|Summon Mastery]]'
levels:
- level: 1
  id: 861
  needs_level: 40
  learn_points: 25
  changes: Summon Gauge Increase +30
- level: 2
  id: 862
  learn_points: 27
  changes: Summon Gauge Increase +40
- level: 3
  id: 863
  learn_points: 29
  changes: Summon Gauge Increase +50
- level: 4
  id: 864
  learn_points: 31
  changes: Summon Gauge Increase +60
- level: 5
  id: 865
  learn_points: 34
  changes: Summon Gauge Increase +70
- level: 6
  id: 866
  learn_points: 37
  changes: Summon Gauge Increase +80
- level: 7
  id: 867
  learn_points: 40
  changes: Summon Gauge Increase +90
- level: 8
  id: 868
  learn_points: 43
  changes: Summon Gauge Increase +100
- level: 9
  id: 869
  learn_points: 46
  changes: Summon Gauge Increase +110
- level: 10
  id: 870
  learn_points: 49
  changes: Summon Gauge Increase +120
source:
  data: LIST_SKILL.STB rows 861, 862, 863, 864, 865, 866, 867, 868, 869, 870
  code: module/src/skills.rs
---
# Summon Mastery

Enables its caster to summon more monsters at a time.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
