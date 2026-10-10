---
kind: skill
id: 1691
name: Sharp Arrowhead
status: in-game
icon: skill/217
type: Continuing (Self)
job: Scout Job
max_level: 10
target: Yourself
needs_weapon: Bow, Crossbow
skill_books:
- '[[items/consumable/774-sharp-arrowhead|Sharp Arrowhead]]'
levels:
- level: 1
  id: 1691
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 10, [[skills/1681-speed-shot|Speed Shot]] level 5'
  learn_points: 25
  cost: MP 30
  cooldown: 80
  duration: 50
  effects: Atk Power Increased
  changes: Attack Power +26
- level: 2
  id: 1692
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 11'
  learn_points: 28
  cost: MP 32
  cooldown: 81.4
  duration: 52
  effects: Atk Power Increased
  changes: Attack Power +30
- level: 3
  id: 1693
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 12'
  learn_points: 31
  cost: MP 34
  cooldown: 82.8
  duration: 54
  effects: Atk Power Increased
  changes: Attack Power +34
- level: 4
  id: 1694
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 13'
  learn_points: 34
  cost: MP 36
  cooldown: 84.2
  duration: 56
  effects: Atk Power Increased
  changes: Attack Power +38
- level: 5
  id: 1695
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 14'
  learn_points: 38
  cost: MP 38
  cooldown: 85.6
  duration: 58
  effects: Atk Power Increased
  changes: Attack Power +42
- level: 6
  id: 1696
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 15'
  learn_points: 42
  cost: MP 40
  cooldown: 87
  duration: 60
  effects: Atk Power Increased
  changes: Attack Power +46
- level: 7
  id: 1697
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 16'
  learn_points: 46
  cost: MP 42
  cooldown: 88.4
  duration: 62
  effects: Atk Power Increased
  changes: Attack Power +50
- level: 8
  id: 1698
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 17'
  learn_points: 51
  cost: MP 44
  cooldown: 89.8
  duration: 64
  effects: Atk Power Increased
  changes: Attack Power +54
- level: 9
  id: 1699
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 18'
  learn_points: 56
  cost: MP 46
  cooldown: 91.2
  duration: 66
  effects: Atk Power Increased
  changes: Attack Power +58
- level: 10
  id: 1700
  needs: '[[skills/1401-bow-mastery|Bow Mastery]] level 19'
  learn_points: 61
  cost: MP 50
  cooldown: 92.6
  duration: 70
  effects: Atk Power Increased
  changes: Attack Power +63
source:
  data: LIST_SKILL.STB rows 1691, 1692, 1693, 1694, 1695, 1696, 1697, 1698, 1699, 1700
  code: module/src/skills.rs
---
# Sharp Arrowhead

Increase Attack Power of Bow and Crossbow weapon for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
