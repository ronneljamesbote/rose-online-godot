---
kind: skill
id: 2101
name: Hire Mercenary
status: in-game
icon: skill/190
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
warps_to: '[[zones/62-shady-jungle|Shady Jungle]]'
skill_books:
- '[[items/consumable/807-hire-mercenary|Hire Mercenary]]'
levels:
- level: 1
  id: 2101
  needs: '[[skills/2001-market-research|Market Research]] level 6'
  learn_points: 15
  changes: Summon Gauge Increase +30
- level: 2
  id: 2102
  needs: '[[skills/2001-market-research|Market Research]] level 7'
  learn_points: 18
  changes: Summon Gauge Increase +40
- level: 3
  id: 2103
  needs: '[[skills/2001-market-research|Market Research]] level 8'
  learn_points: 21
  changes: Summon Gauge Increase +50
- level: 4
  id: 2104
  needs: '[[skills/2001-market-research|Market Research]] level 9'
  learn_points: 25
  changes: Summon Gauge Increase +60
- level: 5
  id: 2105
  needs: '[[skills/2001-market-research|Market Research]] level 10'
  learn_points: 29
  changes: Summon Gauge Increase +70
- level: 6
  id: 2106
  needs: '[[skills/2001-market-research|Market Research]] level 12'
  learn_points: 34
  changes: Summon Gauge Increase +80
- level: 7
  id: 2107
  needs: '[[skills/2001-market-research|Market Research]] level 14'
  learn_points: 39
  changes: Summon Gauge Increase +90
- level: 8
  id: 2108
  needs: '[[skills/2001-market-research|Market Research]] level 16'
  learn_points: 45
  changes: Summon Gauge Increase +100
- level: 9
  id: 2109
  needs: '[[skills/2001-market-research|Market Research]] level 18'
  learn_points: 52
  changes: Summon Gauge Increase +110
- level: 10
  id: 2110
  needs: '[[skills/2001-market-research|Market Research]] level 20'
  learn_points: 60
  changes: Summon Gauge Increase +120
source:
  data: LIST_SKILL.STB rows 2101, 2102, 2103, 2104, 2105, 2106, 2107, 2108, 2109, 2110
  code: module/src/skills.rs
---
# Hire Mercenary

Increase Maximum Summon Gauge.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
