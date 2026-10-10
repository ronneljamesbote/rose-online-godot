---
kind: skill
id: 491
name: Slow Shot
status: in-game
icon: skill/115
type: Magic Spell
job: Knight Job
max_level: 10
target: Hostile Character
damage_type: weapon attack
needs_weapon: Crossbow
skill_books:
- '[[items/consumable/7-herbal-medicine-s|Herbal Medicine (S)]]'
- '[[items/consumable/11-vital-water-m|Vital Water (M)]]'
- '[[items/consumable/23-mana-vial-l|Mana Vial (L)]]'
- '[[items/consumable/58-vital-jam-5|Vital Jam (+5)]]'
- '[[items/consumable/137-pirukiru|PiruKiru]]'
- '[[items/consumable/155-hp-point-500|HP Point (+500)]]'
- '[[items/consumable/166-mp-point-500|MP Point (+500)]]'
- '[[items/consumable/178-stamina-500|Stamina (+500)]]'
- '[[items/consumable/674-slow-shot|Slow Shot]]'
levels:
- level: 1
  id: 491
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 5'
  learn_points: 15
  cost: MP 20
  power: 50
  cooldown: 8.4
  duration: 16
  success: 70
  effects: Slow
  changes: Movement Speed -40%
- level: 2
  id: 492
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 6'
  learn_points: 18
  cost: MP 24
  power: 60
  cooldown: 8.6
  duration: 17
  success: 72
  effects: Slow
  changes: Movement Speed -42%
- level: 3
  id: 493
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 7'
  learn_points: 22
  cost: MP 28
  power: 70
  cooldown: 8.8
  duration: 18
  success: 74
  effects: Slow
  changes: Movement Speed -44%
- level: 4
  id: 494
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 8'
  learn_points: 26
  cost: MP 32
  power: 80
  cooldown: 9
  duration: 19
  success: 76
  effects: Slow
  changes: Movement Speed -46%
- level: 5
  id: 495
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 9'
  learn_points: 31
  cost: MP 36
  power: 90
  cooldown: 9.2
  duration: 20
  success: 78
  effects: Slow
  changes: Movement Speed -48%
- level: 6
  id: 496
  needs: '[[skills/271-crossbow-mastery|Crossbow Mastery]] level 10'
  learn_points: 36
  cost: MP 40
  power: 100
  cooldown: 10
  duration: 20
  success: 80
  effects: Slow
  changes: Movement Speed -49%
- level: 7
  id: 497
  learn_points: 42
  cost: MP 44
  power: 110
  cooldown: 10.4
  duration: 20
  success: 82
  effects: Slow
  changes: Movement Speed -50%
- level: 8
  id: 498
  learn_points: 49
  cost: MP 48
  power: 120
  cooldown: 10.8
  duration: 20
  success: 84
  effects: Slow
  changes: Movement Speed -51%
- level: 9
  id: 499
  learn_points: 57
  cost: MP 52
  power: 130
  cooldown: 11.2
  duration: 20
  success: 86
  effects: Slow
  changes: Movement Speed -52%
- level: 10
  id: 500
  learn_points: 66
  cost: MP 60
  power: 150
  cooldown: 11.6
  duration: 20
  success: 90
  effects: Slow
  changes: Movement Speed -53%
source:
  data: LIST_SKILL.STB rows 491, 492, 493, 494, 495, 496, 497, 498, 499, 500
  code: module/src/skills.rs
---
# Slow Shot

Inflict damage on a target, and decrease its Movement Speed.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
