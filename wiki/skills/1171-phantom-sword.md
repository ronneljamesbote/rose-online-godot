---
kind: skill
id: 1171
name: Phantom Sword
status: in-game
icon: skill/88
type: Summoning Magic
job: Muse Job
max_level: 10
target: Yourself
summons: '[[monsters/821-phantomsword|PhantomSword]]'
skill_books:
- '[[items/consumable/734-phantom-sword|Phantom Sword]]'
levels:
- level: 1
  id: 1171
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 6, [[skills/861-summon-mastery|Summon Mastery]] level 1'
  learn_points: 20
  cost: MP 200
  cooldown: 6
- level: 2
  id: 1172
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 7'
  learn_points: 23
  cost: MP 210
  cooldown: 6.6
- level: 3
  id: 1173
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 8, [[skills/861-summon-mastery|Summon Mastery]] level 2'
  learn_points: 27
  cost: MP 220
  cooldown: 7.2
- level: 4
  id: 1174
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 9'
  learn_points: 31
  cost: MP 230
  cooldown: 7.8
- level: 5
  id: 1175
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 10, [[skills/861-summon-mastery|Summon Mastery]] level 3'
  learn_points: 36
  cost: MP 240
  cooldown: 8.4
- level: 6
  id: 1176
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 11'
  learn_points: 41
  cost: MP 250
  cooldown: 9
- level: 7
  id: 1177
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 12'
  learn_points: 47
  cost: MP 260
  cooldown: 9.6
- level: 8
  id: 1178
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 13'
  learn_points: 54
  cost: MP 270
  cooldown: 10.2
- level: 9
  id: 1179
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 14'
  learn_points: 61
  cost: MP 280
  cooldown: 10.8
- level: 10
  id: 1180
  needs: '[[skills/801-staff-mastery|Staff Mastery]] level 15'
  learn_points: 69
  cost: MP 300
  cooldown: 11.4
source:
  data: LIST_SKILL.STB rows 1171, 1172, 1173, 1174, 1175, 1176, 1177, 1178, 1179, 1180
  code: module/src/skills.rs
---
# Phantom Sword

Summon a magic sword to attack a target from a distance.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
