---
kind: skill
id: 2091
name: Stockpile
status: in-game
icon: skill/169
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
warps_to: zone 63
skill_books:
- '[[items/consumable/806-stockpile|Stockpile]]'
levels:
- level: 1
  id: 2091
  needs: '[[skills/2001-market-research|Market Research]] level 5, [[skills/2041-bagpack-mastery|Bagpack Mastery]] level 3'
  learn_points: 10
  changes: Item Drop Rate Increase +6
- level: 2
  id: 2092
  needs: '[[skills/2001-market-research|Market Research]] level 6'
  learn_points: 13
  changes: Item Drop Rate Increase +8
- level: 3
  id: 2093
  needs: '[[skills/2001-market-research|Market Research]] level 7'
  learn_points: 17
  changes: Item Drop Rate Increase +10
- level: 4
  id: 2094
  needs: '[[skills/2001-market-research|Market Research]] level 8'
  learn_points: 21
  changes: Item Drop Rate Increase +12
- level: 5
  id: 2095
  needs: '[[skills/2001-market-research|Market Research]] level 9'
  learn_points: 26
  changes: Item Drop Rate Increase +13
- level: 6
  id: 2096
  needs: '[[skills/2001-market-research|Market Research]] level 10'
  learn_points: 32
  changes: Item Drop Rate Increase +14
- level: 7
  id: 2097
  needs: '[[skills/2001-market-research|Market Research]] level 12'
  learn_points: 39
  changes: Item Drop Rate Increase +15
- level: 8
  id: 2098
  needs: '[[skills/2001-market-research|Market Research]] level 14'
  learn_points: 48
  changes: Item Drop Rate Increase +16
- level: 9
  id: 2099
  needs: '[[skills/2001-market-research|Market Research]] level 16'
  learn_points: 59
  changes: Item Drop Rate Increase +17
- level: 10
  id: 2100
  needs: '[[skills/2001-market-research|Market Research]] level 18'
  learn_points: 72
  changes: Item Drop Rate Increase +18
source:
  data: LIST_SKILL.STB rows 2091, 2092, 2093, 2094, 2095, 2096, 2097, 2098, 2099, 2100
  code: module/src/skills.rs
---
# Stockpile

Increase the chance of obtaining items from monsters.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
