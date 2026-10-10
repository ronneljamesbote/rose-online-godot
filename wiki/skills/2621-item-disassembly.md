---
kind: skill
id: 2621
name: Item Disassembly
status: in-game
icon: skill/193
type: Crafting Skill
job: Dealer Job
max_level: 1
target: Yourself
damage_type: continuous attack
skill_books:
- '[[items/consumable/848-item-disassembly|Item Disassembly]]'
levels:
- level: 1
  id: 2621
  needs: '[[skills/2081-craft-mastery|Craft Mastery]] level 4'
  learn_points: 20
  cost: MP 50
  power: 41
source:
  data: LIST_SKILL.STB rows 2621
  code: module/src/skills.rs
---
# Item Disassembly

Detach a gem from an item, or take an item apart in order to obtain its materials.

Range and area are in metres, cooldown and duration in seconds, success in percent.
How power turns into damage is on [[rules/skills|Skills (rules)]].
