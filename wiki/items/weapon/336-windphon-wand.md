---
kind: item
id: weapon/336
name: Windphon Wand
status: in-game
icon: item/1331
class: Magic Tool
level: 57
attack: 62
attack_speed: 12
range: 18
damage: magic
two_handed: false
durability: 47
quality: 47
needs: Intelligence 109
bonus: MP Consumption +9
sockets: true
price: 29000
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 3'
recipe:
- material: any Wooden Material
  quantity: 63
- material: '[[items/material/2-tin|Tin]]'
  quantity: 25
- material: '[[items/material/189-predator-claw|Predator Claw]]'
  quantity: 13
craft_difficulty: 34
dropped_by:
- monster: '[[monsters/144-kaiman-ranger|Kaiman Ranger]]'
  level: 61
  slots_of_30: 0.6
- monster: '[[monsters/155-doonga-warrior|Doonga Warrior]]'
  level: 64
  slots_of_30: 0.4
- monster: '[[monsters/156-doonga-hunter|Doonga Hunter]]'
  level: 62
  slots_of_30: 0.2
- monster: '[[monsters/157-doonga-leader|Doonga Leader]]'
  level: 70
  slots_of_30: 0.2
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 336
  code: module/src/items.rs
---
# Windphon Wand

A divine Wand that can call forth the wind.
