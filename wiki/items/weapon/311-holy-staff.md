---
kind: item
id: weapon/311
name: Holy Staff
status: in-game
icon: item/1297
class: Magic Staff
level: 95
attack: 131
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 53
quality: 67
needs: Intelligence 158
bonus: MP Consumption +23
sockets: true
price: 139600
weight: 30
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 7'
recipe:
- material: any Wooden Material
  quantity: 160
- material: '[[items/material/5-iron|Iron]]'
  quantity: 55
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 30
- material: '[[items/material/14-molive|Molive]]'
  quantity: 15
craft_difficulty: 52
dropped_by:
- monster: '[[monsters/343-yeti-guard|Yeti Guard]]'
  level: 101
  slots_of_30: 0.4
- monster: '[[monsters/344-yeti-captain|Yeti Captain]]'
  level: 105
  slots_of_30: 0.4
- monster: '[[monsters/346-ruper-mage|Ruper Mage]]'
  level: 98
  slots_of_30: 0.4
- monster: '[[monsters/347-ruper-wizard|Ruper Wizard]]'
  level: 102
  slots_of_30: 0.4
- monster: '[[monsters/351-yeti-rider|Yeti Rider]]'
  level: 100
  slots_of_30: 0.2
- monster: '[[monsters/352-rider-wizard|Rider Wizard]]'
  level: 104
  slots_of_30: 0.2
- monster: '[[monsters/453-seal-stone|Seal Stone]]'
  level: 100
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 311
  code: module/src/items.rs
---
# Holy Staff

A clergyman's Staff which contains the power of holiness.
