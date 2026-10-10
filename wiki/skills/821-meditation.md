---
kind: skill
id: 821
name: Meditation
status: in-game
icon: skill/69
type: Passive
job: Muse Job
max_level: 20
target: Yourself
warps_to: '[[zones/55-freezing-plateau|Freezing Plateau]]'
skill_books:
- '[[items/consumable/702-meditation|Meditation]]'
levels:
- level: 1
  id: 821
  learn_points: 4
  changes: Max MP +25, MP Recovery Rate +4
- level: 2
  id: 822
  learn_points: 5
  changes: Max MP +35, MP Recovery Rate +7
- level: 3
  id: 823
  learn_points: 7
  changes: Max MP +46, MP Recovery Rate +10
- level: 4
  id: 824
  learn_points: 9
  changes: Max MP +59, MP Recovery Rate +13
- level: 5
  id: 825
  learn_points: 11
  changes: Max MP +73, MP Recovery Rate +16
- level: 6
  id: 826
  learn_points: 13
  changes: Max MP +89, MP Recovery Rate +19
- level: 7
  id: 827
  learn_points: 16
  changes: Max MP +106, MP Recovery Rate +22
- level: 8
  id: 828
  learn_points: 19
  changes: Max MP +125, MP Recovery Rate +25
- level: 9
  id: 829
  learn_points: 22
  changes: Max MP +145, MP Recovery Rate +28
- level: 10
  id: 830
  learn_points: 26
  changes: Max MP +167, MP Recovery Rate +30
- level: 11
  id: 831
  learn_points: 30
  changes: Max MP +190, MP Recovery Rate +32
- level: 12
  id: 832
  learn_points: 34
  changes: Max MP +215, MP Recovery Rate +34
- level: 13
  id: 833
  learn_points: 38
  changes: Max MP +241, MP Recovery Rate +36
- level: 14
  id: 834
  learn_points: 42
  changes: Max MP +269, MP Recovery Rate +38
- level: 15
  id: 835
  learn_points: 47
  changes: Max MP +298, MP Recovery Rate +40
- level: 16
  id: 836
  learn_points: 52
  changes: Max MP +329, MP Recovery Rate +42
- level: 17
  id: 837
  learn_points: 58
  changes: Max MP +361, MP Recovery Rate +44
- level: 18
  id: 838
  learn_points: 64
  changes: Max MP +395, MP Recovery Rate +46
- level: 19
  id: 839
  learn_points: 71
  changes: Max MP +430, MP Recovery Rate +48
- level: 20
  id: 840
  learn_points: 78
  changes: Max MP +470, MP Recovery Rate +50
source:
  data: LIST_SKILL.STB rows 821, 822, 823, 824, 825, 826, 827, 828, 829, 830, 831, 832, 833, 834, 835, 836, 837, 838, 839, 840
  code: module/src/skills.rs
---
# Meditation

Increase Maximum MP and MP Recovery Rate.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
