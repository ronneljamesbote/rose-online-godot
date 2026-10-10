---
kind: item
id: weapon/105
name: Surado
status: in-game
icon: item/1044
class: Two-Handed Sword
level: 47
attack: 67
attack_speed: 10
range: 2.5
damage: physical
two_handed: true
durability: 45
quality: 48
needs: Strength 95
sockets: true
price: 15210
weight: 25
crafted_with: '[[skills/2431-sword-craft|Sword Craft]] level 3'
recipe:
- material: any Metal
  quantity: 38
- material: '[[items/material/51-tough-cloth|Tough Cloth]]'
  quantity: 6
- material: '[[items/material/179-weed-root|Weed Root]]'
  quantity: 5
craft_difficulty: 26
dropped_by:
- monster: '[[monsters/138-grunter-leader|Grunter Leader]]'
  level: 53
  slots_of_30: 0.4
- monster: '[[monsters/141-kaiman|Kaiman]]'
  level: 59
  slots_of_30: 0.2
- monster: '[[monsters/151-doonga|Doonga]]'
  level: 56
  slots_of_30: 0.2
- monster: '[[monsters/161-jewel-golem|Jewel Golem]]'
  level: 47
  slots_of_30: 0.2
- monster: '[[monsters/183-fighter-clown|Fighter Clown]]'
  level: 52
  slots_of_30: 0.2
- monster: '[[monsters/257-bloodbat|BloodBat]]'
  level: 50
  slots_of_30: 0.4
- monster: '[[monsters/272-coal-mine-goblin-worker|Coal Mine Goblin Worker]]'
  level: 55
  slots_of_30: 0.2
sold_by:
- '[[npcs/1093-weapon-merchant-crune|Weapon Merchant Crune]]'
source:
  data: LIST_WEAPON.STB row 105
  code: module/src/items.rs
---
# Surado

A basic Two-Handed Sword.
