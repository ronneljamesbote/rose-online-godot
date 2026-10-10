---
kind: item
id: weapon/261
name: Wooden Launcher
status: in-game
icon: item/1248
class: Launcher
level: 30
attack: 54
attack_speed: 12
range: 25
damage: physical
two_handed: true
durability: 39
quality: 44
needs: Strength 28
price: 12720
weight: 20
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 2'
recipe:
- material: any Metal
  quantity: 38
- material: '[[items/material/51-tough-cloth|Tough Cloth]]'
  quantity: 20
- material: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  quantity: 5
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/51-smouly|Smouly]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/53-old-smouly|Old Smouly]]'
  level: 48
  slots_of_30: 0.2
- monster: '[[monsters/111-moldie|Moldie]]'
  level: 37
  slots_of_30: 0.4
- monster: '[[monsters/113-gold-mine-moldie|Gold Mine Moldie]]'
  level: 39
  slots_of_30: 0.2
- monster: '[[monsters/114-captain-moldie|Captain Moldie]]'
  level: 42
  slots_of_30: 0.2
- monster: '[[monsters/136-grunter-warrior|Grunter Warrior]]'
  level: 45
  slots_of_30: 0.2
- monster: '[[monsters/175-aqua-ranger|Aqua Ranger]]'
  level: 32
  slots_of_30: 0.2
- monster: '[[monsters/176-aqua-hunter|Aqua Hunter]]'
  level: 33
  slots_of_30: 0.4
- monster: '[[monsters/256-needle-bat|Needle Bat]]'
  level: 46
  slots_of_30: 0.2
sold_by:
- '[[npcs/1011-eccentric-inventor-spero|Eccentric Inventor Spero]]'
source:
  data: LIST_WEAPON.STB row 261
  code: module/src/items.rs
---
# Wooden Launcher

A simple Launcher constructed out of wood.
