---
kind: item
id: weapon/232
name: Air Gun
status: in-game
icon: item/1210
class: Gun
level: 20
attack: 34
attack_speed: 11
range: 22
damage: physical
two_handed: true
durability: 40
quality: 38
needs: Concentration 48
price: 3120
weight: 10
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 1'
recipe:
- material: any Wooden Material
  quantity: 18
- material: '[[items/material/1-rusted-iron|Rusted Iron]]'
  quantity: 3
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 1
craft_difficulty: 20
dropped_by:
- monster: '[[monsters/34-dalping-leader|Dalping Leader]]'
  level: 26
  slots_of_30: 0.2
- monster: '[[monsters/67-queen-honeybee|Queen HoneyBee]]'
  level: 26
  slots_of_30: 0.2
- monster: '[[monsters/85-turtle-guard|Turtle Guard]]'
  level: 29
  slots_of_30: 0.4
- monster: '[[monsters/91-honey-rackie|Honey Rackie]]'
  level: 24
  slots_of_30: 0.2
- monster: '[[monsters/172-aqua-warrior|Aqua Warrior]]'
  level: 31
  slots_of_30: 0.2
- monster: '[[monsters/205-woopie-king|Woopie King]]'
  level: 25
  slots_of_30: 0.4
sold_by:
- '[[npcs/1011-eccentric-inventor-spero|Eccentric Inventor Spero]]'
source:
  data: LIST_WEAPON.STB row 232
  code: module/src/items.rs
---
# Air Gun

A Gun operated with an air powered piston to fire small plastic BBs.
