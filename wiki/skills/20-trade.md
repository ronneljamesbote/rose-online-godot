---
kind: skill
id: 20
name: Trade
status: in-game
icon: skill/7
type: Basic Action
job: any
max_level: 0
target: Yourself
skill_books:
- '[[items/consumable/55-mana-jam-20|Mana Jam (+20)]]'
- '[[items/consumable/188-clan-point-20|Clan Point (+20)]]'
- '[[items/consumable/610-trade|Trade]]'
levels:
- level: 0
  id: 20
  learn_points: 0
  range: 0.1
source:
  data: LIST_SKILL.STB rows 20
  code: module/src/skills.rs
---
# Trade

Request a trade with a target.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
