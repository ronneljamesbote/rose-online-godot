---
kind: item
id: weapon/169
name: Halberd
status: in-game
icon: item/1087
class: Spear
level: 74
attack: 104
attack_speed: 9
range: 4
damage: physical
two_handed: true
durability: 49
quality: 52
needs: Strength 129
sockets: true
price: 48550
weight: 30
crafted_with: '[[skills/2451-mace-craft|Mace Craft]] level 5'
recipe:
- material: any Metal
  quantity: 77
- material: '[[items/material/34-oak-wood|Oak Wood]]'
  quantity: 21
- material: '[[items/material/102-sharp-iron-piece|Sharp Iron Piece]]'
  quantity: 5
craft_difficulty: 42
dropped_by:
- monster: '[[monsters/163-master-golem|Master Golem]]'
  level: 84
  slots_of_30: 0.2
- monster: '[[monsters/166-master-stone-golem|Master Stone Golem]]'
  level: 80
  slots_of_30: 0.2
- monster: '[[monsters/201-worm-dragon|Worm Dragon]]'
  level: 88
  slots_of_30: 0.2
- monster: '[[monsters/277-gem-goblin-guard|Gem Goblin Guard]]'
  level: 85
  slots_of_30: 0.2
- monster: '[[monsters/279-gem-goblin-warrior|Gem Goblin Warrior]]'
  level: 89
  slots_of_30: 0.2
- monster: '[[monsters/284-goblin-leader|Goblin Leader]]'
  level: 77
  slots_of_30: 0.2
- monster: '[[monsters/287-grandmaster-goblin|Grandmaster Goblin]]'
  level: 105
  slots_of_30: 0.4
- monster: '[[monsters/451-seal-stone|Seal Stone]]'
  level: 80
  slots_of_30: 1
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 169
  code: module/src/items.rs
---
# Halberd

A versatile Spear that can be used to chop, stab and slice.
