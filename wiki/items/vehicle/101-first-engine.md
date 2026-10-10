---
kind: item
id: vehicle/101
name: First Engine
status: in-game
icon: item/2204
class: Cart Engine
vehicle: cart
part: engine
move_speed: 100
max_fuel: 1600
defence: 1600
durability: 32
quality: 30
price: 116100
weight: 5
crafted_with: '[[skills/2591-cart-craft|Cart Craft]] level 1'
recipe:
- material: any Metal
  quantity: 100
- material: '[[items/material/201-steam-oil|Steam Oil]]'
  quantity: 25
- material: '[[items/material/61-low-essence|Low Essence]]'
  quantity: 10
- material: '[[items/material/221-cart-engine-schematic|Cart Engine Schematic]]'
  quantity: 1
craft_difficulty: 30
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_PAT.STB row 101
  code: module/src/items.rs
---
# First Engine

Basically, an upgraded Cart Engine for Castle Gears.
