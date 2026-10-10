---
kind: item
id: vehicle/102
name: Second Engine
status: in-game
icon: item/2205
class: Cart Engine
vehicle: cart
part: engine
move_speed: 104
max_fuel: 1800
defence: 1800
durability: 34
quality: 34
price: 151200
weight: 5
crafted_with: '[[skills/2591-cart-craft|Cart Craft]] level 1'
recipe:
- material: any Metal
  quantity: 150
- material: '[[items/material/202-steam-heat|Steam Heat]]'
  quantity: 30
- material: '[[items/material/62-essence|Essence]]'
  quantity: 15
- material: '[[items/material/221-cart-engine-schematic|Cart Engine Schematic]]'
  quantity: 1
craft_difficulty: 35
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_PAT.STB row 102
  code: module/src/items.rs
---
# Second Engine

An improvement of the First Engine that provides more power.
