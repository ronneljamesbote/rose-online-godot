---
kind: item
id: weapon/342
name: Holy Wand
status: in-game
icon: item/1337
class: Magic Tool
level: 103
attack: 130
attack_speed: 12
range: 22
damage: magic
two_handed: false
durability: 55
quality: 65
needs: Intelligence 182
bonus: MP Consumption +14
sockets: true
price: 193450
weight: 10
crafted_with: '[[skills/2491-magic-weapon-craft|Magic Weapon Craft]] level 7'
recipe:
- material: any Wooden Material
  quantity: 150
- material: '[[items/material/5-iron|Iron]]'
  quantity: 90
- material: '[[items/material/63-low-ether|Low Ether]]'
  quantity: 30
- material: '[[items/material/14-molive|Molive]]'
  quantity: 10
craft_difficulty: 57
dropped_by:
- monster: '[[monsters/356-tyrant-warship|Tyrant WarShip]]'
  level: 114
  slots_of_30: 1
- monster: '[[monsters/361-frostworm|FrostWorm]]'
  level: 109
  slots_of_30: 0.4
- monster: '[[monsters/362-elder-frostworm|Elder FrostWorm]]'
  level: 115
  slots_of_30: 0.4
- monster: '[[monsters/379-elder-mammoth|Elder Mammoth]]'
  level: 126
  slots_of_30: 0.2
- monster: '[[monsters/454-seal-stone|Seal Stone]]'
  level: 108
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 342
  code: module/src/items.rs
---
# Holy Wand

A magic Wand that flows with the power of holiness.
