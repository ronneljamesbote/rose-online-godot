---
kind: skill
id: 2051
name: Discount
status: in-game
icon: skill/146
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
warps_to: '[[zones/59-luna-clan-field|Luna Clan Field]]'
skill_books:
- '[[items/consumable/804-discount|Discount]]'
levels:
- level: 1
  id: 2051
  needs: '[[skills/2001-market-research|Market Research]] level 4'
  learn_points: 10
  changes: Sales Discount +2
- level: 2
  id: 2052
  needs: '[[skills/2001-market-research|Market Research]] level 5'
  learn_points: 14
  changes: Sales Discount +4
- level: 3
  id: 2053
  needs: '[[skills/2001-market-research|Market Research]] level 6'
  learn_points: 19
  changes: Sales Discount +6
- level: 4
  id: 2054
  needs: '[[skills/2001-market-research|Market Research]] level 7'
  learn_points: 25
  changes: Sales Discount +8
- level: 5
  id: 2055
  needs: '[[skills/2001-market-research|Market Research]] level 8'
  learn_points: 33
  changes: Sales Discount +10
- level: 6
  id: 2056
  needs: '[[skills/2001-market-research|Market Research]] level 9'
  learn_points: 43
  changes: Sales Discount +12
- level: 7
  id: 2057
  needs: '[[skills/2001-market-research|Market Research]] level 10'
  learn_points: 56
  changes: Sales Discount +14
- level: 8
  id: 2058
  needs: '[[skills/2001-market-research|Market Research]] level 12'
  learn_points: 73
  changes: Sales Discount +16
- level: 9
  id: 2059
  needs: '[[skills/2001-market-research|Market Research]] level 14'
  learn_points: 94
  changes: Sales Discount +18
- level: 10
  id: 2060
  needs: '[[skills/2001-market-research|Market Research]] level 16'
  learn_points: 121
  changes: Sales Discount +20
source:
  data: LIST_SKILL.STB rows 2051, 2052, 2053, 2054, 2055, 2056, 2057, 2058, 2059, 2060
  code: module/src/skills.rs
---
# Discount

Enables its user to purchase items from NPC shops for less Zulie than the original price.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
