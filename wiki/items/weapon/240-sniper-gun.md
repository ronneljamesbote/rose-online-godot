---
kind: item
id: weapon/240
name: Sniper Gun
status: in-game
icon: item/1218
class: Gun
level: 88
attack: 137
attack_speed: 11
range: 25
damage: physical
two_handed: true
durability: 52
quality: 68
needs: Concentration 150
sockets: true
price: 98400
weight: 30
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 6'
recipe:
- material: any Metal
  quantity: 100
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 20
- material: '[[items/material/102-sharp-iron-piece|Sharp Iron Piece]]'
  quantity: 10
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 2
craft_difficulty: 49
dropped_by:
- monster: '[[monsters/336-wolf|Wolf]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/339-lunar-wolf|Lunar Wolf]]'
  level: 90
  slots_of_30: 0.2
- monster: '[[monsters/340-shadow-wolf|Shadow Wolf]]'
  level: 97
  slots_of_30: 0.2
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.2
- monster: '[[monsters/389-wolf-keeper|Wolf Keeper]]'
  level: 112
  slots_of_30: 0.2
- monster: '[[monsters/390-wolf-keeper|Wolf Keeper]]'
  level: 117
  slots_of_30: 0.2
source:
  data: LIST_WEAPON.STB row 240
  code: module/src/items.rs
---
# Sniper Gun

A rifle that is well suited to accurately targeting enemies from a great distance.
