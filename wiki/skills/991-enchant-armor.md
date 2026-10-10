---
kind: skill
id: 991
name: Enchant Armor
status: in-game
icon: skill/102
type: Continuing (Self)
job: Muse Job
max_level: 10
target: Yourself
warps_to: '[[zones/19-zone-19|Zone 19]]'
skill_books:
- '[[items/consumable/6-health-bottle-l|Health Bottle (L)]]'
- '[[items/consumable/9-herbal-medicine-l|Herbal Medicine (L)]]'
- '[[items/consumable/13-vital-water-xl|Vital Water (XL)]]'
- '[[items/consumable/59-vital-jam-10|Vital Jam (+10)]]'
- '[[items/consumable/141-mysterious-fruit|Mysterious Fruit]]'
- '[[items/consumable/157-hp-point-1000|HP Point (+1000)]]'
- '[[items/consumable/168-mp-point-1000|MP Point (+1000)]]'
- '[[items/consumable/180-stamina-1000|Stamina (+1000)]]'
- '[[items/consumable/716-enchant-armor|Enchant Armor]]'
levels:
- level: 1
  id: 991
  needs: '[[skills/821-meditation|Meditation]] level 7, [[skills/971-blessing|Blessing]] level 3'
  learn_points: 15
  cost: MP 40
  cooldown: 50
  duration: 30
  effects: Def Up
  changes: Defense +70
- level: 2
  id: 992
  needs: '[[skills/821-meditation|Meditation]] level 8'
  learn_points: 18
  cost: MP 43
  cooldown: 52
  duration: 31
  effects: Def Up
  changes: Defense +85
- level: 3
  id: 993
  needs: '[[skills/821-meditation|Meditation]] level 9'
  learn_points: 21
  cost: MP 46
  cooldown: 54
  duration: 32
  effects: Def Up
  changes: Defense +100
- level: 4
  id: 994
  needs: '[[skills/821-meditation|Meditation]] level 10'
  learn_points: 25
  cost: MP 49
  cooldown: 56
  duration: 33
  effects: Def Up
  changes: Defense +115
- level: 5
  id: 995
  needs: '[[skills/821-meditation|Meditation]] level 11'
  learn_points: 29
  cost: MP 52
  cooldown: 58
  duration: 34
  effects: Def Up
  changes: Defense +130
- level: 6
  id: 996
  needs: '[[skills/821-meditation|Meditation]] level 12'
  learn_points: 34
  cost: MP 55
  cooldown: 60
  duration: 35
  effects: Def Up
  changes: Defense +145
- level: 7
  id: 997
  needs: '[[skills/821-meditation|Meditation]] level 13'
  learn_points: 39
  cost: MP 58
  cooldown: 62
  duration: 36
  effects: Def Up
  changes: Defense +160
- level: 8
  id: 998
  needs: '[[skills/821-meditation|Meditation]] level 14'
  learn_points: 45
  cost: MP 61
  cooldown: 64
  duration: 37
  effects: Def Up
  changes: Defense +175
- level: 9
  id: 999
  needs: '[[skills/821-meditation|Meditation]] level 15'
  learn_points: 52
  cost: MP 64
  cooldown: 66
  duration: 40
  effects: Def Up
  changes: Defense +190
- level: 10
  id: 1000
  needs: '[[skills/821-meditation|Meditation]] level 16'
  learn_points: 60
  cost: MP 70
  cooldown: 68
  duration: 45
  effects: Def Up
  changes: Defense +210
source:
  data: LIST_SKILL.STB rows 991, 992, 993, 994, 995, 996, 997, 998, 999, 1000
  code: module/src/skills.rs
---
# Enchant Armor

Increase one's Defense for skill's duration.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
