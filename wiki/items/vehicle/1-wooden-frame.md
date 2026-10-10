---
kind: item
id: vehicle/1
name: Wooden Frame
status: in-game
icon: item/2165
class: Cart Body
vehicle: cart
part: body
durability: 34
quality: 30
bonus: Defense +26, Magic Resistance +12
price: 197000
weight: 10
crafted_with: '[[skills/2591-cart-craft|Cart Craft]] level 1'
recipe:
- material: any Wooden Material
  quantity: 80
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 20
- material: '[[items/material/201-steam-oil|Steam Oil]]'
  quantity: 10
- material: '[[items/material/211-wooden-cart-schematic|Wooden Cart Schematic]]'
  quantity: 1
craft_difficulty: 30
sold_by:
- '[[npcs/1096-ferrell-guild-merchant-mildun|Ferrell Guild Merchant Mildun]]'
source:
  data: LIST_PAT.STB row 1
  code: module/src/items.rs
---
# Wooden Frame

A basic frame, the first and only one available during the days when the Cart was just invented.
