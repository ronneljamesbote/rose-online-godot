---
kind: skill
id: 291
name: Spiritual Training
status: in-game
icon: skill/69
type: Passive
job: Soldier Job
max_level: 10
target: Yourself
skill_books:
- '[[items/consumable/4-health-bottle-s|Health Bottle (S)]]'
- '[[items/consumable/22-mana-vial-m|Mana Vial (M)]]'
- '[[items/consumable/109-cooked-meat|Cooked Meat]]'
- '[[items/consumable/117-meatball|Meatball]]'
- '[[items/consumable/134-ice-berry|Ice Berry]]'
- '[[items/consumable/154-hp-point-300|HP Point (+300)]]'
- '[[items/consumable/165-mp-point-300|MP Point (+300)]]'
- '[[items/consumable/176-stamina-300|Stamina (+300)]]'
- '[[items/consumable/658-spiritual-training|Spiritual Training]]'
levels:
- level: 1
  id: 291
  learn_points: 6
  changes: Max MP +30
- level: 2
  id: 292
  learn_points: 8
  changes: Max MP +40
- level: 3
  id: 293
  learn_points: 11
  changes: Max MP +52
- level: 4
  id: 294
  learn_points: 14
  changes: Max MP +66
- level: 5
  id: 295
  learn_points: 18
  changes: Max MP +82
- level: 6
  id: 296
  learn_points: 23
  changes: Max MP +100
- level: 7
  id: 297
  learn_points: 29
  changes: Max MP +120
- level: 8
  id: 298
  learn_points: 36
  changes: Max MP +142
- level: 9
  id: 299
  learn_points: 44
  changes: Max MP +168
- level: 10
  id: 300
  learn_points: 54
  changes: Max MP +200
source:
  data: LIST_SKILL.STB rows 291, 292, 293, 294, 295, 296, 297, 298, 299, 300
  code: module/src/skills.rs
---
# Spiritual Training

Increase Maximum MP amount.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
