---
kind: skill
id: 1191
name: Call Firegon
status: in-game
icon: skill/203
type: Summoning Magic
job: Mage Job
max_level: 10
target: Yourself
summons: '[[monsters/831-firegon|Firegon]]'
skill_books:
- '[[items/consumable/736-call-firegon|Call Firegon]]'
levels:
- level: 1
  id: 1191
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 15, [[skills/861-summon-mastery|Summon Mastery]] level 1, [[skills/1061-fire-burn|Fire Burn]] level 5'
  learn_points: 30
  cost: MP 250
  cooldown: 9
- level: 2
  id: 1192
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 16'
  learn_points: 35
  cost: MP 260
  cooldown: 9.6
- level: 3
  id: 1193
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 17, [[skills/861-summon-mastery|Summon Mastery]] level 2'
  learn_points: 40
  cost: MP 270
  cooldown: 10.2
- level: 4
  id: 1194
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 18'
  learn_points: 46
  cost: MP 280
  cooldown: 10.8
- level: 5
  id: 1195
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 19, [[skills/861-summon-mastery|Summon Mastery]] level 3'
  learn_points: 53
  cost: MP 290
  cooldown: 11.4
- level: 6
  id: 1196
  needs: '[[skills/841-spell-mastery|Spell Mastery]] level 20'
  learn_points: 60
  cost: MP 300
  cooldown: 12
- level: 7
  id: 1197
  needs: '[[skills/861-summon-mastery|Summon Mastery]] level 4'
  learn_points: 68
  cost: MP 310
  cooldown: 12.6
- level: 8
  id: 1198
  learn_points: 77
  cost: MP 320
  cooldown: 13.2
- level: 9
  id: 1199
  needs: '[[skills/861-summon-mastery|Summon Mastery]] level 5'
  learn_points: 87
  cost: MP 330
  cooldown: 13.8
- level: 10
  id: 1200
  learn_points: 98
  cost: MP 350
  cooldown: 14.4
source:
  data: LIST_SKILL.STB rows 1191, 1192, 1193, 1194, 1195, 1196, 1197, 1198, 1199, 1200
  code: module/src/skills.rs
---
# Call Firegon

Summon a fire dragon.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
