---
kind: item
id: weapon/308
name: Golden Staff
status: in-game
icon: item/1294
class: Magic Staff
level: 72
attack: 93
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 49
quality: 57
needs: Intelligence 123
bonus: MP Consumption +21
sockets: true
price: 60400
weight: 25
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 4'
recipe:
- material: any Wooden Material
  quantity: 100
- material: '[[items/material/3-copper|Copper]]'
  quantity: 35
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 15
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 20
craft_difficulty: 40
dropped_by:
- monster: '[[monsters/166-master-stone-golem|Master Stone Golem]]'
  level: 80
  slots_of_30: 0.2
- monster: '[[monsters/284-goblin-leader|Goblin Leader]]'
  level: 77
  slots_of_30: 0.2
- monster: '[[monsters/286-master-goblin|Master Goblin]]'
  level: 100
  slots_of_30: 0.4
- monster: '[[monsters/331-slag|Slag]]'
  level: 78
  slots_of_30: 0.2
- monster: '[[monsters/332-elder-slag|Elder Slag]]'
  level: 82
  slots_of_30: 0.2
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 308
  code: module/src/items.rs
---
# Golden Staff

An everlasting Staff constructed out of pure gold.
