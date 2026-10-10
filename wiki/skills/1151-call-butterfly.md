---
kind: skill
id: 1151
name: Call Butterfly
status: in-game
icon: skill/76
type: Summoning Magic
job: Muse Job
max_level: 10
target: Yourself
summons: '[[monsters/811-butterfly|ButterFly]]'
skill_books:
- '[[items/consumable/732-call-butterfly|Call Butterfly]]'
levels:
- level: 1
  id: 1151
  needs_level: 25
  learn_points: 8
  cost: MP 40
  cooldown: 12
- level: 2
  id: 1152
  learn_points: 10
  cost: MP 44
  cooldown: 12.2
- level: 3
  id: 1153
  learn_points: 13
  cost: MP 48
  cooldown: 12.4
- level: 4
  id: 1154
  learn_points: 16
  cost: MP 52
  cooldown: 12.6
- level: 5
  id: 1155
  learn_points: 20
  cost: MP 56
  cooldown: 12.8
- level: 6
  id: 1156
  learn_points: 24
  cost: MP 60
  cooldown: 13
- level: 7
  id: 1157
  learn_points: 29
  cost: MP 64
  cooldown: 13.2
- level: 8
  id: 1158
  learn_points: 35
  cost: MP 68
  cooldown: 13.4
- level: 9
  id: 1159
  learn_points: 42
  cost: MP 72
  cooldown: 13.6
- level: 10
  id: 1160
  learn_points: 50
  cost: MP 76
  cooldown: 13.8
source:
  data: LIST_SKILL.STB rows 1151, 1152, 1153, 1154, 1155, 1156, 1157, 1158, 1159, 1160
  code: module/src/skills.rs
---
# Call Butterfly

Summon a magic butterfly to confuse nearby enemies.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
