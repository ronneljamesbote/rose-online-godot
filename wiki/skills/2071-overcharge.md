---
kind: skill
id: 2071
name: Overcharge
status: in-game
icon: skill/178
type: Passive
job: Bourgeois Job
max_level: 10
target: Yourself
warps_to: zone 60
skill_books:
- '[[items/consumable/805-overcharge|Overcharge]]'
levels:
- level: 1
  id: 2071
  needs: '[[skills/2001-market-research|Market Research]] level 10, [[skills/2051-discount|Discount]] level 5'
  learn_points: 25
  changes: Sales Premium +2
- level: 2
  id: 2072
  needs: '[[skills/2001-market-research|Market Research]] level 11'
  learn_points: 30
  changes: Sales Premium +4
- level: 3
  id: 2073
  needs: '[[skills/2001-market-research|Market Research]] level 12'
  learn_points: 36
  changes: Sales Premium +6
- level: 4
  id: 2074
  needs: '[[skills/2001-market-research|Market Research]] level 13'
  learn_points: 43
  changes: Sales Premium +8
- level: 5
  id: 2075
  needs: '[[skills/2001-market-research|Market Research]] level 14'
  learn_points: 51
  changes: Sales Premium +10
- level: 6
  id: 2076
  needs: '[[skills/2001-market-research|Market Research]] level 15'
  learn_points: 60
  changes: Sales Premium +12
- level: 7
  id: 2077
  needs: '[[skills/2001-market-research|Market Research]] level 16'
  learn_points: 71
  changes: Sales Premium +13
- level: 8
  id: 2078
  needs: '[[skills/2001-market-research|Market Research]] level 17'
  learn_points: 83
  changes: Sales Premium +14
- level: 9
  id: 2079
  needs: '[[skills/2001-market-research|Market Research]] level 18'
  learn_points: 97
  changes: Sales Premium +15
- level: 10
  id: 2080
  needs: '[[skills/2001-market-research|Market Research]] level 20'
  learn_points: 114
  changes: Sales Premium +16
source:
  data: LIST_SKILL.STB rows 2071, 2072, 2073, 2074, 2075, 2076, 2077, 2078, 2079, 2080
  code: module/src/skills.rs
---
# Overcharge

Enables its user to sell items to NPC shops for more Zulie than the original price.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
