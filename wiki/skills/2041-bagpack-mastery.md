---
kind: skill
id: 2041
name: Bagpack Mastery
status: in-game
icon: skill/169
type: Passive
job: Dealer Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/803-bagpack-mastery|Bagpack Mastery]]'
levels:
- level: 1
  id: 2041
  needs: '[[skills/2001-market-research|Market Research]] level 2'
  learn_points: 6
  changes: Bagpack Capacity +280
- level: 2
  id: 2042
  needs: '[[skills/2001-market-research|Market Research]] level 3'
  learn_points: 8
  changes: Bagpack Capacity +360
- level: 3
  id: 2043
  needs: '[[skills/2001-market-research|Market Research]] level 4'
  learn_points: 10
  changes: Bagpack Capacity +440
- level: 4
  id: 2044
  needs: '[[skills/2001-market-research|Market Research]] level 5'
  learn_points: 12
  changes: Bagpack Capacity +520
- level: 5
  id: 2045
  needs: '[[skills/2001-market-research|Market Research]] level 6'
  learn_points: 15
  changes: Bagpack Capacity +600
- level: 6
  id: 2046
  needs: '[[skills/2001-market-research|Market Research]] level 7'
  learn_points: 18
  changes: Bagpack Capacity +680
- level: 7
  id: 2047
  needs: '[[skills/2001-market-research|Market Research]] level 8'
  learn_points: 22
  changes: Bagpack Capacity +760
- level: 8
  id: 2048
  needs: '[[skills/2001-market-research|Market Research]] level 9'
  learn_points: 26
  changes: Bagpack Capacity +840
- level: 9
  id: 2049
  needs: '[[skills/2001-market-research|Market Research]] level 10'
  learn_points: 31
  changes: Bagpack Capacity +920
- level: 10
  id: 2050
  needs: '[[skills/2001-market-research|Market Research]] level 11'
  learn_points: 36
  changes: Bagpack Capacity +1000
source:
  data: LIST_SKILL.STB rows 2041, 2042, 2043, 2044, 2045, 2046, 2047, 2048, 2049, 2050
  code: module/src/skills.rs
---
# Bagpack Mastery

Increase the number of items which can be carried inside a Bagpack.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
