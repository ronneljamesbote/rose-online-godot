---
kind: item
id: weapon/309
name: Anima Staff
status: in-game
icon: item/1295
class: Magic Staff
level: 80
attack: 105
attack_speed: 10
range: 3
damage: magic
two_handed: true
durability: 50
quality: 61
needs: Intelligence 135
bonus: MP Consumption +22
sockets: true
price: 82350
weight: 25
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 5'
recipe:
- material: any Wooden Material
  quantity: 120
- material: '[[items/material/4-bronze|Bronze]]'
  quantity: 40
- material: '[[items/material/42-thin-leather|Thin Leather]]'
  quantity: 20
- material: '[[items/material/127-misian|Misian]]'
  quantity: 25
craft_difficulty: 44
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 0.2
- monster: '[[monsters/281-goblin-mage|Goblin Mage]]'
  level: 88
  slots_of_30: 0.4
- monster: '[[monsters/336-wolf|Wolf]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/339-lunar-wolf|Lunar Wolf]]'
  level: 90
  slots_of_30: 0.2
- monster: '[[monsters/341-yeti|Yeti]]'
  level: 94
  slots_of_30: 0.4
- monster: '[[monsters/342-yeti-hunter|Yeti Hunter]]'
  level: 95
  slots_of_30: 0.4
- monster: '[[monsters/389-wolf-keeper|Wolf Keeper]]'
  level: 112
  slots_of_30: 0.2
- monster: '[[monsters/451-seal-stone|Seal Stone]]'
  level: 80
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 309
  code: module/src/items.rs
---
# Anima Staff

A Mage's Staff that contains the magic power of Anima.
