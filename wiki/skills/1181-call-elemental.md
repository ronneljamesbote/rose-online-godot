---
kind: skill
id: 1181
name: Call Elemental
status: in-game
icon: skill/204
type: Summoning Magic
job: Muse 2nd Advanced Jobs
max_level: 10
target: Yourself
summons: '[[monsters/841-elemental|Elemental]]'
skill_books:
- '[[items/consumable/735-call-elemental|Call Elemental]]'
levels:
- level: 1
  id: 1181
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 15, [[skills/861-summon-mastery|Summon Mastery]] level 3, [[skills/1171-phantom-sword|Phantom Sword]] level 5'
  learn_points: 35
  cost: MP 400
  cooldown: 10
- level: 2
  id: 1182
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 16'
  learn_points: 40
  cost: MP 420
  cooldown: 10.6
- level: 3
  id: 1183
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 17, [[skills/861-summon-mastery|Summon Mastery]] level 4'
  learn_points: 46
  cost: MP 440
  cooldown: 11.2
- level: 4
  id: 1184
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 18'
  learn_points: 53
  cost: MP 460
  cooldown: 11.8
- level: 5
  id: 1185
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 19, [[skills/861-summon-mastery|Summon Mastery]] level 5'
  learn_points: 60
  cost: MP 480
  cooldown: 12.4
- level: 6
  id: 1186
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 20'
  learn_points: 68
  cost: MP 500
  cooldown: 13
- level: 7
  id: 1187
  needs: '[[skills/861-summon-mastery|Summon Mastery]] level 6'
  learn_points: 77
  cost: MP 520
  cooldown: 13.6
- level: 8
  id: 1188
  learn_points: 87
  cost: MP 540
  cooldown: 14.2
- level: 9
  id: 1189
  needs: '[[skills/861-summon-mastery|Summon Mastery]] level 7'
  learn_points: 98
  cost: MP 560
  cooldown: 14.8
- level: 10
  id: 1190
  learn_points: 111
  cost: MP 600
  cooldown: 15.4
source:
  data: LIST_SKILL.STB rows 1181, 1182, 1183, 1184, 1185, 1186, 1187, 1188, 1189, 1190
  code: module/src/skills.rs
---
# Call Elemental

Summon an Elemental.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
