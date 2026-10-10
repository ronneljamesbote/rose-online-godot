---
kind: item
id: weapon/243
name: Piercing Gun
status: in-game
icon: item/1222
class: Gun
level: 109
attack: 179
attack_speed: 11
range: 27
damage: physical
two_handed: true
durability: 55
quality: 77
needs: Concentration 181
sockets: true
price: 207800
weight: 30
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 8'
recipe:
- material: any Metal
  quantity: 140
- material: '[[items/material/45-tender-leather|Tender Leather]]'
  quantity: 35
- material: '[[items/material/34-oak-wood|Oak Wood]]'
  quantity: 20
- material: '[[items/material/128-hersian|Hersian]]'
  quantity: 3
craft_difficulty: 59
dropped_by:
- monster: '[[monsters/365-vulcan|Vulcan]]'
  level: 118
  slots_of_30: 0.2
- monster: '[[monsters/372-basilisk-captain|Basilisk Captain]]'
  level: 128
  slots_of_30: 0.4
- monster: '[[monsters/375-dreadnaught|Dreadnaught]]'
  level: 128
  slots_of_30: 0.4
- monster: '[[monsters/378-mammoth|Mammoth]]'
  level: 115
  slots_of_30: 0.2
- monster: '[[monsters/379-elder-mammoth|Elder Mammoth]]'
  level: 126
  slots_of_30: 0.4
- monster: '[[monsters/455-seal-stone|Seal Stone]]'
  level: 120
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 243
  code: module/src/items.rs
---
# Piercing Gun

A Gun with the power to pierce absolutely anything.
