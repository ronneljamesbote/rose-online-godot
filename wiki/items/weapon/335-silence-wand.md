---
kind: item
id: weapon/335
name: Silence Wand
status: in-game
icon: item/1330
class: Magic Tool
level: 48
attack: 51
attack_speed: 12
range: 18
damage: magic
two_handed: false
durability: 45
quality: 43
needs: Intelligence 94
bonus: MP Consumption +8
sockets: true
price: 16785
weight: 5
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 3'
recipe:
- material: any Wooden Material
  quantity: 48
- material: '[[items/material/2-tin|Tin]]'
  quantity: 20
- material: '[[items/material/178-sticky-liquid|Sticky Liquid]]'
  quantity: 11
craft_difficulty: 30
dropped_by:
- monster: '[[monsters/141-kaiman|Kaiman]]'
  level: 59
  slots_of_30: 0.2
- monster: '[[monsters/151-doonga|Doonga]]'
  level: 56
  slots_of_30: 0.2
- monster: '[[monsters/208-guardian-tree|Guardian Tree]]'
  level: 57
  slots_of_30: 0.4
- monster: '[[monsters/258-elder-bloodbat|Elder BloodBat]]'
  level: 60
  slots_of_30: 0.2
- monster: '[[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]]'
  level: 55
  slots_of_30: 0.4
- monster: '[[monsters/274-goblin-server|Goblin Server]]'
  level: 52
  slots_of_30: 0.4
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 335
  code: module/src/items.rs
---
# Silence Wand

A magic Wand that contains the power to bring silence to its surroundings.
