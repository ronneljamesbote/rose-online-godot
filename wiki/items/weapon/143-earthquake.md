---
kind: item
id: weapon/143
name: Earthquake
status: in-game
icon: item/1131
class: Two-Handed Axe
level: 117
attack: 212
attack_speed: 12
range: 2.5
damage: physical
two_handed: true
durability: 59
quality: 81
needs: Strength 227
sockets: true
price: 224350
weight: 40
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 9'
recipe:
- material: any Metal
  quantity: 130
- material: '[[items/material/36-pine-wood|Pine Wood]]'
  quantity: 40
- material: '[[items/material/14-molive|Molive]]'
  quantity: 25
- material: '[[items/material/130-lusian|Lusian]]'
  quantity: 7
craft_difficulty: 61
dropped_by:
- monster: '[[monsters/371-basilisk|Basilisk]]'
  level: 121
  slots_of_30: 0.4
- monster: '[[monsters/372-basilisk-captain|Basilisk Captain]]'
  level: 128
  slots_of_30: 0.6
- monster: '[[monsters/455-seal-stone|Seal Stone]]'
  level: 120
  slots_of_30: 1
- monster: '[[monsters/456-seal-stone|Seal Stone]]'
  level: 128
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 143
  code: module/src/items.rs
---
# Earthquake

An Axe filled with the power of the earth, making it capable of starting earthquakes.
