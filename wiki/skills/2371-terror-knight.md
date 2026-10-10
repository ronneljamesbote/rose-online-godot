---
kind: skill
id: 2371
name: Terror Knight
status: in-game
icon: skill/189
type: Summoning Magic
job: Bourgeois Job
max_level: 10
target: Yourself
summons: '[[monsters/871-terror-knight|Terror Knight]]'
skill_books:
- '[[items/consumable/831-terror-knight|Terror Knight]]'
levels:
- level: 1
  id: 2371
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 8, [[skills/2361-hire-hunter|Hire Hunter]] level 3'
  learn_points: 40
  cost: Money 250, MP 100
  cooldown: 8
- level: 2
  id: 2372
  learn_points: 45
  cost: Money 265, MP 105
  cooldown: 8.2
- level: 3
  id: 2373
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 9'
  learn_points: 51
  cost: Money 280, MP 110
  cooldown: 8.4
- level: 4
  id: 2374
  learn_points: 58
  cost: Money 295, MP 115
  cooldown: 8.6
- level: 5
  id: 2375
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 10'
  learn_points: 65
  cost: Money 310, MP 120
  cooldown: 8.8
- level: 6
  id: 2376
  learn_points: 73
  cost: Money 325, MP 125
  cooldown: 9
- level: 7
  id: 2377
  learn_points: 82
  cost: Money 340, MP 130
  cooldown: 9.2
- level: 8
  id: 2378
  learn_points: 92
  cost: Money 355, MP 135
  cooldown: 9.4
- level: 9
  id: 2379
  learn_points: 103
  cost: Money 370, MP 140
  cooldown: 9.6
- level: 10
  id: 2380
  learn_points: 115
  cost: Money 400, MP 145
  cooldown: 9.8
source:
  data: LIST_SKILL.STB rows 2371, 2372, 2373, 2374, 2375, 2376, 2377, 2378, 2379, 2380
  code: module/src/skills.rs
---
# Terror Knight

Summon a Terror Knight.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
