---
kind: item
id: weapon/211
name: Mythril Bow
status: in-game
icon: item/1167
class: Bow
level: 88
attack: 135
attack_speed: 10
range: 24
damage: physical
two_handed: true
durability: 52
quality: 66
needs: Dexterity 151
sockets: true
price: 76600
weight: 20
crafted_with: '[[skills/2471-bow-craft|Bow Craft]] level 6'
recipe:
- material: any Metal
  quantity: 85
- material: '[[items/material/44-tough-leather|Tough Leather]]'
  quantity: 20
- material: '[[items/material/54-high-durability-cloth|High Durability Cloth]]'
  quantity: 15
- material: '[[items/material/127-misian|Misian]]'
  quantity: 4
craft_difficulty: 49
dropped_by:
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
- monster: '[[monsters/346-ruper-mage|Ruper Mage]]'
  level: 98
  slots_of_30: 0.4
- monster: '[[monsters/389-wolf-keeper|Wolf Keeper]]'
  level: 112
  slots_of_30: 0.2
source:
  data: LIST_WEAPON.STB row 211
  code: module/src/items.rs
---
# Mythril Bow

A Bow made of Mythril whose quality is guaranteed after purchase.
