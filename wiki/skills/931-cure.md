---
kind: skill
id: 931
name: Cure
status: in-game
icon: skill/84
type: Magic Recovery
job: Muse Job
max_level: 10
target: Ally
skill_books:
- '[[items/consumable/710-cure|Cure]]'
levels:
- level: 1
  id: 931
  needs: '[[skills/821-meditation|Meditation]] level 1'
  learn_points: 8
  cost: MP 30
  range: 20
  cooldown: 2.4
  changes: HP +200
- level: 2
  id: 932
  needs: '[[skills/821-meditation|Meditation]] level 3'
  learn_points: 10
  cost: MP 35
  range: 20
  cooldown: 2.4
  changes: HP +260
- level: 3
  id: 933
  needs: '[[skills/821-meditation|Meditation]] level 5'
  learn_points: 13
  cost: MP 40
  range: 20
  cooldown: 2.4
  changes: HP +320
- level: 4
  id: 934
  needs: '[[skills/821-meditation|Meditation]] level 7'
  learn_points: 16
  cost: MP 45
  range: 20
  cooldown: 2.4
  changes: HP +380
- level: 5
  id: 935
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 20
  cost: MP 50
  range: 20
  cooldown: 2.4
  changes: HP +440
- level: 6
  id: 936
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 25
  cost: MP 55
  range: 20
  cooldown: 2.4
  changes: HP +500
- level: 7
  id: 937
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 31
  cost: MP 60
  range: 20
  cooldown: 2.4
  changes: HP +560
- level: 8
  id: 938
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 38
  cost: MP 65
  range: 20
  cooldown: 2.4
  changes: HP +620
- level: 9
  id: 939
  needs: '[[skills/821-meditation|Meditation]] level 17'
  learn_points: 46
  cost: MP 70
  range: 20
  cooldown: 2.4
  changes: HP +680
- level: 10
  id: 940
  needs: '[[skills/821-meditation|Meditation]] level 19'
  learn_points: 55
  cost: MP 75
  range: 20
  cooldown: 2.4
  changes: HP +740
source:
  data: LIST_SKILL.STB rows 931, 932, 933, 934, 935, 936, 937, 938, 939, 940
  code: module/src/skills.rs
---
# Cure

Restore HP of a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
