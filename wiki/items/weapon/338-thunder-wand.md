---
kind: item
id: weapon/338
name: Thunder Wand
status: in-game
icon: item/1333
class: Magic Tool
level: 73
attack: 83
attack_speed: 12
range: 20
damage: magic
two_handed: false
durability: 49
quality: 53
needs: Intelligence 134
bonus: MP Consumption +11
sockets: true
price: 59600
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 5'
recipe:
- material: any Wooden Material
  quantity: 100
- material: '[[items/material/3-copper|Copper]]'
  quantity: 50
- material: '[[items/material/183-thick-leaf|Thick Leaf]]'
  quantity: 18
craft_difficulty: 41
dropped_by:
- monster: '[[monsters/166-master-stone-golem|Master Stone Golem]]'
  level: 80
  slots_of_30: 0.4
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
  level: 89
  slots_of_30: 0.2
- monster: '[[monsters/331-slag|Slag]]'
  level: 78
  slots_of_30: 0.2
- monster: '[[monsters/332-elder-slag|Elder Slag]]'
  level: 82
  slots_of_30: 0.2
- monster: '[[monsters/372-basilisk-captain|Basilisk Captain]]'
  level: 128
  slots_of_30: 0.6
sold_by:
- '[[npcs/1091-arumic-merchant-chester|Arumic Merchant Chester]]'
source:
  data: LIST_WEAPON.STB row 338
  code: module/src/items.rs
---
# Thunder Wand

A Mage's Wand that can mystically discharge lightning.
