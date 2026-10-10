---
kind: item
id: weapon/340
name: Recovery Wand
status: in-game
icon: item/1335
class: Magic Tool
level: 89
attack: 107
attack_speed: 12
range: 20
damage: magic
two_handed: false
durability: 52
quality: 60
needs: Intelligence 160
bonus: MP Consumption +13
sockets: true
price: 115900
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 6'
recipe:
- material: any Wooden Material
  quantity: 130
- material: '[[items/material/4-bronze|Bronze]]'
  quantity: 70
- material: '[[items/material/61-low-essence|Low Essence]]'
  quantity: 20
- material: '[[items/material/127-misian|Misian]]'
  quantity: 20
craft_difficulty: 49
dropped_by:
- monster: '[[monsters/286-master-goblin|Master Goblin]]'
  level: 100
  slots_of_30: 0.4
- monster: '[[monsters/344-yeti-captain|Yeti Captain]]'
  level: 105
  slots_of_30: 0.4
- monster: '[[monsters/346-ruper-mage|Ruper Mage]]'
  level: 98
  slots_of_30: 0.4
- monster: '[[monsters/361-frostworm|FrostWorm]]'
  level: 109
  slots_of_30: 0.2
- monster: '[[monsters/362-elder-frostworm|Elder FrostWorm]]'
  level: 115
  slots_of_30: 0.2
source:
  data: LIST_WEAPON.STB row 340
  code: module/src/items.rs
---
# Recovery Wand

A magical Wand that contains restorative energies.
