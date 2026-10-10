---
kind: skill
id: 25
name: Ride Request
status: in-game
icon: skill/5
type: Basic Action
job: any
max_level: 1
target: Yourself
skill_books:
- '[[items/consumable/189-clan-point-25|Clan Point (+25)]]'
- '[[items/consumable/293-engine-fuel-s|Engine Fuel (S)]]'
- '[[items/consumable/615-ride-request|Ride Request]]'
levels:
- level: 1
  id: 25
  learn_points: 0
source:
  data: LIST_SKILL.STB rows 25
  code: module/src/skills.rs
---
# Ride Request

Ask for a ride on a Cart.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
