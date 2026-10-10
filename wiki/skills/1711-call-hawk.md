---
kind: skill
id: 1711
name: Call Hawk
status: in-game
icon: skill/133
type: Summoning Magic
job: Scout Job
max_level: 10
target: Yourself
summons: '[[monsters/881-hawk|Hawk]]'
skill_books:
- '[[items/consumable/776-call-hawk|Call Hawk]]'
levels:
- level: 1
  id: 1711
  needs: '[[skills/1441-relax|Relax]] level 5'
  learn_points: 30
  cost: MP 155
  cooldown: 6
- level: 2
  id: 1712
  learn_points: 34
  cost: MP 160
  cooldown: 6.4
- level: 3
  id: 1713
  needs: '[[skills/1441-relax|Relax]] level 6'
  learn_points: 38
  cost: MP 165
  cooldown: 6.8
- level: 4
  id: 1714
  learn_points: 43
  cost: MP 170
  cooldown: 7.2
- level: 5
  id: 1715
  needs: '[[skills/1441-relax|Relax]] level 7'
  learn_points: 48
  cost: MP 175
  cooldown: 7.6
- level: 6
  id: 1716
  learn_points: 54
  cost: MP 180
  cooldown: 8
- level: 7
  id: 1717
  needs: '[[skills/1441-relax|Relax]] level 8'
  learn_points: 60
  cost: MP 185
  cooldown: 8.4
- level: 8
  id: 1718
  learn_points: 67
  cost: MP 190
  cooldown: 8.8
- level: 9
  id: 1719
  needs: '[[skills/1441-relax|Relax]] level 9'
  learn_points: 75
  cost: MP 195
  cooldown: 9.2
- level: 10
  id: 1720
  learn_points: 84
  cost: MP 200
  cooldown: 9.6
source:
  data: LIST_SKILL.STB rows 1711, 1712, 1713, 1714, 1715, 1716, 1717, 1718, 1719, 1720
  code: module/src/skills.rs
---
# Call Hawk

Summon a hawk.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
