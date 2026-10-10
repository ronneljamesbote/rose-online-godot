---
kind: skill
id: 1161
name: Bonfire
status: in-game
icon: skill/75
type: Summoning Magic
job: Muse Job
max_level: 10
target: Yourself
summons: '[[monsters/801-bonfire|Bonfire]]'
skill_books:
- '[[items/consumable/733-bonfire|Bonfire]]'
levels:
- level: 1
  id: 1161
  needs: '[[skills/821-meditation|Meditation]] level 6, [[skills/931-cure|Cure]] level 2'
  learn_points: 12
  cost: MP 70
  cooldown: 8
- level: 2
  id: 1162
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 15
  cost: MP 73
  cooldown: 8.6
- level: 3
  id: 1163
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 18
  cost: MP 76
  cooldown: 9.2
- level: 4
  id: 1164
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 22
  cost: MP 79
  cooldown: 9.8
- level: 5
  id: 1165
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 27
  cost: MP 82
  cooldown: 10.4
- level: 6
  id: 1166
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 32
  cost: MP 85
  cooldown: 11
- level: 7
  id: 1167
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 38
  cost: MP 88
  cooldown: 11.6
- level: 8
  id: 1168
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 45
  cost: MP 91
  cooldown: 12.2
- level: 9
  id: 1169
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 53
  cost: MP 94
  cooldown: 12.8
- level: 10
  id: 1170
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 62
  cost: MP 100
  cooldown: 13.4
source:
  data: LIST_SKILL.STB rows 1161, 1162, 1163, 1164, 1165, 1166, 1167, 1168, 1169, 1170
  code: module/src/skills.rs
---
# Bonfire

Cast a magic campfire to aid party members in restoring HP and MP faster.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
