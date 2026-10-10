---
kind: item
id: weapon/239
name: Beretta
status: in-game
icon: item/1217
class: Gun
level: 80
attack: 122
attack_speed: 11
range: 25
damage: physical
two_handed: true
durability: 50
quality: 64
needs: Concentration 139
sockets: true
price: 73900
weight: 30
crafted_with: '[[skills/1211-damage-support|Damage Support]] level 5'
recipe:
- material: any Metal
  quantity: 80
- material: '[[items/material/41-worn-out-leather|Worn-out Leather]]'
  quantity: 15
- material: '[[items/material/102-sharp-iron-piece|Sharp Iron Piece]]'
  quantity: 8
- material: '[[items/material/126-lesian|Lesian]]'
  quantity: 1
craft_difficulty: 45
dropped_by:
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
  level: 89
  slots_of_30: 0.2
- monster: '[[monsters/451-seal-stone|Seal Stone]]'
  level: 80
  slots_of_30: 1
- monster: '[[monsters/452-seal-stone|Seal Stone]]'
  level: 88
  slots_of_30: 1
source:
  data: LIST_WEAPON.STB row 239
  code: module/src/items.rs
---
# Beretta

A Gun with enough power to exterminate zombies.
