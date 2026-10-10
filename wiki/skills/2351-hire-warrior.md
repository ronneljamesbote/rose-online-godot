---
kind: skill
id: 2351
name: Hire Warrior
status: in-game
icon: skill/179
type: Summoning Magic
job: Dealer Job
max_level: 10
target: Yourself
summons: '[[monsters/851-mercenary-warrior|Mercenary Warrior]]'
skill_books:
- '[[items/consumable/829-hire-warrior|Hire Warrior]]'
levels:
- level: 1
  id: 2351
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 1'
  learn_points: 20
  cost: Money 150
  cooldown: 5
- level: 2
  id: 2352
  learn_points: 22
  cost: Money 160
  cooldown: 5
- level: 3
  id: 2353
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 2'
  learn_points: 25
  cost: Money 170
  cooldown: 5
- level: 4
  id: 2354
  learn_points: 28
  cost: Money 180
  cooldown: 5
- level: 5
  id: 2355
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 3'
  learn_points: 31
  cost: Money 190
  cooldown: 5
- level: 6
  id: 2356
  learn_points: 34
  cost: Money 200
  cooldown: 5
- level: 7
  id: 2357
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 4'
  learn_points: 37
  cost: Money 210
  cooldown: 5
- level: 8
  id: 2358
  learn_points: 41
  cost: Money 220
  cooldown: 5
- level: 9
  id: 2359
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 5'
  learn_points: 45
  cost: Money 230
  cooldown: 5
- level: 10
  id: 2360
  needs: '[[skills/2101-hire-mercenary|Hire Mercenary]] level 6'
  learn_points: 49
  cost: Money 250
  cooldown: 5
source:
  data: LIST_SKILL.STB rows 2351, 2352, 2353, 2354, 2355, 2356, 2357, 2358, 2359, 2360
  code: module/src/skills.rs
---
# Hire Warrior

Summon a melee attack warrior by spending Zulie.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
