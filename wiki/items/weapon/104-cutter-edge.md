---
kind: item
id: weapon/104
name: Cutter Edge
status: in-game
icon: item/1043
class: Two-Handed Sword
level: 38
attack: 55
attack_speed: 10
range: 2.5
damage: physical
two_handed: true
durability: 43
quality: 44
needs: Strength 80
sockets: true
price: 8640
weight: 25
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 2'
recipe:
- material: any Metal
  quantity: 28
- material: '[[items/material/51-tough-cloth|Tough Cloth]]'
  quantity: 5
- material: '[[items/material/176-insect-leg|Insect Leg]]'
  quantity: 4
craft_difficulty: 22
dropped_by:
- monster: '[[monsters/51-smouly|Smouly]]'
  level: 44
  slots_of_30: 0.2
- monster: '[[monsters/114-captain-moldie|Captain Moldie]]'
  level: 42
  slots_of_30: 0.4
- monster: '[[monsters/134-grunter|Grunter]]'
  level: 42
  slots_of_30: 0.2
- monster: '[[monsters/181-small-clown|Small Clown]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/207-aqua-king|Aqua King]]'
  level: 40
  slots_of_30: 0.4
- monster: '[[monsters/271-goblin-worker|Goblin Worker]]'
  level: 51
  slots_of_30: 0.2
sold_by:
- '[[npcs/1008-weapon-seller-raffle|Weapon Seller Raffle]]'
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 104
  code: module/src/items.rs
---
# Cutter Edge

Two-Handed Sword that specializes in cutting.
