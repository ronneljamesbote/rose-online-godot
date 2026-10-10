---
kind: skill
id: 2001
name: Market Research
status: in-game
icon: skill/143
type: Passive
job: Dealer Job
max_level: 20
target: Yourself
warps_to: zone 14
skill_books:
- '[[items/consumable/801-market-research|Market Research]]'
levels:
- level: 1
  id: 2001
  learn_points: 4
  changes: Charm +2
- level: 2
  id: 2002
  learn_points: 5
  changes: Charm +4
- level: 3
  id: 2003
  learn_points: 6
  changes: Charm +6
- level: 4
  id: 2004
  learn_points: 7
  changes: Charm +8
- level: 5
  id: 2005
  learn_points: 9
  changes: Charm +10
- level: 6
  id: 2006
  learn_points: 11
  changes: Charm +12
- level: 7
  id: 2007
  learn_points: 13
  changes: Charm +14
- level: 8
  id: 2008
  learn_points: 15
  changes: Charm +16
- level: 9
  id: 2009
  learn_points: 17
  changes: Charm +18
- level: 10
  id: 2010
  learn_points: 19
  changes: Charm +20
- level: 11
  id: 2011
  learn_points: 21
  changes: Charm +22
- level: 12
  id: 2012
  learn_points: 23
  changes: Charm +24
- level: 13
  id: 2013
  learn_points: 26
  changes: Charm +26
- level: 14
  id: 2014
  learn_points: 29
  changes: Charm +28
- level: 15
  id: 2015
  learn_points: 32
  changes: Charm +30
- level: 16
  id: 2016
  learn_points: 35
  changes: Charm +32
- level: 17
  id: 2017
  learn_points: 38
  changes: Charm +34
- level: 18
  id: 2018
  learn_points: 42
  changes: Charm +36
- level: 19
  id: 2019
  learn_points: 46
  changes: Charm +38
- level: 20
  id: 2020
  learn_points: 50
  changes: Charm +40
source:
  data: LIST_SKILL.STB rows 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011, 2012, 2013, 2014, 2015, 2016, 2017, 2018, 2019, 2020
  code: module/src/skills.rs
---
# Market Research

Study economy. Increase Charm by 2 each time the level of this skill is increased.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
