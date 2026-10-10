---
kind: skill
id: 2361
name: Hire Hunter
status: in-game
icon: skill/182
type: Summoning Magic
job: Bourgeois Job
max_level: 10
target: Yourself
summons: '[[monsters/861-mercenary-hunter|Mercenary Hunter]]'
skill_books:
- '[[items/consumable/830-hire-hunter|Hire Hunter]]'
levels:
- level: 1
  id: 2361
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 5, [[skills/2351-hire-warrior|Hire Warrior]] level 3'
  learn_points: 25
  cost: Money 200
  cooldown: 6
- level: 2
  id: 2362
  learn_points: 28
  cost: Money 210
  cooldown: 6
- level: 3
  id: 2363
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 6'
  learn_points: 32
  cost: Money 220
  cooldown: 6
- level: 4
  id: 2364
  learn_points: 36
  cost: Money 230
  cooldown: 6
- level: 5
  id: 2365
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 7'
  learn_points: 40
  cost: Money 240
  cooldown: 6
- level: 6
  id: 2366
  learn_points: 45
  cost: Money 250
  cooldown: 6
- level: 7
  id: 2367
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 8'
  learn_points: 50
  cost: Money 260
  cooldown: 6
- level: 8
  id: 2368
  learn_points: 56
  cost: Money 270
  cooldown: 6
- level: 9
  id: 2369
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 9'
  learn_points: 62
  cost: Money 280
  cooldown: 6
- level: 10
  id: 2370
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 10'
  learn_points: 69
  cost: Money 300
  cooldown: 6
source:
  data: LIST_SKILL.STB rows 2361, 2362, 2363, 2364, 2365, 2366, 2367, 2368, 2369, 2370
  code: module/src/skills.rs
---
# Hire Hunter

Use Zulie to summon a Hunter that specializes in long distance attacks.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
